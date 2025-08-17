use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct ParserError {
    message: String,
    structure: String
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ParserError: {}. Structure: {}.", &self.message, &self.structure)
    }
}

impl Error for ParserError {

}