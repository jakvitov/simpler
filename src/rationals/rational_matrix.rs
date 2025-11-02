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

    pub fn dim(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }

    pub fn mul(lhs: &RationalMatrix, rhs: &RationalMatrix, gcd_cache: &mut GcdCache) -> Result<RationalMatrix, Box<NumericalError>> {
        if lhs.dim().1 != rhs.dim().0 {
            return Err(Box::new(NumericalError::new("Cannot multiply matrixes, incompatible dimensions.", format!("R: {}x{}. L: {}x{}.", rhs.dim().0, rhs.dim().1, lhs.dim().0, lhs.dim().1))));
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
    fn matrix_from_diff_length_rows_succeeds(){
        let mut rows = Vec::new();
        rows.push(vec![Rational::zero(), Rational::zero()]);
        rows.push(vec![Rational::zero()]);
        let a = RationalMatrix::from_rows(rows);
        assert!(a.is_none());
    }
   #[test]
   fn vector_multiplication_succeeds() {
       let mut gcd_cache = GcdCache::init();
       let a = RationalMatrix::from_value(1,2, Rational::from_integer(2));
       let b = RationalMatrix::from_value(2,1, Rational::from_integer(1));
       assert_eq!(RationalMatrix::mul(&a,&b, &mut gcd_cache).expect("Error"), RationalMatrix::from_value(1,1, Rational::from_integer(4)));
   }

   #[test]
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
}