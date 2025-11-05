use crate::document::html_convertible_error::HtmlConvertibleError;
use crate::document::html_output::HtmlOutput;
use crate::rationals::{GcdCache, Rational, RationalMatrix};
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;
use crate::solvers::SimplexSoverAlgorithm::REVISED_SIMPLEX;
use crate::utils::ApplicationError;

pub fn solve_revised_simplex(initial_simplex_table: &BasicSimplexTable, gcd_cache: &mut GcdCache, html_output: &mut HtmlOutput) -> Result<Option<Rational>, Box<dyn HtmlConvertibleError>> {
    html_output.add_simplex_solver_header(REVISED_SIMPLEX);
    Ok(None)



}

/// Get basis matrix for basic variables
fn get_basis_matrix(basic_variables: &Vec<String>, initial_simplex_table: &BasicSimplexTable) -> Result<RationalMatrix, Box<ApplicationError>> {
    let mut b_matrix_rows = Vec::with_capacity(basic_variables.len());
    for basic_variable in basic_variables {
        let basic_variable_index = initial_simplex_table.column_variable_names.get(basic_variable);
        let Some(basic_variable_index) =  basic_variable_index else {
            return Err(Box::new(ApplicationError::from_string_details("Basic variable not found in initial simplex table.", format!("Basic variable {}. Available variables {:?}", basic_variable, initial_simplex_table.column_variable_names.keys().collect::<Vec<_>>()))));
        };

        let mut row = Vec::with_capacity(basic_variable.len());
        for i in 0..basic_variables.len() {
            row.push(initial_simplex_table.rows[i][*basic_variable_index])
        }
        b_matrix_rows.push(row);
    }

    //todo optimise this allocation only after error occurs
    let b_matrix_rows_dims = b_matrix_rows.iter().map(|x|x.len()).collect::<Vec<_>>();
    let Some(res) = RationalMatrix::from_rows(b_matrix_rows) else {
         return Err(Box::new(ApplicationError::from_string_details("Wrong dimensions error encountered while constructing basis matrix.", format!("Row dimensions: {:?}", b_matrix_rows_dims))));
    };
    debug_assert_eq!(res.dim().0, res.dim().1);
    //todo get rid of transpose by introducing column initialised matrix
    Ok(res.transpose())
}

#[cfg(test)]
mod tests {
    use crate::rationals::Rational;
    use crate::solvers::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing;
    use crate::solvers::revised_simpler_solver::get_basis_matrix;

    #[test]
    fn get_basis_matrix_succeeds() {
        let simplex_table = create_minimal_simplex_table_for_testing();
        let basis_matrix = get_basis_matrix(&vec!["x1".to_owned(), "S2".to_owned()], &simplex_table);

        assert!(basis_matrix.is_ok());
        let basis_matrix = basis_matrix.unwrap();

        assert_eq!(basis_matrix.dim(), (2,2));
        dbg!(&basis_matrix);
        assert_eq!(*basis_matrix.get(0,0), Rational::from_integer(1));
        assert_eq!(*basis_matrix.get(0,1), Rational::zero());
        assert_eq!(*basis_matrix.get(1,0), Rational::from_integer(2));
        assert_eq!(*basis_matrix.get(1,1), Rational::from_integer(1));

    }


}