use criterion::{criterion_group, criterion_main};

mod get_resolution;

criterion_group!(benches, get_resolution::bench,);

criterion_main!(benches);
