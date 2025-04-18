use approx::assert_relative_eq;
use quadbin::types::Tile;
use quadbin::utils::*;

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
    assert_eq!(point_to_tile(-180.0, 0.0, 0), Tile::new(0, 0, 0));
    assert_eq!(point_to_tile(-180.0, 85.0, 2), Tile::new(0, 0, 2));
    assert_eq!(point_to_tile(180.0, 85.0, 2), Tile::new(0, 0, 2));
    assert_eq!(point_to_tile(-185.0, 85.0, 2), Tile::new(3, 0, 2));
    assert_eq!(point_to_tile(185.0, 85.0, 2), Tile::new(0, 0, 2));

    // Y-axis
    assert_eq!(point_to_tile(-175.0, -95.0, 2), Tile::new(0, 3, 2));
    assert_eq!(point_to_tile(-175.0, 95.0, 2), Tile::new(0, 0, 2));
}

#[test]
fn test_tile_area() {
    assert_relative_eq!(
        tile_area(&Tile::new(8108, 14336, 14)),
        210619.87609208928_f64,
        epsilon = ACC
    );
}

// Additional tests
#[test]
fn test_tile_conversion() {
    // TODO:
    // - Add more test cases
    // - Add tests for invalid inputs

    let lon = -45.0_f64;
    let lat = 45.0_f64;
    let tile = point_to_tile(lon, lat, 10);

    // Check Tile conversion
    assert_eq!(tile.x, 384_usize);
    assert_eq!(tile.y, 368_usize);
    assert_eq!(tile.z, 10_u8);

    // Convert back to coordinates
    let new_lon = tile_to_longitude(&tile, 0.0);
    let new_lat = tile_to_latitude(&tile, 0.0);

    // Check conversion with approximate equality
    assert_relative_eq!(new_lat, 45.08903556483104_f64, epsilon = ACC);
    assert_relative_eq!(new_lon, lon, epsilon = ACC);

    // Check offset with approximate equality
    let new_lon_offset = tile_to_longitude(&tile, 0.5);
    let new_lat_offset = tile_to_latitude(&tile, 0.5);
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
