use std::fmt::Display;
use chrono::Utc;
use crate::parsers::mps::{Constraints, MpsModel};
use crate::rationals::Rational;

const VERSION: &str = env!("CARGO_PKG_VERSION");

///Html document to contain math output from the parser and solver
pub struct HtmlOutput {
    data: String
}

impl Display for HtmlOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl HtmlOutput {

    fn empty() -> Self {
        HtmlOutput{data: String::from("")}
    }

    pub fn with_application_header() -> Self {
        HtmlOutput{data: format!("<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <title>Simpler output</title>
</head>
<body>
<h1>Simpler output</h1>
<p>
    Version: {}, Generated {}
</p>", VERSION, Utc::now())}
    }

    fn add_h1(&mut self, header: String) {
        self.data.push_str("\n<h1>");
        self.data.push_str(&header);
        self.data.push_str("</h1>\n");
    }

    fn add_h2(&mut self, header: String) {
        self.data.push_str("\n<h2>");
        self.data.push_str(&header);
        self.data.push_str("</h2>\n");
    }


    fn start_div(&mut self, class: String) {
        self.data.push_str(format!("\n<div class=\"{class}\">\n").as_str());
    }

    fn end_div(&mut self) {
        self.data.push_str("\n</div>\n");
    }

    fn add_p(&mut self, data: String) {
        self.data.push_str(format!("\n<p>{data}</p>\n").as_str())
    }
    pub fn add_parsed_mps(&mut self, mps_model: &MpsModel) {
        self.data.push_str("<div class=\"parsed_mps_model\">\n");
        self.data.push_str(format!("<h2>MPS model {}</h2>\n", mps_model.name).as_str());
        self.data.push_str(format!("<p>Rhs order: {}</p>\n", mps_model.rhs.rhs.keys().map(|x| format!("|{x}|")).collect::<Vec<String>>().join("")).as_str());
        self.data.push_str("<h3>Rows:</h3>\n");
        for (row_name, constraint) in &mps_model.rows.rows {
            self.data.push_str("<p>\n");
            self.data.push_str(format!("<b>{}:</b>\n", &row_name).as_str());
            self.data.push_str("<math display=\"block\">\n");
            self.data.push_str("<mrow>\n");
            for (variable_name, variable_values) in &mps_model.columns.variables {
                let variable_value = variable_values.get(row_name).map_or(Rational::zero(), |x| x.to_owned());
                self.data.push_str(variable_value.to_mmdn_with_sign().as_str());
                self.data.push_str("<mo>×</mo>");
                self.data.push_str(format!("<mi>{}</mi>", variable_name).as_str());
            }
            self.data.push_str(format!("<mo>{}</mo>", constraint.to_sign()).as_str());
            for rhs_values_for_rows in mps_model.rhs.rhs.values() {
                if let Some(rhs_value) =  rhs_values_for_rows.get(row_name) {
                    self.data.push_str("<mo>|</mo>");
                    self.data.push_str(rhs_value.to_mmdn_with_sign().as_str());
                    self.data.push_str("<mo>|</mo>");
                }
                else if *constraint == Constraints::N {
                    self.data.push_str("<mo>|</mo>");
                    self.data.push_str("<mi>objective</mi>");
                    self.data.push_str("<mo>|</mo>");
                    //in case of objective row, we break, we do not want to repeat this row for each rhs
                    break;
                }
                else {
                    self.data.push_str("<mo>|</mo>");
                    self.data.push_str("<mi>missing value</mi>");
                    self.data.push_str("<mo>|</mo>");
                }
            }
            self.data.push_str("</mrow>\n");
            self.data.push_str("</math>\n");
            self.data.push_str("</p>\n")
        }

        //Adding bounds to the output
        self.data.push_str("<h3>Bounds:</h3>\n");
        for (bound_name, bound_values) in &mps_model.bounds.bounds {
            self.data.push_str("<p>\n");
            self.data.push_str(format!("<b>{}:</b>\n", &bound_name).as_str());
            for (variable_name, value, bound_type) in bound_values {
                self.data.push_str("<p>");
                self.data.push_str("<math display=\"block\">\n");
                self.data.push_str("<mrow>");

                self.data.push_str(format!("<mi>{variable_name}</mi>").as_str());
                self.data.push_str(format!("<mo>{}</mo>", bound_type.to_sign()).as_str());
                self.data.push_str(value.to_mmdn_with_sign().as_str());
                self.data.push_str("</mrow>");
                self.data.push_str("</math>\n");
                self.data.push_str("</p>");
            }
            self.data.push_str("</p>\n");
        }

        self.data.push_str("</div>\n");
    }
}
