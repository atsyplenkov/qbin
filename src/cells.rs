use crate::constants::*;
use crate::types::Tile;
use crate::utils::*;

/// Quadbin cell validation
pub fn is_valid_cell(cell: u64) -> bool {
    let header = HEADER;
    let mode = (cell >> 59) & 7;
    let resolution = (cell >> 52) & 0x1F;
    let resolution_shift = resolution.saturating_mul(4);
    let unused = if resolution_shift >= 64 {
        0
    } else {
        FOOTER >> resolution_shift
    };

    // Checks
    (cell & header == header) && mode == 1 && resolution <= 26 && (cell & unused == unused)
}

/// Get resolution of an Quadbin cell
pub fn cell_resolution(cell: u64) -> u8 {
    ((cell >> 52) & 0x1F) as u8
}

/// Convert a tile into a Quadbin cell.
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

/// Convert Quadbin cell into a tile
pub fn cell_to_tile(cell: u64) -> Option<Tile> {
    // TODO:
    // Replace with proper Error
    if !is_valid_cell(cell) {
        return None;
    }
    let z = cell >> 52 & 31;
    let q = (cell & FOOTER) << 12;
    let mut x = q;
    let mut y = q >> 1;

    x = x & B[0];
    y = y & B[0];

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

    x = x >> (32 - z);
    y = y >> (32 - z);

    Some(Tile::new(x as u32, y as u32, z as u8))
}

/// Convert a geographic point into a cell.
pub fn point_to_cell(longitude: f64, latitude: f64, resolution: u8) -> Option<u64> {
    let long = clip_longitude(longitude);
    let lat = clip_latitude(latitude);

    let tile = point_to_tile(long, lat, resolution);

    tile_to_cell(Some(&tile))
}

/// Convert cell into point
pub fn cell_to_point(cell: u64) -> Option<(f64, f64)> {
    // TODO:
    // Replace with proper Error
    if !is_valid_cell(cell) {
        return None;
    }

    let tile = cell_to_tile(cell)?;
    let lat = tile_to_latitude(&tile, 0.5);
    let lon = tile_to_longitude(&tile, 0.5);

    Some((lon, lat))
}

/// Compute the parent cell for a specific resolution.
pub fn cell_to_parent(cell: u64, parent_resolution: u8) -> Option<u64> {
    let resolution = cell_resolution(cell);

    // TODO:
    // Replace with Error
    if parent_resolution > resolution {
        return None;
    }

    let result = (cell & !(0x1F << 52))
        | ((parent_resolution as u64) << 52)
        | (FOOTER >> ((parent_resolution as u64) << 1));

    Some(result)
}

/// Approximate area of a cell in square meters.
pub fn cell_area(cell: u64) -> Option<f64> {
    let tile = cell_to_tile(cell)?;
    Some(tile_area(&tile))
}
