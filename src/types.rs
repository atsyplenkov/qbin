use std::fmt;

/// A single tile coordinates
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub z: u8,
}

impl Tile {
    pub fn new(x: usize, y: usize, z: u8) -> Tile {
        Tile { x, y, z }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile {{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}
