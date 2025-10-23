use super::html_output::HtmlOutput;
use crate::rationals::Rational;
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;
use crate::solvers::SimplexSoverAlgorithm;

// Module with simplex solver related methods of the HtmlOutput

impl HtmlOutput {

    //General
    ///Add header for the section with problem being solved
    pub fn add_simplex_solver_header(&mut self, algorithm_type: SimplexSoverAlgorithm) {
        self.body.push_str("<h2>Problem solution</h2>\n");
        self.body.push_str(format!("<h3>Using: {}</h3>\n", algorithm_type).as_str());
    }

    /// Start simplex iteration by adding a header and appropriate div
    pub fn start_simplex_iteration(&mut self, n: usize) {
        self.body.push_str("<div class=\"simplex-iteration\">");
        self.body.push_str(format!("<h3>Simplex iteration {n}</h3>\n").as_str());
    }

    //START - Basic simplex
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

    pub fn add_pivot_information_to_the_html_document(&mut self, basic_simplex_table: &BasicSimplexTable, t_vec: &Vec<Option<Rational>>,
                                                      pivot: &(usize, usize)) {
        self.body.push_str("<hr>");
        self.body.push_str(format!("<p>Optimity check failed for element {} of the objective row.</p>", pivot.1).as_str());
        self.body.push_str("<div class=\"simplex-table-with-t-vec\">");
        self.create_html_table_from_basic_simplex_table_with_row_and_column_marker(basic_simplex_table, pivot.0, pivot.1);
        // The t-vec should stretch by two rows (objective row and marker row)
        self.add_vector_with_header_as_vertical_table_with_given_length(t_vec, basic_simplex_table.rows.len() + 2, "t");
        self.body.push_str("</div>");
        self.body.push_str(format!("Pivot has been chosen as element [{},{}].", pivot.0, pivot.1).as_str())

    }

    pub fn add_base_variable_switch_info_to_html_document(&mut self, basic_simplex_table: &BasicSimplexTable, old_value: &String, new_value: &String) {
        self.body.push_str("<hr>");
        self.body.push_str(format!("<p>At the end of the iteration, base variable is changed. Pivot variable {} enters base instead of {}.</p>", new_value, old_value).as_str());
        self.create_html_table_from_basic_simplex_table(basic_simplex_table);
    }

    /// End simplex iteration section
    pub fn end_simplex_iteration(&mut self) {
        self.body.push_str("</div>\n");
    }

    pub fn add_pivot_row_normalisation_to_html_document(&mut self, basic_simplex_table: &BasicSimplexTable, coefficient: &Rational, pivot: &(usize, usize)) {
        self.body.push_str("<hr>");
        self.body.push_str(format!("<p>Pivot row {} is normalised by {}</p>", pivot.0, coefficient).as_str());
        self.create_html_table_from_basic_simplex_table_with_one_row_marker(basic_simplex_table, pivot.0);
    }

    pub fn add_row_normalisation_by_pivot_row_to_html_document(&mut self, basic_simplex_table: &BasicSimplexTable,
                                                               coefficient: &Rational, pivot: &(usize, usize), target_row: usize) {
        self.body.push_str("<hr>");
        self.body.push_str(format!("<p>Pivot row {} is multiplied by {} and added to target row {}.</p>", pivot.0, coefficient, target_row).as_str());
        self.create_html_table_from_basic_simplex_table_with_row_addition_markers(basic_simplex_table, pivot.0, target_row);
    }

    pub fn add_unbouded_solution_with_t_vec(&mut self, basic_simplex_table: &BasicSimplexTable, t_vec: &Vec<Option<Rational>>) {
        self.body.push_str("<hr>");
        self.body.push_str("<h4>Unbounded solution found</h4>\n");
        self.body.push_str(format!("<p>Unbounded solution found during pivot calculation. All t-vector values are negative or undefined!</p>").as_str());
        self.body.push_str("<div class=\"simplex-table-with-t-vec\">");
        self.create_html_table_from_basic_simplex_table(basic_simplex_table);
        // The t-vec should stretch by one row objective row (table has no markers here)
        self.add_vector_with_header_as_vertical_table_with_given_length(t_vec, basic_simplex_table.rows.len() + 1, "t");
        self.body.push_str("</div>");
    }
    // END Basic simplex

    // START Two-phase simplex
    /// Output for user, that two-phase simplex problem can be solved with basic simplex
    pub fn add_basic_simplex_chosen_instead(&mut self) {
        self.body.push_str("<p>No artificial variables found.<b>Standard form of this LP is initially feasible</b>, therefore proceeding with basic simplex.</p>")
    }

    /// Output initial (not feasible) auxiliary table with auxiliary minimalization objective row
    pub fn add_initial_unfeasible_auxiliary_table(&mut self, basic_simplex_table: &BasicSimplexTable) {
        self.body.push_str("<h3>Initial auxiliary table</h3>\n");
        self.body.push_str("<p>Initially unfeasible. With target to minimise sum of auxiliary variables.</p>\n");
        self.body.push_str("<p>Eliminating auxiliary variables from objective row to create feasible table.</p>\n");
        self.create_html_table_from_basic_simplex_table(basic_simplex_table);
    }

    pub fn add_starting_phase_one_dual_simplex_header(&mut self) {
        self.body.push_str("<h3>Starting phase one with initially feasible table</h3>\n");
    }

    pub fn add_finished_phase_one_dual_simplex_info(&mut self, objective_rhs: Rational) {
        self.body.push_str(format!("<p>Finished Phase I with objective value: {}</p>\n", objective_rhs).as_str());
        if objective_rhs == Rational::zero() {
            self.body.push_str("<p>That means, that initial feasible solution for phase II was found!<p/>")
        } else {
            self.body.push_str("<p>Oops! Since we could not minimise sum of artificials to zero, the problem is not feasible!</p>")
        }
    }

    pub fn add_starting_phase_two_dual_simplex_header(&mut self) {
        self.body.push_str("<h3>Starting phase two with initially feasible table.</h3>\n");
    }

    pub fn add_eliminated_auxiliary_variables_info(&mut self, basic_simplex_table: &BasicSimplexTable) {
        self.body.push_str("<hr>\n");
        self.body.push_str("<h4>Removed auxiliary variables from simplex table.</h4>\n");
        self.body.push_str("<p>Original objective row was added back to simplex table.</p>\n");
        self.create_html_table_from_basic_simplex_table(basic_simplex_table);
    }

    pub fn add_objective_function_negation_info(&mut self, basic_simplex_table: &BasicSimplexTable) {
        self.body.push_str("<hr>\n");
        self.body.push_str("<h4>Minimalization problem conversion<h4>\n");
        self.body.push_str("<p>LP objective is minimalization. We need to transform the objective function by multiplying with -1.</p>");
        self.create_html_table_from_basic_simplex_table_with_one_row_marker(basic_simplex_table, basic_simplex_table.rows.len());
    }
    
    pub fn add_target_value_negation_for_min_simplex(&mut self, coefficient: &Rational) {
        self.body.push_str("<h4>Minimalization problem conversion final transformation</h4>\n");
        self.body.push_str("<p>Since we transformed objective function by -1 for min to max conversion, we need to transform the objective value back!</p>");
        self.body.push_str(format!("<p><strong>Optimal value {} is transformed to {}.</strong></p>", coefficient, coefficient.negate()).as_str());
    }

    pub fn add_found_degenerate_column_cycle(&mut self, basic_simplex_table: &BasicSimplexTable) {
        self.body.push_str("<hr>");
        self.body.push_str("<h4>Cycle detected during </h4>\n");
        self.body.push_str(format!("<p>Base has not changed between !</p>").as_str());
        self.body.push_str("<p>In this case, it is recommended to switch to Bland’s rule. Is has not yet been implemented in Simpler.</p>");
        self.create_html_table_from_basic_simplex_table(basic_simplex_table);
    }


    // END - Two-phase simplex
}