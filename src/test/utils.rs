use crate::types::Tile;
use crate::utils::{point_to_tile_fraction, tile_scalefactor};
use approx::assert_relative_eq;

// Declare accuracy for float values comparison on various OS
const ACC: f64 = 1e-10;

// Tests inspired by `quadbin-py` library
// See https://github.com/CartoDB/quadbin-py/blob/master/tests/unit/test_utils.py
#[test]
fn test_point_to_tile_fraction() {
    let tile = point_to_tile_fraction(-95.93965530395508_f64, 41.26000108568697_f64, 9_u8);
    assert_relative_eq!(tile.0, 119.552490234375_f64, epsilon = ACC);
    assert_relative_eq!(tile.1, 191.47119140625_f64, epsilon = ACC);
    assert_eq!(tile.2, 9_u8);
}

#[test]
fn test_point_to_tile() {
    // X axis
    assert_eq!(Tile::from_point(-180.0, 0.0, 0), Tile::new(0, 0, 0));
    assert_eq!(Tile::from_point(-180.0, 85.0, 2), Tile::new(0, 0, 2));
    assert_eq!(Tile::from_point(180.0, 85.0, 2), Tile::new(0, 0, 2));
    assert_eq!(Tile::from_point(-185.0, 85.0, 2), Tile::new(3, 0, 2));
    assert_eq!(Tile::from_point(185.0, 85.0, 2), Tile::new(0, 0, 2));

    // Y-axis
    assert_eq!(Tile::from_point(-175.0, -95.0, 2), Tile::new(0, 3, 2));
    assert_eq!(Tile::from_point(-175.0, 95.0, 2), Tile::new(0, 0, 2));
}

// Estimate tile's area
// TODO:
// Investigate why it differs from Python's version.
// I have the impression that Python's code is erratic.
#[test]
fn test_tile_area() {
    let cases = [
        (Tile::new(0, 0, 0), 508164597540055.75_f64),
        (Tile::new(1, 0, 1), 127516518279497.11_f64),
        (Tile::new(0, 1, 1), 127516518279497.11_f64),
        (Tile::new(0, 0, 2), 3731444586048.1396_f64),
        (Tile::new(46, 3584, 12), 3366113.9540235824_f64),
        (Tile::new(8108, 14336, 14), 210619.87609208928_f64),
        (Tile::new(8108, 14336, 23), 0.17313075165235314_f64),
        (Tile::new(8108, 14336, 26), 0.0026549956100831765_f64),
    ];

    for (tile, expected) in cases.iter() {
        assert_relative_eq!(Tile::area(tile), *expected, epsilon = ACC);
    }
}

// Additional tests
#[test]
fn test_tile_conversion() {
    // TODO:
    // - Add more test cases
    // - Add tests for invalid inputs

    let lon = -45.0_f64;
    let lat = 45.0_f64;
    let tile = Tile::from_point(lon, lat, 10);

    // Check Tile conversion
    assert_eq!(tile.x, 384_u32);
    assert_eq!(tile.y, 368_u32);
    assert_eq!(tile.z, 10_u8);

    // Convert back to coordinates
    let new_lon = Tile::to_longitude(&tile, 0.0);
    let new_lat = Tile::to_latitude(&tile, 0.0);

    // Check conversion with approximate equality
    assert_relative_eq!(new_lat, 45.08903556483104_f64, epsilon = ACC);
    assert_relative_eq!(new_lon, lon, epsilon = ACC);

    // Check offset with approximate equality
    let new_lon_offset = Tile::to_longitude(&tile, 0.5);
    let new_lat_offset = Tile::to_latitude(&tile, 0.5);
    assert_relative_eq!(new_lat_offset, 44.96479793033102_f64, epsilon = ACC);
    assert_relative_eq!(new_lon_offset, -44.82421875_f64, epsilon = ACC);
}

#[test]
fn test_tile_scalefactor() {
    assert_relative_eq!(
        tile_scalefactor(&Tile::new(384, 368, 10)),
        0.7075410884638627_f64,
        epsilon = ACC
    );
    assert_relative_eq!(
        tile_scalefactor(&Tile::new(384, 368, 26)),
        0.08626970361752928_f64,
        epsilon = ACC
    );
    assert_relative_eq!(
        tile_scalefactor(&Tile::new(100, 100, 10)),
        0.15910754230624527_f64,
        epsilon = ACC
    );
}

// Find tiles neighbours (aka siblings)
#[test]
fn test_tile_sibling() {
    // Test zoom level 0 (should always return None)
    assert_eq!(Tile::get_sibling(&Tile::new(0, 0, 0), 0), None); // UP
    assert_eq!(Tile::get_sibling(&Tile::new(0, 0, 0), 1), None); // RIGHT
    assert_eq!(Tile::get_sibling(&Tile::new(0, 0, 0), 2), None); // LEFT
    assert_eq!(Tile::get_sibling(&Tile::new(0, 0, 0), 3), None); // DOWN

    // Test UP direction (0)
    let up_cases = [
        (Tile::new(1, 0, 2), None),
        (Tile::new(2, 3, 3), Some(Tile::new(2, 2, 3))),
    ];

    for (tile, expected) in up_cases.iter() {
        assert_eq!(Tile::get_sibling(tile, 0), *expected);
    }

    // Test RIGHT direction (1)
    let right_cases = [
        (Tile::new(3, 1, 2), None),
        (Tile::new(8108, 14336, 14), Some(Tile::new(8109, 14336, 14))),
    ];

    for (tile, expected) in right_cases.iter() {
        assert_eq!(Tile::get_sibling(tile, 1), *expected);
    }

    // Test LEFT direction (2)
    let left_cases = [
        (Tile::new(0, 1, 2), None),
        (Tile::new(5, 5, 3), Some(Tile::new(4, 5, 3))),
    ];

    for (tile, expected) in left_cases.iter() {
        assert_eq!(Tile::get_sibling(tile, 2), *expected);
    }

    // Test DOWN direction (3)
    let down_cases = [
        (Tile::new(1, 3, 2), None),
        (Tile::new(7, 2, 3), Some(Tile::new(7, 3, 3))),
    ];

    for (tile, expected) in down_cases.iter() {
        assert_eq!(Tile::get_sibling(tile, 3), *expected);
    }

    // Test invalid direction
    assert_eq!(Tile::get_sibling(&Tile::new(1, 1, 2), 4), None);
    assert_eq!(Tile::get_sibling(&Tile::new(1, 1, 2), 255), None);
}
