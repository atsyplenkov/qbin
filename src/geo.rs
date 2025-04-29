use crate::Cell;
use crate::errors::*;
use geo::{MultiPoint, Point};

/// Support for [geo].
impl Cell {
    /// Get Quadbin cell index from [geo::Point].
    ///
    /// Similar to [Cell::from_point], but requires a [geo::Point] for input.
    pub fn from_geopoint(point: Point, res: u8) -> Result<Self, QuadbinError> {
        Cell::from_point(point.y(), point.x(), res)
    }

    /// Get Quadbin cell index from [geo::MultiPoint]
    ///
    /// The output may contain duplicate indexes in case of overlapping
    /// input geometries.
    pub fn from_multipoint(
        multipoint: MultiPoint,
        res: u8,
    ) -> impl Iterator<Item = Result<Self, QuadbinError>> {
        multipoint
            .into_iter()
            .map(move |point| Cell::from_geopoint(point, res))
    }
}
