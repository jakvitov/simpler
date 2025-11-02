use std::iter;
use super::Rational;

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
}

impl From<Vec<Vec<Rational>>> for RationalMatrix {
    fn from(data: Vec<Vec<Rational>>) -> RationalMatrix {
        RationalMatrix { data }
    }
}

#[cfg(test)]
mod tests {
    use crate::rationals::Rational;
    use crate::rationals::rational_matrix::RationalMatrix;

    #[test]
    fn matrix_from_value_succeeds() {
        let a = RationalMatrix::from_value(2,2, Rational::zero());
        assert_eq!(a.get_row(0).len(), 2);
        assert_eq!(a.get_row(1).len(), 2);

        assert_eq!(*a.get_row(0), vec![Rational::zero(), Rational::zero()]);
        assert_eq!(*a.get_row(1), vec![Rational::zero(), Rational::zero()]);
    }
}