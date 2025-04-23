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

// Direction for sibling detection
// enum Direction {
//     Up,
//     Right,
//     Left,
//     Down,
// }

impl Tile {
    /// Create a new tile.
    ///
    /// # Examples
    ///
    /// ```
    /// use quadbin::types::Tile;
    ///
    /// let tile = Tile::new(8108, 14336, 14);
    /// ```
    pub fn new(x: u32, y: u32, z: u8) -> Tile {
        Tile { x, y, z }
    }

    /// Compute the tile for a longitude and latitude in a specific resolution.
    ///
    /// # Examples
    /// ```
    /// use quadbin::types::Tile;
    /// // Create a tile from geographic coordinates:
    /// let tile = Tile::from_point(-175.0, 95.0, 2);
    /// assert_eq!(tile, Tile::new(0, 0, 2));
    /// ```
    pub fn from_point(longitude: f64, latitude: f64, resolution: u8) -> Self {
        point_to_tile(longitude, latitude, resolution)
    }

    /// Approximate tile area in square meters
    ///
    /// # Examples
    /// ```
    /// use quadbin::types::Tile;
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

    /// Return tile's latitude
    ///
    /// # Examples
    /// ```
    /// use quadbin::types::Tile;
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
    /// # Examples
    /// ```
    /// use quadbin::types::Tile;
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

    // /// Get tile's siblings
    // pub fn get_sibling(&self, direction: &Direction) -> Option<Self> {
    //     tile_sibling(&self, direction)
    // }

    /// Compute a hash from the tile.
    pub fn to_hash(&self) -> u64 {
        to_tile_hash(&self)
    }

    /// Compute a tile from the hash.
    pub fn from_hash(tile_hash: u64) -> Tile {
        from_tile_hash(tile_hash)
    }
}
