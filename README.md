# QuadBin

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/atsyplenkov/quadbin/blob/main/LICENSE) [![crates.io](https://img.shields.io/crates/v/quadbin.svg?logo=rust)](https://crates.io/crates/quadbin) [![Build & Test](https://github.com/atsyplenkov/quadbin/actions/workflows/rust.yml/badge.svg)](https://github.com/atsyplenkov/quadbin/actions/workflows/rust.yml)

A Rust implementation of QuadBin, a hierarchical geospatial index tiling approach developed by Carto. Unlike [Microsoft's Bing Maps Tile System](https://docs.microsoft.com/en-us/bingmaps/articles/bing-maps-tile-system), QuadBin stores the information to uniquely identify any of the grid cells that result from uniformly subdividing a map in Mercator projection into four squares at different resolution levels, from 0 to 26 (less than 1 m² at the equator), in a 64-bit unsigned integer.

This crate is a complete rewrite of the original implementation in [JavaScript](https://github.com/CartoDB/quadbin-js) and [Python](https://github.com/CartoDB/quadbin-py). Learn more about QuadBin in the [Carto documentation](https://docs.carto.com/data-and-analysis/analytics-toolbox-for-snowflake/sql-reference/quadbin).
    
## License and Attribution
This project includes a reimplementation of logic based on [`quadbin-py`](https://github.com/CartoDB/quadbin-py) by CARTO, which is licensed under the BSD 3-Clause License.
See [`LICENSE-THIRD-PARTY`](LICENSE-THIRD-PARTY) for full license text.

## See also
* [`quadbin-js`](https://github.com/CartoDB/quadbin-js) and [`quadbin-py`](https://github.com/CartoDB/quadbin-py) — original implementation in JavaScript and Python by CARTO
* [`geo-quadkey-rs`](https://github.com/masaishi/geo-quadkey-rs) — a Rust crate for QuadKey, the original implementation of Microsoft's Bing Maps Tile System


