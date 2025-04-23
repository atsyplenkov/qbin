use crate::cells::*;
use crate::types::*;
use approx::assert_relative_eq;

// Test validiy of Quadbin cell indexes
#[test]
fn test_is_cell_valid() {
    let cases = [
        (5209574053332910079_u64, true),
        (5192650370358181887_u64, true),
        (5202361257054699519_u64, true),
        (5291729562728627583_u64, true),
        (5209574053332910078_u64, false),
        (6362495557939757055_u64, false),
    ];

    for (cell, expected) in cases.iter() {
        assert_eq!(is_valid_cell(Cell::new(*cell)), *expected);
    }
}

// Validation test from original CARTO's `quadbin-js`
// https://github.com/CartoDB/quadbin-js/blob/40cce2fc6b9dc72bf19c69ffb6705f8b73d24b2c/test/index.spec.ts#L30-L34
#[test]
fn test_tile_and_cell_conversion() {
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
        assert_eq!(Tile::new(*x, *y, *z).to_cell(), Cell::new(*cell));
    }

    // Cell to tile conversion
    for (x, y, z, cell) in cases.iter() {
        assert_eq!(Cell::new(*cell).to_tile(), Tile::new(*x, *y, *z));
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
        assert_eq!(Cell::from_point(*x, *y, *res), Cell::new(*cell));
    }
}

// Convert quadbin cell back to coords
#[test]
fn test_cell_to_point() {
    assert_eq!(
        Cell::new(5209574053332910079_u64).to_point(),
        (33.75, -11.178401873711776)
    )
}

// Get cell resolution
#[test]
fn test_get_cell_resolution() {
    let qb_cell = Cell::new(5209574053332910079_u64);
    assert_eq!(Cell::resolution(qb_cell), 4_u8)
}

// Get parent cell
#[test]
fn test_cell_to_parent() {
    let cases = [
        (5209574053332910079, 2, 5200813144682790911),
        (5209574053332910079, 0, 5192650370358181887),
    ];

    for (cell, res, parent) in cases.iter() {
        assert_eq!(Cell::new(*cell).parent(*res), Cell::new(*parent));
    }
}
#[test]
#[should_panic(expected = "parent resolution should be greater than current resolution")]
fn test_cell_to_parent_invalid_resolution() {
    let cell = Cell::new(5209574053332910079);
    let _ = cell.parent(4);
}

// Estimate cell area
#[test]
fn test_cell_area() {
    let area = Cell::new(5209574053332910079_u64).area_m2();
    assert_relative_eq!(area, 6023040823252.6641, epsilon = 1e-2);
}
