# Qbin

<p align="center">
    <a href="https://github.com/atsyplenkov/qbin/releases">
        <img src="https://img.shields.io/github/v/release/atsyplenkov/qbin?style=flat&labelColor=1C2C2E&color=dea584&logo=GitHub&logoColor=white"></a>
    <a href="https://crates.io/crates/qbin/">
        <img src="https://img.shields.io/crates/v/qbin?style=flat&labelColor=1C2C2E&color=dea584&logo=Rust&logoColor=white"></a>
    <a href="https://codecov.io/gh/atsyplenkov/qbin">
        <img src="https://img.shields.io/codecov/c/gh/atsyplenkov/qbin?style=flat&labelColor=1C2C2E&color=dea584&logo=Codecov&logoColor=white"></a>
    <br>
    <a href="https://github.com/atsyplenkov/qbin/actions/workflows/rust-ci.yml">
        <img src="https://img.shields.io/github/actions/workflow/status/atsyplenkov/qbin/rust-ci.yml?style=flat&labelColor=1C2C2E&color=dea584&logo=GitHub%20Actions&logoColor=white"></a>
    <a href="https://docs.rs/qbin/">
        <img src="https://img.shields.io/docsrs/qbin?style=flat&labelColor=1C2C2E&color=dea584&logo=Rust&logoColor=white"></a>
    <br>
</p>

<h4 align="center">
  <a href="https://docs.rs/qbin/">Documentation</a> |
  <a href="https://crates.io/crates/qbin/">Website</a>
</h4>


A Rust implementation of **Quadbin**, a hierarchical geospatial index tiling approach developed by [CARTO](https://github.com/CartoDB). Like the [Microsoft's Bing Maps Tile System](https://docs.microsoft.com/en-us/bingmaps/articles/bing-maps-tile-system) (aka Quadkey), Quadbin uniformly subdivides a map in Mercator projection into four squares at different resolution levels, from 0 to 26 (less than 1 m² at the equator). However, unlike Quadkey, Quadbin stores the grid cell index in a 64-bit integer.

This crate is a complete rewrite of the original implementation in [JavaScript](https://github.com/CartoDB/quadbin-js) and [Python](https://github.com/CartoDB/quadbin-py). Learn more about Quadbin in the [CARTO documentation](https://docs.carto.com/data-and-analysis/analytics-toolbox-for-snowflake/sql-reference/quadbin).
    

## Example

```rust
use qbin::Cell;
use approx::assert_relative_eq;

// Convert a point into a Quadbin cell
let longitude = -3.7038;
let latitude = 40.4168;
let resolution = 10_u8;
let qb = Cell::from_point(latitude, longitude, resolution).expect("cell index");
assert_eq!(qb, Cell::try_from(5234261499580514303_u64).expect("cell index"));

// Get a point from a Quadbin cell
let coords = Cell::new(5209574053332910079_u64).to_point();
assert_eq!(coords, [-11.178401873711776, 33.75]);

// Quadbin resolution at equator in m²
let area = Cell::from_point(0.0, 0.0, 26).expect("cell index").area_m2();
assert_relative_eq!(area, 0.36, epsilon = 1e-2)
```

## Quadbin vs. Quadkey
Similar to Quadkey, Quadbin divides each tile into four sub-tiles with a minor difference in tiling approach. However, the key difference lies in how the tiles are indexed. **Quadkey** uses a variable-length index, where the number of digits corresponds to the resolution level. For example, a Quadkey can range from 1 to 23 digits long. This format inherently encodes the hierarchy, as each digit represents a parent tile, making it convenient for human interpretation. With just the Quadkey string, you can infer the location, resolution, and parent-child relationship of the tile.

In contrast, **Quadbin** (in its current implementation) uses a fixed-length 64-bit index (`NonZeroU64`) with a constant length of 19 digits. The bit layout is as follows:

```text
 ┏━┳━━━┳━━━━┳━━━━━━━┳━━━━━━━━━━━┈┈┈┈┈┈┈┈━━━━━━━━┓
 ┃U┃ H ┃ M  ┃   R   ┃    XY in Morton order     ┃
 ┗━┻━━━┻━━━━┻━━━━━━━┻━━━━━━━━━━━┈┈┈┈┈┈┈┈━━━━━━━━┛
 63  62   59    56    52                        0
```
- `U`: Unused reserved bit (bit 63), always set to `0`;
- `H`: Header bit (bit 62), always set to `1`;
- `M`: Index mode, fixed to `1`, encoded over 4 bits (bits 59–62); 
- `R`: Cell resolution, ranging from `0` to `26`, encoded in bits 52–56;
- Remaining bits (0–51) encode the cell’s XY position in Morton order (Z-order curve).

This structure makes Quadbin a more memory-efficient way to store tile indices, which is important when working with large spatial datasets or arrays.

For example, Australia and New Zealand are located in the third tile at Level 1, and in the second tile at Level 2. Their corresponding Quadkey would be `31` (since tile numbering starts at 0). However, in the Quadbin spatial indexing, the same location is represented by the Quadbin cell `5201094619659501567`. Another example: at the highest resolution possible, the best beer in Wellington can be found in the Quadbin index `5309133744805926483` (level 26) or Quadkey `31311100030030030211121` (level 23).

## Reasoning
This repository is a proof-of-concept project, where I practised writing Rust code, and, moreover, writing Rust with R and Python bindings as a single project. Recently, I was excited by the newly proposed  [`raquet`](https://github.com/CartoDB/raquet) format by [CARTO](https://github.com/CartoDB) for storing raster data in Parquet files and was eager to try it in my projects. However, the `raquet` file specification and conversion are written in pure Python and heavily relies on `gdal`; therefore, instead of implementing R-to-Python, I decided to rewrite everything in Rust, merely for fun and practice. This repository is the first step towards native, GDAL-free raster to `raquet` conversion.

## License and Attribution
This project includes a reimplementation of logic based on [`quadbin-py`](https://github.com/CartoDB/quadbin-py) and [`quadbin-js`](https://github.com/CartoDB/quadbin-js) developed and maintained by CARTO, which are licensed under the BSD 3-Clause License.
See [`LICENSE-THIRD-PARTY`](LICENSE-THIRD-PARTY) for full license text.

## See also
* [`quadbin-js`](https://github.com/CartoDB/quadbin-js) and [`quadbin-py`](https://github.com/CartoDB/quadbin-py) – the original **Quadbin** implementations in JavaScript and Python by CARTO;
* [`geo-quadkey-rs`](https://github.com/masaishi/geo-quadkey-rs) – a Rust crate for Quadkey (Microsoft's Bing Maps Tile System);
* [`quadkeyr`](https://docs.ropensci.org/quadkeyr/) – an R package for working with Quadkey (Microsoft's Bing Maps Tile System);


