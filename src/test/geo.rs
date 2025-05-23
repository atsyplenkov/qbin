use super::data::*;
use crate::Cell;
use crate::errors::*;
use geo::{LineString, Polygon};

#[test]
fn test_quadbin_from_point() {
    let orig = point_2d();
    let cell = Cell::from_geopoint(orig, 10).expect("cell index");
    assert_eq!(cell.get(), 5234261499580514303_u64)
}

#[test]
fn test_quadbin_from_multipoint() {
    let orig = multi_point_2d();
    let cells_iter = Cell::from_multipoint(orig, 4);
    let truth = [5207251884775047167_u64, 5209574053332910079_u64];

    for (i, cell_result) in cells_iter.enumerate() {
        let cell = cell_result.expect("cell index");
        assert_eq!(cell.get(), truth[i]);
    }
}

#[test]
fn test_invalid_resolution() {
    let orig = point_2d();
    let res = 27;
    let cell = Cell::from_geopoint(orig, res);
    assert_eq!(cell.err(), Some(QuadbinError::InvalidResolution(res)));
}

#[test]
fn test_invalid_multipoint_resolution() {
    let orig = multi_point_2d();
    let res = 27;
    let cell = Cell::from_multipoint(orig, res);
    assert_eq!(
        cell.filter_map(|i| i.err()).next(),
        Some(QuadbinError::InvalidResolution(res))
    );
}

#[test]
fn test_cell_to_polygon() {
    let bbox = [22.5, -21.943045533438166, 45.0, 0.0];

    let polygon = Polygon::new(
        LineString::from(vec![
            (bbox[0], bbox[1]), // bottom-left
            (bbox[2], bbox[1]), // bottom-right
            (bbox[2], bbox[3]), // top-right
            (bbox[0], bbox[3]), // top-left
            (bbox[0], bbox[1]), // back to bottom-left to close the loop
        ]),
        vec![],
    );

    let qb_cell = Cell::new(5209574053332910079);

    assert_eq!(qb_cell.to_polygon(), polygon)
}
