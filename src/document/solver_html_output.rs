use super::html_output::HtmlOutput;
use crate::rationals::Rational;
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;
use crate::solvers::SimplexSoverAlgorithm;

// Module with simplex solver related methods of the HtmlOutput

impl HtmlOutput {
    pub fn add_basic_simplex_optimal_solution_to_the_html_document(&mut self, basic_simplex_table: &BasicSimplexTable) {
        self.body.push_str("<hr>");
        self.body.push_str("<div class=\"optimal-solution\">");
        self.body.push_str("<h3>Solution reached</h3>\n");
        self.body.push_str("<p>Final simplex table:</p>\n");
        self.create_html_table_from_basic_simplex_table(basic_simplex_table);

        self.body.push_str("<h4>Optimal values:</h4>\n");
        self.body.push_str("<ul>\n");
        self.body.push_str(format!("<li>Optimal function value: <b>{}</b></li>\n", basic_simplex_table.objective_rhs).as_str());
        for (index, value) in basic_simplex_table.base_variable_names.iter().enumerate() {
            self.body.push_str(format!("<li>Variable {} optimal value {}</li>\n", value, basic_simplex_table.rhs[index]).as_str());
        }
        self.body.push_str("</ul>\n");
        self.body.push_str("</div>\n");
    }

    ///Add header for the section with problem being solved
    pub fn add_simplex_solver_header(&mut self, algorithm_type: SimplexSoverAlgorithm) {
        self.body.push_str("<h2>Problem solution</h2>\n");
        self.body.push_str(format!("<h3>Using: {}</h3>\n", algorithm_type).as_str());
    }

    pub fn add_pivot_information_to_the_html_document(&mut self, basic_simplex_table: &BasicSimplexTable, t_vec: &Vec<Rational>,
                                                      pessimal_column: &(usize, Rational), pivot: &(usize, usize)) {
        self.body.push_str("<hr>");
        self.body.push_str(format!("<p>Optimity check failed for element {} of the objective row.</p>", pivot.1).as_str());
        self.body.push_str("<div class=\"simplex-table-with-t-vec\">");
        self.create_html_table_from_basic_simplex_table_with_row_and_column_marker(basic_simplex_table, pivot.0, pivot.1);
        self.create_table_from_vector(t_vec, "t");
        self.body.push_str("</div>");
        self.body.push_str(format!("Pivot has been chosed as element [{},{}].", pivot.0, pivot.1).as_str())

    }

    /// Start simplex iteration by adding a header and appropriate div
    pub fn start_simplex_iteration(&mut self, n: usize) {
        self.body.push_str("<div class=\"simplex-iteration\">");
        self.body.push_str(format!("<h3>Simplex iteration {n}</h3>\n").as_str());
    }

    /// End simplex iteration section
    pub fn end_simplex_iteration(&mut self) {
        self.body.push_str("</div>\n");
    }

    pub fn add_pivot_row_normalisation_to_html_document(&mut self, basic_simplex_table: &BasicSimplexTable, coefficient: &Rational, pivot: &(usize, usize)) {
        self.body.push_str("<hr>");
        self.body.push_str(format!("<p>Pivot row {} is normalised by {}</p>", pivot.0, coefficient.to_mmdn_with_sign()).as_str());
        self.create_html_table_from_basic_simplex_table_with_one_row_marker(basic_simplex_table, pivot.0);
    }

    pub fn add_row_normalisation_by_pivot_row_to_html_document(&mut self, basic_simplex_table: &BasicSimplexTable,
                                                               coefficient: &Rational, pivot: &(usize, usize), target_row: usize) {
        self.body.push_str("<hr>");
        self.body.push_str(format!("<p>Pivot row {} is multiplied by {} and added to target row {}.</p>", pivot.0, coefficient.to_mmdn_with_sign(), target_row).as_str());
        self.create_html_table_from_basic_simplex_table_with_row_addition_markers(basic_simplex_table, pivot.0, target_row);
    }
}