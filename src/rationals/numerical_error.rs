use std::fmt::Display;
use std::fmt::Formatter;


#[derive(Debug)]
struct NumericalError {
    reason: String,
    input: String
}

impl Display for NumericalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Numerical error: {}. Input: {}.", &self.reason, &self.input)
    }
}

impl NumericalError {
    fn new(reason: &str, input: &str) -> NumericalError{
        return NumericalError { reason: String::from(reason), input: String::from(input) }
    }
}