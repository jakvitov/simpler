use std::fmt::{Display, Formatter};
use crate::parsers::ParserError;
use crate::rationals::gcd_cache::GcdCache;
use crate::rationals::numerical_error::NumericalError;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rational {
    numerator: i128,
    denominator: i128,
}

impl Rational {

    fn new(mut numerator: i128, mut denominator: i128) -> Self {
        if denominator < 0 {
            numerator = -numerator;
            denominator = - denominator;
        }
        Rational {numerator, denominator}
    }

    fn from_str(input: &str) -> Result<Rational, Box<ParserError>> {
        let split: Vec<&str> = input.split("/").collect();
        
        if split.len() == 2 {
            let Ok(numerator) = split[0].parse() else {
                return Err(Box::new(ParserError::new("Rational has invalid numerator", input)));
            };
            
            let Ok(denominator) = split[1].parse() else {
                return Err(Box::new(ParserError::new("Rational has invalid denominator", input)));
            };

            return Ok(Rational::new(numerator, denominator));
        }
        else if split.len() == 1 {
            let Ok(numerator) = split[0].parse() else {
                return Err(Box::new(ParserError::new("Rational has invalid numerator", input)));
            };

            return Ok(Rational{numerator, denominator: 1});
        }
        else {
            return Err(Box::new(ParserError::new("Invalid string passed as Rational number.", input)));
        }
    }

    fn to_latex_string(&self) -> String {
        if self.denominator != 1 {
            return format!("\\frac{{{}}}{{{}}}", self.numerator, self.denominator);
        } else {
            return self.numerator.to_string();
        }
    }

    ///Reduce given rational
    /// 2/4 -> 1/2
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
    fn add(&self, other: &Rational, gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        let den_lcm = gcd_cache.lcm(self.denominator, other.denominator)?;
        let numerator = ((den_lcm/self.denominator)*self.numerator) + ((den_lcm/other.denominator)*other.numerator);
        let mut res = Rational::new(numerator, den_lcm);
        (&mut res).reduce(gcd_cache)?;
        Ok(res)
    }

    ///self - other = new_rational
    /// Uses provided gcd_cache for gcd and lcm operations
    fn subtract(&self, other: &Self, gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        let den_lcm = gcd_cache.lcm(self.denominator, other.denominator)?;
        let numerator = ((den_lcm/self.denominator)*self.numerator) - ((den_lcm/other.denominator)*other.numerator);
        let mut res = Rational::new(numerator, den_lcm);
        (&mut res).reduce(gcd_cache)?;
        Ok(res)
    }

    ///self * other = new_rational
    /// Uses provided gcd_cache for gcd and lcm operations
    fn multiply(&self, other: &Self, gcd_cache: &mut GcdCache) -> Result<Rational, Box<NumericalError>> {
        let numerator = self.numerator * other.numerator;
        let denominator = self.denominator * other.denominator;
        let mut res = Rational::new(numerator, denominator);
        res.reduce(gcd_cache);
        Ok(res)
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


#[cfg(test)]
mod tests {
    use crate::rationals::gcd_cache::GcdCache;
    use crate::rationals::Rational;

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
    fn print_to_latex_string_suceeds() {
        let input_numer_denom_str = "87373/4";
        let input_only_numer_str = "1";

        let Ok(input_numer_denom_str_result) = super::Rational::from_str(input_numer_denom_str) else {
            panic!("{input_numer_denom_str} could not be parsed into Rational!");
        };

        let Ok(input_only_numer_str_result) = super::Rational::from_str(input_only_numer_str) else {
            panic!("{input_only_numer_str} could not be parsed into Rational!");
        };
        
        assert_eq!(input_numer_denom_str_result.to_latex_string(), "\\frac{87373}{4}");
        assert_eq!(input_only_numer_str_result.to_latex_string(), "1");
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

        let Ok(res) = a.multiply(&b, &mut gcd_cache) else {
            panic!("Error during calculation!");
        };

        assert_eq!(res, Rational{numerator: 1, denominator: 7});
    }

    #[test]
    fn multiplication_with_reduction_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(6, 7);
        let b = Rational::new(33, 28);

        let Ok(res) = a.multiply(&b, &mut gcd_cache) else {
            panic!("Error during calculation!");
        };

        assert_eq!(res, Rational{numerator: 99, denominator: 98});
    }

    #[test]
    fn multiplication_with_negatives_is_correct() {
        let mut gcd_cache = GcdCache::init();
        let a = Rational::new(-6, -7);
        let b = Rational::new(-33, 28);

        let Ok(res) = a.multiply(&b, &mut gcd_cache) else {
            panic!("Error during calculation!");
        };

        assert_eq!(res, Rational{numerator: -99, denominator: 98});
    }
}