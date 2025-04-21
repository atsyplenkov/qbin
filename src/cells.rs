use crate::constants::*;
use crate::types::Tile;
use crate::utils::*;

/// Convert a tile into a cell.
pub fn tile_to_cell(tile: Option<&Tile>) -> Option<u64> {
    let tile = tile?;
    let mut x = tile.x as u64;
    let mut y = tile.y as u64;
    let z = tile.z as u64;

    x = x << (32 - z);
    y = y << (32 - z);

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

    Some(HEADER | (1 << 59) | ((z as u64) << 52) | ((x | (y << 1)) >> 12) | (FOOTER >> (z * 2)))
}

/// Convert a geographic point into a cell.
pub fn point_to_cell(longitude: f64, latitude: f64, resolution: u8) -> Option<u64> {
    let long = clip_longitude(longitude);
    let lat = clip_latitude(latitude);

    let tile = point_to_tile(long, lat, resolution);

    tile_to_cell(Some(&tile))
}
