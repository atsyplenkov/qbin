use extendr_api::prelude::*;

use qbin::Cell;

#[derive(Debug, Clone, Copy)]
pub struct Quadbin {
    pub index: Cell,
}

#[extendr]
impl Quadbin {}

impl From<Cell> for Quadbin {
    fn from(index: Cell) -> Self {
        Quadbin { index: index }
    }
}

impl TryFrom<&Robj> for Quadbin {
    type Error = Error;

    fn try_from(robj: &Robj) -> extendr_api::Result<Self> {
        // Convert Option to Result with an error message
        let value = robj
            .as_real()
            .ok_or_else(|| Error::Other("Expected a numeric value".into()))?;
        let cell = Cell::try_from(value as u64)
            .map_err(|_| Error::Other("Invalid Quadbin cell value".into()))?;
        Ok(Quadbin { index: cell })
    }
}

// returns an array of strings with the appropriate vctrs class
#[extendr]
pub fn vctrs_class() -> [String; 3] {
    [
        String::from("Quadbin"),
        String::from("vctrs_vctr"),
        String::from("list"),
    ]
}

#[extendr]
fn quadbin_to_ints(x: List) -> Doubles {
    let res = x
        .into_iter()
        .map(|(_, robj)| match Quadbin::try_from(&robj) {
            Ok(quadbin) => Rfloat::from(quadbin.index.get() as f64),
            Err(_) => Rfloat::na(),
        })
        .collect::<Vec<Rfloat>>();

    Doubles::from_values(res)
}

extendr_module! {
    mod qb;
    fn quadbin_to_ints;
    fn vctrs_class;
}
