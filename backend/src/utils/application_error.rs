use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::document::html_convertible_error::HtmlConvertibleError;

/// Error that indicates failure of application logic 
/// and fault on user part
/// 
#[derive(Debug)]
pub struct ApplicationError {
    reason: String,
    details: String
}

impl ApplicationError {
    pub fn new(reason: &str, details: &str) -> Self {
        Self{reason: reason.to_owned(), details: details.to_owned()}
    }
    
    pub fn with_reason(reason: &str) -> Self {
        Self{reason: reason.to_owned(), details: String::new()}
    }
    
    pub fn from_string_reason(reason: String, details: &str) -> Self {
        Self{reason, details: details.to_owned()}
    }

    pub fn from_string_details(reason: &str, details: String) -> Self {
        Self{reason: reason.to_owned(), details}
    }
}

impl Error for ApplicationError {}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.details.is_empty() {
            write!(f, "Application error occurred: {}.", self.reason)
        } else {
            write!(f, "Application error occurred: {}\n. Details: {}.", self.reason, self.details)
        }
    }
}

impl HtmlConvertibleError for ApplicationError {
    fn to_html_string(&self) -> String {
        if self.details.is_empty() {
            format!("<p><code><b>Message:</b> {}</code></p>\n", self.reason)
        } else {
            format!("<p><code><b>Message:</b> {}</code></p>\n<p><code><b>Structure:</b> {}</code></p>\n", self.reason, self.details)
        }
    }

    fn get_error_name(&self) -> String {
        "Application error".to_owned()
    }
}