use crate::parsers::ParserError;
use crate::rationals::gcd_cache::GcdCache;
use crate::rationals::numerical_error::NumericalError;
use crate::utils::math::divide_exact;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Rational number
#[derive(Debug, Copy, Clone, Eq)]
pub struct Rational {
    numerator: i128,
    denominator: i128,
}

impl Rational {

    pub fn new(mut numerator: i128, mut denominator: i128) -> Self {
        if denominator < 0 {
            numerator = -numerator;
            denominator = - denominator;
        }
        Rational {numerator, denominator}
    }

    pub fn from_integer(numerator: i128) -> Self {
        Self::new(numerator, 1)
    }

    pub fn zero() -> Self {
        Rational {numerator: 0, denominator: 1}
    }

    pub fn negate(&self) -> Self {
        Rational {numerator: -self.numerator, denominator: self.denominator}
    }

    pub fn negate_mut(&mut self) {
        self.numerator = -self.numerator;
    }

    /// self > 0
    pub fn is_positive(&self) -> bool {
        !(self.numerator == 0 || self.is_negative())
    }

    /// self < 0
    pub fn is_negative(&self) -> bool {
        (self.numerator < 0) ^ (self.denominator < 0)
    }

    pub fn invert(&self) -> Rational {
        Self::new(self.denominator, self.numerator)
    }

    pub fn invert_mut(&mut self) {
        std::mem::swap(&mut self.denominator, &mut self.numerator);
    }

    ///Reduce given rational
    /// 2/4 -> 1/2
    /// -2/4 -> -1/4
    #[allow(dead_code)]
    fn reduce(&mut self, gcd_cache: &mut GcdCache) -> Result<(), Box<NumericalError>> {
        let gcd = gcd_cache.gcd(self.numerator.abs(), self.denominator.abs())?;
        if gcd == 1 {
            Ok(())
        } else {
            self.numerator /= gcd;
            self.denominator /= gcd;
            Ok(())
        }
    }

    ///self + other = new_rational
    ///Uses provided gcd_cache for gcd and lcm operations
    #[allow(dead_code)]
    pub fn add(&self, other: &Rational, gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        let den_lcm = gcd_cache.lcm(self.denominator, other.denominator)?;
        let numerator = ((den_lcm/self.denominator)*self.numerator) + ((den_lcm/other.denominator)*other.numerator);
        let mut res = Rational::new(numerator, den_lcm);
        (&mut res).reduce(gcd_cache)?;
        Ok(res)
    }

    /// self + other = self
    /// Self is mutated and other is added to it
    pub fn add_mut(&mut self, other: &Self, gcd_cache: &mut GcdCache) -> Result<(), Box<NumericalError>> {
        let den_lcm = gcd_cache.lcm(self.denominator, other.denominator)?;
        let numerator = ((den_lcm/self.denominator)*self.numerator) + ((den_lcm/other.denominator)*other.numerator);
        self.numerator = numerator;
        self.denominator = den_lcm;
        self.reduce(gcd_cache)?;
        Ok(())
    }

    ///self - other = new_rational
    /// Uses provided gcd_cache for gcd and lcm operations
    #[allow(dead_code)]
    pub fn subtract(&self, other: &Self, gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        let den_lcm = gcd_cache.lcm(self.denominator, other.denominator)?;
        let numerator = ((den_lcm/self.denominator)*self.numerator) - ((den_lcm/other.denominator)*other.numerator);
        let mut res = Rational::new(numerator, den_lcm);
        (&mut res).reduce(gcd_cache)?;
        Ok(res)
    }

    ///self * other = new_rational
    /// Uses provided gcd_cache for gcd and lcm operations
    #[allow(dead_code)]
    pub fn multiply(&self, other: &Self, gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        let numerator = self.numerator * other.numerator;
        let denominator = self.denominator * other.denominator;
        let mut res = Rational::new(numerator, denominator);
        res.reduce(gcd_cache)?;
        Ok(res)
    }
    
    ///Mutate self as result of self * other
    pub fn multiply_mut(&mut self, other: &Self, gcd_cache: &mut GcdCache) -> Result<(), Box<NumericalError>> {
        self.numerator = self.numerator * other.numerator;
        self.denominator = self.denominator * other.denominator;
        self.reduce(gcd_cache)?;
        Ok(())
    }

    /// self / other = new_rational
    /// Usees provided gcd_cache for gcd and lcm operations
    #[allow(dead_code)]
    pub fn divide(&self, other: &Self, gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        let numerator = self.numerator * other.denominator;
        let denominator = self.denominator * other.numerator;
        let mut res = Rational::new(numerator, denominator);
        res.reduce(gcd_cache)?;
        Ok(res)
    }

    /// Mutate self to contain self/other
    pub fn divide_by(&mut self, other: &Self, gcd_cache: &mut GcdCache)-> Result<(), Box<NumericalError>> {
        let numerator = self.numerator * other.denominator;
        let denominator = self.denominator * other.numerator;
        self.numerator = numerator;
        self.denominator = denominator;
        self.reduce(gcd_cache)?;
        Ok(())
    }

    pub fn to_mmdn_with_sign(&self) -> String {
        let mut res = String::new();
        if self.is_positive() {
            res.push_str("\n<mo>+</mo>");
            res.push_str(self.to_mmdn().as_str());
        } else {
            res.push_str("\n<mo>-</mo>");
            //After we add - we must negate the rational for the string
            res.push_str(self.negate().to_mmdn().as_str());
        }
        res
    }

    fn to_mmdn(&self) -> String {
        if self.denominator == 1 {
            format!("\n<mn>{}</mn>\n", self.numerator)
        } else {
            format!("\n<mfrac><mn>{}</mn><mn>{}</mn></mfrac>\n", self.numerator, self.denominator)
        }
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.denominator == 0 || other.denominator == 0 {
            println!("here");
        }
        let common_denominator = self.denominator * other.denominator;
        ((common_denominator / self.denominator) * self.numerator).partial_cmp(&((common_denominator / other.denominator) * other.numerator))
    }
}

impl FromStr for Rational {
    type Err = Box<ParserError>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = input.split("/").collect();

        if split.len() == 2 {
            let Ok(numerator) = split[0].parse() else {
                return Err(Box::new(ParserError::new("Rational has invalid numerator", input)));
            };

            let Ok(denominator) = split[1].parse() else {
                return Err(Box::new(ParserError::new("Rational has invalid denominator", input)));
            };

            Ok(Rational::new(numerator, denominator))
        }
        else if split.len() == 1 {
            let Ok(numerator) = split[0].parse() else {
                return Err(Box::new(ParserError::new("Rational has invalid numerator", input)));
            };

            Ok(Rational{numerator, denominator: 1})
        }
        else {
            Err(Box::new(ParserError::new("Invalid string passed as Rational number.", input)))
        }
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.denominator != 1 {
            write!(f, "{}/{}", self.numerator, self.denominator)
        } else {
            write!(f, "{}", self.numerator)
        }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        if (self.numerator == other.numerator) && (self.denominator == other.denominator) {
            true
        }
        else if self.numerator.abs() > other.numerator.abs() {
            let Some(ratio) = divide_exact(self.numerator, other.numerator) else {
                return false;
            };
            if other.denominator * ratio == self.denominator {
                return true;
            }
            false
        }
        else  {
            //This way we would get ratio as None since we canot divide by zero and eval as false
            if self.numerator == 0 && 0 == other.numerator {
                return true;
            }
            let Some(ratio) = divide_exact(other.numerator, self.numerator) else {
                return false;
            };
            if self.denominator * ratio == other.denominator {
                return true;
            }
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::rationals::gcd_cache::GcdCache;
    use crate::rationals::Rational;
    use std::str::FromStr;

    #[test]
    fn negative_numerator_and_denominator_conversion_succeeds() {
        let res1 = Rational::new(-2, -3);
        let result2 = Rational::from_str("-2/-3");

        assert!(!result2.is_err());

        let Ok(res2) = result2 else {
            panic!("Conversion from string to rational failed");
        };

        assert_eq!(res1, Rational::new(2, 3));
        assert_eq!(res2, Rational::new(2, 3));
    }

    #[test]
    fn from_str_valid_numer_denom_suceeds() {
        let test_str = "2833/8383";
        let res = super::Rational::from_str(test_str);
        assert!(res.is_err() == false);
        
        let Ok(rational_parsed) = res else {
            return;
        };
        
        assert_eq!(rational_parsed, super::Rational{numerator: 2833, denominator: 8383});
    }

    #[test]
    fn from_str_valid_only_numer_suceeds() {
        let test_str = "2833338383";
        let res = super::Rational::from_str(test_str);
        assert!(res.is_err() == false);
        
        let Ok(rational_parsed) = res else {
            return;
        };

        assert_eq!(rational_parsed, super::Rational {numerator: 2833338383, denominator: 1})

    }

    #[test]
    fn from_str_invalid_string_fails() {
        let test_str_multiple_slashes = "39393/944/93";
        let test_str_not_a_number = "94944J949K";
        let test_empty_string = "";

        let test_str_multiple_slashes_res = super::Rational::from_str(test_str_multiple_slashes);
        let test_str_not_a_number_res = super::Rational::from_str(test_str_not_a_number);
        let test_empty_string_res = super::Rational::from_str(test_empty_string);

        assert!(test_str_multiple_slashes_res.is_err());
        assert!(test_str_not_a_number_res.is_err());
        assert!(test_empty_string_res.is_err());
    }

    #[test]
    fn print_to_string_suceeds() {
        let input_numer_denom_str = "28373793/2";
        let input_only_numer_str = "1";

        let Ok(input_numer_denom_str_result) = super::Rational::from_str(input_numer_denom_str) else {
            panic!("{input_numer_denom_str} could not be parsed into Rational!");
        };

        let Ok(input_only_numer_str_result) = super::Rational::from_str(input_only_numer_str) else {
            panic!("{input_only_numer_str} could not be parsed into Rational!");
        };
        
        assert_eq!(input_numer_denom_str_result.to_string(), input_numer_denom_str);
        assert_eq!(input_only_numer_str_result.to_string(), input_only_numer_str);
    }

    #[test]
    fn addition_without_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(1, 2);
        let b = Rational::new(1, 7);

        let Ok(res) = a.add(&b, &mut gcd_cache) else {
            panic!("Error during calculation!")
        };

        assert_eq!(res, Rational{numerator: 9, denominator: 14});
    }

    #[test]
    fn addition_with_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(1, 2);
        let b = Rational::new(3, 2);

        let Ok(res) = a.add(&b, &mut gcd_cache) else {
            panic!("Error during calculation!")
        };

        assert_eq!(res, Rational{numerator: 2, denominator: 1});
    }

    #[test]
    fn add_to_without_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let mut a = Rational::new(1, 2);
        let b = Rational::new(1, 7);

        a.add_mut(&b, &mut gcd_cache).unwrap();

        assert_eq!(a, Rational{numerator: 9, denominator: 14});
    }

    #[test]
    fn add_to_with_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let mut a = Rational::new(1, 2);
        let b = Rational::new(3, 2);

        a.add_mut(&b, &mut gcd_cache).unwrap();

        assert_eq!(a, Rational{numerator: 2, denominator: 1});
    }

    #[test]
    fn subtraction_without_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(1, 2);
        let b = Rational::new(2, 6);

        let Ok(res) = a.subtract(&b, &mut gcd_cache) else {
            panic!("Error during calculation!");
        };

        assert_eq!(res, Rational{numerator: 1, denominator: 6});
    }

    #[test]
    fn subtraction_with_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(3, 2);
        let b = Rational::new(2, 4);

        let Ok(res) = a.subtract(&b, &mut gcd_cache) else {
            panic!("Error during calculation!");
        };

        assert_eq!(res, Rational{numerator: 1, denominator: 1});
    }

    #[test]
    fn subtraction_with_overlow_below_zero_switches_signs() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(1, 2);
        let b = Rational::new(5, 6);

        let Ok(res) = a.subtract(&b, &mut gcd_cache) else {
            panic!("Error during calculation!");
        };

        assert_eq!(res, Rational{numerator: -1, denominator: 3});
    }

    #[test]
    fn multiplication_without_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(1, 7);
        let b = Rational::new(1, 1);

        let res = a.multiply(&b, &mut gcd_cache).unwrap();

        assert_eq!(res, Rational{numerator: 1, denominator: 7});
    }

    #[test]
    fn multiplication_with_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(6, 7);
        let b = Rational::new(33, 28);

        let res = a.multiply(&b, &mut gcd_cache).unwrap();

        assert_eq!(res, Rational{numerator: 99, denominator: 98});
    }

    #[test]
    fn multiplication_with_negatives_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(-6, -7);
        let b = Rational::new(-33, 28);

        let res = a.multiply(&b, &mut gcd_cache).unwrap();

        assert_eq!(res, Rational{numerator: -99, denominator: 98});
    }

    #[test]
    fn multiplication_by_without_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let mut a = Rational::new(1, 7);
        let b = Rational::new(1, 1);

        a.multiply_mut(&b, &mut gcd_cache).unwrap();

        assert_eq!(a, Rational{numerator: 1, denominator: 7});
    }

    #[test]
    fn multiplication_by_with_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let mut a = Rational::new(6, 7);
        let b = Rational::new(33, 28);

       a.multiply_mut(&b, &mut gcd_cache).unwrap();

        assert_eq!(a, Rational{numerator: 99, denominator: 98});
    }

    #[test]
    fn multiplication_by_with_negatives_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let mut a = Rational::new(-6, -7);
        let b = Rational::new(-33, 28);

        a.multiply_mut(&b, &mut gcd_cache).unwrap();

        assert_eq!(a, Rational{numerator: -99, denominator: 98});
    }

    #[test]
    fn division_without_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(1, 2);
        let b = Rational::new(2, 5);

        let res = a.divide(&b, &mut gcd_cache).unwrap();

        assert_eq!(res, Rational{numerator: 5, denominator: 4});
    }

    #[test]
    fn division_with_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(2, 5);
        let b = Rational::new(2, 10);

        let res = a.divide(&b, &mut gcd_cache).unwrap();

        assert_eq!(res, Rational{numerator: 2, denominator: 1});
    }

    #[test]
    fn division_with_negative_values_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(-1, -10);
        let b = Rational::new(2, -5);

        let res = a.divide(&b, &mut gcd_cache).unwrap();

        assert_eq!(res, Rational{numerator: -1, denominator: 4});
    }

    #[test]
    fn division_by_without_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let mut a = Rational::new(1, 2);
        let b = Rational::new(2, 5);

       a.divide_by(&b, &mut gcd_cache).unwrap();

        assert_eq!(a, Rational{numerator: 5, denominator: 4});
    }

    #[test]
    fn division_by_with_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let mut a = Rational::new(2, 5);
        let b = Rational::new(2, 10);

        a.divide_by(&b, &mut gcd_cache).unwrap();

        assert_eq!(a, Rational{numerator: 2, denominator: 1});
    }

    #[test]
    fn division_by_with_negative_values_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let mut a = Rational::new(-1, -10);
        let b = Rational::new(2, -5);

        a.divide_by(&b, &mut gcd_cache).unwrap();

        assert_eq!(a, Rational{numerator: -1, denominator: 4});
    }

    #[test]
    fn equality_between_positive_rationals_succeeds() {
        let first = Rational::new(2, 3);
        let second = Rational::new(6, 9);
        assert_eq!(first, second);
    }

    #[test]
    fn equality_between_negative_rationals_succeeds() {
        let first = Rational::new(-2, 3);
        let second = Rational::new(6, -9);
        assert_eq!(first, second);
    }

    #[test]
    fn equality_between_zeroes_suceeds() {
        let first = Rational::zero();
        assert_eq!(first, first);
    }

    #[test]
    fn equality_between_unequal_positive_rationals_fails() {
        let first = Rational::new(2, 3);
        let second = Rational::new(7, 9);
        assert_ne!(first, second);
    }

    #[test]
    fn equality_between_unequal_negative_rationals_fails() {
        let first = Rational::new(2, -3);
        let second = Rational::new(-7, 9);
        assert_ne!(first, second);
    }

    #[test]
    fn is_positive_succeeds() {
        let first = Rational::new(2, 3);
        let second = Rational::new(-7, -9);
        assert!(first.is_positive());
        assert!(second.is_positive());
    }

    #[test]
    fn is_positive_fails_for_zero() {
        let first = Rational::zero();
        assert!(!first.is_positive());
    }

    #[test]
    fn is_negative_succeeds() {
        let first = Rational::new(-2, 3);
        let second = Rational::new(7, -9);
        assert!(first.is_negative());
        assert!(second.is_negative());
    }

    #[test]
    fn negate_positive_number_succeeds() {
        let first = Rational::new(2, 3);
        let negated = first.negate();
        assert_eq!(negated, Rational::new(-2, 3));
    }

    #[test]
    fn negate_negative_number_succeeds() {
        let first = Rational::new(2, -3);
        let negated = first.negate();
        assert_eq!(negated, Rational::new(2, 3));
    }

    #[test]
    fn negate_mut_positive_number_succeeds() {
        let mut first = Rational::new(2, 3);
        first.negate_mut();
        assert_eq!(first, Rational::new(-2, 3));
    }

    #[test]
    fn negate_mut_negative_number_succeeds() {
        let mut first = Rational::new(2, -3);
        first.negate_mut();
        assert_eq!(first, Rational::new(2, 3));
    }

    #[test]
    fn from_integer_suceeds_for_positive() {
        let num = Rational::from_integer(1);
        assert_eq!(num, Rational::new(1, 1));
    }

    #[test]
    fn from_integer_suceeds_for_positive_negative() {
        let num = Rational::from_integer(-1);
        assert_eq!(num, Rational::new(-1, 1));
    }

    #[test]
    fn partial_ord_works_for_positives() {
        let num1 = Rational::new(2,3);
        let num2 = Rational::new(1,2);
        assert!(num1 > num2);
        assert!(num2 < num1);
    }

    #[test]
    fn partial_ord_works_for_one_negative() {
        let num1 = Rational::new(2,3);
        let num2 = Rational::new(-1,1);
        assert!(num1 > num2);
        assert!(num2 < num1);
    }

    #[test]
    fn partial_ord_works_for_two_negatives() {
        let num1 = Rational::new(-2,3);
        let num2 = Rational::new(-1,1);
        assert!(num1 > num2);
        assert!(num2 < num1);
    }

    #[test]
    fn to_mmdn_suceeds_for_whole_number() {
        let num = Rational::new(2, 1);
        let res = num.to_mmdn();
        assert_eq!(res, "\n<mn>2</mn>\n")
    }

    #[test]
    fn to_mmdn_suceeds_for_partial_number() {
        let num1 = Rational::new(2, 3);
        let res = num1.to_mmdn();
        assert_eq!(res, "\n<mfrac><mn>2</mn><mn>3</mn></mfrac>\n")
    }

    #[test]
    fn to_mmdn_with_sign_succeeds_for_positive_numbers() {
        let num1 = Rational::new(2, 1);
        let num2 = Rational::new(1, 5);
        let res = num1.to_mmdn_with_sign();
        assert_eq!(res, "\n<mo>+</mo>\n<mn>2</mn>\n");
        let res2 = num2.to_mmdn_with_sign();
        assert_eq!(res2, "\n<mo>+</mo>\n<mfrac><mn>1</mn><mn>5</mn></mfrac>\n");
    }

    #[test]
    fn to_mmdn_with_sign_suceeds_for_negative_numbers() {
        let num1 = Rational::new(-2, 1);
        let num2 = Rational::new(-1, 5);
        let res = num1.to_mmdn_with_sign();
        let res2 = num2.to_mmdn_with_sign();
        assert_eq!(res, "\n<mo>-</mo>\n<mn>2</mn>\n");
        assert_eq!(res2, "\n<mo>-</mo>\n<mfrac><mn>1</mn><mn>5</mn></mfrac>\n");
    }

    #[test]
    fn inverse_mut_suceeds() {
        let mut num = Rational::new(2, 3);
        num.invert_mut();
        assert_eq!(num, Rational::new(3, 2));
    }
}