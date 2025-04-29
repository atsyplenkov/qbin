use criterion::{Criterion, black_box};
use qbin::Cell;

const QBIN: u64 = 5246083350086549503;
const GEOHASH: &str = "rbsm1hsuvshv";

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("decodePoint");

    group.bench_function("geohash", |b| {
        let (index, _, _) = geohash::decode(GEOHASH).unwrap();
        b.iter(|| black_box(&index))
    });

    group.bench_function("qbin", |b| {
        let index = Cell::new(QBIN).to_point();
        b.iter(|| black_box(&index))
    });

    group.finish();
}
