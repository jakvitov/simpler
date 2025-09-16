use std::fmt::Display;
use chrono::Utc;
use crate::parsers::mps::{Constraints, MpsModel};
use crate::rationals::Rational;
use crate::solvers::basic_simplex_table::BasicSimplexTable;

const VERSION: &str = env!("CARGO_PKG_VERSION");

///Html document to contain math output from the parser and solver
pub struct HtmlOutput {
    body: String,
    head: String,
    styles: String
}

impl Display for HtmlOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<!DOCTYPE html>\n<html>\n<head>\n<style>\n{}\n</style>\n{}</head>\n<body>\n{}\n</body></html>", self.styles, self.head, self.body)
    }
}

impl HtmlOutput {

    fn empty() -> Self {
        HtmlOutput{
            head: String::new(),
            body: String::new(),
            styles: String::new()
        }
    }

    pub fn with_application_header() -> Self {
        let mut res = HtmlOutput{
            head: String::from("<meta charset=\"UTF-8\"><title>Simpler output</title>\n"),
            body: format!("<h1>Simpler output</h1><p>Version: {}, Generated {}</p>", VERSION, Utc::now()),
            styles: String::new()
        };
        res.add_styles();
        res
    }

    fn add_styles(&mut self) {
        self.styles.push_str("table, th, td {
  border: 1px solid black;
  border-collapse: collapse;
}")
    }

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
                self.body.push_str("<mo>×</mo>");
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

    fn create_table_from_simplex_table(&mut self, basic_simplex_table: &BasicSimplexTable) {
        self.body.push_str("<table>\n");

        //Add the row names
        self.body.push_str("<tr>");
        self.body.push_str("<th>Base</th>");
        for variable_name in basic_simplex_table.column_variable_names.keys() {
            self.body.push_str(format!("<th>{}</th>", variable_name).as_str());
        }
        self.body.push_str("<th>RHS</th>");
        self.body.push_str("</tr>");

        //Add the base variable and row and rhs value, we add by rows, so base variable needs to be the first element in it
        let mut base_variable_names_iterator = basic_simplex_table.base_variable_names.iter();
        let mut rhs_values_iterator = basic_simplex_table.rhs.iter();

        //Unwraps are safe, RHS and base having all items necessary should be checked in simplex table construction
        for row_values in &basic_simplex_table.rows {
            self.body.push_str("<tr>");
            //Fill in the base variable name as the first value
            let base_variable_name = base_variable_names_iterator.next().unwrap();
            self.body.push_str(format!("<td>{}</td>", base_variable_name).as_str());

            for row_value in row_values {
                self.body.push_str(format!("<td>{}</td>", row_value).as_str());
            }
            //Fill in the RHS value
            let rhs_value_for_row = rhs_values_iterator.next().unwrap();
            self.body.push_str(format!("<td>{}</td>", rhs_value_for_row).as_str());
            self.body.push_str("</tr>");
        }

        //Fill in the objective function row
        self.body.push_str("<tr>\n");
        self.body.push_str("<td>objective</td>");
        for obj_row_value in &basic_simplex_table.objective_row {
            self.body.push_str(format!("<td>{}</td>", obj_row_value).as_str());
        }
        self.body.push_str(format!("<td>{}</td>", basic_simplex_table.objective_rhs).as_str());
        self.body.push_str("<tr>\n");

        self.body.push_str("</table>\n")
    }

    pub fn add_parsed_basic_simplex_table(&mut self, basic_simplex_table: &BasicSimplexTable) {
        self.body.push_str("<div class=\"parsed_basic_simplex_table\">\n");
        self.body.push_str("<h2>Parsed Simplex table</h2>\n");
        self.body.push_str("<p>Simplex table parser uses irrelevant bound optimisation.</p>\n");
        self.create_table_from_simplex_table(basic_simplex_table);
        self.body.push_str("</div>\n");
    }
}
