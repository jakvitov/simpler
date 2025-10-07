use crate::document::html_convertible_error::HtmlConvertibleError;
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

impl HtmlConvertibleError for NumericalError {
    fn to_html_string(&self) -> String {
        format!("<p><code><b>Reason:</b> {}</code></p>\n<p><code><b>Input:</b> {}</code></p>", &self.reason, &self.input)
    }

    fn get_error_name(&self) -> String {
        "Numerical error".to_owned()
    }
}