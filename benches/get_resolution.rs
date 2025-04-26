use criterion::{Criterion, black_box};
use h3o::CellIndex;
use quadbin::Cell;

const INPUT_H3: u64 = 0x8f734e64992d6d8;
const INPUT_QB: u64 = 5209574053332910079;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("getResolution");

    group.bench_function("h3o", |b| {
        let index = CellIndex::try_from(INPUT_H3).expect("cell index");
        b.iter(|| black_box(index).resolution())
    });
    group.bench_function("qbin", |b| {
        let index = Cell::new(INPUT_QB);
        b.iter(|| black_box(index).resolution())
    });

    group.finish();
}