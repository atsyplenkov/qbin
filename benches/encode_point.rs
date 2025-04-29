use criterion::{Criterion, black_box};
use h3o::{LatLng, Resolution};
use qbin::Cell;

const LAT: f64 = -41.28303675124842;
const LNG: f64 = 174.77727344223067;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("encodePoint");

    group.bench_function("geohash", |b| {
        let coord = geohash::Coord { x: LNG, y: LAT };
        let index = geohash::encode(coord, 12).expect("Invalid coordinate");
        b.iter(|| black_box(&index))
    });
    group.bench_function("qbin", |b| {
        let index = Cell::from_point(LAT, LNG, 12);
        b.iter(|| black_box(&index))
    });

    group.bench_function("h3o", |b| {
        let latlng = LatLng::new(LAT, LNG).expect("Invalid coordinate");
        let index = latlng.to_cell(Resolution::Twelve);
        b.iter(|| black_box(&index))
    });

    group.finish();
}
