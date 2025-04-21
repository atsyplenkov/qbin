use crate::constants::*;
use crate::types::Tile;
use std::f64::consts::PI;

// TODO:
// Make these functions available only in the crate
// Find a way to keep approx crate as a dev dependency only

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
        // TODO:
        // Replace with Result for better error handling
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
    let x: u32 = x.floor() as u32;
    let y: u32 = y.floor() as u32;
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

/// Compute the longitude for a tile with an offset.
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

/// Inverse of the scale factor at the tile center.
pub fn tile_scalefactor(tile: &Tile) -> f64 {
    // Get Tile coords
    let y = tile.y as f64;
    let z2 = (1 << tile.z) as f64;
    let y_offset = 0.5_f64;

    // Estimate scale factor
    f64::cos(2.0 * PI * (f64::atan(f64::exp(-(2.0 * (y + y_offset) / z2 - 1.0) * PI)) / PI - 0.25))
}

/// Approximate area of a tile in square meters.
pub fn tile_area(tile: &Tile) -> f64 {
    // Get Tile coords
    let x = tile.x;
    let y = tile.y as f64;
    let z = tile.z;

    // Estimate area
    let index = std::cmp::min(AF_LEN as usize - 1, z as usize);
    let area_factor = AREA_FACTORS[index];
    let mut area = area_factor * REF_AREA / (1 << (z << 1)) as f64;

    // Adjust centering
    let center_y = if z == 0 { 0 } else { 1 << (z - 1) };
    let center_y = center_y as f64;

    if y < center_y - 1.0 || y > center_y {
        let z_factor = |y_val: f64| -> f64 {
            // Create a new tile with the same x and z but different y
            let temp_tile = Tile::new(x, y_val as u32, z);
            tile_scalefactor(&temp_tile).powf(2.0)
        };

        area *= z_factor(y) / z_factor(center_y);
    }

    area
}

/// Compute the sibling (neighbour) tile in a specific direction.
///
/// # Parameters
/// - `tile`: Reference to the tile ([Tile](cci:2://file:///home/atsyp/Projects/quadbin/src/types.rs:4:0-8:1)) to find a neighbour for.
/// - `direction`: Direction as an integer:
///     - `0`: up
///     - `1`: right
///     - `2`: left
///     - `3`: down
///
/// # Returns
/// - `Some(Tile)`: The sibling tile in the specified direction, if it exists.
/// - `None`: If there is no valid sibling in that direction.
pub fn tile_sibling(tile: &Tile, direction: u8) -> Option<Tile> {
    // Early return for a low level == no neighbors
    if tile.z == 0_u8 {
        return None;
    }

    // Get Tile params
    let mut x = tile.x;
    let mut y = tile.y;
    let z = tile.z;

    let tiles_per_level = 1u32 << z;

    match direction {
        0 => {
            // UP
            if y > 0 {
                y -= 1;
            } else {
                return None;
            }
        }
        1 => {
            // RIGHT
            if x < tiles_per_level - 1 {
                x += 1;
            } else {
                return None;
            }
        }
        2 => {
            // LEFT
            if x > 0 {
                x -= 1;
            } else {
                return None;
            }
        }
        3 => {
            // DOWN
            if y < tiles_per_level - 1 {
                y += 1;
            } else {
                return None;
            }
        }
        _ => return None,
    }

    Some(Tile::new(x, y, z))
}
