use std::collections::HashMap;
use std::str::FromStr;
use crate::rationals::Rational;

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

pub struct Rows {
    pub rows: HashMap<String, Constraints>,
}

impl Rows {
    pub fn empty() -> Self {
        Rows {rows: HashMap::new()}
    }
}


pub struct Columns {
    pub variables: HashMap<String, Vec<(String, Rational)>>,
}

impl Columns {
    pub fn empty() -> Self {
        Columns{variables: HashMap::new()}
    }
}

pub struct Bound {

}

pub struct Rhs {

}

pub struct MpsModel {
    name: String,
}



