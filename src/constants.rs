// Coordinate borders
pub(crate) const MAX_LONGITUDE: f64 = 180.0;
pub(crate) const MIN_LONGITUDE: f64 = -180.0;
pub(crate) const MAX_LATITUDE: f64 = 89.0;
pub(crate) const MIN_LATITUDE: f64 = -89.0;

// Resolution
// https://docs.carto.com/data-and-analysis/analytics-toolbox-for-postgresql/key-concepts/spatial-indexes#quadbin
pub(crate) const MAX_RESOLUTION: u8 = 26;
pub(crate) const MIN_RESOLUTION: u8 = 0;

// Area estimation
pub(crate) const REF_AREA: f64 = 508164597540055.75;
pub(crate) const AREA_FACTORS: [f64; 12] = [
    1.0,
    1.003741849761155,
    1.8970972739048304,
    2.7118085839548,
    3.0342500406694364,
    3.1231014735135538,
    3.1457588045774316,
    3.151449027223487,
    3.1528731677136914,
    3.1532293013524657,
    3.1533183409109418,
    3.1533406011847736,
];
pub(crate) const AF_LEN: u8 = AREA_FACTORS.len() as u8;
