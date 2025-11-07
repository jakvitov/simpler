use crate::document::html_convertible_error::HtmlConvertibleError;
use crate::document::html_output::HtmlOutput;
use crate::rationals::{GcdCache, Rational, RationalMatrix};
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;
use crate::solvers::SimplexSoverAlgorithm::REVISED_SIMPLEX;
use crate::utils::ApplicationError;
use std::collections::HashSet;

pub fn solve_revised_simplex(initial_simplex_table: &BasicSimplexTable, gcd_cache: &mut GcdCache, html_output: &mut HtmlOutput) -> Result<Option<Rational>, Box<dyn HtmlConvertibleError>> {
    html_output.add_simplex_solver_header(REVISED_SIMPLEX);

    let mut base_variables: Vec<String> = initial_simplex_table.base_variable_names.clone();
    let (basis_matrix, non_base_matrix) = get_basis_matrix_split(&base_variables, initial_simplex_table).map_err(|x| x as Box<dyn HtmlConvertibleError>)?;
    let basis_inverse = basis_matrix.inverse(gcd_cache).map_err(|x| x as Box<dyn HtmlConvertibleError>)?;

    if basis_inverse.is_none() {
        return Err(Box::new(ApplicationError::from_string_details("Singular basis matrix encountered.", format!("Basis matrix: {:?}", basis_matrix))));
    }

    let (c_b, c_nb) = get_basis_split_cost_vector(initial_simplex_table, &base_variables).map_err(|x| x as Box<dyn HtmlConvertibleError>)?;

    //Reduced costs for non-basic variables




    Ok(None)
}

/// Return pair of basic and non-basic cost vectors
/// Return (c_b, c_nb)
fn get_basis_split_cost_vector(initial_simplex_table: &BasicSimplexTable, basic_variable_names: &Vec<String>) ->  Result<(RationalMatrix, RationalMatrix), Box<ApplicationError>> {
    let mut basic_cost_vector: Vec<Rational> = Vec::with_capacity(basic_variable_names.len());
    let mut non_basic_cost_vector: Vec<Rational> = Vec::with_capacity(initial_simplex_table.rows.len() - basic_variable_names.len());

    let Some(first) = initial_simplex_table.rows.first() else {
        return Ok((RationalMatrix::empty(), RationalMatrix::empty()));
    };

    let basic_variable_indexes = get_basic_variable_indexes(basic_variable_names, initial_simplex_table)?;

    for i in 0..first.len() {
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
fn get_basis_matrix_split(basic_variables: &Vec<String>, initial_simplex_table: &BasicSimplexTable) -> Result<(RationalMatrix, RationalMatrix), Box<ApplicationError>> {

    let Some(first_initial_simplex_table_row) = initial_simplex_table.rows.first() else {
        return Ok((RationalMatrix::empty(), RationalMatrix::empty()));
    };

    let mut b_matrix_rows: Vec<Vec<Rational>> = Vec::with_capacity(basic_variables.len());
    let mut n_matrix_rows: Vec<Vec<Rational>> = Vec::with_capacity(initial_simplex_table.rows.len() - basic_variables.len());

    let basic_variable_indexes = get_basic_variable_indexes(basic_variables, initial_simplex_table)?;

    for j in 0..first_initial_simplex_table_row.len() {
        let mut row = Vec::with_capacity(basic_variables.len());
        if basic_variable_indexes.contains(&j) {
            for i in 0..basic_variables.len() {
                row.push(initial_simplex_table.rows[i][j])
            }
            b_matrix_rows.push(row);
        } else {
            for i in 0..basic_variables.len() {
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
    let mut basic_variable_indexes:HashSet<usize> = HashSet::new();
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
    use crate::rationals::Rational;
    use crate::solvers::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing;
    use crate::solvers::revised_simpler_solver::{get_basis_matrix_split, get_basis_split_cost_vector};

    #[test]
    fn get_basis_matrix_split_succeeds() {
        let simplex_table = create_minimal_simplex_table_for_testing();
        let (basis_matrix, non_basis_matrix) = get_basis_matrix_split(&vec!["x1".to_owned(), "S2".to_owned()], &simplex_table).expect("Failed to split basis matrix.");

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
        let (c_b, c_nb) = get_basis_split_cost_vector(&simplex_table, &vec!["x1".to_owned(), "S2".to_owned()]).expect("Basis vector should be correct");

        assert_eq!(*c_b.get_row(0), vec![Rational::from_integer(-1), Rational::zero()]);
        assert_eq!(*c_nb.get_row(1), vec![Rational::from_integer(-2), Rational::zero()]);
    }
}