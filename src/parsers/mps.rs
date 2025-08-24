use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use crate::parsers::mps_parser::MpsInParsing;
use crate::parsers::ParserError;
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
    pub(super) rows: HashMap<String, Constraints>,
}

impl Rows {
    pub fn empty() -> Self {
        Rows {rows: HashMap::new()}
    }
}


pub struct Columns {
    //HashMap variable_name ->( column_name -> value)
    pub(super) variables: HashMap<String, HashMap<String, Rational>>,
}

impl Columns {
    pub(super) fn empty() -> Self {
        Columns{variables: HashMap::new()}
    }
}

pub struct Bounds {
    // HashMap (bound_name, Vec(variable_name, value, bound_type)
    pub(super) bounds: HashMap<String, Vec<(String, Rational, BoundType)>>,
}

impl Bounds {
    pub(super) fn empty() -> Self {
        Bounds{bounds: HashMap::new()}
    }
}

pub struct Rhs {
    //HashMap rhs_name -> (row_name -> value)
    pub(super) rhs: HashMap<String, HashMap<String, Rational>>,
}

impl Rhs {
    pub(super) fn empty() -> Self {
        Rhs{rhs: HashMap::new()}
    }
}

pub struct MpsModel {
    name: String,
    rows: Rows,
    columns: Columns,
    rhs: Rhs,
    bounds: Bounds
}

impl TryFrom<MpsInParsing> for MpsModel {
    type Error = Box<ParserError>;

    fn try_from(value: MpsInParsing) -> Result<Self, Self::Error> {
        value.is_filled()?;
        //Unwrap is safe, because we checked filled above
        Ok(MpsModel {
            name: value.name.unwrap(),
            rows: value.rows.unwrap(),
            columns: value.columns.unwrap(),
            bounds: value.bounds.unwrap(),
            rhs: value.rhs.unwrap(),
        })
    }
}



