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
   fn vector_multiplication_succeeds() {
       let mut gcd_cache = GcdCache::init();
       let a = RationalMatrix::from_value(1,2, Rational::from_integer(2));
       let b = RationalMatrix::from_value(2,1, Rational::from_integer(1));
       assert_eq!(RationalMatrix::mul(&a,&b, &mut gcd_cache).expect("Error"), RationalMatrix::from_value(1,1, Rational::from_integer(4)));
   }
}