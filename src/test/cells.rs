use crate::cells::*;
use crate::types::Tile;

// Test validiy of Quadbin cell indexes
#[test]
fn test_is_cell_valid() {
    let cases = [
        (5209574053332910079_u64, true),
        (5192650370358181887_u64, true),
        (5202361257054699519_u64, true),
        (5291729562728627583_u64, true),
        (0_u64, false),
        (5209574053332910078_u64, false),
        (6362495557939757055_u64, false),
    ];

    for (cell, expected) in cases.iter() {
        assert_eq!(is_valid_cell(*cell), *expected);
    }
}

// Validation test from original CARTO's `quadbin-js`
// https://github.com/CartoDB/quadbin-js/blob/40cce2fc6b9dc72bf19c69ffb6705f8b73d24b2c/test/index.spec.ts#L30-L34
#[test]
fn test_tile_and_cell_conversion() {
    // Test early return
    assert_eq!(tile_to_cell(None), None);

    // Test real examples
    // TODO:
    // Add more cases
    let cases = [
        (9_u32, 8_u32, 4_u8, 5209574053332910079_u64),
        (0_u32, 0_u32, 0_u8, 5192650370358181887_u64),
        (1_u32, 2_u32, 3_u8, 5202361257054699519_u64),
        (1023_u32, 2412_u32, 23_u8, 5291729562728627583_u64),
    ];

    // Tile to cell conversion
    for (x, y, z, cell) in cases.iter() {
        assert_eq!(tile_to_cell(Some(&Tile::new(*x, *y, *z))), Some(*cell));
    }

    // Cell to tile conversion
    for (x, y, z, cell) in cases.iter() {
        assert_eq!(cell_to_tile(*cell), Some(Tile::new(*x, *y, *z)));
    }
}

// Validation test from original CARTO's documentation:
// See https://docs.carto.com/data-and-analysis/analytics-toolbox-for-snowflake/sql-reference/quadbin#quadbin_fromgeogpoint
// and
// https://github.com/CartoDB/quadbin-py?tab=readme-ov-file#usage
#[test]
fn test_point_to_cell() {
    // TODO:
    // Add tests for invalid resolution
    let cases = [
        (-3.7038, 40.4168, 10_u8, 5234261499580514303_u64),
        (-3.7038, 40.4168, 4_u8, 5207251884775047167_u64),
        (33.75, -11.178401873711776, 4_u8, 5209574053332910079_u64),
        (0.0, 85.05112877980659, 26_u8, 5306366260949286912_u64),
        (0.0, 88.0, 26_u8, 5306366260949286912_u64),
        (0.0, 90.0, 26_u8, 5306366260949286912_u64),
        (0.0, -85.05112877980659, 26_u8, 5309368660700867242_u64),
        (0.0, -88.0, 26_u8, 5309368660700867242_u64),
        (0.0, -90.0, 26_u8, 5309368660700867242_u64),
    ];

    for (x, y, res, cell) in cases.iter() {
        assert_eq!(point_to_cell(*x, *y, *res), Some(*cell));
    }
}

// Convert quadbin cell back to coords
#[test]
fn test_cell_to_point() {
    assert_eq!(
        cell_to_point(5209574053332910079_u64),
        Some((33.75, -11.178401873711776))
    )
}

// Get cell resolution
#[test]
fn test_get_cell_resolution() {
    assert_eq!(cell_resolution(5209574053332910079_u64), 4_u8)
}
