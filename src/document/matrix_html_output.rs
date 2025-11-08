use crate::document::html_output::HtmlOutput;
use crate::rationals::RationalMatrix;

impl HtmlOutput {

    pub(super) fn start_aligned_matrix_container(&mut self) {
        self.body.push_str("<div class=\"matrix-aligned-container\">\n");
    }

    pub(super) fn end_aligned_matrix_container(&mut self) {
        self.body.push_str("</div>\n");
    }

    pub(super) fn matrix_as_html_string(a: &RationalMatrix, input_name: Option<&String>) -> String {
        let mut res = String::from("<math><mrow>\n");
        if let Some(name) = input_name {
            res.push_str(format!("{name}<mo>=</mo>").as_str());
        }
        res.push_str("<mo>[</mo><mtable>\n");

        for j in 0..a.dim().1 {
            res.push_str("<mtr>\n");
            for i in 0..a.dim().0 {
                res.push_str(format!("<mtd>{}</mtd>", a.get(i,j).to_mmdn_with_sign_for_neg()).as_str());
            }
            res.push_str("</mtr>\n");
        }

        res.push_str("</mtable><mo>]</mo>\n");
        res.push_str("</mrow></math>\n");
        res
    }

    pub(super) fn vector_as_html_string(a: &Vec<String>, input_name: Option<&String>) -> String {
        let mut res = String::from("<math><mrow>\n");
        if let Some(name) = input_name {
            res.push_str(format!("{name}<mo>=</mo>").as_str());
        }

        res.push_str("<mo>[</mo><mtable>\n");
        for i in a {
            res.push_str(format!("<mtr>\n<mtd>{}</mtd></mtr>\n", i).as_str());
        }

        res.push_str("</mtable><mo>]</mo>\n");
        res.push_str("</mrow></math>\n");
        res
    }

    pub(super) fn create_mmdn_matrix_name(name: &str, sub: Option<&str>, sup: Option<&str>) -> String {
        match (sub, sup) {
            (Some(sub), Some(sup)) => {
                format!("<msubsup><mtext>{name}</mtext><mtext>{sub}</mtext><mtext>{sup}</mtext></msubsup>")
            },
            (Some(sub), None) => {
                format!("<msubsup><mtext>{name}</mtext><mtext>{sub}</mtext></msubsup>")
            },
            (None, Some(sup)) => {
                format!("<msubsup><mtext>{name}</mtext><mtext>{sup}</mtext></msubsup>")
            },
            (None, None) => {
                format!("<mi>{name}</mi>")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::document::html_output::HtmlOutput;
    use crate::rationals::{Rational, RationalMatrix};

    //#[test]
    fn create_html_matrix() {
        let matrix = RationalMatrix::from_value(3,2, Rational::new(2,5));
        let res = HtmlOutput::matrix_as_html_string(&matrix, Some(&"<mi>A</mi>".to_owned()));
        println!("{res}");
    }

    //#[test]
    fn create_html_vector() {
        let vec = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let res = HtmlOutput::vector_as_html_string(&vec, Some(&"<mi>A</mi>".to_owned()));
        println!("{res}");
    }
}