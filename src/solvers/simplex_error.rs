use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub(super) struct SimplexError {
    reason: String,
}

impl Display for SimplexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Simplex error occured. Reason:\n{}", self.reason)
    }
}

impl SimplexError {
    pub fn new(reason: &str) -> Self {
        Self { reason: reason.to_owned() }
    }

    pub fn from_string_reason(reason: String) -> SimplexError {
        SimplexError { reason }
    }
}

impl Error for SimplexError {

}