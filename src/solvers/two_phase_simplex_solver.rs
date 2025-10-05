
/*
  Solver, which solves simplex problem, given by simplex table with filled auxiliary variables, using
  two-phase simplex method.
 */
use crate::document::html_output::HtmlOutput;
use crate::rationals::Rational;
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;
use crate::solvers::simplex_error::SimplexError;
use crate::solvers::SimplexSoverAlgorithm::TWO_PHASE_SIMPLEX;

pub fn solve_two_phase_simplex(simplex_table: &mut BasicSimplexTable, html_output: &mut HtmlOutput) -> Result<(), Box<SimplexError>> {
    html_output.add_simplex_solver_header(TWO_PHASE_SIMPLEX);
    let original_objective_row = simplex_table.objective_row.clone();
    
    
        
    
    Ok(())
}

fn make_objective_row_for_auxiliary_minimalization(basic_simplex_table: &mut BasicSimplexTable) {
    basic_simplex_table.objective_row.clear();
    for i in 0..basic_simplex_table.column_variable_names.len() {
        if basic_simplex_table.artificial_variable_index.is_some() && i >= basic_simplex_table.artificial_variable_index.unwrap() {
            basic_simplex_table.objective_row.push(Rational::from_integer(1));
        } else {
            basic_simplex_table.objective_row.push(Rational::zero());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rationals::Rational;
    use crate::solvers::basic_simplex_table_data::test_utils::create_simplex_table_with_artificial_variables;
    use crate::solvers::two_phase_simplex_solver::make_objective_row_for_auxiliary_minimalization;

    #[test]
    fn make_objective_row_for_auxiliary_minimalization_succeeds() {
        let mut simplex_table = create_simplex_table_with_artificial_variables();
        make_objective_row_for_auxiliary_minimalization(&mut simplex_table);
        assert_eq!(simplex_table.objective_row.len(), 5);
        assert_eq!(simplex_table.objective_row, vec![Rational::zero(), Rational::zero(), Rational::zero(), Rational::zero(), Rational::from_integer(1)]);
    }
    
}




