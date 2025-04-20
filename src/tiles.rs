use crate::types::Tile;

/// Compute a hash from the tile.
pub fn to_tile_hash(tile: &Tile) -> u64 {
    let x = tile.x as u64;
    let y = tile.y as u64;
    let z = tile.z as u64;

    let dim = 2 * (1 << z);

    let hash = ((dim * y + x) * 32) + z;

    hash as u64
}

/// Compute a tile from the hash.
pub fn from_tile_hash(tile_hash: u64) -> Tile {
    // TODO:
    // Return None if hash is invalid
    let z = tile_hash % 32_u64;
    let dim = 2_u64 * (1_u64 << z);
    let xy = (tile_hash - z) / 32;
    let x = xy % dim;
    let y = ((xy - x) / dim) % dim;

    Tile::new(x as u32, y as u32, z as u8)
}
