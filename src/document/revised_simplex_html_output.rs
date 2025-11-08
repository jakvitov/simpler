use crate::document::html_output::HtmlOutput;
use crate::rationals::RationalMatrix;

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

    pub fn rev_simpl_output_reduced_cost_computation(&mut self, c_b: &RationalMatrix, c_nb: &RationalMatrix, basis_inverse: &RationalMatrix, pi: &RationalMatrix, N: &RationalMatrix, red_costs: &RationalMatrix) {
        self.body.push_str(format!("<h3>Reduced cost computation</h3>\n").as_str());

        //PI computation
        self.start_aligned_matrix_container();
        self.body.push_str("<math><mi>π</mi><mo>=</mo><msubsup><mo>c</mo><mn>B</mn><mn>T</mn></msubsup><msubsup><mo>B</mo><mn>s</mn><mn>-1</mn></msubsup><mo>=</mo>\n");
        self.body.push_str(&HtmlOutput::matrix_as_html_string(&c_b.transpose(), None));
        self.body.push_str(&HtmlOutput::matrix_as_html_string(basis_inverse, None));
        self.body.push_str("<math><mo>=</mo></math>\n");
        self.body.push_str(&HtmlOutput::matrix_as_html_string(pi, None));
        self.end_aligned_matrix_container();

        //Red costs computation
        self.start_aligned_matrix_container();
        self.body.push_str("<math><mi>red_costs</mi><mo>=</mo><msubsup><mo>c</mo><mn>NB</mn><mn></mn></msubsup><mo>-</mo><mi>π</mi><mi>N</mi><mo>=</mo></math>\n");
        self.body.push_str(&HtmlOutput::matrix_as_html_string(&c_nb, None));
        self.body.push_str("<mo>-</mo>\n");
        self.body.push_str(&HtmlOutput::matrix_as_html_string(&pi, None));
        self.body.push_str(&HtmlOutput::matrix_as_html_string(&N, None));
        self.body.push_str("<mo>=</mo>\n");
        self.body.push_str(&HtmlOutput::matrix_as_html_string(&red_costs, None));
        self.end_aligned_matrix_container();
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