# Quadbin

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/atsyplenkov/quadbin/blob/main/LICENSE) 
[![crates.io](https://img.shields.io/crates/v/quadbin.svg?logo=rust)](https://crates.io/crates/quadbin) 
[![Build & Test](https://github.com/atsyplenkov/quadbin/actions/workflows/rust.yml/badge.svg)](https://github.com/atsyplenkov/quadbin/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/atsyplenkov/quadbin/graph/badge.svg?token=4SZ4RI3ILS)](https://codecov.io/gh/atsyplenkov/quadbin)

A Rust implementation of Quadbin, a hierarchical geospatial index tiling approach developed by [CARTO](https://github.com/CartoDB). Unlike [Microsoft's Bing Maps Tile System](https://docs.microsoft.com/en-us/bingmaps/articles/bing-maps-tile-system), Quadbin stores the information to uniquely identify any of the grid cells that result from uniformly subdividing a map in Mercator projection into four squares at different resolution levels, from 0 to 26 (less than 1 m² at the equator), in a 64-bit unsigned integer.

This crate is a complete rewrite of the original implementation in [JavaScript](https://github.com/CartoDB/quadbin-js) and [Python](https://github.com/CartoDB/quadbin-py). Learn more about Quadbin in the [CARTO documentation](https://docs.carto.com/data-and-analysis/analytics-toolbox-for-snowflake/sql-reference/quadbin).
    

## Usage examples

<!-- ### Convert a point into a Quadbin cell

```rust
let longitude = -3.7038;
let latitude = 40.4168;
let resolution = 10_u8;
let qb = quadbin::cells::point_to_cell(longitude, latitude, resolution);
assert_eq!(qb, Some(5234261499580514303_u64));
```

### Get a point from a Quadbin cell

```rust
let coords = quadbin::cells::cell_to_point(5209574053332910079_u64);
assert_eq!(coords, Some((33.75, -11.178401873711776)));
``` -->

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


