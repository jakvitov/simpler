use std::ops::{Div, Rem};

/// Function accepting two integer types and returning
/// Some(a) if the two integers can be divided without remainder
/// None otherwise
/// divide_exact(9,3) = 9/3 = Some(3)
/// divide_exact(10,3) ~ 10/3 -> None
pub fn divide_exact<T>(a: T, b: T) -> Option<T>
where
    T: Div<Output = T> + Rem<Output = T> + PartialEq + Copy + From<u8>,
{
    let zero = T::from(0);
    if b == zero || a % b != zero {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::math::divide_exact;

    #[test]
    fn divide_exact_returns_some_for_positives_successfully() {
        let result = divide_exact(22i32, 11i32);
        assert!(result.is_some());
        let res = result.unwrap();
        assert_eq!(res, 2i32);
    }

    #[test]
    fn divide_exact_returns_some_for_one_negative_successfully() {
        let result = divide_exact(-22i32, 11i32);
        assert!(result.is_some());
        let res = result.unwrap();
        assert_eq!(res, -2i32);
    }

    #[test]
    fn divide_exact_returns_some_for_both_negative_successfully() {
        let result = divide_exact(-22i32, -11i32);
        assert!(result.is_some());
        let res = result.unwrap();
        assert_eq!(res, 2i32);
    }

    #[test]
    fn divide_exact_returns_none_successfully() {
        let result = divide_exact(-22i32, 12i32);
        assert!(result.is_none());
    }
}