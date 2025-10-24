use crate::document::html_convertible_error::HtmlConvertibleError;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseIntError;

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

impl From<ParseIntError> for NumericalError {
    fn from(e: ParseIntError) -> NumericalError {
        NumericalError::new(e.to_string().as_str(), String::new())
    }
}

impl From<std::num::TryFromIntError> for NumericalError {
    fn from(e: std::num::TryFromIntError) -> NumericalError {
        NumericalError::new(e.to_string().as_str(), String::new())
    }
}

impl HtmlConvertibleError for NumericalError {
    fn to_html_string(&self) -> String {
        if self.input.is_empty() {
            format!("<p><code><b>Reason:</b> {}</code></p>\n", &self.reason)
        } else {
        format!("<p><code><b>Reason:</b> {}</code></p>\n<p><code><b>Input:</b> {}</code></p>", &self.reason, &self.input)
        }
    }

    fn get_error_name(&self) -> String {
        "Numerical error".to_owned()
    }
}