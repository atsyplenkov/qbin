use criterion::{BenchmarkId, Criterion, black_box};
use h3o::CellIndex;
use quadbin::Cell;

pub const HEXAGONS: [u64; 16] = [
    0x801ffffffffffff,
    0x811fbffffffffff,
    0x821fb7fffffffff,
    0x831fb4fffffffff,
    0x841fb47ffffffff,
    0x851fb467fffffff,
    0x861fb4667ffffff,
    0x871fb4662ffffff,
    0x881fb46623fffff,
    0x891fb46622fffff,
    0x8a1fb46622dffff,
    0x8b1fb46622d8fff,
    0x8c1fb46622d85ff,
    0x8d1fb46622d85bf,
    0x8e1fb46622d8597,
    0x8f1fb46622d8591,
];

pub const QUADBINS: [u64; 16] = [
    5192650370358181887,
    5197153969985552383,
    5201094619659501567,
    5205598219286872063,
    5210066634542153727,
    5214561438076502015,
    5219062838680616959,
    5223565613674266623,
    5228069007143206911,
    5232572555230969855,
    5237076154858340351,
    5241579751264485375,
    5246083350086549503,
    5250586949713919999,
    5255090549290958847,
    5259594148905746431,
];

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("getCellArea");

    // Benchmark each resolution for h3o
    for (i, &h3_index) in HEXAGONS.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("h3o", i), &h3_index, |b, &index| {
            let cell = CellIndex::try_from(index).expect("cell index");
            b.iter(|| black_box(cell).area_m2())
        });
    }

    // Benchmark each resolution for quadbin
    for (i, &qb_index) in QUADBINS.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("qbin", i), &qb_index, |b, &index| {
            let cell = Cell::new(index);
            b.iter(|| black_box(cell).area_m2())
        });
    }

    group.finish();
}
