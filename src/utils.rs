use crate::constants::*;
use crate::types::Tile;
use std::f64::consts::PI;

/// Clip a value between a minimum and maximum value
pub fn clip_number(num: f64, lower: f64, upper: f64) -> f64 {
    num.max(lower).min(upper)
}

/// Limit longitude bounds.
pub fn clip_longitude(longitude: f64) -> f64 {
    clip_number(longitude, MIN_LONGITUDE, MAX_LONGITUDE)
}

/// Limit latitude bounds.
pub fn clip_latitude(latitude: f64) -> f64 {
    clip_number(latitude, MIN_LATITUDE, MAX_LATITUDE)
}

/// Compute the tile in fractions for a longitude and latitude in a
/// specific resolution.
pub fn point_to_tile_fraction(longitude: f64, latitude: f64, resolution: u8) -> (f64, f64, u8) {
    // Check resolution to avoid overflow
    if resolution > MAX_RESOLUTION || resolution < MIN_RESOLUTION {
        panic!(
            "Resolution should be between {} and {}",
            MIN_RESOLUTION, MAX_RESOLUTION
        );
    }

    // Compute tile coordinates
    let z2: f64 = (1 << resolution) as f64;
    let sinlat = f64::sin(latitude * PI / 180.0);
    let x = z2 * (longitude / 360.0 + 0.5);
    let yfraction = 0.5 - 0.25 * ((1.0 + sinlat) / (1.0 - sinlat)).ln() / PI;
    let y = clip_number(z2 * yfraction, 0.0, z2 - 1.0);

    let x = x % z2;
    let x = if x < 0.0 { x + z2 } else { x };

    // Return the tile coordinates
    (x, y, resolution)
}

/// Compute the tile for a longitude and latitude in a specific resolution.
pub fn point_to_tile(longitude: f64, latitude: f64, resolution: u8) -> Tile {
    let (x, y, z) = point_to_tile_fraction(longitude, latitude, resolution);
    let x: usize = x.floor() as usize;
    let y: usize = y.floor() as usize;
    Tile::new(x, y, z)
}

/// Compute the latitude for a tile with an offset.
pub fn tile_to_latitude(tile: &Tile, offset: f64) -> f64 {
    // Check if offset is between 0 and 1
    if offset < 0.0 || offset > 1.0 {
        panic!("Offset should be between 0 and 1");
    }

    // Get Tile coords
    let y = tile.y as f64;
    let z2 = (1 << tile.z) as f64;

    // Compute latitude
    let expy = f64::exp(-(2.0 * (y + offset) / z2 - 1.0) * PI);
    360.0 * (f64::atan(expy) / PI - 0.25)
}

pub fn tile_to_longitude(tile: &Tile, offset: f64) -> f64 {
    // Check if offset is between 0 and 1
    if offset < 0.0 || offset > 1.0 {
        panic!("Offset should be between 0 and 1");
    }

    // Get Tile coords
    let x = tile.x as f64;
    let z2 = (1 << tile.z) as f64;

    // Compute longitude
    180.0 * (2.0 * (x + offset) / z2 - 1.0)
}
