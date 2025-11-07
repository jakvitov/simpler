use super::basic_simplex_table_data::BasicSimplexTable;
use super::simplex_error::SimplexError;
use crate::document::html_output::HtmlOutput;
use crate::rationals::{GcdCache, NumericalError, Rational};
use crate::solvers::SimplexSoverAlgorithm::BASIC_SIMPLEX;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use fxhash::FxHasher;
use crate::document::html_convertible_error::HtmlConvertibleError;
use crate::utils::ApplicationError;
use crate::utils::env_parameters::ApplicationEnvParameter;

/// Solve the given simplex table using the basic simplex algoritm
/// Both simplex table and html output are edited
/// This method returns resulting optimal value
/// Since all errors are added to the
pub fn solve_basic_simplex(simplex_table: &mut BasicSimplexTable, html_output: &mut HtmlOutput) -> Result<Option<Rational>, Box<dyn HtmlConvertibleError>> {
    html_output.add_simplex_solver_header(BASIC_SIMPLEX);
    Ok(solve_basic_simplex_table(simplex_table, html_output, None)?)
}


/// Solves table using simplex, but without header. Used also by other solvers for steps, that require primary simplex
/// If iteration limit is not specified, default will be used
pub(super) fn solve_basic_simplex_table(simplex_table: &mut BasicSimplexTable, html_output: &mut HtmlOutput, iteration_limit: Option<u8> ) -> Result<Option<Rational>, Box<dyn HtmlConvertibleError>> {
    let mut iteration_counter:u8 = 1;
    let mut gcd_cache = GcdCache::init();

    //We hash the bases and store how many times we met them.
    //Hashing is used to prevent hash map storing all the vectors as keys
    //FxHasher is deterministic in opposition to rusts DefaultHasher
    let mut hasher = FxHasher::default();
    simplex_table.base_variable_names.hash(&mut hasher);
    let base_hash = hasher.finish();

    let mut visited_bases: HashMap<u64, u8> = HashMap::new();
    visited_bases.insert(base_hash, 1);

    loop {

        let pessimal_column = check_optimity(simplex_table);
        if pessimal_column.is_none() {
            html_output.add_basic_simplex_optimal_solution_to_the_html_document(simplex_table);
            return Ok(Some(simplex_table.objective_rhs));
        }

        html_output.start_simplex_iteration(iteration_counter);
        let mut t_vec = get_t_vector(simplex_table, &pessimal_column.unwrap(), &mut gcd_cache).map_err(|e| e as Box<dyn HtmlConvertibleError>)?;

        //Check unbounded solution
        let mut negative_count = 0;
        t_vec.iter().for_each(|element| {if element.is_some() && element.unwrap().is_negative() {negative_count +=1 ;}});
        if negative_count == t_vec.len() {
            html_output.add_unbouded_solution_with_t_vec(simplex_table, &t_vec);
            html_output.end_simplex_iteration();
            return Ok(None);
        }

        //Pivot calculation
        let pivot = get_pivot(&t_vec, &pessimal_column.unwrap());
        html_output.add_pivot_information_to_the_html_document(simplex_table, &t_vec, &pivot);


        // Row elimination with output
        basic_simplex_gauss_elimination(simplex_table, &pivot,  html_output, &mut gcd_cache).map_err(|e| e as Box<dyn HtmlConvertibleError>)?;
        html_output.end_simplex_iteration();

        let (iteration_counter, overflowed) = iteration_counter.overflowing_add(1);
        if overflowed {
            return Err(Box::new(ApplicationError::from_string_details("Iteration counter overflow. Number of iterations too high.", format!("Highest iteration counter {}", u8::MAX))))
        }
        
        //Check if base was met MaxCycleIterations
        if cycle_or_iterations_limit_exceeded(&mut visited_bases, iteration_counter, iteration_limit, simplex_table, html_output).map_err(|e| e as Box<dyn HtmlConvertibleError>)? {
            return Ok(None)
        }
    }
}

/// Return true if check failed
/// if iteration_limit is None, then it is obtained from the env parameter
/// In case of check failing, message with cycle or iteration limit exceeded is added to html output
pub(super) fn cycle_or_iterations_limit_exceeded(visited_bases: &mut HashMap<u64, u8>, iteration_counter: u8, iteration_limit: Option<u8>, simplex_table: &BasicSimplexTable, html_output: &mut HtmlOutput) -> Result<bool, Box<NumericalError>> {
    let hasher = FxHasher::default();
    let base_hash = hasher.finish();
    if let Some(visited_count) = visited_bases.get(&base_hash) {
        if visited_count + 1 > ApplicationEnvParameter::MaxCycleIterations.get_or_default().parse::<u8>().map_err(|x| Box::new(NumericalError::from(x)))? {
            html_output.add_found_degenerate_column_cycle();
            html_output.end_simplex_iteration();
            return Ok(true);
        } else {
            visited_bases.insert(base_hash, visited_count + 1);
        }
    } else {
        visited_bases.insert(base_hash, 1);
    }

    let limit = iteration_limit.unwrap_or(ApplicationEnvParameter::MaxIterationsLimit.get_or_default().parse::<u8>().map_err(|x| Box::new(NumericalError::from(x)))?);
    if iteration_counter == limit {
        html_output.maximum_iterations_reached(limit);
    };
    Ok(false)
}

/// Perform one simplex iteration with output to the HtmlOutput
pub(super) fn basic_simplex_gauss_elimination(simplex_table: &mut BasicSimplexTable, pivot: &(usize, usize),
                           html_output: &mut HtmlOutput, gcd_cache: &mut GcdCache) -> Result<(), Box<NumericalError>> {

    //Normalise row
    let coefficient = simplex_table.normalize_pivot_row(&pivot, gcd_cache)?;
    html_output.add_pivot_row_normalisation_to_html_document(simplex_table, &coefficient, &pivot);

    // We iterate with the rows.len included (methods are written that, thats the target function row)
    for target_row_index in 0..=simplex_table.rows.len() {
        if target_row_index == pivot.0 {
            continue;
        }
        let coefficient = simplex_table.normalize_row_by_pivot_row(&pivot, target_row_index, gcd_cache)?;
        html_output.add_row_normalisation_by_pivot_row_to_html_document(simplex_table, &coefficient, &pivot, target_row_index);
    }

    // base variable switchd
    let old_variable = simplex_table.base_variable_names[pivot.0].clone();
    simplex_table.base_variable_names[pivot.0] = simplex_table.column_variable_names.keys()[pivot.1].clone();
    html_output.add_base_variable_switch_info_to_html_document(simplex_table, &old_variable, &simplex_table.base_variable_names[pivot.0]);

    Ok(())
}

/// Return pivot in the current simplex table based on the pessimal column and t_vec
/// Pivot has format (row_index, column_index)
pub(super) fn get_pivot(t_vec: &Vec<Option<Rational>>, pessimal_column: &(usize, Rational)) -> (usize, usize) {
    // t-vec elements must always be greater than zero!
    let mut min_value: Option<&Rational> = None;
    let mut min_index = 0usize;

    for (index, value) in t_vec.iter().enumerate() {
        if let Some(value) = value {
            if (min_value.is_some() && *min_value.unwrap() > *value && *value >= Rational::zero()) || min_value.is_none() {
                min_value = Some(value);
                min_index = index;
            }
        } else {
            continue;
        }
    }
    //Only case when this is none is when the t_vec is empty or all negative
    //All-negative case is handled in the iteration method (unbounded solution)
    //t_vec is empty only on none rows in the simplex table, which application does not allow to enter
    debug_assert!(min_value.is_some());
    (min_index, pessimal_column.0)
}

pub(super) fn get_t_vector(simplex_table: &BasicSimplexTable, pessimal_column: &(usize, Rational), gcd_cache: &mut GcdCache) -> Result<Vec<Option<Rational>>, Box<NumericalError>> {
    simplex_table.rows.iter().for_each(|row| debug_assert!(pessimal_column.0 < row.len()));

    let mut res: Vec<Option<Rational>> = Vec::with_capacity(simplex_table.rows.len());
    for (i, row) in simplex_table.rows.iter().enumerate() {
        if row[pessimal_column.0] == Rational::zero() {
            res.push(None);
            continue;
        }
        let t_val = simplex_table.rhs[i].divide(&row[pessimal_column.0], gcd_cache)?;
        res.push(Some(t_val));
    }

    debug_assert!(res.len() == simplex_table.rhs.len());
    Ok(res)
}

/// Return Some(position, &value) if there is suboptimal element in the objective row
/// Return None if the objective row signals optimality
pub(super) fn check_optimity(simplex_table: &BasicSimplexTable) ->  Option<(usize, Rational)> {
    let mut pessimal_element: Option<(usize, &Rational)> = None;

    for (position, i) in simplex_table.objective_row.iter().enumerate() {
        if (i.is_negative() && pessimal_element.is_some() && *pessimal_element.unwrap().1 > *i)
            || (i.is_negative() && pessimal_element.is_none()){
            pessimal_element = Some((position, i))
        }
    }

    if let Some(pe) = pessimal_element {
        Some((pe.0, pe.1.clone()))
    } else {
        None
    }
}

fn check_basic_simplex_compatibility(simplex_table: &BasicSimplexTable) -> Result<(), Box<SimplexError>> {
    if simplex_table.artificial_variable_index.is_some() {
        return Err(Box::new(SimplexError::new("Standard form of the LP is not feasible.\nAuxiliary variables were created and two-phase simplex needs to be used.\nProbably greater than or equal constraints were met.\nNote that ≥ constraints might be introduced with bounds as well.\nFor pure ≥ problems you can use conversion and convert your table to pure ≤ using duality.")))
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::basic_simplex_solver::check_optimity;
    use super::super::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing;
    use super::super::basic_simplex_table_data::test_utils::create_optimal_simplex_table;
    use crate::document::html_output::HtmlOutput;
    use crate::rationals::{GcdCache, Rational};
    use crate::solvers::basic_simplex_solver::{basic_simplex_gauss_elimination, get_pivot, get_t_vector, solve_basic_simplex};
    use crate::solvers::basic_simplex_table_data::test_utils::{create_cycling_simplex_table, create_unbounded_simplex_table};
    use std::fs;
    use std::hash::{Hash, Hasher};

    #[test]
    fn get_pivot_suceeds() {
        let mut gcd_cache = GcdCache::init();
        let simplex_table = create_minimal_simplex_table_for_testing();
        let t_vector = vec![Some(Rational::new(1, 1)), Some(Rational::new(3, 1))];
        let pivot = get_pivot(&t_vector, &(1,  Rational::new(-2, 1)));
        assert_eq!(pivot, (0,  1));
    }

    #[test]
    fn get_basic_simplex_t_vector_succeeds() {
        let mut gcd_cache = GcdCache::init();
        let simplex_table = create_minimal_simplex_table_for_testing();
        let t_vector = get_t_vector(&simplex_table, &(1, Rational::new(-2, 1)), &mut gcd_cache);
        assert!(t_vector.is_ok());
        assert_eq!(t_vector.unwrap(), vec![Some(Rational::new(1, 1)), Some(Rational::new(3,1))]);

    }

    #[test]
    fn check_basic_simplex_optimity_succeeds_on_non_optimal_table() {
        let simplex_table = create_minimal_simplex_table_for_testing();
        let res = check_optimity(&simplex_table);
        assert!(res.is_some());
        assert_eq!(res.unwrap().0, 1);
        assert_eq!(res.unwrap().1, Rational::from_integer(-2));
    }

    #[test]
    fn check_basic_simplex_optimity_succeeds_on_optimal_table() {
        let simplex_table = create_optimal_simplex_table();
        let res = check_optimity(&simplex_table);
        assert!(res.is_none());
    }

    #[test]
    fn check_basic_simplex_gauss_elimination_succeeds() {
        let mut simplex_table = create_minimal_simplex_table_for_testing();
        let mut html_output = HtmlOutput::with_application_header();
        let mut gcd_cache = GcdCache::init();
        let res = basic_simplex_gauss_elimination(&mut simplex_table, &(0, 1), &mut html_output, &mut gcd_cache);
        assert!(res.is_ok());

        assert_eq!(simplex_table.rows[0], vec![Rational::new(1, 2), Rational::from_integer(1),Rational::new(1, 2),Rational::zero()]);
        assert_eq!(simplex_table.rows[1], vec![Rational::new(3, 2), Rational::zero(),Rational::new(-1, 2), Rational::from_integer(1)]);
        assert_eq!(simplex_table.objective_row, vec![Rational::zero(), Rational::zero(),Rational::from_integer(1), Rational::zero()]);

        assert_eq!(simplex_table.objective_rhs, Rational::from_integer(2));
        assert_eq!(simplex_table.base_variable_names, vec!["x2", "S2"]);
    }

    #[test]
    fn check_basic_simplex_solve_succeeds() {
        let mut simplex_table = create_minimal_simplex_table_for_testing();
        let mut html_output = HtmlOutput::with_application_header();
        let res = solve_basic_simplex(&mut simplex_table, &mut html_output);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.is_some());
        assert_eq!(res.unwrap(), Rational::from_integer(2));
    }

    #[test]
    fn check_basic_unbounded_simplex_suceeds() {
        let mut simplex_table = create_unbounded_simplex_table();
        let mut html_output = HtmlOutput::with_application_header();
        let res = solve_basic_simplex(&mut simplex_table, &mut html_output);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn check_simplex_with_cycle_fails() {
        let mut simplex_table = create_cycling_simplex_table();
        let mut html_output = HtmlOutput::with_application_header();
        let res = solve_basic_simplex(&mut simplex_table, &mut html_output);
        fs::write("check_simplex_with_cycle_fails.html",html_output.to_string());
        assert!(res.is_ok());
        assert!(res.unwrap().is_none());
        assert!(html_output.to_string().contains("Cycle"));
    }
}