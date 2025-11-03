use std::iter;
use super::GcdCache;
use super::{NumericalError, Rational};

/// General matrix of rational numbers
/// Implemented using dense vectors
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct RationalMatrix {
    data: Vec<Vec<Rational>>
}

impl RationalMatrix {
    ///Return A from Q^{mxn} containing val at all positions
    pub fn from_value(m: usize, n: usize, val: Rational) -> RationalMatrix {
        let mut data = Vec::with_capacity(m);
        for i in 0..m {
            data.push(iter::repeat(val).take(n).collect());
        }
        RationalMatrix{data}
    }

    /// Create matrix from rows
    /// Return Option::empty() in case of different row lengths
    pub fn from_rows(data: Vec<Vec<Rational>>) -> Option<RationalMatrix> {
        if data.is_empty() || data.len() == 1 {
            return Some(RationalMatrix { data })
        }
        if data.len() > 1 {
            let len = data[0].len();
            for i in &data {
                if i.len() != len {
                    return None;
                }
            }
        }
        Some(RationalMatrix {data})
    }

    pub fn get_row(&self, i: usize) -> &Vec<Rational> {
        &self.data[i]
    }

    pub fn get(&self, m: usize, n: usize) -> &Rational {
        &self.data[m][n]
    }

    /// dim(rows,cols)
    pub fn dim(&self) -> (usize, usize) {
        if self.data.is_empty() {
            return (0, 0);
        }
        (self.data.len(), self.data[0].len())
    }

    ///Multiply given matrices, yield new result as newly allocated matrix
    pub fn mul(lhs: &RationalMatrix, rhs: &RationalMatrix, gcd_cache: &mut GcdCache) -> Result<RationalMatrix, Box<NumericalError>> {
        if lhs.dim().1 != rhs.dim().0 {
            return Err(Box::new(NumericalError::new("Cannot multiply matrices, incompatible dimensions.", format!("R: {}x{}. L: {}x{}.", rhs.dim().0, rhs.dim().1, lhs.dim().0, lhs.dim().1))));
        }
        let mut res = Self::from_value(lhs.dim().0, rhs.dim().1, Rational::zero());
        for lhs_row in 0..lhs.dim().0 {
            for rhs_col in 0..rhs.dim().1 {
                let mut sum = Rational::zero();
                for i in 0..lhs.dim().1 {
                    let mul_res = lhs.get(lhs_row, i).multiply(rhs.get(i, rhs_col), gcd_cache)?;
                    sum.add_mut(&mul_res, gcd_cache)?;
                }
                res.data[lhs_row][rhs_col] = sum;
            }
        }
        Ok(res)
    }

    /// Return new matrix with addition of the lhs + rhs
    /// Fails on incompatible dimensions or addition failure
    pub fn add(lhs: &RationalMatrix, rhs: &RationalMatrix, gcd_cache: &mut GcdCache) -> Result<RationalMatrix, Box<NumericalError>> {
        if lhs.dim() != rhs.dim() {
            return Err(Box::new(NumericalError::new("Cannot add matrices, incompatible dimensions.", format!("R: {}x{}. L: {}x{}.", rhs.dim().0, rhs.dim().1, lhs.dim().0, lhs.dim().1))));
        }

        let mut res = lhs.clone();
        for i in 0..lhs.dim().0 {
            for j in 0..lhs.dim().1 {
                res.data[i][j].add_mut(rhs.get(i, j), gcd_cache)?;
            }
        }
        Ok(res)
    }

    /// Return new matrix as transpose of slef
    pub fn transpose(&self) -> RationalMatrix {
        let mut res = Self::from_value(self.dim().1, self.dim().0, Rational::zero());
        for i in 0..self.dim().0 {
            for j in 0..self.dim().1 {
                res.data[j][i] = self.data[i][j];
            }
        }
        res
    }

    pub fn is_unit_matrix(&self) -> bool {
        let one = Rational::from_integer(1);
        let zero = Rational::zero();
        for i in 0..self.dim().0 {
            for j in 0..self.dim().1 {
                if (i == j && self.data[i][j] != one) || (i != j && self.data[i][j] != zero) {
                    return false;
                }
            }
        }

        true
    }

    ///Return new matrix as inverse of the current one
    ///Return Ok(None) in case of singular matrix
    ///Return Numerical error in case of numerical error during computation
    pub fn inverse(&self, gcd_cache: &mut GcdCache) -> Result<Option<RationalMatrix>, Box<NumericalError>> {
        if self.dim().0 != self.dim().1 {
            return Ok(None);
        }
        if self.dim().0 == 1 {
            return Ok(Some(RationalMatrix::from_value(1,1,self.data[0][0].invert())))
        }
        if self.dim().0 == 2 {
            return Ok(Some(self.construct_inverse_matrix_for_two_by_two_matrix()));
        }
        Ok(None)
    }

    fn construct_inverse_matrix_for_two_by_two_matrix(&self) -> RationalMatrix {
        debug_assert_eq!(self.dim(), (2,2));
        let mut res = self.clone();
        let temp = res.data[1][1];
        res.data[1][1] = res.data[0][0];
        res.data[0][0] = temp;

        res.data[0][1].invert_mut();
        res.data[1][0].invert_mut();
        res
    }
}

impl From<Vec<Vec<Rational>>> for RationalMatrix {
    fn from(data: Vec<Vec<Rational>>) -> RationalMatrix {
        RationalMatrix { data }
    }
}

#[cfg(test)]
mod tests {
    use crate::rationals::{GcdCache, Rational};
    use crate::rationals::rational_matrix::RationalMatrix;

    #[test]
    fn matrix_from_value_succeeds() {
        let a = RationalMatrix::from_value(2,2, Rational::zero());
        assert_eq!(a.get_row(0).len(), 2);
        assert_eq!(a.get_row(1).len(), 2);

        assert_eq!(*a.get_row(0), vec![Rational::zero(), Rational::zero()]);
        assert_eq!(*a.get_row(1), vec![Rational::zero(), Rational::zero()]);
    }

    #[test]
    fn matrix_from_empty_rows_succeeds() {
        let a = RationalMatrix::from_rows(vec![]);
        assert!(a.is_some());
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    fn matrix_from_diff_length_rows_succeeds(){
        let mut rows = Vec::new();
        rows.push(vec![Rational::zero(), Rational::zero()]);
        rows.push(vec![Rational::zero()]);
        let a = RationalMatrix::from_rows(rows);
        assert!(a.is_none());
    }

    #[test]
    fn dim_for_empty_matrix_succeeds() {
        let a = RationalMatrix::from_value(0,0, Rational::zero());
        assert_eq!(a.dim(), (0,0));
    }

    #[test]
    fn vector_multiplication_succeeds() {
       let mut gcd_cache = GcdCache::init();
       let a = RationalMatrix::from_value(1,2, Rational::from_integer(2));
       let b = RationalMatrix::from_value(2,1, Rational::from_integer(1));
       assert_eq!(RationalMatrix::mul(&a,&b, &mut gcd_cache).expect("Error"), RationalMatrix::from_value(1,1, Rational::from_integer(4)));
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    fn matrix_multiplication_succeeds() {
       let mut gcd_cache = GcdCache::init();
       let mut a_rows = Vec::with_capacity(2);
       a_rows.push(vec![Rational::from_integer(1), Rational::from_integer(2)]);
       a_rows.push(vec![Rational::from_integer(2), Rational::from_integer(3)]);
       let a = RationalMatrix::from_rows(a_rows);
       assert!(a.is_some());
       let a = a.unwrap();

       let mut b_rows = Vec::with_capacity(2);
       b_rows.push(vec![Rational::from_integer(1), Rational::from_integer(2), Rational::from_integer(3)]);
       b_rows.push(vec![Rational::from_integer(3), Rational::from_integer(2), Rational::from_integer(5)]);

       let b = RationalMatrix::from_rows(b_rows);
       assert!(b.is_some());
       let b = b.unwrap();

       let c = RationalMatrix::mul(&a, &b, &mut gcd_cache);
       assert!(c.is_ok());

       let c = c.unwrap();

       let mut d_rows = Vec::with_capacity(2);
       d_rows.push(vec![Rational::from_integer(7), Rational::from_integer(6), Rational::from_integer(13)]);
       d_rows.push(vec![Rational::from_integer(11), Rational::from_integer(10), Rational::from_integer(21)]);

       let d = RationalMatrix::from_rows(d_rows);
       assert!(d.is_some());
       let d = d.unwrap();

       assert_eq!(c, d);
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    fn matrix_multiplication_fails_for_wrong_dimensions() {
        let mut gcd_cache = GcdCache::init();
        let mut a_rows = Vec::with_capacity(1);
        a_rows.push(vec![Rational::from_integer(1)]);
        let a = RationalMatrix::from_rows(a_rows);
        assert!(a.is_some());
        let a = a.unwrap();

        let mut b_rows = Vec::with_capacity(2);
        b_rows.push(vec![Rational::from_integer(1), Rational::from_integer(2), Rational::from_integer(3)]);
        b_rows.push(vec![Rational::from_integer(3), Rational::from_integer(2), Rational::from_integer(5)]);

        let b = RationalMatrix::from_rows(b_rows);
        assert!(b.is_some());
        let b = b.unwrap();

        let c = RationalMatrix::mul(&a, &b, &mut gcd_cache);
        assert!(c.is_err());
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    fn matrix_transpose_succeeds_for_nonempty_matrix() {
        let mut a_rows = Vec::with_capacity(2);
        a_rows.push(vec![Rational::from_integer(1), Rational::from_integer(4), Rational::from_integer(6)]);
        a_rows.push(vec![Rational::from_integer(5), Rational::from_integer(3), Rational::from_integer(2)]);
        let a = RationalMatrix::from_rows(a_rows);
        assert!(a.is_some());
        let a = a.unwrap();
        let at = a.transpose();

        let mut a_transposed_rows = Vec::with_capacity(2);
        a_transposed_rows.push(vec![Rational::from_integer(1), Rational::from_integer(5)]);
        a_transposed_rows.push(vec![Rational::from_integer(4), Rational::from_integer(3)]);
        a_transposed_rows.push(vec![Rational::from_integer(6), Rational::from_integer(2)]);
        let a_transposed = RationalMatrix::from_rows(a_transposed_rows);
        assert!(a_transposed.is_some());
        let a_transposed = a_transposed.unwrap();

        assert_eq!(at, a_transposed);
    }

    #[test]
    fn matrix_transpose_suceeds_for_empty_matrix() {
        let empty_matrix = RationalMatrix::from_value(0,0, Rational::from_integer(1));
        let b = empty_matrix.transpose();
        assert_eq!(b,  RationalMatrix::from_value(0,0, Rational::from_integer(10)))
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    fn matrix_addition_succeeds() {
        let mut gcd_cache = GcdCache::init();

        let mut b_rows = Vec::with_capacity(2);
        b_rows.push(vec![Rational::from_integer(1), Rational::from_integer(2), Rational::from_integer(3)]);
        b_rows.push(vec![Rational::from_integer(3), Rational::from_integer(2), Rational::from_integer(5)]);

        let b = RationalMatrix::from_rows(b_rows);
        assert!(b.is_some());
        let b = b.unwrap();

        let c = RationalMatrix::add(&b, &b, &mut gcd_cache);
        assert!(c.is_ok());
        let c = c.unwrap();

        let mut d_rows = Vec::with_capacity(2);
        d_rows.push(vec![Rational::from_integer(2), Rational::from_integer(4), Rational::from_integer(6)]);
        d_rows.push(vec![Rational::from_integer(6), Rational::from_integer(4), Rational::from_integer(10)]);

        let d = RationalMatrix::from_rows(d_rows);
        assert!(d.is_some());
        let d = d.unwrap();

        assert_eq!(c,d);
    }

    #[test]
    fn matrix_addition_for_different_lengths_fails() {
        let mut gcd_cache = GcdCache::init();
        let a = RationalMatrix::from_value(2,2, Rational::from_integer(1));
        let b = RationalMatrix::from_value(3,2, Rational::from_integer(3));
        let c = RationalMatrix::add(&a, &b, &mut gcd_cache);
        assert!(c.is_err());
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    fn is_unit_matrix_succeeds_for_unit_matrix() {
        let mut a_rows = Vec::with_capacity(2);
        a_rows.push(vec![Rational::from_integer(1), Rational::from_integer(0), Rational::from_integer(0)]);
        a_rows.push(vec![Rational::from_integer(0), Rational::from_integer(1), Rational::from_integer(0)]);
        let a = RationalMatrix::from_rows(a_rows);
        assert!(a.is_some());
        let a = a.unwrap();

        assert!(a.is_unit_matrix());
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    fn is_unit_matrix_succeeds_for_non_unit_matrix() {
        let mut a_rows = Vec::with_capacity(2);
        a_rows.push(vec![Rational::from_integer(1), Rational::from_integer(0), Rational::from_integer(0)]);
        a_rows.push(vec![Rational::from_integer(1), Rational::from_integer(1), Rational::from_integer(0)]);
        let a = RationalMatrix::from_rows(a_rows);
        assert!(a.is_some());
        let a = a.unwrap();

        assert!(!a.is_unit_matrix());
    }

    #[test]
    #[allow(clippy::vec_init_then_push)]
    fn inverse_matrix_for_two_by_two_matrix_succeeds() {
        let mut a_rows = Vec::with_capacity(2);
        a_rows.push(vec![Rational::from_integer(4), Rational::from_integer(3)]);
        a_rows.push(vec![Rational::from_integer(1), Rational::from_integer(1)]);
        let a = RationalMatrix::from_rows(a_rows);
        assert!(a.is_some());
        let a = a.unwrap();

        let mut b_rows = Vec::with_capacity(2);
        b_rows.push(vec![Rational::from_integer(1), Rational::from_integer(-3)]);
        b_rows.push(vec![Rational::from_integer(-1), Rational::from_integer(4)]);
        let b = RationalMatrix::from_rows(b_rows);
        assert!(b.is_some());
        let b = b.unwrap();
    }

    #[test]
    fn inverse_matrix_for_one_by_one_matrix_succeeds() {
        let mut gcd_cache = GcdCache::init();
        let a = RationalMatrix::from_value(1,1, Rational::from_integer(2));
        let b = a.inverse(&mut gcd_cache);
        assert!(b.is_ok());
        let b = b.unwrap();
        assert!(b.is_some());
        let b = b.unwrap();
        assert_eq!(b.data[0][0], Rational::new(1,2));
    }

}