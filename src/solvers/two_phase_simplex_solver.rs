use crate::document::html_convertible_error::HtmlConvertibleError;
use crate::document::html_output::HtmlOutput;
use crate::rationals::{GcdCache, Rational};
use crate::solvers::basic_simplex_solver;
use crate::solvers::basic_simplex_solver::solve_basic_simplex_table;
use crate::solvers::basic_simplex_table_data::{BasicSimplexTable, OptimizationType};
use crate::solvers::simplex_error::SimplexError;
use crate::solvers::SimplexSoverAlgorithm::TWO_PHASE_SIMPLEX;

/*
  Solver, which solves simplex problem, given by simplex table with filled auxiliary variables, using
  two-phase simplex method.
 */

pub fn solve_two_phase_simplex(simplex_table: &mut BasicSimplexTable, html_output: &mut HtmlOutput) -> Result<Option<Rational>, Box<dyn HtmlConvertibleError>> {
    html_output.add_simplex_solver_header(TWO_PHASE_SIMPLEX);

    if simplex_table.artificial_variable_index.is_none() {
        html_output.add_basic_simplex_chosen_instead();
        return basic_simplex_solver::solve_basic_simplex(simplex_table, html_output).map_err(|e| e as Box<dyn HtmlConvertibleError>);
    }

    let mut gcd_cache = GcdCache::init();
    let original_objective_row = simplex_table.objective_row.clone();

    make_objective_row_for_auxiliary_minimalization(simplex_table);

    html_output.add_initial_unfeasible_auxiliary_table(simplex_table);

    // Make initially feasible phase I tableau - eliminate artificial variables from objective row
    for artificial_variable_index in simplex_table.artificial_variable_index.unwrap()..simplex_table.column_variable_names.len() {
        let Some(row_index) = find_row_where_artificial_variable_is_non_zero(artificial_variable_index, simplex_table) else {
            return Err(Box::new(SimplexError::from_string_reason(format!("Cannot eliminate artificial variable {} from objective row, since it is not present in any row.", simplex_table.column_variable_names[artificial_variable_index]))))
        };
        let pivot = (row_index, artificial_variable_index);
        let coefficient = simplex_table.normalize_row_by_pivot_row(&pivot, simplex_table.rows.len(), &mut gcd_cache).map_err(|e| e as Box<dyn HtmlConvertibleError>)?;
        html_output.add_row_normalisation_by_pivot_row_to_html_document(simplex_table, &coefficient, &pivot, simplex_table.rows.len());
    }

    //Phase I
    html_output.add_starting_phase_one_dual_simplex_header();
    let mut iteration_counter = 1;
    loop {
        let pessimal_column = basic_simplex_solver::check_optimity(simplex_table);
        if pessimal_column.is_none() && simplex_table.objective_rhs == Rational::zero() {
            html_output.add_finished_phase_one_dual_simplex_info(simplex_table.objective_rhs);
            break;
        } else if pessimal_column.is_none() {
            //Infeasible found in Phase I
            html_output.add_finished_phase_one_dual_simplex_info(simplex_table.objective_rhs);
            return Ok(None);
        }
        
        html_output.start_simplex_iteration(iteration_counter);
        let t_vec = basic_simplex_solver::get_t_vector(simplex_table, &pessimal_column.unwrap(), &mut gcd_cache).map_err(|e| e as Box<dyn HtmlConvertibleError>)?;
        let mut all_negative = true;
        t_vec.iter().for_each(|element| {if element.is_positive() {all_negative = false;}});
        if all_negative {
            html_output.add_unbouded_solution_with_t_vec(simplex_table, &t_vec);
            html_output.end_simplex_iteration();
            return Ok(None);
        }

        let pivot = basic_simplex_solver::get_pivot(&t_vec, &pessimal_column.unwrap());
        html_output.add_pivot_information_to_the_html_document(simplex_table, &t_vec, &pivot);

        basic_simplex_solver::basic_simplex_gauss_elimination(simplex_table, &pivot, html_output, &mut gcd_cache).map_err(|e| e as Box<dyn HtmlConvertibleError>)?;
        iteration_counter += 1;

        html_output.end_simplex_iteration();
    }
    
    //Phase II
    html_output.add_starting_phase_one_dual_simplex_header();
    simplex_table.eliminate_artifical_variables_from_simplex_table(original_objective_row);
    html_output.add_eliminated_auxiliary_variables_info(simplex_table);

    if simplex_table.optimization_type == OptimizationType::MIN {
        simplex_table.objective_row.iter_mut().for_each(|x| x.negate_mut());
        html_output.add_objective_function_negation_info(simplex_table);
    }


    let coefficient = solve_basic_simplex_table(simplex_table, html_output).map_err(|e| e as Box<dyn HtmlConvertibleError>)?;
    if let Some(coefficient) = coefficient {
        if simplex_table.optimization_type == OptimizationType::MIN {
            html_output.add_target_value_negation_for_min_simplex(&coefficient);
        }
    }
    Ok(coefficient)
}

/// Given column index, return first index, where artificial variable is not zero
fn find_row_where_artificial_variable_is_non_zero(column: usize, simplex_table: &BasicSimplexTable) -> Option<usize> {
    for (index, row) in  simplex_table.rows.iter().enumerate(){
        if row[column] != Rational::zero() {
            return Some(index);
        }
    }
    None
}

/// Transforms objective row from original state to auxiliary problem sum(Ai) -> min
/// Effectively makes all auxiliary variables 1 in the objective row and everything else 0
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
    use std::fs;
    use crate::document::html_output::HtmlOutput;
    use crate::rationals::Rational;
    use crate::solvers::basic_simplex_table_data::test_utils::{create_minimal_simplex_table_for_testing, create_optimal_simplex_table, create_simplex_table_with_artificial_variables};
    use crate::solvers::two_phase_simplex_solver::{find_row_where_artificial_variable_is_non_zero, make_objective_row_for_auxiliary_minimalization, solve_two_phase_simplex};

    #[test]
    fn make_objective_row_for_auxiliary_minimalization_succeeds() {
        let mut simplex_table = create_simplex_table_with_artificial_variables();
        make_objective_row_for_auxiliary_minimalization(&mut simplex_table);
        assert_eq!(simplex_table.objective_row.len(), 5);
        assert_eq!(simplex_table.objective_row, vec![Rational::zero(), Rational::zero(), Rational::zero(), Rational::zero(), Rational::from_integer(1)]);
    }

    ///Test that make_objective_row_for_auxiliary_minimalization returns option none, when artificial variable has no value in row
    #[test]
    fn find_row_where_artificial_variable_is_non_zero_fails_on_missing_auxiliary_row() {
        let mut simplex_table = create_simplex_table_with_artificial_variables();
        simplex_table.rows[1][4] = Rational::zero();
        let a = find_row_where_artificial_variable_is_non_zero(4, &simplex_table);
        assert!(a.is_none());
    }

    /// Dual simplex switches to normal simplex method and solves the LP
    #[test]
    fn solve_with_dual_simplex_method_succeeds_for_basic_simplex() {
        let mut simplex_table = create_minimal_simplex_table_for_testing();
        let mut html_output = HtmlOutput::with_application_header();
        let solution = solve_two_phase_simplex(&mut simplex_table, &mut html_output);
        assert!(solution.is_ok());
        let solution = solution.unwrap();
        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert_eq!(solution, Rational::from_integer(2));
    }

    //todo fix to pass this test !!
    #[test]
    fn solve_with_dual_simplex_method_succeeds_for_artificial_simplex() {
        let mut simplex_table = create_simplex_table_with_artificial_variables();
        let mut html_output = HtmlOutput::with_application_header();
        let solution = solve_two_phase_simplex(&mut simplex_table, &mut html_output);
        fs::write("solve_with_dual_simplex_method_succeeds_for_artificial_simplex.html", html_output.to_string()).expect("Writing to html_output failed");
        assert!(solution.is_ok());
        let solution = solution.unwrap();
        assert!(solution.is_some());
        assert_eq!(solution.unwrap(), Rational::from_integer(2));
    }

}




