// Coordinate borders
pub(crate) const MAX_LONGITUDE: f64 = 180.0;
pub(crate) const MIN_LONGITUDE: f64 = -180.0;
pub(crate) const MAX_LATITUDE: f64 = 89.0;
pub(crate) const MIN_LATITUDE: f64 = -89.0;

// Resolution
// https://docs.carto.com/data-and-analysis/analytics-toolbox-for-postgresql/key-concepts/spatial-indexes#quadbin
pub(crate) const MAX_RESOLUTION: u8 = 26;
pub(crate) const MIN_RESOLUTION: u8 = 0;