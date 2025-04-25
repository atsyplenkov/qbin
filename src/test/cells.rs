use crate::direction::Direction;
use crate::types::*;
use approx::assert_relative_eq;

// Constants to save some typing
const UP: Direction = Direction::Up;
const DOWN: Direction = Direction::Down;
const RIGHT: Direction = Direction::Right;
const LEFT: Direction = Direction::Left;

// Test validiy of Quadbin cell indexes
#[test]
fn test_is_cell_valid() {
    let cases = [
        (5209574053332910079_u64, true),
        (5192650370358181887_u64, true),
        (5202361257054699519_u64, true),
        (5291729562728627583_u64, true),
    ];

    for (cell, expected) in cases.iter() {
        assert_eq!(Cell::new(*cell).is_valid(), *expected);
    }
}

// Expect panic due to invalid cell index
#[test]
#[should_panic(expected = "Provided Quadbin Cell index is invalid")]
fn test_new_invalid_cell() {
    let _ = Cell::new(5209574053332910078_u64);
    let _ = Cell::new(6362495557939757055_u64);
}

// Validation test from original CARTO's `quadbin-js`
// https://github.com/CartoDB/quadbin-js/blob/40cce2fc6b9dc72bf19c69ffb6705f8b73d24b2c/test/index.spec.ts#L30-L34
#[test]
fn test_tile_and_cell_conversion() {
    // Test real examples
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
        assert_eq!(Cell::from_point(*y, *x, *res), Cell::new(*cell));
    }
}

// Convert quadbin cell to bounding box
#[test]
fn test_cell_to_bbox() {
    // Conversion works
    assert_eq!(
        Cell::new(5209574053332910079).to_bbox(),
        [22.5, -21.943045533438166, 45.0, 0.0]
    );

    // Coords are located in correct order
    let cases: [u64; 4] = [
        5211632339100106751,
        5212472365983727615,
        5226055182877458431,
        5264708239044902911,
    ];

    for i in cases.iter() {
        let bbox = Cell::new(*i).to_bbox();
        assert_eq!(bbox[0] < bbox[2], true);
        assert_eq!(bbox[1] < bbox[3], true);
    }
}

// Convert quadbin cell back to coords
#[test]
fn test_cell_to_point() {
    assert_eq!(
        Cell::new(5209574053332910079_u64).to_point(),
        [-11.178401873711776, 33.75]
    );

    let coords = Cell::new(5309133744805926483_u64).to_point();
    assert_relative_eq!(coords[0], -41.28303708488909, epsilon = 1e-6);
    assert_relative_eq!(coords[1], 174.77727502584457, epsilon = 1e-6)
}

// Get cell resolution
#[test]
fn test_get_cell_resolution() {
    let qb_cell = Cell::new(5209574053332910079_u64);
    assert_eq!(qb_cell.resolution(), 4_u8)
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

// Find cell's neighbors
// Identical to
// https://github.com/CartoDB/quadbin-py/blob/39a0adbb238ff214fbbca7b73200cfebf2aef38c/tests/unit/test_main.py#L203
#[test]
fn test_cell_neighbor() {
    assert_eq!(Cell::new(5192650370358181887).neighbor(UP), None);
    assert_eq!(Cell::new(5193776270265024511).neighbor(UP), None);
    assert_eq!(Cell::new(5194902170171867135).neighbor(UP), None);
    assert_eq!(Cell::new(5194902170171867135).neighbor(RIGHT), None);

    // Resolution 1
    assert_eq!(
        Cell::new(5193776270265024511).neighbor(DOWN),
        Some(Cell::new(5196028070078709759))
    );
    assert_eq!(
        Cell::new(5193776270265024511).neighbor(RIGHT),
        Some(Cell::new(5194902170171867135))
    );
    assert_eq!(
        Cell::new(5194902170171867135).neighbor(DOWN),
        Some(Cell::new(5197153969985552383))
    );
    assert_eq!(
        Cell::new(5194902170171867135).neighbor(LEFT),
        Some(Cell::new(5193776270265024511))
    );
    assert_eq!(
        Cell::new(5209574053332910079).neighbor(UP),
        Some(Cell::new(5208061125333090303))
    );

    // Resolution 4
    assert_eq!(
        Cell::new(5209574053332910079).neighbor(DOWN),
        Some(Cell::new(5209609237704998911))
    );
    assert_eq!(
        Cell::new(5209574053332910079).neighbor(LEFT),
        Some(Cell::new(5209556461146865663))
    );
    assert_eq!(
        Cell::new(5209574053332910079).neighbor(RIGHT),
        Some(Cell::new(5209626829891043327))
    );
}

// List all Cell's neighbors
#[test]
fn test_cell_neighbors() {
    let center_cells = [
        Cell::new(5209574053332910079),
        Cell::new(5194902170171867135),
        Cell::new(5192650370358181887),
        Cell::new(5201094619659501567),
    ];

    for i in center_cells.iter() {
        assert_eq!(
            i.neighbors(),
            [
                i.neighbor(UP),
                i.neighbor(RIGHT),
                i.neighbor(LEFT),
                i.neighbor(DOWN)
            ]
        )
    }

    // Test that None is returned alongside with Some(Cell)
    let nn = Cell::new(5201094619659501567).neighbors();
    assert_eq!(nn[1], None)
}
