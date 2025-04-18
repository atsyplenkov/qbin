use approx::assert_relative_eq;
use quadbin::utils::*;

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
    assert_relative_eq!(new_lat, 45.08903556483104_f64, epsilon = 1e-10);
    assert_relative_eq!(new_lon, lon, epsilon = 1e-10);

    // Check offset with approximate equality
    let new_lon_offset = tile_to_longitude(&tile, 0.5);
    let new_lat_offset = tile_to_latitude(&tile, 0.5);
    assert_relative_eq!(new_lat_offset, 44.96479793033102_f64, epsilon = 1e-10);
    assert_relative_eq!(new_lon_offset, -44.82421875_f64, epsilon = 1e-10);
}
