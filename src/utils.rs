use crate::constants::*;
use crate::direction::Direction;
use crate::types::Tile;
use std::f64::consts::PI;

/// Clip a value between a minimum and maximum value
pub(crate) fn clip_number(num: f64, lower: f64, upper: f64) -> f64 {
    num.max(lower).min(upper)
}

/// Limit longitude bounds.
pub(crate) fn clip_longitude(lng: f64) -> f64 {
    clip_number(lng, MIN_LONGITUDE, MAX_LONGITUDE)
}

/// Limit latitude bounds.
pub(crate) fn clip_latitude(lat: f64) -> f64 {
    clip_number(lat, MIN_LATITUDE, MAX_LATITUDE)
}

/// Compute the tile in fractions for a longitude and latitude in a
/// specific resolution.
pub(crate) fn point_to_tile_fraction(lat: f64, lng: f64, res: u8) -> (f64, f64, u8) {
    // Check resolution to avoid overflow
    assert!(
        (res <= MAX_RESOLUTION),
        "Resolution should be between 0 and 26"
    );

    // Compute tile coordinates
    let z2: f64 = (1 << res) as f64;
    let sinlat = f64::sin(lat * PI / 180.0);
    let x = z2 * (lng / 360.0 + 0.5);
    let yfraction = 0.5 - 0.25 * ((1.0 + sinlat) / (1.0 - sinlat)).ln() / PI;
    let y = clip_number(z2 * yfraction, 0.0, z2 - 1.0);

    let x = x % z2;
    let x = if x < 0.0 { x + z2 } else { x };

    // Return the tile coordinates
    (x, y, res)
}

/// Compute the tile for a longitude and latitude in a specific resolution.
pub(crate) fn point_to_tile(lat: f64, lng: f64, res: u8) -> Tile {
    let (x, y, z) = point_to_tile_fraction(lat, lng, res);
    let x: u32 = x.floor() as u32;
    let y: u32 = y.floor() as u32;
    Tile::new(x, y, z)
}

/// Compute the latitude for a tile with an offset.
pub(crate) fn tile_to_latitude(tile: &Tile, offset: f64) -> f64 {
    // Check if offset is between 0 and 1
    assert!(
        (0.0..=1.0).contains(&offset),
        "Offset should be between 0 and 1"
    );

    // Get Tile coords
    let y = tile.y as f64;
    let z2 = (1 << tile.z) as f64;

    // Compute latitude
    let expy = f64::exp(-(2.0 * (y + offset) / z2 - 1.0) * PI);
    360.0 * (f64::atan(expy) / PI - 0.25)
}

/// Compute the longitude for a tile with an offset.
pub(crate) fn tile_to_longitude(tile: &Tile, offset: f64) -> f64 {
    // Check if offset is between 0 and 1
    assert!(
        (0.0..=1.0).contains(&offset),
        "Offset should be between 0 and 1"
    );

    // Get Tile coords
    let x = tile.x as f64;
    let z2 = (1 << tile.z) as f64;

    // Compute longitude
    180.0 * (2.0 * (x + offset) / z2 - 1.0)
}

/// Inverse of the scale factor at the tile center.
pub(crate) fn tile_scalefactor(tile: &Tile) -> f64 {
    // Get Tile coords
    let y = tile.y as f64;
    let z2 = (1 << tile.z) as f64;
    let y_offset = 0.5_f64;

    // Estimate scale factor
    f64::cos(2.0 * PI * (f64::atan(f64::exp(-(2.0 * (y + y_offset) / z2 - 1.0) * PI)) / PI - 0.25))
}

/// Approximate area of a tile in square meters.
pub(crate) fn tile_area(tile: &Tile) -> f64 {
    // Get Tile coords
    let x = &tile.x;
    let y = tile.y as f64;
    let z = tile.z as usize;

    // Estimate area
    let index = std::cmp::min(AF_LEN as usize - 1, z);
    let area_factor = AREA_FACTORS[index];

    // !NB: Use saturation to avoid overflow for high z values
    let shift_amount = z.saturating_mul(2);
    let denominator = 1u64 << std::cmp::min(shift_amount, 63);
    let mut area = area_factor * REF_AREA / denominator as f64;

    // Adjust centering
    let center_y = if z == 0 { 0 } else { 1 << (z - 1) };
    let center_y = center_y as f64;

    if y < center_y - 1.0 || y > center_y {
        let z_factor = |y_val: f64| -> f64 {
            // Create a new tile with the same x and z but different y
            let temp_tile = Tile::new(*x, y_val as u32, z as u8);
            tile_scalefactor(&temp_tile).powf(2.0)
        };

        area *= z_factor(y) / z_factor(center_y);
    }

    area
}

/// Compute the neighbour (sibling) tile in a specific direction.
pub(crate) fn tile_neighbor(tile: &Tile, direction: Direction) -> Option<Tile> {
    // Early return for a low level == no neighbors
    // TODO: Think about what should one return instead of None
    if tile.z == 0_u8 {
        return None;
    }

    // Get Tile params
    let mut x = tile.x;
    let mut y = tile.y;
    let z = tile.z;

    let tiles_per_level = 1u32 << z;

    match direction {
        Direction::Up => {
            if y > 0 {
                y -= 1;
            } else {
                return None;
            }
        }
        Direction::Right => {
            if x < tiles_per_level - 1 {
                x += 1;
            } else {
                return None;
            }
        }
        Direction::Left => {
            if x > 0 {
                x -= 1;
            } else {
                return None;
            }
        }
        Direction::Down => {
            if y < tiles_per_level - 1 {
                y += 1;
            } else {
                return None;
            }
        }
    }

    Some(Tile::new(x, y, z))
}

/// Compute a hash from the tile.
pub(crate) fn to_tile_hash(tile: &Tile) -> u64 {
    let x = tile.x as u64;
    let y = tile.y as u64;
    let z = tile.z as u64;

    let dim = 2 * (1 << z);

    ((dim * y + x) * 32) + z
}

/// Compute a tile from the hash.
#[allow(dead_code)]
pub(crate) fn from_tile_hash(tile_hash: u64) -> Tile {
    // TODO:
    // Return None if hash is invalid
    // Understand why do we need tile hashing
    let z = tile_hash % 32_u64;
    let dim = 2_u64 * (1_u64 << z);
    let xy = (tile_hash - z) / 32;
    let x = xy % dim;
    let y = ((xy - x) / dim) % dim;

    Tile::new(x as u32, y as u32, z as u8)
}

/// Return the tiles hashes that cover a point.
///
/// _For internal use._
pub fn point_cover(lat: f64, lng: f64, res: u8) -> u64 {
    let tile = Tile::from_point(lat, lng, res);
    to_tile_hash(&tile)
}
