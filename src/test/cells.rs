use crate::cells::*;
use crate::types::Tile;

// Validation test from original CARTO's `quadbin-js`
// https://github.com/CartoDB/quadbin-js/blob/40cce2fc6b9dc72bf19c69ffb6705f8b73d24b2c/test/index.spec.ts#L30-L34
#[test]
fn test_tile_to_cell() {
    // Test early return
    assert_eq!(tile_to_cell(None), None);

    // Test real examples
    let cases = [
        (9_u32, 8_u32, 4_u8, 5209574053332910079_u64),
        (0_u32, 0_u32, 0_u8, 5192650370358181887_u64),
        (1_u32, 2_u32, 3_u8, 5202361257054699519_u64),
        (1023_u32, 2412_u32, 23_u8, 5291729562728627583_u64),
    ];

    for (x, y, z, cell) in cases.iter() {
        assert_eq!(tile_to_cell(Some(&Tile::new(*x, *y, *z))), Some(*cell));
    }
}

// Validation test from original CARTO's documentation:
// See https://docs.carto.com/data-and-analysis/analytics-toolbox-for-snowflake/sql-reference/quadbin#quadbin_fromgeogpoint
// and
// https://github.com/CartoDB/quadbin-py?tab=readme-ov-file#usage
#[test]
fn test_point_to_cell() {
    let cases = [
        ((-3.7038, 40.4168), 10_u8, 5234261499580514303_u64),
        ((-3.7038, 40.4168), 4_u8, 5207251884775047167_u64),
    ];

    for (coords, res, cell) in cases.iter() {
        assert_eq!(point_to_cell(coords.0, coords.1, *res), Some(*cell));
    }
}
