use crate::Direction;
use crate::cells::*;
use crate::utils::*;
use core::{fmt, num::NonZeroU64};

/// Represents a cell in the Quadbin grid system at a
/// particular resolution.
///
/// The index is encoded on 64-bit with the following bit layout:
///
/// ```text
///  ┏━┳━━━┳━━━━┳━━━━━━━┳━━━━━━━━━━━┈┈┈┈┈┈┈┈━━━━━━━━┓
///  ┃U┃ H ┃ M  ┃   R   ┃    XY in Morton order     ┃
///  ┗━┻━━━┻━━━━┻━━━━━━━┻━━━━━━━━━━━┈┈┈┈┈┈┈┈━━━━━━━━┛
///  63  62   59    56    52                        0
/// ```
///
/// Where:
/// - `U`: Unused reserved bit (bit 63), always set to `0`;
/// - `H`: Header bit (bit 62), always set to `1`;
/// - `M`: Index mode, fixed to `1`, encoded over 4 bits (bits 59–62);
/// - `R`: Cell resolution, ranging from `0` to `26`, encoded in bits 52–56;
/// - Remaining bits (0–51) encode the cell’s XY position in Morton order (Z-order curve).
///
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Cell(NonZeroU64);

impl Cell {
    /// Returns the inner u64 value of the cell.
    pub fn get(&self) -> u64 {
        self.0.get()
    }

    /// Create new Quadbin cell from index.
    ///
    /// # Example
    /// ```
    /// let qb_cell = qbin::Cell::new(5234261499580514303);
    /// ```
    pub fn new(value: u64) -> Cell {
        assert!(
            is_valid_cell(value),
            "Provided Quadbin Cell index is invalid"
        );
        Cell(NonZeroU64::new(value).expect("non-zero cell index"))
    }

    /// Quadbin cell index validation.
    ///
    /// # Example
    /// ```
    /// let qb_cell = qbin::Cell::new(5234261499580514303);
    /// assert_eq!(qb_cell.is_valid(), true)
    /// ```
    pub fn is_valid(&self) -> bool {
        is_valid_cell(self.get())
    }

    /// Returns the resolution of the cell index.
    ///
    /// # Example
    /// ```
    /// let qb_cell = qbin::Cell::new(5234261499580514303);
    /// let res = qb_cell.resolution();
    /// assert_eq!(res, 10)
    /// ```
    pub fn resolution(&self) -> u8 {
        ((self.0.get() >> 52) & 0x1F) as u8
    }

    /// Compute the parent cell for a specific resolution.
    ///
    /// # Example
    /// ```
    /// let qb_cell = qbin::Cell::new(5209574053332910079);
    /// let parent = qb_cell.parent(2_u8);
    /// assert_eq!(parent, qbin::Cell::new(5200813144682790911))
    /// ```
    pub fn parent(&self, parent_res: u8) -> Cell {
        cell_to_parent(self, parent_res)
    }

    // TODO:
    // Add child and/or children

    /// Find the Cell's neighbor in a specific [Direction].
    ///
    /// In the original JavaScript implementation, this operation is called
    /// sibling. However, following the H3 naming convention, we decided
    /// to name sibling's as neighbors.
    ///
    /// See [Direction] for allowed arguments.
    ///
    /// # Example
    /// ```
    /// use qbin::{Cell, Direction};
    ///
    /// let sibling = Cell::new(5209574053332910079).neighbor(Direction::Right);
    /// assert_eq!(sibling, Some(Cell::new(5209626829891043327)));
    /// ```
    pub fn neighbor(&self, direction: Direction) -> Option<Self> {
        let tile = self.to_tile().neighbor(direction);
        tile.map(Tile::to_cell)
    }

    /// Find the Cell's sibling in a specific [Direction].
    ///
    /// See [Cell::neighbor].
    pub fn sibling(&self, direction: Direction) -> Option<Self> {
        let tile = self.to_tile().neighbor(direction);
        tile.map(Tile::to_cell)
    }

    /// List all Cell's neighbors.
    pub fn neighbors(&self) -> [Option<Cell>; 4] {
        let mut neighbors = [None; 4];

        for (i, neighbor) in neighbors.iter_mut().enumerate() {
            *neighbor = self.neighbor(Direction::new_unchecked(i as u8));
        }

        neighbors
    }

    // TODO:
    // Add `direction_to_neighbor` -- return Direction to neighbor

    /// Computes the area of this Quadbin cell, in m².
    ///
    /// See also [Cell::area_km2].
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    ///
    /// let area = qbin::Cell::new(5234261499580514303_u64).area_m2();
    /// assert_relative_eq!(area, 888546364.7859862, epsilon = 1e-6)
    ///
    /// ```
    pub fn area_m2(&self) -> f64 {
        self.to_tile().area()
    }

    /// Computes the area of this Quadbin cell, in km².
    ///
    /// See also [Cell::area_m2].
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    ///
    /// let area = qbin::Cell::new(5234261499580514303_u64).area_km2();
    /// assert_relative_eq!(area, 888.5463647859862, epsilon = 1e-6)
    ///
    /// ```
    pub fn area_km2(&self) -> f64 {
        self.area_m2() / 1_000_000_f64
    }

    /// Convert a Quadbin cell into geographic point.
    ///
    /// Returns a tuple with latitude and longitude in degrees.
    ///
    /// # Example
    /// ```
    /// use qbin::Cell;
    ///
    /// let coords = Cell::new(5209574053332910079_u64).to_point();
    /// assert_eq!(coords, [-11.178401873711776, 33.75]);
    /// ```
    ///
    pub fn to_point(&self) -> [f64; 2] {
        cell_to_point(self)
    }

    /// Convert a Quadbin cell into a bounding box.
    ///
    /// Returns an array with [xmin, ymin, xmax, ymax]
    /// in degrees.
    ///
    /// # Example
    /// ```
    /// let bbox = qbin::Cell::new(5209574053332910079).to_bbox();
    /// assert_eq!( bbox, [22.5, -21.943045533438166, 45.0, 0.0])
    /// ```
    pub fn to_bbox(&self) -> [f64; 4] {
        let tile = self.to_tile();

        let xmin = tile.to_longitude(0.0);
        let xmax = tile.to_longitude(1.0);
        let ymin = tile.to_latitude(1.0);
        let ymax = tile.to_latitude(0.0);

        [xmin, ymin, xmax, ymax]
    }

    /// Convert a geographic point into a Quadbin cell.
    ///
    /// # Example
    ///
    /// ```
    /// let cell = qbin::Cell::from_point(-41.28303675124842, 174.77727344223067, 26);
    /// assert_eq!(cell.get(), 5309133744805926483_u64)
    /// ```
    pub fn from_point(lat: f64, lng: f64, res: u8) -> Cell {
        point_to_cell(lat, lng, res)
    }

    /// Convert a Quadbin cell into a tile.
    pub(crate) fn to_tile(self) -> Tile {
        cell_to_tile(&self)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

// TODO:
// Detect direction from neighbor https://github.com/HydroniumLabs/h3o/blob/ad2bebf52eab218d66b0bf213b14a2802bf616f7/src/base_cell.rs#L135C1-L150C6

// --------------------------------------------------------

/// A single tile coordinates
///
/// _Internal struct_
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) struct Tile {
    pub x: u32,
    pub y: u32,
    pub z: u8,
}

impl Tile {
    /// Create a new tile.
    pub fn new(x: u32, y: u32, z: u8) -> Tile {
        Tile { x, y, z }
    }

    /// Convert to Quadbin cell.
    pub fn to_cell(self) -> Cell {
        tile_to_cell(self)
    }

    /// Compute the tile for a longitude and latitude in a specific resolution.
    pub fn from_point(lat: f64, lng: f64, res: u8) -> Self {
        point_to_tile(lat, lng, res)
    }

    /// Approximate tile area in square meters.
    pub fn area(&self) -> f64 {
        tile_area(self)
    }

    /// Return tile's latitude.
    ///
    /// See also [Tile::to_longitude].
    ///
    pub fn to_latitude(self, offset: f64) -> f64 {
        tile_to_latitude(&self, offset)
    }

    /// Return tile's longitude.
    ///
    /// See also [Tile::to_latitude].
    ///
    pub fn to_longitude(self, offset: f64) -> f64 {
        tile_to_longitude(&self, offset)
    }

    /// Get tile's siblings.
    pub fn neighbor(&self, direction: Direction) -> Option<Self> {
        tile_neighbor(self, direction)
    }

    /// Compute a hash from the tile.
    #[allow(dead_code)]
    pub fn to_hash(self) -> u64 {
        to_tile_hash(&self)
    }

    /// Compute a tile from the hash.
    #[allow(dead_code)]
    pub fn from_hash(tile_hash: u64) -> Tile {
        from_tile_hash(tile_hash)
    }
}
