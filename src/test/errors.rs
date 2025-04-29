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
fn test_parent_resolution() {
    let cell = Cell::new(5209574053332910079);
    let message = cell.parent(26).err();
    assert_eq!(
        message,
        Some(QuadbinError::InvalidResolution(InvalidResolution::new(
            26,
            "Parent resolution should be lower than the current resolution"
        )))
    );
}
