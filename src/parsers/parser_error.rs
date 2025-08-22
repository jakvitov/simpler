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

impl ParserError {

    pub fn new(message: &str, structure: &str) -> Self {
        ParserError { message: String::from(message), structure: String::from(structure)}
    }

    pub fn from_string_structure(message: &str, structure: String) -> Self {
        ParserError { message: String::from(message), structure: structure}
    }

}