use crate::rationals::{GcdCache, NumericalError, Rational};
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;
use crate::utils::collections::{get_two_elements_mut, get_two_rows_mut};

impl BasicSimplexTable {

    /// Normalize the pivot row and rhs to contain 1 in the pivot element
    /// Return coefficient by which the row was multiplied
   pub  fn normalize_pivot_row(&mut self, pivot: &(usize, usize), gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
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
    pub fn normalize_row_by_pivot_row(&mut self, pivot: &(usize, usize), target_row_index: usize, gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        debug_assert!(target_row_index != pivot.0);
        //We normalise the objective row
        if target_row_index == self.rows.len() {
            let coefficient = (self.objective_row[pivot.1].divide(&self.rows[pivot.0][pivot.1], gcd_cache)?).negate();
            for (index, target_row_value) in self.objective_row.iter_mut().enumerate() {
                let add_val = self.rows[pivot.0][index].multiply(&coefficient, gcd_cache)?;
                target_row_value.add_mut(&add_val, gcd_cache)?;
            }

            self.objective_rhs.add_mut(&coefficient.multiply(&self.rhs[pivot.0], gcd_cache)?, gcd_cache)?;
            return Ok(coefficient)
        }

        //We normalize row in rows
        let coefficient = (self.rows[target_row_index][pivot.1].divide(&self.rows[pivot.0][pivot.1], gcd_cache)?).negate();
        
        let (target_row, pivot_row) = get_two_rows_mut(&mut self.rows, target_row_index, pivot.0);
        for (index, target_row_value) in target_row.iter_mut().enumerate() {
            let add_val = pivot_row[index].multiply(&coefficient, gcd_cache)?;
            target_row_value.add_mut(&add_val, gcd_cache)?;
        }

        let (target_rhs, pivot_rhs) = get_two_elements_mut(&mut self.rhs, target_row_index, pivot.0);
        target_rhs.add_mut(&coefficient.multiply(pivot_rhs, gcd_cache)?, gcd_cache)?;
        Ok(coefficient)
    }
}


#[cfg(test)]
mod tests {
    use super::super::basic_simplex_table_data::test_utils::create_minimal_simplex_table_for_testing;
    use crate::rationals::{GcdCache, Rational};

    #[test]
    fn normalize_pivot_row_succeeds() {
        let mut gcd_cache = GcdCache::init();
        let mut simplex_table = create_minimal_simplex_table_for_testing();
        let coefficient = simplex_table.normalize_pivot_row(&(0,1), &mut gcd_cache).unwrap();
        assert_eq!(coefficient, Rational::new(1,2));

        assert_eq!(simplex_table.rows[0], vec![Rational::new(1,2), Rational::new(1,1), Rational::new(1,2), Rational::zero()]);
        assert_eq!(simplex_table.rhs[0], Rational::new(1,1));
    }

    /// Standard row = row in simplex_table.rows
    #[test]
    fn normalize_standard_row_by_pivot_row_succeeds() {
        let mut gcd_cache = GcdCache::init();
        let mut simplex_table = create_minimal_simplex_table_for_testing();

        simplex_table.normalize_row_by_pivot_row(&(0, 1), 1, &mut gcd_cache).unwrap();
        assert_eq!(simplex_table.rows[1], vec!(Rational::new(3,2), Rational::zero(), Rational::new(-1,2), Rational::from_integer(1)));
        assert_eq!(simplex_table.rhs[1], Rational::from_integer(2));
    }

    #[test]
    fn normalize_objective_row_by_pivot_row_succeeds() {
        let mut gcd_cache = GcdCache::init();
        let mut simplex_table = create_minimal_simplex_table_for_testing();

        simplex_table.normalize_row_by_pivot_row(&(0, 1), 2, &mut gcd_cache).unwrap();
        assert_eq!(simplex_table.objective_row, vec!(Rational::zero(), Rational::zero(),  Rational::from_integer(1), Rational::zero()));
        assert_eq!(simplex_table.objective_rhs, Rational::from_integer(2));
    }

}