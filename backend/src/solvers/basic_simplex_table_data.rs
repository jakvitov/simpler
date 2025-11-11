use super::simplex_error::SimplexError;
use crate::document::html_convertible_error::HtmlConvertibleError;
use crate::parsers::mps::{BoundType, Constraints, CroppedMpsModel, MpsModel, MpsModelWithSelectedVariants};
use crate::rationals::Rational;
use crate::utils::ApplicationError;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use serde::{Deserialize, Serialize};

///Simplex table used for non-optimised simplex algorithms
pub struct BasicSimplexTable {
    pub(crate) base_variable_names: Vec<String>,
    pub(crate) column_variable_names: IndexMap<String, usize>,
    pub(crate) rows: Vec<Vec<Rational>>,
    pub(crate) rhs: Vec<Rational>,
    pub(crate) objective_row: Vec<Rational>,
    pub(crate) objective_rhs: Rational,
    pub(crate) optimization_type: OptimizationType,
    pub(crate) artificial_variable_index: Option<usize>
}

#[derive(Serialize, Deserialize, Debug, Copy, Eq, PartialEq, Clone)]
pub enum OptimizationType {
    MAX,
    MIN
}

impl Display for OptimizationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl BasicSimplexTable {

    fn empty(optimization_type: OptimizationType) -> Self {
        BasicSimplexTable {base_variable_names: Vec::new(), column_variable_names: IndexMap::new(),
            rows: Vec::new(), rhs: Vec::new(), objective_row: Vec::new(), objective_rhs: Rational::zero(),
            optimization_type: optimization_type, artificial_variable_index: None
        }
    }

    pub fn get_column_count_without_rhs_and_base(&self) -> usize {
        if self.rows.is_empty() {
            0
        } else {
            self.rows[0].len()
        }
    }
    
    /// Remove artificial variables from the simplex table
    /// and add the original_objective_row
    pub(super) fn eliminate_artifical_variables_from_simplex_table(&mut self, original_objective_row: Vec<Rational>) {
        let Some(artificial_variable_index) = self.artificial_variable_index else {
            return;
        };

        self.column_variable_names.drain(artificial_variable_index..);
        self.rows.iter_mut().for_each(|row| {row.drain(artificial_variable_index..);});
        self.objective_row = original_objective_row;
        self.objective_row.drain(artificial_variable_index..);
    }
}

impl TryFrom<&CroppedMpsModel> for BasicSimplexTable {

    type Error = Box<dyn HtmlConvertibleError>;

    /// Construct simplex table from supplied MPS model
    fn try_from(mps_model: &CroppedMpsModel) -> Result<Self, Self::Error> {
        let mut simplex_table = BasicSimplexTable::empty(mps_model.optimization_type);

        let (variable_count, slack_surplus_variable_count, artificial_variable_count) = get_simplex_table_column_parts_length(mps_model);
        let (mut slack_surplus_index, mut artificial_index) = (variable_count, variable_count + slack_surplus_variable_count);
        let row_constraint_names_ordered = get_row_names_with_selected_objective_function(mps_model).map_err(|e| e as Box<dyn HtmlConvertibleError>)?;
        simplex_table.column_variable_names = create_column_variable_names(&mps_model.model, slack_surplus_variable_count, artificial_variable_count);

        let rhs: &HashMap<String, Rational> = get_selected_rhs_from_the_model(mps_model).map_err(|x| x as Box<dyn HtmlConvertibleError>)?.deref();

        if artificial_variable_count > 0 {
            simplex_table.artificial_variable_index = Some(artificial_index);
        }

        //Fill in rows except the objective one
        for &(row_name, constraint) in &row_constraint_names_ordered {
            //Skip the objective row, we'll add that separately
            if *constraint == Constraints::N {
                continue;
            }
            let mut row: Vec<Rational> = Vec::new();
            //We iterate over row names (keys) unwrap is safe

            //Fill in the variable values for rows
            for (variable_name, variable_values) in &mps_model.model.columns.variables {
                let variable_value_for_row = variable_values.get(row_name).map_or(Rational::zero(), |x| x.to_owned());
                row.push(variable_value_for_row);
            }

            //Without this, we update slack_surplus_index and trigger that if on next iteration
            let mut pushed_slack_surplus = false;
            //Fill in the slack/surplus variables
            for i in variable_count..(variable_count + slack_surplus_variable_count) {
                //If we need to insert value for slack/surplus, we do it here
                if i == slack_surplus_index && !pushed_slack_surplus {
                    match constraint {
                        Constraints::L => {
                            row.push(Rational::new(1, 1));
                            pushed_slack_surplus = true;
                            slack_surplus_index += 1;
                            simplex_table.base_variable_names.push(simplex_table.column_variable_names.keys()[i].to_owned());
                        },
                        Constraints::G => {
                            row.push(Rational::new(-1, 1));
                            pushed_slack_surplus = true;
                            slack_surplus_index += 1;
                        }
                        _ => {
                            row.push(Rational::zero());
                        }
                    }
                } else {
                    row.push(Rational::zero());
                }
            }

            //Fill in the artificial variables
            let mut pushed_artificial_variable = false;
            for i in (variable_count + slack_surplus_variable_count)..(variable_count + slack_surplus_variable_count + artificial_variable_count) {
                if i == artificial_index && !pushed_artificial_variable {
                    match constraint {
                        Constraints::E => {
                            row.push(Rational::new(1, 1));
                            pushed_artificial_variable = true;
                            artificial_index += 1;
                            simplex_table.base_variable_names.push(simplex_table.column_variable_names.keys()[i].to_owned());
                        },
                        Constraints::G => {
                            row.push(Rational::new(1, 1));
                            pushed_artificial_variable = true;
                            artificial_index += 1;
                            simplex_table.base_variable_names.push(simplex_table.column_variable_names.keys()[i].to_owned());
                        }
                        _ => {
                            row.push(Rational::zero());
                        }
                    }
                } else {
                    row.push(Rational::zero());
                }
            }
            simplex_table.rows.push(row);

            //Handle selected RHS, objective function is added at the end once
            let Some(rhs_value) = rhs.get(row_name) else {
                return Err(Box::new(SimplexError::from_string_reason(format!("Row {row_name} is not specified in the supplied RHS.\nSolver cannot optimise."))));
            };
            simplex_table.rhs.push(rhs_value.to_owned());
    }

    //Fill in the objective row
    let objective_row_name = row_constraint_names_ordered.iter().next_back().unwrap().0;
    for (_, variable_values) in &mps_model.model.columns.variables {
            let variable_value_for_row = variable_values.get(objective_row_name).map_or(Rational::zero(), |x| x.to_owned());
            simplex_table.objective_row.push(variable_value_for_row.negate());
    }
    for i in 0..(artificial_variable_count + slack_surplus_variable_count) {
        simplex_table.objective_row.push(Rational::zero());
    }
    simplex_table.objective_rhs = Rational::zero();

    if let Some(model_bounds) = mps_model.model.bounds.bounds.first() {
        //Fill in the bounds
        for(variable_name, value, bound_type) in model_bounds.1 {
            let mut row = Vec::new();
            let Some(variable_index) = simplex_table.column_variable_names.get(variable_name) else {
                return Err(Box::new(SimplexError::from_string_reason(format!("Variable {variable_name} specified in bounds was not found among the model variables.\nSolver cannot optimise."))));
            };
            simplex_table.rhs.push(value.to_owned());
            for i in 0..variable_count {
                if i == *variable_index {
                    row.push(Rational::new(1, 1));
                } else {
                    row.push(Rational::zero());
                }
            }

            let mut push_slack_surplus_variable = false;
            for i in variable_count..(variable_count + slack_surplus_variable_count) {
                if i == slack_surplus_index && !push_slack_surplus_variable {
                    match bound_type {
                        BoundType::UP => {
                            row.push(Rational::new(1, 1));
                            simplex_table.base_variable_names.push(simplex_table.column_variable_names.keys()[i].to_owned());
                            push_slack_surplus_variable = true;
                            slack_surplus_index += 1;
                        },
                        BoundType::LO => {
                            row.push(Rational::new(-1, 1));
                            push_slack_surplus_variable = true;
                            slack_surplus_index += 1;
                        }
                    }
                } else {
                    row.push(Rational::zero());
                }
            }

            let mut pushed_artificial_variable = false;
            for i in (variable_count + slack_surplus_variable_count)..(variable_count + slack_surplus_variable_count + artificial_variable_count) {
                if i == artificial_index && !pushed_artificial_variable {
                    match bound_type {
                        BoundType::UP => {
                            row.push(Rational::zero());
                        },
                        BoundType::LO => {
                            row.push(Rational::new(1, 1));
                            simplex_table.base_variable_names.push(simplex_table.column_variable_names.keys()[i].to_owned());
                            pushed_artificial_variable = true;
                            artificial_index += 1;
                        }
                    }
                } else {
                    row.push(Rational::zero());
                }
            }
    simplex_table.rows.push(row);
        }
    }
    Ok(simplex_table)
    }
}

fn create_column_variable_names(mps_model: &MpsModel, slack_surplus_count: usize, artificial_variable_count: usize) -> IndexMap<String, usize> {
    let mut variable_names = IndexMap::with_capacity(mps_model.columns.variables.len() + slack_surplus_count + artificial_variable_count);
    let mut index = 0usize;
    mps_model.columns.variables.keys().for_each(|variable_name| {variable_names.insert(variable_name.to_owned(), index); index += 1});
    for i in 0..slack_surplus_count {
        variable_names.insert(format!("S{}", i+1), index);
        index += 1;
    }
    for i in 0..artificial_variable_count {
        variable_names.insert(format!("A{}", i+1), index);
        index += 1;
    }
    variable_names
}

/// Set selected RHS to target_rhs or Return error explaining, why that failed. If none are specified and one exist, return that.
fn get_selected_rhs_from_the_model(mps_model: &CroppedMpsModel) -> Result<Box<&HashMap<String,Rational>>, Box<ApplicationError>> {
    if let Some(rhs) = mps_model.model.rhs.rhs.first() {
        Ok(Box::new(rhs.1))
    } else {
        Err(Box::new(ApplicationError::new("No RHS was supplied in cropped model. Cannot optimise model without RHS.", "Application error during simplex table parsing.")))
    }
}

///Return (variable count, slack/surplus variables count, artificial variables count)
fn get_simplex_table_column_parts_length(mps_model: &CroppedMpsModel) -> (usize, usize, usize) {
    let variables = mps_model.model.columns.variables.len();
    let mut slack_surplus_variables = 0;
    let mut artificial_variables = 0;

    for (_, constraint) in &mps_model.model.rows.rows {
        match  constraint {
            Constraints::N => (),
            Constraints::E => artificial_variables += 1,
            Constraints::G => {
                slack_surplus_variables += 1;
                artificial_variables += 1;
            }
            Constraints::L => slack_surplus_variables += 1,
        }
    }

    if let Some(model_bounds) = mps_model.model.bounds.bounds.first() {
        for (_, _, bound_type) in model_bounds.1 {
            match *bound_type {
                BoundType::UP => slack_surplus_variables += 1,
                BoundType::LO => {
                    artificial_variables += 1;
                    slack_surplus_variables += 1;
                }
            }
        }
    }

    (variables, slack_surplus_variables, artificial_variables)
}

/// Return row names with the selected objective at the end
/// Return error if it is not possible with explanation why
fn get_row_names_with_selected_objective_function(mps_model: &CroppedMpsModel) -> Result<Vec<(&String,&Constraints)>, Box<ApplicationError>> {
    let mut objective_row_name = None;
    let mut non_objective_row_names: Vec<(&String, &Constraints)> = Vec::new();
    for (row_name, constraint) in &mps_model.model.rows.rows {
        if *constraint == Constraints::N {
            objective_row_name = Some(row_name);
        } else {
            non_objective_row_names.push((row_name, constraint));
        }
    }
    if let Some(obj_r_name) = objective_row_name {
        non_objective_row_names.push((obj_r_name, &Constraints::N));
        Ok(non_objective_row_names)
    }
    else {
        return Err(Box::new(ApplicationError::new("Cropped model does not contain any objective function.", "Error occured while obtaining objective row in simplex table parsing.")));
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::mps;
    use crate::parsers::mps::CroppedMpsModel;
    use crate::rationals::Rational;
    use crate::solvers::basic_simplex_table_data::test_utils::create_simplex_table_with_artificial_variables;
    use crate::solvers::basic_simplex_table_data::{BasicSimplexTable, OptimizationType};

    ///Shortened version of Rational::from_integer
    pub fn rfi(input: i128) -> Rational {
        Rational::from_integer(input)
    }

    ///Shortened version of Rational::zero
    pub fn rz() -> Rational {
        Rational::zero()
    }

    #[test]
    fn try_from_simple_mps_model_no_bounds_one_rhs_one_objective_succeeds() {
        let model = mps::test_utils::create_simple_mps_model_for_tests_no_bounds_one_rhs_one_objective();

        let cropped_model = CroppedMpsModel::new(model, OptimizationType::MIN);
        let simplex_table = BasicSimplexTable::try_from(&cropped_model).unwrap();

        assert_eq!(simplex_table.base_variable_names, vec!("S1", "A1", "A2"));
        assert_eq!(simplex_table.column_variable_names.keys().collect::<Vec<&String>>(), vec!("x1", "x2", "S1", "S2", "A1", "A2"));
        assert_eq!(simplex_table.rhs, vec![Rational::from_integer(6),Rational::from_integer(4)
                                           ,Rational::from_integer(1)]);
        assert_eq!(simplex_table.rows.len(), 3);
        assert_eq!(simplex_table.artificial_variable_index.unwrap(), 4);
        assert_eq!(simplex_table.rows[0], vec![Rational::from_integer(2),Rational::from_integer(1),
                                               Rational::from_integer(1),Rational::from_integer(0),
                                               Rational::from_integer(0),Rational::from_integer(0),]);
        assert_eq!(simplex_table.rows[1], vec![Rational::from_integer(1),Rational::from_integer(1),
                                               Rational::from_integer(0),Rational::from_integer(0),
                                               Rational::from_integer(1),Rational::from_integer(0),]);
        assert_eq!(simplex_table.rows[2], vec![Rational::from_integer(1),Rational::from_integer(-1),
                                               Rational::from_integer(0),Rational::from_integer(-1),
                                               Rational::from_integer(0),Rational::from_integer(1),]);
        assert_eq!(simplex_table.objective_row, vec![Rational::from_integer(-3),Rational::from_integer(-2),
                                               Rational::from_integer(0),Rational::from_integer(0),
                                               Rational::from_integer(0),Rational::from_integer(0),]);
        assert_eq!(simplex_table.objective_rhs, Rational::zero());
    
    }

    #[test]
    fn try_from_only_equals_mps_model_succeeds() {
        let model = mps::test_utils::create_mps_model_with_only_equals_no_bounds_one_rhs_one_objective();
        let cropped_model = CroppedMpsModel::new(model, OptimizationType::MIN);
        let simplex_table = BasicSimplexTable::try_from(&cropped_model).unwrap();


        assert_eq!(simplex_table.base_variable_names, vec!("A1", "A2"));
        assert_eq!(simplex_table.column_variable_names.keys().collect::<Vec<&String>>(), vec!("x1", "x2", "A1", "A2"));
        assert_eq!(simplex_table.rhs, vec![Rational::new(5,2), Rational::new(-10,3)]);
        assert_eq!(simplex_table.rows.len(), 2);
        assert_eq!(simplex_table.artificial_variable_index.unwrap(), 2);
        assert_eq!(simplex_table.rows[0], vec![Rational::new(2,5), Rational::new(-3,2), Rational::from_integer(1), Rational::zero()]);
        assert_eq!(simplex_table.rows[1], vec![Rational::new(3,2), Rational::new(1,5), Rational::zero(), Rational::from_integer(1)]);
        assert_eq!(simplex_table.objective_row, vec![Rational::from_integer(-1), Rational::from_integer(-1), Rational::zero(), Rational::zero()]);
        assert_eq!(simplex_table.objective_rhs, Rational::zero());
    }

    #[test]
    fn try_from_mps_model_with_chosen_rhs_chosen_objective_chosen_optimisable_bounds() {
        let cropped_model = mps::test_utils::create_rich_cropped_mps_model_for_test_with_optimised_bounds();



        let simplex_table = BasicSimplexTable::try_from(&cropped_model).unwrap();
        assert_eq!(simplex_table.column_variable_names.keys().collect::<Vec<&String>>(), vec!("x1", "x2", "S1", "S2", "S3", "S4", "S5", "A1", "A2", "A3") );
        assert_eq!(simplex_table.rhs, vec![Rational::from_integer(6), Rational::from_integer(4), Rational::from_integer(1), Rational::from_integer(10), Rational::from_integer(10), Rational::from_integer(20)]);
        assert_eq!(simplex_table.artificial_variable_index.unwrap(), 7);
        assert_eq!(simplex_table.rows.len(), 6);
        assert_eq!(simplex_table.rows[0], vec![rfi(2), rfi(1), rfi(1), rz(), rz(), rz(), rz(), rz(), rz(), rz()]);
        assert_eq!(simplex_table.rows[1], vec![rfi(1), rfi(1), rz(), rz(), rz(), rz(), rz(), rfi(1), rz(), rz()]);
        assert_eq!(simplex_table.rows[2], vec![rfi(1), rfi(-1), rfi(0), rfi(-1), rz(), rz(), rz(), rz(), rfi(1), rz()]);
        assert_eq!(simplex_table.rows[3], vec![rfi(1), rz(), rz(), rz(), rfi(1), rz(), rz(), rz(), rz(), rz()]);
        assert_eq!(simplex_table.rows[4], vec![rz(), rfi(1), rz(), rz(), rz(), rfi(-1), rz(), rz(), rz(), rfi(1)]);
        assert_eq!(simplex_table.rows[5], vec![rz(), rfi(1), rz(), rz(), rz(), rz(), rfi(1), rz(), rz(), rz()]);
        assert_eq!(simplex_table.objective_row, vec![rfi(-3), rfi(-1), rz(),  rz(), rz(), rz(), rz(), rz(), rz(), rz(),])
    }

    #[test]
    fn create_simplex_table_with_artificial_variables_succeeds() {
        let mut simplex_table = create_simplex_table_with_artificial_variables();
        let original_objective_row = vec![rfi(-10), rfi(-20), rfi(0), rfi(0), rfi(100)];

        simplex_table.eliminate_artifical_variables_from_simplex_table(original_objective_row);

        assert_eq!(simplex_table.column_variable_names.keys().collect::<Vec<&String>>(), vec!["x1","x2","S1","S2"]);
        assert_eq!(simplex_table.rows[0], vec![rfi(1), rfi(2), rfi(1), rfi(0)]);
        assert_eq!(simplex_table.rows[1], vec![rfi(2), rfi(1), rfi(0), rfi(-1)]);
        assert_eq!(simplex_table.objective_row, vec![rfi(-10), rfi(-20), rfi(0), rfi(0)]);
    }

}

#[cfg(test)]
pub mod test_utils {
    use crate::solvers::basic_simplex_table_data::tests::{rfi, rz};
    use crate::solvers::basic_simplex_table_data::{BasicSimplexTable, OptimizationType};
    use indexmap::IndexMap;

    /// Base x1 x2 s1 s2 RHS
    /// s1  1   2   1  0  2
    /// s2  2   1   0  1  3
    /// ob -1  -2   0  0  0
    pub fn create_minimal_simplex_table_for_testing() -> BasicSimplexTable{
        let mut column_variable_names = IndexMap::new();
        column_variable_names.insert("x1".to_string(), 0);
        column_variable_names.insert("x2".to_string(), 1);
        column_variable_names.insert("S1".to_string(), 2);
        column_variable_names.insert("S2".to_string(), 3);
        BasicSimplexTable {
            base_variable_names: vec!["S1".to_owned(), "S2".to_owned()],
            column_variable_names:  column_variable_names,
            rows: vec![
                vec![rfi(1), rfi(2), rfi(1), rfi(0)],
                vec![rfi(2), rfi(1), rfi(0), rfi(1)],
            ],
            rhs: vec![rfi(2),rfi(3)],
            objective_row: vec![rfi(-1), rfi(-2), rfi(0), rfi(0)],
            objective_rhs: rfi(0),
            artificial_variable_index: None,
            optimization_type: super::OptimizationType::MAX
        }
    }
    
    /// Base x1 x2 s1 s2 RHS
    /// s1  1   2   1  0  2
    /// s2  2   1   0  1  3
    /// ob  1   2   0  0  0
    pub fn create_optimal_simplex_table() -> BasicSimplexTable {
        let mut res = create_minimal_simplex_table_for_testing();
        res.objective_row[0] = rfi(1);
        res.objective_row[1] = rfi(2);
        res
    }

    /// Base x1  x2 s1 s2 RHS
    /// s1  1   -2   1  0  2
    /// s2  2   -1   0  1  3
    /// ob -1   -2   0  0  0
    pub fn create_unbounded_simplex_table() -> BasicSimplexTable {
        let mut res = create_minimal_simplex_table_for_testing();
        res.rows[0][1] = res.rows[0][1].negate();
        res.rows[1][1] = res.rows[1][1].negate();
        res
    }

    /// Base x1 x2 s1 s2  A1 RHS
    /// s1  1   2   1  0   0  2
    /// A1  2   1   0 -1   1  3
    /// ob -1  -2   0  0   0  0
    /// Solution 1
    pub fn create_simplex_table_with_artificial_variables() -> BasicSimplexTable {
        let mut res = create_minimal_simplex_table_for_testing();
        res.column_variable_names.insert("A1".to_owned(), 4);
        res.rows[0].push(rfi(0));
        res.rows[1].push(rfi(1));
        res.rows[1][3] = res.rows[1][3].negate();
        res.base_variable_names[1] = "A1".to_owned();
        res.artificial_variable_index = Some(4);
        res
    }

    pub fn create_cycling_simplex_table() -> BasicSimplexTable {
        let mut simplex_table = BasicSimplexTable::empty(OptimizationType::MAX);
        simplex_table.column_variable_names.insert("x1".to_owned(), 1);
        simplex_table.column_variable_names.insert("x2".to_owned(), 2);
        simplex_table.column_variable_names.insert("x3".to_owned(), 3);
        simplex_table.column_variable_names.insert("S1".to_owned(), 4);
        simplex_table.column_variable_names.insert("S2".to_owned(), 5);
        simplex_table.column_variable_names.insert("S3".to_owned(), 6);

        simplex_table.base_variable_names = vec!["S1".to_owned(), "S2".to_owned(), "S3".to_owned()];

        simplex_table.rhs = vec![rz(), rz(), rfi(-1)];

        simplex_table.rows = vec![vec![rfi(1), rfi(-2), rfi(-1), rfi(1), rz(), rz()],
        vec![rfi(-2), rfi(1), rfi(-1), rz(), rfi(1), rz()],
        vec![rfi(-1), rfi(-1), rfi(1), rz(), rz(), rfi(1)]];

        simplex_table.objective_row = vec![rfi(1), rfi(-10), rfi(-10), rz(), rz(), rz()];
        simplex_table.objective_rhs = rz();

        simplex_table.artificial_variable_index = None;
        simplex_table
    }
}

