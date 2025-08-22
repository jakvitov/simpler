use std::str::FromStr;

/// Sections in which is the MPS format devided
#[derive(PartialEq, Debug, Clone)]
pub enum Sections {
    NAME,
    ROWS,
    COLUMNS,
    RHS,
    BOUNDS,
    ENDATA
}


/// Constrains used in the MPS format
/// N - objective
/// L - less than or equal to constraint
/// G - greater than or equal to constraint
/// E - equality constraint
#[derive(PartialEq, Debug, Clone)]
pub enum Constraints {
    N,
    L, // <
    G, // >
    E  // ==
}

impl FromStr for Constraints {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Constraints::N),
            "L" => Ok(Constraints::L),
            "G" => Ok(Constraints::G),
            "E" => Ok(Constraints::E),
            _ => Err(())
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Row {
    constraint: Constraints,
    name: String
}

impl Row {
    pub fn new(constraint: Constraints, name: String) -> Self {
        return Row{constraint: constraint, name: name};
    }
}

pub struct Column {

}

pub struct Bound {

}

pub struct Rhs {

}

pub struct MpsModel {
    name: String,
    rows: Vec<Row>
}



