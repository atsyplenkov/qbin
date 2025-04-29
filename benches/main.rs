use criterion::{criterion_group, criterion_main};

mod decode_point;
mod encode_point;
// mod get_cell_area;
mod get_resolution;

criterion_group!(
    benches,
    get_resolution::bench,
    // get_cell_area::bench,
    encode_point::bench,
    decode_point::bench
);

criterion_main!(benches);
