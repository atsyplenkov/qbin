use criterion::{criterion_group, criterion_main};

mod get_cell_area;
mod get_resolution;

criterion_group!(benches, get_resolution::bench, get_cell_area::bench);

criterion_main!(benches);
