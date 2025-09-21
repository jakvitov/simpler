use crate::rationals::Rational;
use crate::solvers::basic_simplex_table::BasicSimplexTable;
use crate::solvers::SimplexSoverAlgorithm;
use super::html_output::HtmlOutput;

// Module with simplex solver related methods of the HtmlOutput

impl HtmlOutput {
    pub fn add_basic_simplex_optimal_solution_to_the_html_document(&mut self, basic_simplex_table: &BasicSimplexTable) {
        unimplemented!("Not implemented yet")
    }

    ///Add header for the section with problem being solved
    pub fn add_simplex_solver_header(&mut self, algorithm_type: SimplexSoverAlgorithm) {
        self.body.push_str("<h2>Problem solution</h2>\n");
        self.body.push_str(format!("<h3>Using: {}</h3>\n", algorithm_type).as_str());
    }

    pub fn add_pivot_information_to_the_html_document(&mut self, basic_simplex_table: &BasicSimplexTable, t_vec: &Vec<Rational>,
                                                      pessimal_row: (usize, &Rational), pivot: (usize, usize)) {
        unimplemented!("Not implemented yet")
    }
}