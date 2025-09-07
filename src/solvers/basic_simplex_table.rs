use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use crate::parsers::mps::{Constraints, MpsModel};
use crate::rationals::Rational;
use crate::solvers::simplex_error::SimplexError;

///Simplex table used for non-optimised simplex algorithms
struct BasicSimplexTable {
    base_variable_names: Vec<String>,
    column_variable_names: Vec<String>,
    rows: Vec<Vec<Rational>>,
    rhs: Vec<Rational>,
}

struct MpsModelWithSelectedVariants {
    model: MpsModel,
    selected_rhs: Option<String>,
    selected_bounds: Option<String>,
    selected_opt_row_name: Option<String>,
}

impl BasicSimplexTable {

    fn empty() -> Self {
        BasicSimplexTable {base_variable_names: Vec::new(), column_variable_names: Vec::new(),
            rows: Vec::new(), rhs: Vec::new()}
    }

}

impl TryFrom<&MpsModelWithSelectedVariants> for BasicSimplexTable {

    type Error = Box<SimplexError>;

    //todo account for bounds selection
    /// Construct simplex table from supplied MPS model
    fn try_from(mps_model_with_selected_variants: &MpsModelWithSelectedVariants) -> Result<Self, Self::Error> {
        let mut simplex_table = BasicSimplexTable::empty();

        let (variable_count, slack_surplus_variable_count, artificial_variable_count) = get_simplex_table_column_parts_length(mps_model_with_selected_variants);
        let (mut slack_surplus_index, mut artifical_index) = (variable_count, variable_count + slack_surplus_variable_count);
        let row_constraint_names_ordered = get_row_names_with_selected_objective_function(mps_model_with_selected_variants)?;
        simplex_table.column_variable_names = create_column_variable_names(&mps_model_with_selected_variants.model, slack_surplus_variable_count, artificial_variable_count);

        let rhs: &HashMap<String, Rational> = get_selected_rhs_from_the_model(mps_model_with_selected_variants)?.deref();

        for (row_name, constraint) in row_constraint_names_ordered {
            let mut row: Vec<Rational> = Vec::new();
            //We iterate over row names (keys) unwrap is safe

            //Fill in the variable values for rows
            for (variable_name, variable_values) in &mps_model_with_selected_variants.model.columns.variables {
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
                            simplex_table.base_variable_names.push(simplex_table.column_variable_names[i].to_owned());
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
                if i == artifical_index && !pushed_artificial_variable {
                    match constraint {
                        Constraints::E => {
                            row.push(Rational::new(1, 1));
                            pushed_artificial_variable = true;
                            artifical_index += 1;
                            simplex_table.base_variable_names.push(simplex_table.column_variable_names[i].to_owned());
                        },
                        Constraints::G => {
                            row.push(Rational::new(1, 1));
                            pushed_artificial_variable = true;
                            artifical_index += 1;
                            simplex_table.base_variable_names.push(simplex_table.column_variable_names[i].to_owned());
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
            if *constraint != Constraints::N {
                let Some(rhs_value) = rhs.get(row_name) else {
                    return Err(Box::new(SimplexError::from_string_reason(format!("Row {row_name} is not specified in the supplied RHS.\nSolver cannot optimise."))));
                };
                simplex_table.rhs.push(rhs_value.to_owned());
            }
    }
    //Fill in the RHS for objective function
    simplex_table.rhs.push(Rational::zero());
    //todo add bounds
        Ok(simplex_table)
    }
}

fn create_column_variable_names(mps_model: &MpsModel, slack_surplus_count: usize, artificial_variable_count: usize) -> Vec<String> {
    let mut variable_names = Vec::with_capacity(mps_model.columns.variables.len() + slack_surplus_count + artificial_variable_count);
    mps_model.columns.variables.keys().for_each(|variable_name| {variable_names.push(variable_name.to_owned());});
    for i in 0..slack_surplus_count {
        variable_names.push(format!("S{}", i+1));
    }
    for i in 0..artificial_variable_count {
        variable_names.push(format!("A{}", i+1));
    }
    variable_names
}

/// Set selected RHS to target_rhs or Return error explaining, why that failed
fn get_selected_rhs_from_the_model(mps_model_with_selected_variants: &MpsModelWithSelectedVariants) -> Result<Box<&HashMap<String,Rational>>, Box<SimplexError>> {
    if mps_model_with_selected_variants.selected_rhs.is_none() && mps_model_with_selected_variants.model.rhs.rhs.len() > 1 {
        return Err(Box::new(SimplexError::new("No RHS selected, but model contains multiple RHS.\nIf more RHS are supplied one needs to be chosen.")));
    } else if mps_model_with_selected_variants.model.rhs.rhs.is_empty() {
        return Err(Box::new(SimplexError::new("No RHS was supplied. Cannot optimise model without RHS.")));
    } else if mps_model_with_selected_variants.selected_rhs.is_none() && mps_model_with_selected_variants.model.rhs.rhs.len() == 1 {
        return Ok(Box::new(mps_model_with_selected_variants.model.rhs.rhs.iter().next().unwrap().1));
    }
    let rhs_target_name = mps_model_with_selected_variants.selected_rhs.as_ref().unwrap();
    let matching_rhs: Vec<(&String, &HashMap<String, Rational>)> = mps_model_with_selected_variants.model.rhs.rhs.iter().filter(|(name, _)| **name == *rhs_target_name).collect();
    if matching_rhs.is_empty() {
        return Err(Box::new(SimplexError::from_string_reason(format!("No RHS with name {rhs_target_name} found in the model.\n Solver cannot optimise."))));
    } else if matching_rhs.len() > 1 {
        return Err(Box::new(SimplexError::from_string_reason(format!("Multiple RHS with name {rhs_target_name} found.\nSolver can optimise only on exactly one selected RHS."))));
    }
    Ok(Box::new(matching_rhs[0].1))
}

///Return (variable count, slack/surplus variables count, artificial variables count)
fn get_simplex_table_column_parts_length(mps_model_with_selected_variants: &MpsModelWithSelectedVariants) -> (usize, usize, usize) {
    let variables = mps_model_with_selected_variants.model.columns.variables.len();
    let mut slack_surplus_variables = 0;
    let mut artificial_variables = 0;

    for (_, constraint) in &mps_model_with_selected_variants.model.rows.rows {
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
    (variables, slack_surplus_variables, artificial_variables)
}

/// Return row names with the selected objective at the end
/// Return error if it is not possible with explanation why
fn get_row_names_with_selected_objective_function(mps_model_with_selected_variants: &MpsModelWithSelectedVariants) -> Result<Vec<(&String,&Constraints)>, Box<SimplexError>> {
    let mut objective_row_names = HashSet::new();
    let mut non_objective_row_names: Vec<(&String, &Constraints)> = Vec::new();
    for (row_name, constraint) in &mps_model_with_selected_variants.model.rows.rows {
        if *constraint == Constraints::N {
            objective_row_names.insert(row_name);
        } else {
            non_objective_row_names.push((row_name, constraint));
        }
    }
    if mps_model_with_selected_variants.selected_opt_row_name.is_none() && objective_row_names.len() > 1 {
        Err(Box::new(SimplexError::new("No objective function name was chosen and model contains more than one objective function.\n Chose one objective function to be used.")))
    } else if objective_row_names.is_empty() {
        Err(Box::new(SimplexError::new("Model does not contain any objective function.\nSimplex solver cannot optimise this model without objective function.")))
    } else if mps_model_with_selected_variants.selected_opt_row_name.is_none() && objective_row_names.len() == 1{
        non_objective_row_names.push((objective_row_names.iter().next().unwrap(), &Constraints::N));
            Ok(non_objective_row_names)
    }
    else {
        let obj_function_row_name = mps_model_with_selected_variants.selected_opt_row_name.as_ref().unwrap();
        if !objective_row_names.contains(obj_function_row_name) {
            return Err(Box::new(SimplexError::from_string_reason(format!("Objective function named {obj_function_row_name} was not found in specified rows.\nSimplex solver cannot optimise this model."))));
        }
        non_objective_row_names.push((obj_function_row_name, &Constraints::N));
        Ok(non_objective_row_names)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::mps;
    use crate::rationals::Rational;
    use crate::solvers::basic_simplex_table::BasicSimplexTable;

    #[test]
    fn try_from_simple_mps_model_succeeds() {
        /*let model = mps::test_utils::create_simple_mps_model_for_tests();
        let simplex_table = BasicSimplexTable::try_from(&model).unwrap();

        assert_eq!(simplex_table.base_variable_names, vec!("S1", "A1", "A2"));
        assert_eq!(simplex_table.column_variable_names, vec!("x1", "x2", "S1", "S2", "A1", "A2"));
        assert_eq!(simplex_table.rhs, vec![Rational::from_integer(6),Rational::from_integer(4)
                                           ,Rational::from_integer(1),Rational::zero()]);
        assert_eq!(simplex_table.rows.len(), 4);
        assert_eq!(simplex_table.rows[0], vec![Rational::from_integer(2),Rational::from_integer(1),
                                               Rational::from_integer(1),Rational::from_integer(0),
                                               Rational::from_integer(0),Rational::from_integer(0),]);
        assert_eq!(simplex_table.rows[1], vec![Rational::from_integer(1),Rational::from_integer(1),
                                               Rational::from_integer(0),Rational::from_integer(0),
                                               Rational::from_integer(1),Rational::from_integer(0),]);
        assert_eq!(simplex_table.rows[2], vec![Rational::from_integer(1),Rational::from_integer(-1),
                                               Rational::from_integer(0),Rational::from_integer(-1),
                                               Rational::from_integer(0),Rational::from_integer(1),]);
        assert_eq!(simplex_table.rows[3], vec![Rational::from_integer(-3),Rational::from_integer(-2),
                                               Rational::from_integer(0),Rational::from_integer(0),
                                               Rational::from_integer(0),Rational::from_integer(0),]);
    */
    }

    #[test]
    fn try_from_only_equals_mps_model_succeeds() {
        /*let model = mps::test_utils::create_mps_model_with_only_equals();
        let simplex_table = BasicSimplexTable::try_from(&model).unwrap();

        assert_eq!(simplex_table.base_variable_names, vec!("A1", "A2"));
        assert_eq!(simplex_table.column_variable_names, vec!("x1", "x2", "A1", "A2"));
        assert_eq!(simplex_table.rhs, vec![Rational::new(5,2), Rational::new(-10,3), Rational::new(0,1)]);
        assert_eq!(simplex_table.rows.len(), 3);
        assert_eq!(simplex_table.rows[0], vec![Rational::new(2,5), Rational::new(-3,2), Rational::from_integer(1), Rational::zero()]);
        assert_eq!(simplex_table.rows[1], vec![Rational::new(3,2), Rational::new(1,5), Rational::zero(), Rational::from_integer(1)]);
        assert_eq!(simplex_table.rows[2], vec![Rational::from_integer(-1), Rational::from_integer(-1), Rational::zero(), Rational::zero()]);
    */
    }

    //todo test fail on none objective rows
    //todo test fail on multiple objective rows
    //todo test fail on none rhs
    //todo test fail on multiple rhs

}

