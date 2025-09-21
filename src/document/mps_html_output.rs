use crate::document::html_output::HtmlOutput;
use crate::parsers::mps::{Constraints, MpsModel};
use crate::rationals::Rational;

// Module with mps related methods of the HtmlOutput

impl HtmlOutput {
    pub fn add_parsed_mps(&mut self, mps_model: &MpsModel) {
        self.body.push_str("<div class=\"parsed_mps_model\">\n");
        self.body.push_str(format!("<h2>MPS model {}</h2>\n", mps_model.name).as_str());
        self.body.push_str(format!("<p>Rhs order: {}</p>\n", mps_model.rhs.rhs.keys().map(|x| format!("|{x}|")).collect::<Vec<String>>().join("")).as_str());
        self.body.push_str("<h3>Rows:</h3>\n");
        for (row_name, constraint) in &mps_model.rows.rows {
            self.body.push_str("<p>\n");
            self.body.push_str(format!("<b>{}:</b>\n", &row_name).as_str());
            self.body.push_str("<math display=\"block\">\n");
            self.body.push_str("<mrow>\n");
            for (variable_name, variable_values) in &mps_model.columns.variables {
                let variable_value = variable_values.get(row_name).map_or(Rational::zero(), |x| x.to_owned());
                self.body.push_str(variable_value.to_mmdn_with_sign().as_str());
                self.body.push_str("<mo>⋅</mo>");
                self.body.push_str(format!("<mi>{}</mi>", variable_name).as_str());
            }
            self.body.push_str(format!("<mo>{}</mo>", constraint.to_sign()).as_str());
            for rhs_values_for_rows in mps_model.rhs.rhs.values() {
                if let Some(rhs_value) =  rhs_values_for_rows.get(row_name) {
                    self.body.push_str("<mo>|</mo>");
                    self.body.push_str(rhs_value.to_mmdn_with_sign().as_str());
                    self.body.push_str("<mo>|</mo>");
                }
                else if *constraint == Constraints::N {
                    self.body.push_str("<mo>|</mo>");
                    self.body.push_str("<mi>objective</mi>");
                    self.body.push_str("<mo>|</mo>");
                    //in case of objective row, we break, we do not want to repeat this row for each rhs
                    break;
                }
                else {
                    self.body.push_str("<mo>|</mo>");
                    self.body.push_str("<mi>missing value</mi>");
                    self.body.push_str("<mo>|</mo>");
                }
            }
            self.body.push_str("</mrow>\n");
            self.body.push_str("</math>\n");
            self.body.push_str("</p>\n")
        }
    
        //Adding bounds to the output
        self.body.push_str("<h3>Bounds:</h3>\n");
        for (bound_name, bound_values) in &mps_model.bounds.bounds {
            self.body.push_str("<p>\n");
            self.body.push_str(format!("<b>{}:</b>\n", &bound_name).as_str());
            for (variable_name, value, bound_type) in bound_values {
                self.body.push_str("<p>");
                self.body.push_str("<math display=\"block\">\n");
                self.body.push_str("<mrow>");
    
                self.body.push_str(format!("<mi>{variable_name}</mi>").as_str());
                self.body.push_str(format!("<mo>{}</mo>", bound_type.to_sign()).as_str());
                self.body.push_str(value.to_mmdn_with_sign().as_str());
                self.body.push_str("</mrow>");
                self.body.push_str("</math>\n");
                self.body.push_str("</p>");
            }
            self.body.push_str("</p>\n");
        }
    
        self.body.push_str("</div>\n");
    }

}