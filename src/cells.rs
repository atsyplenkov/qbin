use crate::Direction;
use crate::constants::*;
use crate::errors;
use crate::errors::InvalidCell;
use crate::errors::InvalidDirection;
use crate::tiles::*;
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
    pub fn new(value: u64) -> Result<Self, InvalidCell> {
        Cell::try_from(value)
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
    pub fn parent(&self, parent_res: u8) -> Result<Cell, InvalidCell> {
        cell_to_parent(self, parent_res)
    }

    // TODO:
    // Add child and/or children

    // /// Find the Cell's neighbor in a specific [Direction].
    // ///
    // /// In the original JavaScript implementation, this operation is called
    // /// sibling. However, following the H3 naming convention, we decided
    // /// to name sibling's as neighbors.
    // ///
    // /// See [Direction] for allowed arguments.
    // ///
    // /// # Example
    // /// ```
    // /// use qbin::{Cell, Direction};
    // ///
    // /// let sibling = Cell::new(5209574053332910079).neighbor(Direction::Right);
    // /// assert_eq!(sibling, Some(Cell::new(5209626829891043327)));
    // /// ```
    // pub fn neighbor(&self, direction: Direction) -> Result<Cell, InvalidCell, InvalidDirection> {
    //     let tile = self.to_tile().neighbor(direction);
    //     tile.map(Tile::to_cell)
    // }

    // /// Find the Cell's sibling in a specific [Direction].
    // ///
    // /// See [Cell::neighbor].
    // pub fn sibling(&self, direction: Direction) -> Result<Cell, InvalidCell, InvalidDirection> {
    //     let tile = self.to_tile().neighbor(direction);
    //     tile.to_cell()
    // }

    // /// List all Cell's neighbors.
    // pub fn neighbors(&self) -> [Option<Cell>; 4] {
    //     let mut neighbors = [None; 4];

    //     for (i, neighbor) in neighbors.iter_mut().enumerate() {
    //         *neighbor = self.neighbor(Direction::new_unchecked(i as u8));
    //     }

    //     neighbors
    // }

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
    pub fn from_point(lat: f64, lng: f64, res: u8) -> Result<Cell, InvalidCell> {
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

impl TryFrom<u64> for Cell {
    type Error = errors::InvalidCell;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if !is_valid_cell(value) {
            return Err(Self::Error::new(
                Some(value),
                "Provided Quadbin Cell index is invalid",
            ));
        }

        Ok(Self(NonZeroU64::new(value).expect("non-zero cell index")))
    }
}

// TODO:
// Detect direction from neighbor https://github.com/HydroniumLabs/h3o/blob/ad2bebf52eab218d66b0bf213b14a2802bf616f7/src/base_cell.rs#L135C1-L150C6

// Internal functions ------------------------------------------------
/// Quadbin cell validation
fn is_valid_cell(cell64: u64) -> bool {
    let header = HEADER;
    let mode = (cell64 >> 59) & 7;
    let resolution = (cell64 >> 52) & 0x1F;
    let resolution_shift = resolution.saturating_mul(4);
    let unused = if resolution_shift >= 64 {
        0
    } else {
        FOOTER >> resolution_shift
    };

    // Checks
    (cell64 & header == header) && mode == 1 && resolution <= 26 && (cell64 & unused == unused)
}

/// Convert a tile into a Quadbin cell.
pub(crate) fn tile_to_cell(tile: Tile) -> Result<Cell, InvalidCell> {
    let mut x = tile.x as u64;
    let mut y = tile.y as u64;
    let z = tile.z as u64;

    x <<= 32 - z;
    y <<= 32 - z;

    x = (x | (x << S[4])) & B[4];
    y = (y | (y << S[4])) & B[4];

    x = (x | (x << S[3])) & B[3];
    y = (y | (y << S[3])) & B[3];

    x = (x | (x << S[2])) & B[2];
    y = (y | (y << S[2])) & B[2];

    x = (x | (x << S[1])) & B[1];
    y = (y | (y << S[1])) & B[1];

    x = (x | (x << S[0])) & B[0];
    y = (y | (y << S[0])) & B[0];

    let cell = HEADER | (1 << 59) | (z << 52) | ((x | (y << 1)) >> 12) | (FOOTER >> (z * 2));
    Cell::try_from(cell)
}

/// Convert Quadbin cell into a tile
fn cell_to_tile(cell: &Cell) -> Tile {
    assert!(cell.is_valid(), "Quadbin cell index is not valid");

    let cell64 = cell.get();
    let z = (cell64 >> 52) & 31;
    let q = (cell64 & FOOTER) << 12;
    let mut x = q;
    let mut y = q >> 1;

    x &= B[0];
    y &= B[0];

    x = (x | (x >> S[0])) & B[1];
    y = (y | (y >> S[0])) & B[1];

    x = (x | (x >> S[1])) & B[2];
    y = (y | (y >> S[1])) & B[2];

    x = (x | (x >> S[2])) & B[3];
    y = (y | (y >> S[2])) & B[3];

    x = (x | (x >> S[3])) & B[4];
    y = (y | (y >> S[3])) & B[4];

    x = (x | (x >> S[4])) & B[5];
    y = (y | (y >> S[4])) & B[5];

    x >>= 32 - z;
    y >>= 32 - z;

    Tile::new(x as u32, y as u32, z as u8)
}

/// Convert a geographic point into a cell.
fn point_to_cell(lat: f64, lng: f64, res: u8) -> Result<Cell, InvalidCell> {
    let lng = clip_longitude(lng);
    let lat = clip_latitude(lat);

    let tile = Tile::from_point(lat, lng, res);

    tile.to_cell()
}

/// Convert cell into point
fn cell_to_point(cell: &Cell) -> [f64; 2] {
    assert!(cell.is_valid(), "Quadbin cell index is not valid");

    let tile = cell.to_tile();
    let lat = tile.to_latitude(0.5);
    let lon = tile.to_longitude(0.5);

    // Return array, not tuple, as it more memory efficient
    // See https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#the-array-type
    [lat, lon]
}

/// Compute the parent cell for a specific resolution.
fn cell_to_parent(cell: &Cell, parent_res: u8) -> Result<Cell, InvalidCell> {
    // Check resolution
    let resolution = cell.resolution();
    assert!(
        parent_res < resolution,
        "parent resolution should be greater than current resolution"
    );

    let result = (cell.get() & !(0x1F << 52))
        | ((parent_res as u64) << 52)
        | (FOOTER >> ((parent_res as u64) << 1));

    Cell::try_from(result)
}
