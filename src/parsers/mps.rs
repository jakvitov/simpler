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

/// Bound type that can be applied to ROW
/// UP - variable < upperbound
/// LO - variable > lowerbound
#[derive(PartialEq, Debug, Clone)]
pub enum BoundType {
    UP,
    LO
}

impl FromStr for BoundType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UP" => Ok(Self::UP),
            "LO" => Ok(Self::LO),
            _ => Err(())
        }
    }
}

/// Constrains used in the MPS format
/// N - objective
/// L - less than or equal to constraint
/// G - greater than or equal to constraint
/// E - equality constraint
#[derive(PartialEq, Debug, Clone)]
pub enum Constraints {
    N,
    L,
    G,
    E,
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
    //HashMap variable_name ->( column_name -> value)
    pub variables: HashMap<String, HashMap<String, Rational>>,
}

impl Columns {
    pub fn empty() -> Self {
        Columns{variables: HashMap::new()}
    }
}

pub struct Bounds {
    // HashMap (bound_name, Vec(variable_name, value, bound_type)
    pub bounds: HashMap<String, Vec<(String, Rational, BoundType)>>,
}

impl Bounds {
    pub fn empty() -> Self {
        Bounds{bounds: HashMap::new()}
    }
}

pub struct Rhs {

}

pub struct MpsModel {
    name: String,
}



