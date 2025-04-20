/// A single tile coordinates
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub z: u8,
}

impl Tile {
    pub fn new(x: u32, y: u32, z: u8) -> Tile {
        Tile { x, y, z }
    }
}