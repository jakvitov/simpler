use super::mps_parser::MpsInParsing;
use super::ParserError;
use crate::rationals::Rational;
use std::collections::{BTreeMap, HashMap};
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

/// Bound type that can be applied to ROW
/// UP - variable < upperbound
/// LO - variable > lowerbound
#[derive(PartialEq, Debug, Clone)]
pub enum BoundType {
    UP,
    LO
}

impl BoundType {
    pub(crate) fn to_sign(&self) -> char {
        match self {
            BoundType::UP => '<',
            BoundType::LO => '>'
        }
    }
}

impl FromStr for BoundType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" => Ok(Self::UP),
            "lo" => Ok(Self::LO),
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

impl Constraints {
    pub(crate) fn to_sign(&self) -> char {
        match self {
            Self::N => '\u{27FC}',
            Self::L => '<',
            Self::G => '>',
            Self::E => '=',
        }
    }
}

impl FromStr for Constraints {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "n" => Ok(Constraints::N),
            "l" => Ok(Constraints::L),
            "g" => Ok(Constraints::G),
            "e" => Ok(Constraints::E),
            _ => Err(())
        }
    }
}

#[derive(PartialEq, Debug, Clone)]

pub struct Rows {
    pub(crate) rows: HashMap<String, Constraints>,
}

impl Rows {
    pub fn empty() -> Self {
        Rows {rows: HashMap::new()}
    }

    //For integration testing
    #[allow(dead_code)]
    pub fn get_constraint_by_row_name(&self, key: &str) -> Option<&Constraints> {
        self.rows.get(key)
    }
}


pub struct Columns {
    //BTreeMap variable_name ->( row_name -> value)
    //We use BTreeMap to keep the variables ordered
    pub(crate) variables: BTreeMap<String, HashMap<String, Rational>>,
}

impl Columns {
    pub(super) fn empty() -> Self {
        Columns{variables: BTreeMap::new()}
    }

    //For integration testing
    #[allow(dead_code)]
    pub fn get_variables_clone(&self) -> BTreeMap<String, HashMap<String, Rational>> {
        self.variables.clone()
    }
}

pub struct Bounds {
    // HashMap (bound_name, Vec(variable_name, value, bound_type)
    pub bounds: HashMap<String, Vec<(String, Rational, BoundType)>>,
}

impl Bounds {
    pub(super) fn empty() -> Self {
        Bounds{bounds: HashMap::new()}
    }

    //For integration testing
    #[allow(dead_code)]
    pub fn get_bounds_clone(&self) -> HashMap<String, Vec<(String, Rational, BoundType)>> {
        self.bounds.clone()
    }
}

pub struct Rhs {
    //HashMap rhs_name -> (row_name -> value)
    pub(crate) rhs: HashMap<String, HashMap<String, Rational>>,
}

impl Rhs {
    pub(super) fn empty() -> Self {
        Rhs{rhs: HashMap::new()}
    }

    //For integration testing
    #[allow(dead_code)]
    pub fn get_rhs_clone(&self) -> HashMap<String, HashMap<String, Rational>> {
        self.rhs.clone()
    }
}

pub struct MpsModel {
    pub name: String,
    pub rows: Rows,
    pub columns: Columns,
    pub rhs: Rhs,
    pub bounds: Bounds
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



