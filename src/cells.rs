use crate::constants::*;
use crate::types::*;
use crate::utils::*;

/// Quadbin cell validation
pub(crate) fn is_valid_cell(cell64: u64) -> bool {
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

// Internal functions ------------------------------------------------

/// Convert a tile into a Quadbin cell.
pub(crate) fn tile_to_cell(tile: Tile) -> Cell {
    let mut x = tile.x as u64;
    let mut y = tile.y as u64;
    let z = tile.z as u64;

    x <<= 32 - z;
    y <<= 32 - z;

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

    let cell = HEADER | (1 << 59) | (z << 52) | ((x | (y << 1)) >> 12) | (FOOTER >> (z * 2));
    Cell::new(cell)
}

/// Convert Quadbin cell into a tile
pub(crate) fn cell_to_tile(cell: Cell) -> Tile {
    assert!(cell.is_valid(), "Quadbin cell index is not valid");

    let cell64 = cell.get();
    let z = (cell64 >> 52) & 31;
    let q = (cell64 & FOOTER) << 12;
    let mut x = q;
    let mut y = q >> 1;

    x &= B[0];
    y &= B[0];

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

    x >>= 32 - z;
    y >>= 32 - z;

    Tile::new(x as u32, y as u32, z as u8)
}

/// Convert a geographic point into a cell.
pub(crate) fn point_to_cell(lng: f64, lat: f64, res: u8) -> Cell {
    let lng = clip_longitude(lng);
    let lat = clip_latitude(lat);

    let tile = Tile::from_point(lng, lat, res);

    tile.to_cell()
}

/// Convert cell into point
pub(crate) fn cell_to_point(cell: Cell) -> (f64, f64) {
    assert!(cell.is_valid(), "Quadbin cell index is not valid");

    let tile = cell.to_tile();
    let lat = tile.to_latitude(0.5);
    let lon = tile.to_longitude(0.5);

    (lon, lat)
}

/// Compute the parent cell for a specific resolution.
pub(crate) fn cell_to_parent(cell: Cell, parent_res: u8) -> Cell {
    // Check resolution
    let resolution = cell.resolution();
    assert!(
        parent_res < resolution,
        "parent resolution should be greater than current resolution"
    );

    let result = (cell.get() & !(0x1F << 52))
        | ((parent_res as u64) << 52)
        | (FOOTER >> ((parent_res as u64) << 1));

    Cell::new(result)
}
