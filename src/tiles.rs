use crate::Direction;
use crate::cells::*;
use crate::errors::InvalidCell;
use crate::utils::*;

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
    pub fn to_cell(self) -> Result<Cell, InvalidCell> {
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
