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
