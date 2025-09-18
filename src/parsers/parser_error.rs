use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::document::html_convertible_error::HtmlConvertibleError;

#[derive(Debug)]
pub struct ParserError {
    message: String,
    structure: String
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Parser error occurred:\n{}.\nFailing structure:\n{}.", &self.message, &self.structure)
    }
}

impl Error for ParserError {

}

impl ParserError {

    pub fn new(message: &str, structure: &str) -> Self {
        ParserError { message: String::from(message), structure: String::from(structure)}
    }

    #[allow(dead_code)]
    pub fn from_string_structure(message: &str, structure: String) -> Self {
        ParserError { message: String::from(message), structure: structure}
    }

    pub fn from_string_message(message: String, structure: &str) -> Self {
        ParserError { message: message, structure: String::from(structure)}
    }
}

impl HtmlConvertibleError for ParserError {
    fn to_html_string(&self) -> String {
        if self.structure.is_empty() {
            format!("<p><code><b>Message:</b> {}</code></p>\n", self.message)
        } else {
            format!("<p><code><b>Message:</b> {}</code></p>\n<p><code><b>Structure:</b> {}</code></p>\n", self.message, self.structure)
        }
    }

    fn get_error_name(&self) -> String {
        String::from("Parser error")
    }
}