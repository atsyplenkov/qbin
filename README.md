# Quadbin

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/atsyplenkov/quadbin/blob/main/LICENSE) 
[![crates.io](https://img.shields.io/crates/v/quadbin.svg?logo=rust)](https://crates.io/crates/quadbin) 
[![Build & Test](https://github.com/atsyplenkov/quadbin/actions/workflows/rust.yml/badge.svg)](https://github.com/atsyplenkov/quadbin/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/atsyplenkov/quadbin/graph/badge.svg?token=4SZ4RI3ILS)](https://codecov.io/gh/atsyplenkov/quadbin)

A Rust implementation of Quadbin, a hierarchical geospatial index tiling approach developed by [CARTO](https://github.com/CartoDB). Like the [Microsoft's Bing Maps Tile System](https://docs.microsoft.com/en-us/bingmaps/articles/bing-maps-tile-system) (aka Quadkey), Quadbin uniformly subdivides a map in Mercator projection into four squares at different resolution levels, from 0 to 26 (less than 1 m² at the equator). However, unlike Quadkey, Quadbin stores the grid cell index in a 64-bit integer.

This crate is a complete rewrite of the original implementation in [JavaScript](https://github.com/CartoDB/quadbin-js) and [Python](https://github.com/CartoDB/quadbin-py). Learn more about Quadbin in the [CARTO documentation](https://docs.carto.com/data-and-analysis/analytics-toolbox-for-snowflake/sql-reference/quadbin).
    

## Example

```rust
use quadbin::Cell;
use approx::assert_relative_eq;

// Convert a point into a Quadbin cell
let longitude = -3.7038;
let latitude = 40.4168;
let resolution = 10_u8;
let qb = Cell::from_point(longitude, latitude, resolution);
assert_eq!(qb, Cell::new(5234261499580514303_u64));

// Get a point from a Quadbin cell
let coords = Cell::new(5209574053332910079_u64).to_point();
assert_eq!(coords, (33.75, -11.178401873711776));

// Quadbin resolution at equator in m²
let area = Cell::from_point(0.0, 0.0, 26).area_m2();
assert_relative_eq!(area, 0.36, epsilon = 1e-2)
```

## Quadbin vs. Quadkey
TBA.

## Reasoning
This repository is a proof-of-concept project, where I practised writing Rust code, and, moreover, writing Rust and R bindings as a single project. Recently, I was excited by the newly proposed  [`raquet`](https://github.com/CartoDB/raquet) format by [CARTO](https://github.com/CartoDB) for storing raster data in Parquet files and was eager to try it in my projects. However, the `raquet` file specification and conversion are written in pure Python and heavily relies on `gdal`; therefore, instead of implementing R-to-Python, I decided to rewrite everything in Rust, merely for fun and practice. This repository is the first step towards native, GDAL-free raster to `raquet` conversion.

## License and Attribution
This project includes a reimplementation of logic based on [`quadbin-py`](https://github.com/CartoDB/quadbin-py) by CARTO, which is licensed under the BSD 3-Clause License.
See [`LICENSE-THIRD-PARTY`](LICENSE-THIRD-PARTY) for full license text.

## See also
* [`quadbin-js`](https://github.com/CartoDB/quadbin-js) and [`quadbin-py`](https://github.com/CartoDB/quadbin-py) — original implementation in JavaScript and Python by CARTO
* [`geo-quadkey-rs`](https://github.com/masaishi/geo-quadkey-rs) — a Rust crate for QuadKey, the original implementation of Microsoft's Bing Maps Tile System


