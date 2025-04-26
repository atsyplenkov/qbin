use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use quadbin::{Cell, Tile}; // Import your types
use rand::Rng;

// You can also add your optimized implementation versions here
// use quadbin::cells::{optimized_tile_to_cell, optimized_cell_to_tile};

fn generate_random_tiles(count: usize, max_resolution: u8) -> Vec<Tile> {
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| {
            let resolution = rng.gen_range(0..=max_resolution);
            let max_coord = 1u32 << resolution;
            let x = rng.gen_range(0..max_coord);
            let y = rng.gen_range(0..max_coord);
            Tile::new(x, y, resolution)
        })
        .collect()
}

fn generate_random_cells(count: usize, max_resolution: u8) -> Vec<Cell> {
    generate_random_tiles(count, max_resolution)
        .into_iter()
        .map(|tile| tile.to_cell())
        .collect()
}

fn bench_tile_to_cell(c: &mut Criterion) {
    let mut group = c.benchmark_group("Morton Encoding");
    
    for resolution in [5, 10, 15, 20, 26].iter() {
        let tiles = generate_random_tiles(1000, *resolution);
        
        group.bench_with_input(
            BenchmarkId::new("original", resolution), 
            &tiles,
            |b, tiles| {
                b.iter(|| {
                    for tile in tiles {
                        black_box(tile.to_cell());
                    }
                })
            },
        );
        
        // Benchmark the optimized version
        group.bench_with_input(
            BenchmarkId::new("optimized", resolution),
            &tiles,
            |b, tiles| {
                b.iter(|| {
                    for tile in tiles {
                        black_box(tile.optimized_to_cell());
                    }
                })
            }
        );
    }
    
    group.finish();
}

fn bench_cell_to_tile(c: &mut Criterion) {
    let mut group = c.benchmark_group("Morton Decoding");
    
    for resolution in [5, 10, 15, 20, 26].iter() {
        let cells = generate_random_cells(1000, *resolution);
        
        group.bench_with_input(
            BenchmarkId::new("original", resolution),
            &cells,
            |b, cells| {
                b.iter(|| {
                    for cell in cells {
                        black_box(cell.to_tile());
                    }
                })
            },
        );
        
        // Benchmark the optimized version
        group.bench_with_input(
            BenchmarkId::new("optimized", resolution),
            &cells,
            |b, cells| {
                b.iter(|| {
                    for cell in cells {
                        black_box(cell.optimized_to_tile());
                    }
                })
            }
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_tile_to_cell, bench_cell_to_tile);
criterion_main!(benches);
