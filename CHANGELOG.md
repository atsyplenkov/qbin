# Changelog

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Possible sections are:

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

<!-- next-header -->

## [0.2.0] - 2025-05-01

### Added
- Added basic support for geo-primitive types (#1), including encoding Points and MultiPoints into Quadbin Cells and decoding Quadbin Cells into Polygons.
- Added method to list Cell's children.
- Added benchmarks to evaluate performance and compare with `geohash` and `h3o`.

### Changed
- Significantly rewrote the codebase to make it more idiomatic. Most functions now return either `Result<>` or `Option<>`.
- Better errors with `QuadbinError` enum.

## [0.1.0] - 2025-04-26

- initial release, basic coverage of the Quadbin API