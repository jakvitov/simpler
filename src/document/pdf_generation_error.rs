use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub struct PdfGenerationError {
    message: String,
    problem: String
}

impl Display for PdfGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Pdf generation error occured.\n{}\nProblem:\n{}",self.message, self.problem)
    }
}

impl Error for PdfGenerationError {

}

impl PdfGenerationError {
    pub fn new(message: String, problem: String) -> PdfGenerationError {
        PdfGenerationError { message, problem }
    }

    pub fn from_string_problem(message: &str, problem: String) -> PdfGenerationError {
        PdfGenerationError {message: message.to_string(), problem}
    }
}