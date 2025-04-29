use super::data::*;
use crate::Cell;
use crate::errors::*;

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
    assert_eq!(
        cell.err(),
        Some(QuadbinError::InvalidResolution(InvalidResolution::new(
            res,
            "Resolution should be between 0 and 26"
        )))
    );
}
