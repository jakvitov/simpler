use crate::document::html_output::HtmlOutput;
use super::basic_simplex_table::{BasicSimplexTable, OptimizationType};
use super::simplex_error::SimplexError;

/// Solve the given simplex table using the basic simplex algoritm
/// Both simplex table and html output are edited
pub fn solve_basic_simplex(simplex_table: &mut BasicSimplexTable, html_output: &mut HtmlOutput) {

}


fn check_basic_simplex_compatibility(simplex_table: BasicSimplexTable) -> Result<(), Box<SimplexError>> {
    Ok(())
}