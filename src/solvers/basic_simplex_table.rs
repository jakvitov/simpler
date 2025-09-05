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

impl BasicSimplexTable {

    fn empty() -> Self {
        BasicSimplexTable {base_variable_names: Vec::new(), column_variable_names: Vec::new(),
            rows: Vec::new(), rhs: Vec::new()}
    }

}

impl TryFrom<&MpsModel> for BasicSimplexTable {
    type Error = Box<SimplexError>;

    /// Construct simplex table from supplied MPS model
    fn try_from(mps_model: &MpsModel) -> Result<Self, Self::Error> {
        let mut simplex_table = BasicSimplexTable::empty();

        let (variable_count, slack_surplus_variable_count, artificial_variable_count) = get_simplex_table_column_parts_length(mps_model);
        let (mut slack_surplus_index, mut artifical_index) = (variable_count, variable_count + slack_surplus_variable_count);
        check_if_exactly_one_objective_function_exists(mps_model)?;
        let row_names_ordered = mps_model.rows.rows.keys();

        simplex_table.column_variable_names = create_column_variable_names(mps_model, slack_surplus_variable_count, artificial_variable_count);
        if mps_model.rhs.rhs.len() != 1 {return Err(Box::new(SimplexError::from_string_reason(format!("MPS model specifies {} different RHSs.\nSimplex table requires exactly one to be created.", mps_model.rhs.rhs.len()))))}

        for row_name in row_names_ordered {
            let mut row: Vec<Rational> = Vec::new();
            //We iterate over row names (keys) unwrap is safe
            let constraint = mps_model.rows.rows.get(row_name).unwrap();

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
                            row.push(Rational::new(1,1));
                            pushed_artificial_variable = true;
                            artifical_index += 1;
                            simplex_table.base_variable_names.push(simplex_table.column_variable_names[i].to_owned());
                        },
                        Constraints::G => {
                            row.push(Rational::new(1,1));
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
            //Add RHS value
            if *constraint == Constraints::N {
                simplex_table.rhs.push(Rational::zero());
            } else {
                //We checked the exact number of rhs to 1 above. Unwrap is safe.
                let (rhs_name, rhs_values) = mps_model.rhs.rhs.iter().next().unwrap();
                let Some(rhs_value_for_row) = rhs_values.get(row_name) else {
                    return Err(Box::new(SimplexError::from_string_reason(format!("RHS {rhs_name} misses value for ROW {row_name}. Cannot construct simplex table!"))));
                };
                simplex_table.rhs.push(rhs_value_for_row.to_owned());
            }
        }
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

fn check_if_exactly_one_objective_function_exists(mps_model: &MpsModel) -> Result<(), Box<SimplexError>> {
    let mut objective_row_names = Vec::new();
    for (row_name, constraint) in &mps_model.rows.rows {
        if *constraint == Constraints::N {
            objective_row_names.push(row_name);
        }
    }
    if objective_row_names.len() == 1 {
        Ok(())
    } else if objective_row_names.len() == 0 {
        Err(Box::new(SimplexError::new("No objective row found in mps model.\nSimplex algorithm requires exactly one objective row.")))
    } else {
        let mut reason = String::from("Multiple objective rows found in Mps model.\nSimplex algorithm requires exactly one objective row.\nObjective rows: ");
        objective_row_names.iter().for_each(|row_name| {reason.push_str(&format!("\n{}", row_name));});
        Err(Box::new(SimplexError::new(reason.as_str())))
    }

}

/// Return vector of references to row names with the objective row at the end
/// Return SimplexError in case objective row does not exist
fn create_row_names_with_objective_at_the_end(mps_model: &MpsModel) -> Result<Vec<&String>, Box<SimplexError>> {
    let mut res: Vec<&String> = Vec::with_capacity(mps_model.columns.variables.len());
    let mut objective_row_name_optn: Option<&String> = None;

    for (row_name, constraint) in &mps_model.rows.rows {
        if *constraint == Constraints::N && objective_row_name_optn.is_some(){
            return Err(Box::new(SimplexError::new("Model contains more than one objective rows.\n Solver can handle only exactly one objective function at a time.")));
        } else if *constraint == Constraints::N{
            objective_row_name_optn = Some(row_name); continue
        }
        res.push(row_name);
    }

    let Some(objective_row_name) = objective_row_name_optn else {
        return Err(Box::new(SimplexError::new("Objective row does not exist.")));
    };

    res.push(objective_row_name);
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::parsers::mps;
    use crate::rationals::Rational;
    use crate::solvers::basic_simplex_table::{BasicSimplexTable};

    #[test]
    fn try_from_simple_mps_model_succeeds() {
        let model = mps::test_utils::create_simple_mps_model_for_tests();
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
    }

}

