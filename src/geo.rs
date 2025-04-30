use crate::Cell;
use crate::errors::*;
use geo::{LineString, MultiPoint, Point, Polygon};

/// Support for geospatial primitive types from [geo] crate.
impl Cell {
    /// Get Quadbin cell index from [geo::Point].
    ///
    /// Similar to [Cell::from_point], but requires a [geo::Point] for input.
    ///
    /// # Example
    /// ```
    /// use qbin::Cell;
    /// use geo::*;
    ///
    /// let point = point!(x: 174.77727344223067, y: -41.28303675124842);
    ///
    /// let cell = Cell::from_geopoint(point, 26).expect("cell index");
    /// assert_eq!(cell.get(), 5309133744805926483_u64)
    /// ```
    pub fn from_geopoint(point: Point, res: u8) -> Result<Self, QuadbinError> {
        Cell::from_point(point.y(), point.x(), res)
    }

    /// Get Quadbin cell index from [geo::MultiPoint].
    ///
    /// The output may contain duplicate indexes in case of overlapping
    /// input geometries.
    ///
    /// # Example
    /// ```
    /// use qbin::Cell;
    /// use geo::*;
    ///
    /// let points = MultiPoint::new(vec![
    ///     point!(
    ///         x: -3.7038, y: 40.4168
    ///     ),
    ///     point!(
    ///         x: 33.75, y: -11.178401873711776
    ///     ),
    /// ]);
    ///
    /// let cells = Cell::from_multipoint(points, 10).collect::<Vec<_>>();
    /// ```
    pub fn from_multipoint(
        multipoint: MultiPoint,
        res: u8,
    ) -> impl Iterator<Item = Result<Self, QuadbinError>> {
        multipoint
            .into_iter()
            .map(move |point| Cell::from_geopoint(point, res))
    }

    /// Converts Quadbin cell into [geo::Polygon]
    ///
    /// # Example
    /// ```
    /// use qbin::Cell;
    ///
    /// // Create a Polygon out of the Cell's bounding box
    /// let polygon = Cell::new(5309133744805926483).to_polygon();
    /// // Check if the polygon is of type Polygon and with no interior rings
    /// assert_eq!(polygon.num_interior_rings(), 0);
    /// ```
    pub fn to_polygon(&self) -> Polygon {
        let bbox = self.to_bbox();
        Polygon::new(
            LineString::from(vec![
                (bbox[0], bbox[1]), // bottom-left
                (bbox[2], bbox[1]), // bottom-right
                (bbox[2], bbox[3]), // top-right
                (bbox[0], bbox[3]), // top-left
                (bbox[0], bbox[1]), // back to bottom-left to close the loop
            ]),
            vec![],
        )
    }
}
