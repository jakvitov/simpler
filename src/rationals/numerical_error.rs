use std::fmt::Display;
use std::fmt::Formatter;


#[derive(Debug)]
pub struct NumericalError {
    reason: String,
    input: String
}

impl Display for NumericalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Numerical error: {}. Input: {}.", &self.reason, &self.input)
    }
}

impl NumericalError {
    pub fn new(reason: &str, input: String) -> NumericalError{
        NumericalError { reason: String::from(reason), input }
    }
}