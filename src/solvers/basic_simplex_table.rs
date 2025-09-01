use crate::parsers::mps::{BoundType, Constraints, MpsModel};
use crate::rationals::Rational;
use crate::solvers::simplex_error::SimplexError;
use std::collections::HashMap;

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
        let row_names_ordered = create_row_names_with_objective_at_the_end(mps_model);



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

#[cfg(test)]
mod tests {

}

