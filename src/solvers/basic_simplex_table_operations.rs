use crate::rationals::{GcdCache, NumericalError, Rational};
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;

impl BasicSimplexTable {

    /// Normalize the pivot row and rhs to contain 1 in the pivot element
    /// Return coefficient by which the row was multiplied
    fn normalize_pivot_row(&mut self, pivot: &(usize, usize), gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        let coefficient = (&self.rows[pivot.0][pivot.1]).invert();
        for  i in &mut self.rows[pivot.0] {
            i.multiply_by(&coefficient, gcd_cache)?;
        }

        Ok(coefficient)
    }

}