use crate::document::html_output::HtmlOutput;
use crate::rationals::{Rational, RationalMatrix};
use crate::solvers::basic_simplex_table_data::BasicSimplexTable;

impl HtmlOutput {

    pub fn rev_simpl_output_input_matrices_and_base(&mut self, base: &RationalMatrix, inv_base: &RationalMatrix, base_variables: &Vec<String>, c_b: &RationalMatrix, c_nb: &RationalMatrix, N: &RationalMatrix, iteration: u8) {
        self.body.push_str(format!("<h3>Used input matrices for iteration {iteration}</h3>\n").as_str());
        self.start_aligned_matrix_container();
        self.body.push_str(&HtmlOutput::vector_as_html_string(base_variables, Some(&HtmlOutput::create_mmdn_matrix_name("X", Some("B"), None))));
        self.body.push_str(&HtmlOutput::matrix_as_html_string(base, Some(&HtmlOutput::create_mmdn_matrix_name("B", Some(iteration.to_string().as_str()), None))));
        self.body.push_str(&HtmlOutput::matrix_as_html_string(inv_base, Some(&HtmlOutput::create_mmdn_matrix_name("B", Some(iteration.to_string().as_str()), Some("-1")))));
        self.body.push_str(&HtmlOutput::matrix_as_html_string(c_b, Some(&HtmlOutput::create_mmdn_matrix_name("c", Some("B"), None))));
        self.body.push_str(&HtmlOutput::matrix_as_html_string(c_nb, Some(&HtmlOutput::create_mmdn_matrix_name("c", Some("NB"), None))));
        self.body.push_str(&HtmlOutput::matrix_as_html_string(N, Some(&HtmlOutput::create_mmdn_matrix_name("N", Some(iteration.to_string().as_str()), None))));
        self.end_aligned_matrix_container();
    }

    pub fn rev_simpl_output_reduced_cost_computation(&mut self, c_b: &RationalMatrix, c_nb: &RationalMatrix, basis_inverse: &RationalMatrix, pi: &RationalMatrix, N: &RationalMatrix, red_costs: &RationalMatrix, iteration: u8) {
        self.body.push_str("<h3>Reduced cost computation</h3>\n");

        //PI computation
        self.start_aligned_matrix_container();
        self.body.push_str(format!("<math><mi>π</mi><mo>=</mo><msubsup><mo>c</mo><mn>B</mn><mn>T</mn></msubsup><msubsup><mo>B</mo><mn>{iteration}</mn><mn>-1</mn></msubsup><mo>=</mo>\n").as_str());
        self.body.push_str(&HtmlOutput::matrix_as_html_string(&c_b.transpose(), None));
        self.body.push_str(&HtmlOutput::matrix_as_html_string(basis_inverse, None));
        self.body.push_str("<math><mo>=</mo></math>\n");
        self.body.push_str(&HtmlOutput::matrix_as_html_string(pi, None));
        self.end_aligned_matrix_container();

        //Red costs computation
        self.start_aligned_matrix_container();
        self.body.push_str("<math><mi>red_costs</mi><mo>=</mo><msubsup><mo>c</mo><mn>NB</mn><mn></mn></msubsup><mo>-</mo><mi>π</mi><mi>N</mi><mo>=</mo></math>\n");
        self.body.push_str(&HtmlOutput::matrix_as_html_string(&c_nb, None));
        self.body.push_str("<math><mo>-</mo></math>\n");
        self.body.push_str(&HtmlOutput::matrix_as_html_string(&pi, None));
        self.body.push_str(&HtmlOutput::matrix_as_html_string(&N, None));
        self.body.push_str("<math><mo>=</mo></math>\n");
        self.body.push_str(&HtmlOutput::matrix_as_html_string(&red_costs, None));
        self.end_aligned_matrix_container();
    }

    pub fn rev_simpl_output_rhs_computation(&mut self, basis_inverse: &RationalMatrix, b: &RationalMatrix, rhs: &RationalMatrix, iteration:u8) {
        self.body.push_str("<h3>Current RHS computation</h3>\n");
        //Rhs computation
        self.start_aligned_matrix_container();
        self.body.push_str(format!("<math><mi>RHS</mi><mo>=</mo><msubsup><mo>B</mo><mn>{iteration}</mn><mn>-1</mn></msubsup><mi></mi>b<mo>=</mo>").as_str());
        self.body.push_str(&HtmlOutput::matrix_as_html_string(basis_inverse, None));
        self.body.push_str(&HtmlOutput::matrix_as_html_string(b, None));
        self.body.push_str("<math><mo>=</mo></math>\n");
        self.body.push_str(&HtmlOutput::matrix_as_html_string(rhs, None));
        self.end_aligned_matrix_container();
    }

    pub fn rev_simpl_output_optimal_solution(&mut self, rhs: &RationalMatrix, basis_variables: &Vec<String>, optimal_value: &Rational) {
        self.body.push_str("<hr>");
        self.body.push_str("<div class=\"optimal-solution\">");
        self.body.push_str("<h3>Solution reached</h3>\n");
        self.body.push_str("<h4>Optimal values:</h4>\n");
        self.body.push_str("<ul>\n");
        self.body.push_str(format!("<li>Optimal function value: <b>{}</b></li>\n", optimal_value).as_str());
        for (index, value) in basis_variables.iter().enumerate() {
            self.body.push_str(format!("<li>Variable {} optimal value {}</li>\n", value, rhs.get(index,0)).as_str());
        }
        self.body.push_str("</ul>\n");
        self.body.push_str("</div>\n");
    }
}

#[cfg(test)]
mod tests {

    /*#[test]
    fn output_basis_matrices_and_base() {
        let mut html_output = HtmlOutput::with_application_header();

        let base = RationalMatrix::from_value(3,3, Rational::new(2,5));
        let inv_base = RationalMatrix::from_value(3,3, Rational::new(-2,5));
        let base_vec = vec!["x1".to_string(), "x2".to_string(), "x3".to_string()];

        html_output.output_basis_matrices_and_base(&base, &inv_base, &base_vec, 2);
        let res = html_output.to_string();
        println!("{}", res);
    }*/

}