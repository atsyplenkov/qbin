use crate::cells::*;
use crate::utils::*;

/// A single tile coordinates
// TODO:
// Add explanation of x, y and z
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub z: u8,
}

impl Tile {
    /// Create a new tile.
    ///
    /// # Examples
    ///
    /// ```
    /// let tile = quadbin::Tile::new(8108, 14336, 14);
    /// ```
    pub fn new(x: u32, y: u32, z: u8) -> Tile {
        Tile { x, y, z }
    }

    /// Compute the tile for a longitude and latitude in a specific resolution.
    ///
    /// # Examples
    /// ```
    /// use quadbin::Tile;
    /// // Create a tile from geographic coordinates:
    /// let tile = Tile::from_point(-175.0, 95.0, 2);
    /// assert_eq!(tile, Tile::new(0, 0, 2));
    /// ```
    pub fn from_point(longitude: f64, latitude: f64, resolution: u8) -> Self {
        point_to_tile(longitude, latitude, resolution)
    }

    /// Approximate tile area in square meters.
    ///
    /// # Examples
    /// ```
    /// use quadbin::Tile;
    /// use approx::assert_relative_eq;
    ///
    /// // Create new tile
    /// let tile = Tile::new(8108, 14336, 14);
    /// // Estimate tile's area in m2
    /// let area = Tile::area(&tile);
    /// assert_relative_eq!(area, 210619.87609208928_f64, epsilon = 1e-10);
    /// ```
    pub fn area(&self) -> f64 {
        tile_area(&self)
    }

    /// Return tile's latitude.
    ///
    /// See also [Tile::to_longitude].
    ///
    /// # Examples
    /// ```
    /// use quadbin::Tile;
    /// use approx::assert_relative_eq;
    ///
    /// // Create new tile
    /// let tile = Tile::new(8108, 14336, 14);
    /// // Retrieve tile's latitude
    /// let lat = Tile::to_latitude(&tile, 0.0);
    /// assert_relative_eq!(lat, -79.17133464081944_f64, epsilon = 1e-10);
    /// ```
    pub fn to_latitude(&self, offset: f64) -> f64 {
        tile_to_latitude(&self, offset)
    }

    /// Return tile's longitude.
    ///
    /// See also [Tile::to_latitude].
    ///
    /// # Examples
    /// ```
    /// use quadbin::Tile;
    /// use approx::assert_relative_eq;
    ///
    /// // Create new tile
    /// let tile = Tile::new(8108, 14336, 14);
    /// // Retrieve tile's latitude
    /// let lat = Tile::to_longitude(&tile, 0.0);
    /// assert_relative_eq!(lat, -1.845703125_f64, epsilon = 1e-10);
    /// ```
    pub fn to_longitude(&self, offset: f64) -> f64 {
        tile_to_longitude(&self, offset)
    }

    /// Get tile's siblings.
    // TODO:
    // Add examples. See how to properly document direction
    pub fn get_sibling(&self, direction: u8) -> Option<Self> {
        tile_sibling(&self, direction)
    }

    /// Compute a hash from the tile.
    pub fn to_hash(&self) -> u64 {
        to_tile_hash(&self)
    }

    /// Compute a tile from the hash.
    pub fn from_hash(tile_hash: u64) -> Tile {
        from_tile_hash(tile_hash)
    }
}

// --------------------------------------------------------

/// Represents a cell in the Quadbin grid system at a
/// particular resolution.
///
/// The index is encoded on 64-bit with the following bit layout:
///
/// ```text
///  ┏━┳━━━┳━━━━┳━━━━━━━┳━━━━━━━━━━━┈┈┈┈┈┈┈┈━━━━━━━━┓
///  ┃U┃ H ┃ M  ┃   R   ┃           Morton          ┃
///  ┗━┻━━━┻━━━━┻━━━━━━━┻━━━━━━━━━━━┈┈┈┈┈┈┈┈━━━━━━━━┛
/// 64 63   59   52     51                          0
/// ```
///
/// Where:
/// - `U` are unused reserved bit, always set to 0 (bit 63).
/// - `H` is the header bit (bit 62, always 1).
/// - `M` is the index mode, always set to 1, coded on 4 bits (59-62).
/// - `R` is the cell resolution, in [0; 26], coded on bits 52-56.
/// - Remaining bits encode the cell's XY position in Morton order (0-51).
///
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Cell(u64); // TODO: replace with NonZeroU64

impl Cell {
    /// Create new
    ///
    /// # Example
    /// ```
    /// let qb_cell = quadbin::Cell::new(5234261499580514303);
    /// ```
    pub fn new(cell: u64) -> Cell {
        Cell(cell)
    }

    /// Returns the resolution of the index.
    ///
    /// # Example
    /// ```
    /// let qb_cell = quadbin::Cell::new(5234261499580514303);
    /// let res = qb_cell.resolution();
    /// assert_eq!(res, 10)
    /// ```
    pub fn resolution(self) -> u8 {
        cell_resolution(self.0)
    }
}
