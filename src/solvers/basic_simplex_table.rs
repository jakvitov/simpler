use crate::parsers::mps::{Constraints, MpsModel};
use crate::rationals::Rational;
use crate::solvers::simplex_error::SimplexError;

///Simplex table used for non-optimised simplex algorithms
struct BasicSimplexTable {
    base_variable_names: Vec<String>,
    column_variable_names: Vec<String>,
    rows: Vec<Vec<Rational>>,
    rhs: Vec<Rational>,
    objective_function_row: Vec<Rational>,
}

impl BasicSimplexTable {

    fn empty() -> Self {
        BasicSimplexTable {base_variable_names: Vec::new(), column_variable_names: Vec::new(),
            rows: Vec::new(), rhs: Vec::new(), objective_function_row: Vec::new()}
    }

}

impl TryFrom<&MpsModel> for BasicSimplexTable {
    type Error = Box<SimplexError>;

    /// Construct simplex table from supplied MPS model
    fn try_from(mps_model: &MpsModel) -> Result<Self, Self::Error> {
        let mut simplex_table = BasicSimplexTable::empty();

        let (variable_count, slack_surplus_variable_count, artificial_variable_count) = get_simplex_table_column_parts_length(mps_model);
        let (mut slack_surplus_index, mut artifical_index) = (variable_count, variable_count + slack_surplus_variable_count);
        let row_names_ordered = create_row_names_with_objective_at_the_end(mps_model)?;

        for row_name in row_names_ordered {
            let mut row: Vec<Rational> = Vec::new();
            let Some(constraint) = mps_model.rows.rows.get(row_name) else {
                return Err(create_generic_simplex_table_construction_error(mps_model));
            };

            //Fill in the variable values for rows
            for (variable_name, variable_values) in &mps_model.columns.variables {
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
                        },
                        Constraints::G => {
                            row.push(Rational::new(-1, 1));
                            pushed_slack_surplus = true;
                            slack_surplus_index += 1;
                        }
                        _ => {
                            row.push(Rational::new(0, 0));
                        }
                    }
                } else {
                    row.push(Rational::new(0, 0));
                }
            }

            //Fill in the artificial variables
            let mut pushed_artificial_variable = false;
            for i in (variable_count + slack_surplus_variable_count)..(variable_count + slack_surplus_variable_count + artificial_variable_count) {
                if i == artifical_index && !pushed_artificial_variable {
                    match constraint {
                        Constraints::E => {
                            row.push(Rational::new(1,1));
                            pushed_artificial_variable = true;
                            artifical_index += 1;
                        },
                        Constraints::G => {
                            row.push(Rational::new(1,1));
                            pushed_artificial_variable = true;
                            artifical_index += 1;
                        }
                        _ => {
                            row.push(Rational::new(0,0));
                        }
                    }
                }
            }
            simplex_table.rows.push(row);
        }



        Ok(simplex_table)
    }
}

///Return (variable count, slack/surplus variables count, artificial variables count)
fn get_simplex_table_column_parts_length(mps_model: &MpsModel) -> (usize, usize, usize) {
    let variables = mps_model.columns.variables.len();
    let mut slack_surplus_variables = 0;
    let mut artificial_variables = 0;

    for (_, constraint) in &mps_model.rows.rows {
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

/// Return vector of references to row names with the objective row at the end
/// Return SimplexError in case objective row does not exist
fn create_row_names_with_objective_at_the_end(mps_model: &MpsModel) -> Result<Vec<&String>, Box<SimplexError>> {
    let mut res: Vec<&String> = Vec::with_capacity(mps_model.columns.variables.len());
    let mut objective_row_name_optn: Option<&String> = None;
    for (row_name, constraint) in &mps_model.rows.rows {
        if constraint == &Constraints::N {objective_row_name_optn = Some(row_name); continue}
        res.push(row_name);
    }
    let Some(objective_row_name) = objective_row_name_optn else {
        return Err(Box::new(SimplexError::new("Objective row does not exist.")));
    };

    res.push(objective_row_name);
    Ok(res)
}

fn does_exactly_one_objective_function_exist(model: &MpsModel)  -> bool {
    let mut ob_function_met = false;
    for (_, constraint) in &model.rows.rows {
        if constraint == &Constraints::N {
            if ob_function_met{
                return true;
            }
            ob_function_met = true;
        }
    }
    ob_function_met == true
}

fn create_generic_simplex_table_construction_error(model: &MpsModel) -> Box<SimplexError> {
    Box::new(SimplexError::from_string_reason(format!("Internal application error occured while constructing the simplex table for model {}.\nThe fault is not at your side.\n", model.name)))
}

#[cfg(test)]
mod tests {

}

