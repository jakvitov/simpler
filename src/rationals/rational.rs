mod parsers;

use parsers::ParserError;

pub struct Rational {
    numerator: i128,
    denominator: i128,   
}

impl Rational {

    fn from_str(input: &str) -> Result<Rational, Box<ParserError>> {
        let split: Vec<&str> = input.split("/").collect();
        
        if split.len() > 2 {
            return Box::new(ParserError::new("Rational contains more than one /.", input));
        }
        else if split.len() == 2 {
            let Ok(numerator) = split[0].parse() else {
                return Box::new(ParserError("Rational has invalid numerator", input));
            };
            
            let Ok(denominator) = split[1].parse() else {
                return Box::new(ParserError("Rational has invalid denominator", input));
            };

            return Result::Ok(Rational{numerator: numerator, denominator: denominator});
        }
        else if split.len() == 1 {
            let Ok(numerator) = split[0].parse() else {
                return Box::new(ParserError("Rational has invalid numerator", input));
            };

            return Rational{numerator: numerator, denominator: denominator};
        }

    }

    fn to_latex_string(&self) -> String {
        return format!("\\frac{{}}{{}}", self.numerator, self.denominator);
    }

    fn to_string(&self) -> String {
        return format!("{}/{}", self.numerator, self.denominator);
    }

}

#[tests]
mod tests {


}