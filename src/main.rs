use quadbin::utils::*;

fn main() {
    let num = 10.5;
    let lower = 0.0;
    let upper = 5.0;
    let clipped = clip_number(num, lower, upper);
    println!("{}", clipped);

    let longitude = 190.0;
    let clipped_longitude = clip_longitude(longitude);
    println!("{}", clipped_longitude);

    let latitude = 90.0;
    let clipped_latitude = clip_latitude(latitude);
    println!("{}", clipped_latitude);

    let lon = -45.0;
    let lat = 45.0;
    let tile = point_to_tile(lon, lat, 10);
    println!("{}", tile);

    let latitude = tile_to_latitude(&tile, 0.0);
    println!("{}", latitude);

    let longitude = tile_to_longitude(&tile, 0.0);
    println!("{}", longitude)
}