use super::mps_parser::MpsInParsing;
use super::ParserError;
use crate::rationals::Rational;
use std::collections::HashMap;
use std::str::FromStr;
use indexmap::IndexMap;
use crate::solvers::basic_simplex_table_data::OptimizationType;

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
#[derive(PartialEq, Debug, Clone, Hash, Eq)]
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

    pub(crate) fn inverse_mut(&mut self) {
        match self {
            Self::L => *self =  Self::G,
            Self::G => *self = Self::L,
            _ => (),
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
    pub(crate) rows: IndexMap<String, Constraints>,
}

impl Rows {
    pub fn empty() -> Self {
        Rows {rows: IndexMap::new()}
    }

    //For integration testing
    #[allow(dead_code)]
    pub fn get_constraint_by_row_name(&self, key: &str) -> Option<&Constraints> {
        self.rows.get(key)
    }
}

#[derive(Clone)]
pub struct Columns {
    //IndexMap variable_name ->( row_name -> value)
    //Use index map to keep variables in order
    pub(crate) variables: IndexMap<String, HashMap<String, Rational>>,
}

impl Columns {
    pub(super) fn empty() -> Self {
        Columns{variables: IndexMap::new()}
    }

    //For integration testing
    #[allow(dead_code)]
    pub fn get_variables_clone(&self) -> IndexMap<String, HashMap<String, Rational>> {
        self.variables.clone()
    }
}

#[derive(Clone)]
pub struct Bounds {
    // HashMap (bound_name, Vec(variable_name, value, bound_type)
    pub bounds: IndexMap<String, Vec<(String, Rational, BoundType)>>,
}

impl Bounds {
    pub(super) fn empty() -> Self {
        Bounds{bounds: IndexMap::new()}
    }

    //For integration testing
    #[allow(dead_code)]
    pub fn get_bounds_clone(&self) -> IndexMap<String, Vec<(String, Rational, BoundType)>> {
        self.bounds.clone()
    }
}

#[derive(Clone)]
pub struct Rhs {
    //HashMap rhs_name -> (row_name -> value)x
    pub(crate) rhs: IndexMap<String, HashMap<String, Rational>>,
}

impl Rhs {
    pub(super) fn empty() -> Self {
        Rhs{rhs: IndexMap::new()}
    }

    //For integration testing
    #[allow(dead_code)]
    pub fn get_rhs_clone(&self) -> IndexMap<String, HashMap<String, Rational>> {
        self.rhs.clone()
    }
}

#[derive(Clone)]
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

pub struct MpsModelWithSelectedVariants {
    pub model: MpsModel,
    pub selected_rhs: Option<String>,
    pub selected_bounds: Option<String>,
    pub selected_opt_row_name: Option<String>,
    pub optimization_type: OptimizationType
}

impl MpsModelWithSelectedVariants {
    pub fn new(model: MpsModel, selected_rhs: Option<String>, selected_bounds: Option<String>, selected_opt_row_name: Option<String>, optimization_type: OptimizationType) -> Self {
        MpsModelWithSelectedVariants {
            model,
            selected_rhs: selected_rhs.map_or(None, |x| Some(x.to_lowercase())),
            selected_bounds: selected_bounds.map_or(None, |x| Some(x.to_lowercase())),
            selected_opt_row_name: selected_opt_row_name.map_or(None, |x| Some(x.to_lowercase())),
            optimization_type: optimization_type
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use crate::parsers::mps::{BoundType, Bounds, Columns, Constraints, MpsModel, Rhs, Rows};
    use crate::rationals::Rational;
    use std::collections::HashMap;

    /// Create simple MPS for tests
    /// 2x1 + x2 <= 6
    /// x1 + x2 = 4
    /// x1 - x2 >= 1
    /// 3x1 + x2 -> N
    pub fn create_simple_mps_model_for_tests_no_bounds_one_rhs_one_objective() -> MpsModel {
        let name = "SimpleMPSModel".to_owned();
        let mut rows = Rows::empty();
        rows.rows.insert("ROW1".to_owned(), Constraints::L);
        rows.rows.insert("ROW2".to_owned(), Constraints::E);
        rows.rows.insert("ROW3".to_owned(), Constraints::G);
        rows.rows.insert("OBJ".to_owned(), Constraints::N);

        let mut columns = Columns::empty();
        let mut x1_values = HashMap::new();
        x1_values.insert("ROW1".to_owned(), Rational::new(2,1));
        x1_values.insert("ROW2".to_owned(), Rational::new(1,1));
        x1_values.insert("ROW3".to_owned(), Rational::new(1,1));
        x1_values.insert("OBJ".to_owned(), Rational::new(3,1));
        columns.variables.insert("x1".to_owned(), x1_values);
        let mut x2_values = HashMap::new();
        x2_values.insert("ROW1".to_owned(), Rational::new(1,1));
        x2_values.insert("ROW2".to_owned(), Rational::new(1,1));
        x2_values.insert("ROW3".to_owned(), Rational::new(-1,1));
        x2_values.insert("OBJ".to_owned(), Rational::new(2,1));
        columns.variables.insert("x2".to_owned(), x2_values);

        let mut rhs = Rhs::empty();
        let mut rhs_values = HashMap::new();
        rhs_values.insert("ROW1".to_owned(), Rational::new(6,1));
        rhs_values.insert("ROW2".to_owned(), Rational::new(4,1));
        rhs_values.insert("ROW3".to_owned(), Rational::new(1,1));
        rhs.rhs.insert("RHS1".to_owned(), rhs_values);

        let bounds = Bounds::empty();
        MpsModel {
            name,
            rows,
            columns,
            rhs,
            bounds
        }
    }

    /// Create MPS with multiple objective functions, optimisable bounds, and two rhs
    /// 2x1 + x2 <= [6 (RHS1), 2 (RHS2)]   ROW1
    /// x1 + x2 = [4 (RHS1), 1(RHS2)]     ROW2
    /// x1 - x2 >= [1 (RHS1), 3(RHS2)]    ROW3
    /// 3x1 + x2 -> N   OBJ1
    /// 2x1 + 8x2 -> N  OBJ2
    /// x1 <= 10    BND1
    /// x1 <= 20    BND1
    /// x2 >= 5     BND1
    /// x2 ≥ 10     BND1
    /// x2 <= 2     BND1
    /// x1 >= 10    BND2
    /// x2 <= 50    BND2
    pub fn create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives() -> MpsModel {
        let name = "SimpleMPSModel".to_owned();
        let mut rows = Rows::empty();
        rows.rows.insert("ROW1".to_owned(), Constraints::L);
        rows.rows.insert("ROW2".to_owned(), Constraints::E);
        rows.rows.insert("ROW3".to_owned(), Constraints::G);
        rows.rows.insert("OBJ1".to_owned(), Constraints::N);
        rows.rows.insert("OBJ2".to_owned(), Constraints::N);

        let mut columns = Columns::empty();
        let mut x1_values = HashMap::new();
        x1_values.insert("ROW1".to_owned(), Rational::new(2,1));
        x1_values.insert("ROW2".to_owned(), Rational::new(1,1));
        x1_values.insert("ROW3".to_owned(), Rational::new(1,1));
        x1_values.insert("OBJ1".to_owned(), Rational::new(3,1));
        x1_values.insert("OBJ2".to_owned(), Rational::new(2,1));
        columns.variables.insert("x1".to_owned(), x1_values);
        let mut x2_values = HashMap::new();
        x2_values.insert("ROW1".to_owned(), Rational::new(1,1));
        x2_values.insert("ROW2".to_owned(), Rational::new(1,1));
        x2_values.insert("ROW3".to_owned(), Rational::new(-1,1));
        x2_values.insert("OBJ1".to_owned(), Rational::new(2,1));
        x2_values.insert("OBJ2".to_owned(), Rational::new(8,1));
        columns.variables.insert("x2".to_owned(), x2_values);

        let mut rhs = Rhs::empty();
        let mut rhs_values = HashMap::new();
        rhs_values.insert("ROW1".to_owned(), Rational::new(6,1));
        rhs_values.insert("ROW2".to_owned(), Rational::new(4,1));
        rhs_values.insert("ROW3".to_owned(), Rational::new(1,1));
        rhs.rhs.insert("RHS1".to_owned(), rhs_values);

        let mut rhs_values2 = HashMap::new();
        rhs_values2.insert("ROW1".to_owned(), Rational::new(2,1));
        rhs_values2.insert("ROW2".to_owned(), Rational::new(1,1));
        rhs_values2.insert("ROW3".to_owned(), Rational::new(3,1));
        rhs.rhs.insert("RHS2".to_owned(), rhs_values2);

        let mut bounds = Bounds::empty();
        let bnd1: Vec<(String, Rational, BoundType)> = vec![
        ("x1".to_owned(), Rational::from_integer(10), BoundType::UP),
        ("x1".to_owned(), Rational::from_integer(20), BoundType::UP),
        ("x2".to_owned(), Rational::from_integer(5), BoundType::LO),
        ("x2".to_owned(), Rational::from_integer(10), BoundType::LO),
        ("x2".to_owned(), Rational::from_integer(20), BoundType::UP),];
        bounds.bounds.insert("BND1".to_owned(), bnd1);

        let bnd2: Vec<(String, Rational, BoundType)> = vec![
            ("x1".to_owned(), Rational::from_integer(10), BoundType::LO),
            ("x2".to_owned(), Rational::from_integer(50), BoundType::UP),
        ];
        bounds.bounds.insert("BND2".to_owned(), bnd2);

        MpsModel {
            name,
            rows,
            columns,
            rhs,
            bounds
        }
    }

    /// Create mps model for tests
    /// 2/5x1 - 2/3x2 = 5/2
    /// 3/2x1 + 1/5x2 = = -10/3
    /// x1 + x2 -> N
    pub fn create_mps_model_with_only_equals_no_bounds_one_rhs_one_objective() -> MpsModel {
        let name = "OnlyEqualsModel".to_owned();
        let mut rows = Rows::empty();
        rows.rows.insert("ROW1".to_owned(), Constraints::E);
        rows.rows.insert("ROW2".to_owned(), Constraints::E);
        rows.rows.insert("OBJ".to_owned(), Constraints::N);

        let mut columns = Columns::empty();
        let mut x1_values = HashMap::new();
        x1_values.insert("ROW1".to_owned(), Rational::new(2,5));
        x1_values.insert("ROW2".to_owned(), Rational::new(3,2));
        x1_values.insert("OBJ".to_owned(), Rational::from_integer(1));
        columns.variables.insert("x1".to_owned(), x1_values);
        let mut x2_values = HashMap::new();
        x2_values.insert("ROW1".to_owned(), Rational::new(-3,2));
        x2_values.insert("ROW2".to_owned(), Rational::new(1,5));
        x2_values.insert("OBJ".to_owned(), Rational::from_integer(1));
        columns.variables.insert("x2".to_owned(), x2_values);

        let mut rhs = Rhs::empty();
        let mut rhs_values = HashMap::new();
        rhs_values.insert("ROW1".to_owned(), Rational::new(5,2));
        rhs_values.insert("ROW2".to_owned(), Rational::new(-10,3));
        rhs.rhs.insert("RHS1".to_owned(), rhs_values);

        let bounds = Bounds::empty();
        MpsModel {
            name,
            rows,
            columns,
            rhs,
            bounds
        }

    }

    #[test]
    fn inverse_mut_constraint_succeeds() {
        let mut g = Constraints::G;
        let mut e = Constraints::E;
        g.inverse_mut();
        e.inverse_mut();
        assert_eq!(g, Constraints::L);
        assert_eq!(e, Constraints::E);
    }

}


