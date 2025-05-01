use crate::Cell;
use crate::errors::*;

#[test]
fn test_invalid_cellindex() {
    assert!(
        !InvalidCell::new(Some(5209574053332910078_u64), "error")
            .to_string()
            .is_empty()
    );
    assert_eq!(
        Cell::try_from(5209574053332910078_u64).err(),
        Some(QuadbinError::InvalidCell(InvalidCell::new(
            Some(5209574053332910078_u64),
            "Provided Quadbin Cell index is invalid"
        )))
    );
    assert_eq!(
        Cell::try_from(5209574053332910079_u64)
            .expect("cell index")
            .get(),
        5209574053332910079_u64
    );
}

#[test]
fn test_cell_to_parent_invalid_resolution() {
    let cell = Cell::try_from(5209574053332910079).expect("cell index");
    let result = cell.parent(4);

    assert!(result.is_err());

    assert_eq!(
        result.err(),
        Some(QuadbinError::InvalidResolution(InvalidResolution::new(
            4,
            "Parent resolution should be lower than the current resolution"
        )))
    );
}

#[test]
fn test_invalid_cell_index() {
    let val: [u64; 2] = [5209574053332910078, 6362495557939757055];

    for i in val.iter() {
        let cell = Cell::try_from(*i);
        assert!(cell.is_err());

        assert_eq!(
            cell.err(),
            Some(QuadbinError::InvalidCell(InvalidCell::new(
                Some(*i),
                "Provided Quadbin Cell index is invalid"
            )))
        );
    }
}

#[test]
fn test_invalid_child_res() {
    let cell = Cell::try_from(5209574053332910079).expect("cell index");

    // Resolution 3
    let kids = cell.children(3);
    assert!(kids.is_err());
    assert_eq!(
        kids.err(),
        Some(QuadbinError::InvalidResolution(InvalidResolution::new(
            3,
            "Children resolution should be greater than the current resolution"
        )))
    );

    // Resolution 27
    let kids27 = cell.children(27);
    assert!(kids27.is_err());
    assert_eq!(
        kids27.err(),
        Some(QuadbinError::InvalidResolution(InvalidResolution::new(
            27,
            "Children resolution should be between 0 and 26"
        )))
    );
}
