use crate::constants::*;
use crate::types::*;
use crate::utils::*;

/// Quadbin cell validation
pub fn is_valid_cell(cell: Cell) -> bool {
    let cell64 = cell.get();
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
pub(crate) fn cell_to_tile(cell: Cell) -> Tile {
    assert!(is_valid_cell(cell), "Quadbin is not valid");

    let cell64 = cell.get();
    let z = cell64 >> 52 & 31;
    let q = (cell64 & FOOTER) << 12;
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

    Tile::new(x as u32, y as u32, z as u8)
}

/// Convert a geographic point into a cell.
pub fn point_to_cell(longitude: f64, latitude: f64, resolution: u8) -> Option<u64> {
    let long = clip_longitude(longitude);
    let lat = clip_latitude(latitude);

    let tile = point_to_tile(long, lat, resolution);

    tile_to_cell(Some(&tile))
}

/// Convert cell into point
pub fn cell_to_point(cell: Cell) -> Option<(f64, f64)> {
    // TODO:
    // Replace with proper Error
    if !is_valid_cell(cell) {
        return None;
    }

    let tile = cell_to_tile(cell);
    let lat = Tile::to_latitude(&tile, 0.5);
    let lon = Tile::to_longitude(&tile, 0.5);

    Some((lon, lat))
}

/// Compute the parent cell for a specific resolution.
pub(crate) fn cell_to_parent(cell: Cell, parent_resolution: u8) -> Cell {
    // Check resolution
    let resolution = cell.resolution();
    assert!(
        parent_resolution < resolution,
        "parent resolution should be greater than current resolution"
    );

    let result = (cell.get() & !(0x1F << 52))
        | ((parent_resolution as u64) << 52)
        | (FOOTER >> ((parent_resolution as u64) << 1));

    Cell::new(result)
}
