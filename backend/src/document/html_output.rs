use chrono::Utc;
use std::fmt::Display;

const VERSION: &str = env!("CARGO_PKG_VERSION");

///Html document to contain math output from the parser and solver
pub struct HtmlOutput {
    pub(super) body: String,
    head: String,
    styles: String
}

impl Display for HtmlOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<!DOCTYPE html>\n<html>\n<head>\n<style>\n{}\n</style>\n{}</head>\n<body>\n{}\n</body></html>", self.styles, self.head, self.body)
    }
}

impl HtmlOutput {

    pub(super) fn empty() -> Self {
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
}
.simplex-table-with-t-vec {
  display: flex;
  gap: 20px; /* Space between tables */
  flex-wrap: wrap; /* Allow wrapping on smaller screens */
}

.simplex-table-with-t-vec table {
  flex: 0 0 auto; /* Don't grow/shrink, use natural size */
}

.matrix-aligned-container {
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 20px;
            padding: 20px;
            flex-wrap: wrap;
        }
")
    }

    pub fn add_bug_report_panel(&mut self) {
        unimplemented!()
    }

    pub fn add_html_convertible_error(&mut self, error: Box<dyn super::html_convertible_error::HtmlConvertibleError>) {
        self.body.push_str("<div class=\"error\"\n>");
        self.body.push_str(format!("<h2>{} occurred</h2\n>", error.get_error_name()).as_str());
        self.body.push_str(error.to_html_string().as_str());
        self.body.push_str("</div>\n");
    }

}