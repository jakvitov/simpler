use crate::document::html_convertible_error::HtmlConvertibleError;
use crate::document::html_output::HtmlOutput;
use crate::rationals::{GcdCache, Rational, RationalMatrix};
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;
use crate::solvers::SimplexSoverAlgorithm::REVISED_SIMPLEX;
use crate::utils::ApplicationError;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use fxhash::FxHasher;
use crate::solvers::basic_simplex_solver::cycle_or_iterations_limit_exceeded;

#[derive(Debug, PartialEq, Eq)]
enum VariableType {
    Basic,
    NonBasic
}

pub fn solve_revised_simplex(initial_simplex_table: &BasicSimplexTable, gcd_cache: &mut GcdCache, html_output: &mut HtmlOutput) -> Result<Option<Rational>, Box<dyn HtmlConvertibleError>> {
    html_output.add_simplex_solver_header(REVISED_SIMPLEX);

    let mut iteration_counter:u8 = 1;
    let mut visited_bases:HashMap<u64, u8> = HashMap::new();
    let mut hasher = FxHasher::default();
    initial_simplex_table.base_variable_names.hash(&mut hasher);
    let base_hash = hasher.finish();
    visited_bases.insert(base_hash, iteration_counter);


    //todo deal with these unholy heap copies of base variable names
    let mut base_variables: Vec<String> = initial_simplex_table.base_variable_names.clone();
    // Base variable indexes in the original simplex table
    let mut base_variable_indexes = get_basic_variable_indexes(&base_variables, &initial_simplex_table).map_err(|x| x as Box<dyn HtmlConvertibleError>)?;
    // Mapping from relative indexes in the base and non-base cropped structures, where indexes of basic and non basic variables are different
    let (basic_variable_index_mapping, non_basic_variable_index_mapping) = get_basic_non_basic_index_to_var_index_mapping(initial_simplex_table, &base_variable_indexes);

    let (basis_matrix, non_base_matrix) = get_basis_matrix_split(initial_simplex_table, &base_variable_indexes).map_err(|x| x as Box<dyn HtmlConvertibleError>)?;

    let Some(basis_inverse) = basis_matrix.inverse(gcd_cache).map_err(|x| x as Box<dyn HtmlConvertibleError>)? else {
        return Err(Box::new(ApplicationError::from_string_details("Singular basis matrix encountered.", format!("Basis matrix: {:?}", basis_matrix))));
    };

    let (c_b, c_nb) = get_basis_split_cost_vector(initial_simplex_table, &base_variable_indexes).map_err(|x| x as Box<dyn HtmlConvertibleError>)?;

    //todo can any of these operations be mut, so that we don't need allocate new matrices again?
    let pi = RationalMatrix::mul(&c_b.transpose(), &basis_inverse, gcd_cache).map_err(|x| x as Box<dyn HtmlConvertibleError>)?;
    let pi_n = RationalMatrix::mul(&pi, &non_base_matrix, gcd_cache).map_err(|x| x as Box<dyn HtmlConvertibleError>)?;
    //Reduced costs for non-basic variables
    let red_costs = RationalMatrix::subtract(&c_nb, &pi_n,gcd_cache).map_err(|x| x as Box<dyn HtmlConvertibleError>)?;

    debug_assert_eq!(red_costs.dim().0, 1usize);


    while let Some(minimal_rc_index) = get_minimal_reduced_cost(&red_costs) {



        //Check if base was met MaxCycleIterations
        if cycle_or_iterations_limit_exceeded(&mut visited_bases, iteration_counter, None, initial_simplex_table, html_output).map_err(|e| e as Box<dyn HtmlConvertibleError>)? {
            return Ok(None)
        }
    }

    Ok(None)
}

/// Translate relative index among basic/non-basic variables to global index in simplex table
/// mappings - (basic_variable_index_mapping, non_basic_variable_index_mapping)
fn translate_relative_variable_index_to_global(i: usize, variable_type: VariableType, mappings: (&HashMap<usize, String>, &HashMap<usize, String>), initial_simplex_table: &BasicSimplexTable) -> Result<usize, Box<ApplicationError>> {
    let name = if variable_type == VariableType::Basic {
        let Some(name) = mappings.0.get(&i) else {
          return Err(Box::new(ApplicationError::from_string_details("Variable name was not able to be translated from its relative index.", format!("Variable: type - {:?}, relative index - {i}", variable_type))));
        };
        name
    } else {
        let Some(name) = mappings.1.get(&i) else {
            return Err(Box::new(ApplicationError::from_string_details("Variable name was not able to be translated from its relative index.", format!("Variable: type - {:?}, relative index - {i}", variable_type))));
        };
        name
    };

    let Some(index) = initial_simplex_table.column_variable_names.get(name) else {
        return Err(Box::new(ApplicationError::from_string_details("Variable name was not able to be translated from its simplex table index.", format!("Variable: type - {:?}, name - {name}", variable_type))));
    };

    Ok(*index)
}

/// Return index of the minimal negative element in reduced costs
/// Return None if all are positive
fn get_minimal_reduced_cost(red_costs: &RationalMatrix) -> Option<usize> {
    debug_assert_eq!(red_costs.dim().0, 1usize);
    debug_assert!(red_costs.dim().1 > 0usize);

    let mut minimal_index = 0usize;
    let mut minimal_set = false;
    for (i, val) in red_costs.get_row(0).iter().enumerate() {
        if val.is_negative() && *val < red_costs.get_row(0)[minimal_index]{
            minimal_index = i;
            minimal_set = true;
        }
    }

    if minimal_set {
        Some(minimal_index)
    } else {
        None
    }
}

/// Return Hash maps, that maps non_basic and basic variable mapping index to var_name
/// Example: Non-basic variable 0 -> x1
/// return (Basic variable map, non_basic_variable_map)
fn get_basic_non_basic_index_to_var_index_mapping(initial_simplex_table: &BasicSimplexTable, basic_variable_indexes: &HashSet<usize>) -> (HashMap<usize, String>, HashMap<usize, String>) {
    let mut basic_variables: HashMap<usize, String> = HashMap::new();
    let mut non_basic_variables: HashMap<usize, String> = HashMap::new();
    let mut basic_variable_index = 0usize;
    let mut non_basic_variable_index = 0usize;

    for i in 0..initial_simplex_table.objective_row.len() {
        if basic_variable_indexes.contains(&i) {
            basic_variables.insert(basic_variable_index, initial_simplex_table.column_variable_names.keys()[i].clone());
            basic_variable_index += 1;
        } else {
            non_basic_variables.insert(non_basic_variable_index, initial_simplex_table.column_variable_names.keys()[i].clone());
            non_basic_variable_index += 1;
        }
    }

    (basic_variables, non_basic_variables)
}

/// Return pair of basic and non-basic cost vectors
/// Return (c_b, c_nb)
fn get_basis_split_cost_vector(initial_simplex_table: &BasicSimplexTable, basic_variable_indexes: &HashSet<usize>) ->  Result<(RationalMatrix, RationalMatrix), Box<ApplicationError>> {
    let Some(first) = initial_simplex_table.rows.first() else {
        return Ok((RationalMatrix::empty(), RationalMatrix::empty()));
    };

    let mut basic_cost_vector: Vec<Rational> = Vec::with_capacity(basic_variable_indexes.len());
    let mut non_basic_cost_vector: Vec<Rational> = Vec::with_capacity(first.len() - basic_variable_indexes.len());

    for i in 0..initial_simplex_table.objective_row.len() {
        if basic_variable_indexes.contains(&i) {
            basic_cost_vector.push(initial_simplex_table.objective_row[i]);
        } else {
            non_basic_cost_vector.push(initial_simplex_table.objective_row[i]);
        }
    }

    Ok((RationalMatrix::from_row(basic_cost_vector), RationalMatrix::from_row(non_basic_cost_vector)))
}

/// Return (B,N) where N is the matrix equivalent to column in the initial matrix X, which are non-basic
/// B is basis matrix associated with the current basic variables
/// A = (B|N)
fn get_basis_matrix_split(initial_simplex_table: &BasicSimplexTable, basic_variable_indexes: &HashSet<usize>) -> Result<(RationalMatrix, RationalMatrix), Box<ApplicationError>> {

    let Some(first_initial_simplex_table_row) = initial_simplex_table.rows.first() else {
        return Ok((RationalMatrix::empty(), RationalMatrix::empty()));
    };

    let mut b_matrix_rows: Vec<Vec<Rational>> = Vec::with_capacity(basic_variable_indexes.len());
    let mut n_matrix_rows: Vec<Vec<Rational>> = Vec::with_capacity(first_initial_simplex_table_row.len() - basic_variable_indexes.len());

    for j in 0..first_initial_simplex_table_row.len() {
        let mut row = Vec::with_capacity(basic_variable_indexes.len());
        if basic_variable_indexes.contains(&j) {
            for i in 0..basic_variable_indexes.len() {
                row.push(initial_simplex_table.rows[i][j])
            }
            b_matrix_rows.push(row);
        } else {
            for i in 0..basic_variable_indexes.len() {
                row.push(initial_simplex_table.rows[i][j])
            }
            n_matrix_rows.push(row);
        }
    }

    // todo make this allocation only after error occurs
    let b_res_dims = b_matrix_rows.iter().map(|x| x.len()).collect::<Vec<_>>();
    let n_res_dims = n_matrix_rows.iter().map(|x| x.len()).collect::<Vec<_>>();
    let Some(b_res) = RationalMatrix::from_rows(b_matrix_rows) else {
         return Err(Box::new(ApplicationError::from_string_details("Wrong dimensions error encountered while constructing basis matrix.", format!("Row dimensions: {:?}", b_res_dims))));
    };
    debug_assert_eq!(b_res.dim().0, b_res.dim().1);

    let Some(n_res) = RationalMatrix::from_rows(n_matrix_rows) else {
        return Err(Box::new(ApplicationError::from_string_details("Wrong dimensions error encountered while constructing basis matrix.", format!("Row dimensions: {:?}", n_res_dims))));
    };
    debug_assert_eq!(b_res.dim().0, b_res.dim().1);

    //todo get rid of transpose by introducing column initialised matrix
    Ok((b_res.transpose(), n_res.transpose()))
}

/// Return hash set containing the indexes of the basic variables
fn get_basic_variable_indexes(basic_variables: &Vec<String>, initial_simplex_table: &BasicSimplexTable) -> Result<HashSet<usize>, Box<ApplicationError>> {
    let mut basic_variable_indexes:HashSet<usize> = HashSet::with_capacity(basic_variables.len());
    for basic_variable_name in  basic_variables {
        let basic_variable_index = initial_simplex_table.column_variable_names.get(basic_variable_name);
        let Some(basic_variable_index) =  basic_variable_index else {
            return Err(Box::new(ApplicationError::from_string_details("Basic variable not found in initial simplex table.", format!("Basic variable {}. Available variables {:?}", basic_variable_name, initial_simplex_table.column_variable_names.keys().collect::<Vec<_>>()))));
        };
        basic_variable_indexes.insert(*basic_variable_index);
    }
    Ok(basic_variable_indexes)
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::rationals::{Rational, RationalMatrix};
    use crate::solvers::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing;
    use crate::solvers::revised_simpler_solver::{get_basic_non_basic_index_to_var_index_mapping, get_basic_variable_indexes, get_basis_matrix_split, get_basis_split_cost_vector, get_minimal_reduced_cost, translate_relative_variable_index_to_global, VariableType};

    #[test]
    fn get_basis_matrix_split_succeeds() {
        let simplex_table = create_minimal_simplex_table_for_testing();
        let mut basic_variable_indexes = HashSet::new();
        basic_variable_indexes.insert(0usize);  //x1
        basic_variable_indexes.insert(3usize);  //S2
        let (basis_matrix, non_basis_matrix) = get_basis_matrix_split(&simplex_table, &basic_variable_indexes).expect("Failed to split basis matrix.");

        assert_eq!(basis_matrix.dim(), (2,2));
        assert_eq!(*basis_matrix.get(0,0), Rational::from_integer(1));
        assert_eq!(*basis_matrix.get(0,1), Rational::zero());
        assert_eq!(*basis_matrix.get(1,0), Rational::from_integer(2));
        assert_eq!(*basis_matrix.get(1,1), Rational::from_integer(1));

        assert_eq!(non_basis_matrix.dim(), (2,2));
        assert_eq!(*non_basis_matrix.get(0,0), Rational::from_integer(2));
        assert_eq!(*non_basis_matrix.get(0,1), Rational::from_integer(1));
        assert_eq!(*non_basis_matrix.get(1,0), Rational::from_integer(1));
        assert_eq!(*non_basis_matrix.get(1,1), Rational::from_integer(0));
    }

    #[test]
    fn get_basis_split_cost_vector_succeeds() {
        let simplex_table = create_minimal_simplex_table_for_testing();
        let mut basic_variable_indexes = HashSet::new();
        basic_variable_indexes.insert(0usize); //x1
        basic_variable_indexes.insert(3usize); //S2
        let (c_b, c_nb) = get_basis_split_cost_vector(&simplex_table, &basic_variable_indexes).expect("Basis vector should be correct");

        assert_eq!(*c_b.get_row(0), vec![Rational::from_integer(-1), Rational::zero()]);
        assert_eq!(*c_nb.get_row(0), vec![Rational::from_integer(-2), Rational::zero()]);
    }

    #[test]
    fn get_basic_non_basic_index_to_var_index_mapping_succeeds() {
        let simplex_table = create_minimal_simplex_table_for_testing();
        let mut basic_variable_indexes = HashSet::new();
        basic_variable_indexes.insert(0); //x1
        basic_variable_indexes.insert(3); //S2
        let (basic_variable_mapping, non_basic_variable_mapping) = get_basic_non_basic_index_to_var_index_mapping(&simplex_table, &basic_variable_indexes);
        assert_eq!(basic_variable_mapping.len(), 2);
        assert_eq!(non_basic_variable_mapping.len(), 2);

        assert_eq!(basic_variable_mapping.get(&0usize).unwrap(), &"x1".to_owned());
        assert_eq!(basic_variable_mapping.get(&1usize).unwrap(), &"S2".to_owned());

        assert_eq!(non_basic_variable_mapping.get(&0usize).unwrap(), &"x2".to_owned());
        assert_eq!(non_basic_variable_mapping.get(&1usize).unwrap(), &"S1".to_owned());
    }

    #[test]
    fn get_minimal_reduced_cost_succeeds_for_existing_negative_rc() {
        let rc = RationalMatrix::from_row(vec![Rational::from_integer(-1), Rational::from_integer(-2), Rational::zero(), Rational::from_integer(10)]);
        let min_index = get_minimal_reduced_cost(&rc).expect("Should be able to find min.");
        assert_eq!(min_index, 1usize);
    }

    #[test]
    fn get_minimal_reduced_cost_succeeds_non_existing_negative_rc() {
        let rc = RationalMatrix::from_row(vec![Rational::from_integer(1), Rational::from_integer(2), Rational::zero(), Rational::from_integer(10)]);
        let min_index = get_minimal_reduced_cost(&rc);
        assert!(min_index.is_none());
    }

    #[test]
    fn translate_relative_variable_index_to_global_succeeds_for_basic_variable() {
        let simplex_table = create_minimal_simplex_table_for_testing();
        let mut basic_variable_index_mapping = HashMap::new();
        let mut non_basic_variable_index_mapping = HashMap::new();
        basic_variable_index_mapping.insert(1usize, "x1".to_owned());
        let res = translate_relative_variable_index_to_global(1, VariableType::Basic, (&basic_variable_index_mapping, &non_basic_variable_index_mapping), &simplex_table).expect("Should be able to translate.");
        assert_eq!(res, 0usize);
    }

    #[test]
    fn translate_relative_variable_index_to_global_succeeds_for_non_basic_variable() {
        let simplex_table = create_minimal_simplex_table_for_testing();
        let mut basic_variable_index_mapping = HashMap::new();
        let mut non_basic_variable_index_mapping = HashMap::new();
        non_basic_variable_index_mapping.insert(1usize, "x1".to_owned());
        let res = translate_relative_variable_index_to_global(1, VariableType::NonBasic, (&basic_variable_index_mapping, &non_basic_variable_index_mapping), &simplex_table).expect("Should be able to translate.");
        assert_eq!(res, 0usize);
    }

    #[test]
    fn translate_relative_variable_index_to_global_fails_for_non_existent_variable_name() {
        let simplex_table = create_minimal_simplex_table_for_testing();
        let mut basic_variable_index_mapping = HashMap::new();
        let mut non_basic_variable_index_mapping = HashMap::new();
        non_basic_variable_index_mapping.insert(1usize, "non_existent_variable_name".to_owned());
        let res = translate_relative_variable_index_to_global(1, VariableType::NonBasic, (&basic_variable_index_mapping, &non_basic_variable_index_mapping), &simplex_table);
        assert!(res.is_err());
    }
}