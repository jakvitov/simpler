use crate::parsers::ParserError;

#[derive(Debug, PartialEq)]
pub struct Rational {
    numerator: i128,
    denominator: i128,   
}

impl Rational {

    fn from_str(input: &str) -> Result<Rational, Box<ParserError>> {
        let split: Vec<&str> = input.split("/").collect();
        
        if split.len() == 2 {
            let Ok(numerator) = split[0].parse() else {
                return Result::Err(Box::new(ParserError::new("Rational has invalid numerator", input)));
            };
            
            let Ok(denominator) = split[1].parse() else {
                return Result::Err(Box::new(ParserError::new("Rational has invalid denominator", input)));
            };

            return Result::Ok(Rational{numerator: numerator, denominator: denominator});
        }
        else if split.len() == 1 {
            let Ok(numerator) = split[0].parse() else {
                return Result::Err(Box::new(ParserError::new("Rational has invalid numerator", input)));
            };

            return Result::Ok(Rational{numerator: numerator, denominator: 1});
        }
        else {
            return Result::Err(Box::new(ParserError::new("Invalid string passed as Rational number.", input)));
        }
    }

    fn to_latex_string(&self) -> String {
        if self.denominator != 1 { 
            return format!("\\frac`{{`{}`}}``{{`{}`}}`", self.numerator, self.denominator);
        } else {
            return format!("{}", self.numerator);
        }
    }

    fn to_string(&self) -> String {
        if self.denominator != 1 {
            return format!("{}/{}", self.numerator, self.denominator);
        } else {
            return format!("{}", self.numerator);
        }
    }

}

#[cfg(test)]
mod tests {

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
}