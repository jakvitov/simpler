use super::mps_parser::MpsInParsing;
use super::ParserError;
use crate::rationals::Rational;
use crate::solvers::basic_simplex_table_data::OptimizationType;
use indexmap::IndexMap;
use std::collections::HashMap;
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

    pub(crate) fn invert_mut(&mut self) {
        match self {
            BoundType::UP => *self = BoundType::LO,
            BoundType::LO => *self = BoundType::UP
        }
    }

    pub(crate) fn invert(&self) -> BoundType {
        match self {
            BoundType::UP => BoundType::LO,
            BoundType::LO => BoundType::UP
        }
    }
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

impl Constraints {
    pub(crate) fn to_sign(&self) -> char {
        match self {
            Self::N => '\u{27FC}',
            Self::L => '<',
            Self::G => '>',
            Self::E => '=',
        }
    }

    pub(crate) fn invert_mut(&mut self) {
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
            selected_rhs: selected_rhs,
            selected_bounds: selected_bounds,
            selected_opt_row_name: selected_opt_row_name,
            optimization_type: optimization_type
        }
    }
}

pub struct CroppedMpsModel {
    pub model: MpsModel,
    pub optimization_type: OptimizationType,
}

impl CroppedMpsModel {
    pub fn new(model: MpsModel, optimization_type: OptimizationType) -> Self {
        CroppedMpsModel {model, optimization_type}
    }
}

impl From<MpsModelWithSelectedVariants> for CroppedMpsModel {

    /// Crop MPS model leaving only user selected or default parts
    /// This method does not perform full validation, only fails if required working component
    fn from(mut initial_model: MpsModelWithSelectedVariants) -> Self {
        // Crop RHS
        if let Some(selected_rhs_name) = initial_model.selected_rhs {
            initial_model.model.rhs.rhs.retain(|name, _| *name == selected_rhs_name);
        }

        // Crop objective function
        let mut rows_to_remove: Vec<String> = Vec::new();
        if let Some(selected_obj_row_name) = initial_model.selected_opt_row_name {
            for (row_name, constriant) in &initial_model.model.rows.rows {
                if *constriant == Constraints::N && *row_name != selected_obj_row_name {
                    rows_to_remove.push(row_name.clone());
                }
            }
        }
        rows_to_remove.iter().for_each(|key| {initial_model.model.rows.rows.shift_remove(key);()});

        //Crop bounds
        if let Some(selected_bounds) = initial_model.selected_bounds {
            initial_model.model.bounds.bounds.retain(|x,y| *x == selected_bounds);
        }

        CroppedMpsModel::new(initial_model.model, initial_model.optimization_type)
    }
}

#[cfg(test)]
pub mod test_utils {
    use crate::parsers::mps::{BoundType, Bounds, Columns, Constraints, CroppedMpsModel, MpsModel, MpsModelWithSelectedVariants, Rhs, Rows};
    use crate::rationals::Rational;
    use crate::solvers::basic_simplex_table_data::OptimizationType;
    use crate::solvers::basic_simplex_table_data::OptimizationType::MIN;
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

    /// 2x1 + x2 <= -6 RHS1  ROW1
    /// x1 + x2 = 4 RHS1    ROW2
    /// x1 - x2 >= 1 RHS1    ROW3
    /// 3x1 + x2 -> N   OBJ1
    /// x1 <= -20    BND1
    /// x2 >= 5     BND1
    /// x2 ≥ 10     BND1
    pub fn create_cropped_mps_model_with_initially_unfeasible_rhs_and_bounds() -> CroppedMpsModel {
        let mut model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        model.rows.rows.shift_remove(&"OBJ2".to_owned());
        model.rhs.rhs.swap_remove(&"RHS2".to_owned());
        model.bounds.bounds.swap_remove(&"BND2".to_owned());

        model.rhs.rhs.get_mut(&"RHS1".to_owned()).unwrap().get_mut("ROW1").unwrap().negate_mut();
        model.bounds.bounds.get_mut(&"BND1".to_owned()).unwrap().remove(0);
        model.bounds.bounds.get_mut(&"BND1".to_owned()).unwrap().remove(3);
        model.bounds.bounds.get_mut(&"BND1".to_owned()).unwrap()[0].1.negate_mut();
        CroppedMpsModel::new(model, MIN)
    }

    /// 2x1 + x2 <= [6 (RHS1)]   ROW1
    /// x1 + x2 = [4 (RHS1)]     ROW2
    /// x1 - x2 >= [1 (RHS1)]    ROW3
    /// 3x1 + x2 -> N   OBJ1
    /// x1 <= 10    BND1
    /// x1 <= 20    BND1
    /// x2 >= 5     BND1
    /// x2 ≥ 10     BND1
    /// x2 <= 2     BND1
    pub fn create_rich_cropped_mps_model_for_test() -> CroppedMpsModel {
        let mut model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        model.rows.rows.shift_remove(&"OBJ2".to_owned());
        model.rhs.rhs.swap_remove(&"RHS2".to_owned());
        CroppedMpsModel::new(model, MIN)
    }

    #[test]
    fn invert_mut_constraint_succeeds() {
        let mut g = Constraints::G;
        let mut e = Constraints::E;
        g.invert_mut();
        e.invert_mut();
        assert_eq!(g, Constraints::L);
        assert_eq!(e, Constraints::E);
    }

    #[test]
    fn invert_mut_bound_type_succeeds() {
        let mut a = BoundType::UP;
        let mut b = BoundType::LO;

        a.invert_mut();
        b.invert_mut();

        assert_eq!(a, BoundType::LO);
        assert_eq!(b, BoundType::UP);
    }

    #[test]
    fn invert_bound_type_succeeds() {
        let mut a = BoundType::UP;
        let mut b = BoundType::LO;

        let c = a.invert();
        let d = b.invert();

        assert_eq!(c, BoundType::LO);
        assert_eq!(d, BoundType::UP);
    }

    #[test]
    fn mps_model_with_selected_variants_to_cropped_mps_succeeds() {
        let model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        let cropped = CroppedMpsModel::from(mps_model_with_selected_variants);

        assert_eq!(cropped.optimization_type, OptimizationType::MAX);
        assert_eq!(cropped.model.rhs.rhs.len(), 1);
        assert!(cropped.model.rhs.rhs.contains_key("RHS1"));


        let optimal_row_names = cropped.model.rows.rows.iter().filter(|(_, constraint)| **constraint == Constraints::N ).map(|(name, constraint)| name).collect::<Vec<&String>>();
        assert_eq!(optimal_row_names.len(), 1);
        assert_eq!(*optimal_row_names.first().unwrap(), "OBJ1");

        assert_eq!(cropped.model.bounds.bounds.len(), 1);
        assert_eq!(cropped.model.bounds.bounds.keys()[0], "BND1");
    }

}


