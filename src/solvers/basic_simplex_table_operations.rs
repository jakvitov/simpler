use crate::rationals::{GcdCache, NumericalError, Rational};
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;
use crate::utils::collections::get_two_rows_mut;

impl BasicSimplexTable {

    /// Normalize the pivot row and rhs to contain 1 in the pivot element
    /// Return coefficient by which the row was multiplied
    fn normalize_pivot_row(&mut self, pivot: &(usize, usize), gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        let coefficient = (&self.rows[pivot.0][pivot.1]).invert();
        for  i in &mut self.rows[pivot.0] {
            i.multiply_by(&coefficient, gcd_cache)?;
        }

        self.rhs[pivot.0].multiply_by(&coefficient, gcd_cache)?;
        Ok(coefficient)
    }

    /// Normalize i-th row by adding n*pivot row to it
    /// Objective row's index is treated as last row + 1 (rows.len())
    /// Return coefficient by which the row was multiplied
    fn normalize_row_by_pivot_row(&mut self, pivot: &(usize, usize), target_row: usize, gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        let coefficient = (self.rows[target_row][pivot.1].divide(&self.rows[pivot.0][pivot.1], gcd_cache)?).negate();
        
        let (target_row, pivot_row) = get_two_rows_mut(&mut self.rows, target_row, pivot.0);
        for (index, target_row_value) in target_row.iter_mut().enumerate() {
            let add_val = pivot_row[index].multiply(&coefficient, gcd_cache)?;
            target_row_value.add_to(&add_val, gcd_cache)?;
        }
        Ok(coefficient)
    }
}


#[cfg(test)]
mod tests {
    use crate::rationals::{GcdCache, Rational};
    use super::super::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing;

    #[test]
    fn normalize_pivot_row_succeeds() {
        let mut gcd_cache = GcdCache::init();
        let mut simplex_table = create_minimal_simplex_table_for_testing();
        let coefficient = simplex_table.normalize_pivot_row(&(0,1), &mut gcd_cache).unwrap();
        assert_eq!(coefficient, Rational::new(1,2));

        assert_eq!(simplex_table.rows[0], vec![Rational::new(1,2), Rational::new(1,1), Rational::new(1,2), Rational::zero()]);
        assert_eq!(simplex_table.rhs[0], Rational::new(1,1));
    }
    
    #[test]
    fn normalize_row_by_pivot_row_succeeds() {
        let mut gcd_cache = GcdCache::init();
        let mut simplex_table = create_minimal_simplex_table_for_testing();

        simplex_table.normalize_row_by_pivot_row(&(0, 1), 1, &mut gcd_cache).unwrap();
        assert_eq!(simplex_table.rows[1], vec!(Rational::new(3,2), Rational::zero(), Rational::new(-1,2), Rational::from_integer(1)));
        assert_eq!(simplex_table.rhs[0], Rational::from_integer(2));
    }

}