use quadbin::utils::*;

#[test]
fn test_tile_conversion() {

    let lon = -45.0f64;
    let lat = 45.0f64;
    let tile = point_to_tile(lon, lat, 10);

    // Check Tile conversion
    assert_eq!(tile.x, 384usize);
    assert_eq!(tile.y, 368usize);
    assert_eq!(tile.z, 10u8);

    // Convert back to coordinates
    let new_lon = tile_to_longitude(&tile, 0.0);
    let new_lat = tile_to_latitude(&tile, 0.0);

    // Check conversion
    assert_eq!(new_lat, 45.08903556483104f64);
    assert_eq!(new_lon, lon);

    // Check offset
    let new_lon_offset = tile_to_longitude(&tile, 0.5);
    let new_lat_offset = tile_to_latitude(&tile, 0.5);
    assert_eq!(new_lat_offset, 44.96479793033102f64);
    assert_eq!(new_lon_offset, -44.82421875f64);
}
