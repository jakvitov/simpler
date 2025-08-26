use crate::document::typst_wrapper_world::TypstWrapperWorld;
use chrono::Utc;
use typst_pdf::PdfOptions;
use crate::parsers::ParserError;
use crate::document::pdf_generation_error::PdfGenerationError;
use crate::parsers::mps::{Constraints, MpsModel};
use crate::rationals::Rational;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct TypstDocument {
    data: String
}

impl TypstDocument {

    ///Sets up empty Typst document
    fn empty() -> Self {
        TypstDocument {data: String::new()}
    }

    ///Sets up Typst document with the Simpler output header
    pub fn init() -> Self {
        return TypstDocument {data: format!(r#"
            #set page(header: [
            _Simpler output. Version {}_
            #h(1fr)
            Generated: {}
           ])
        "#,VERSION, Utc::now().to_string())};
    }

    pub fn add_header(&mut self, header: &str) {
        self.data.push_str("\n= ");
        self.data.push_str(header);
        self.data.push('\n');
    }

    pub fn add_sub_sub_header(&mut self, header: &str) {
        self.data.push_str("\n=== ");
        self.data.push_str(header);
        self.data.push('\n');
    }

    pub fn new_line(&mut self) {
        self.data.push_str("\\\n");
    }

    pub fn start_equation(&mut self) {
        self.data.push_str("\n$");
    }

    pub fn end_equation(&mut self) {
        self.data.push_str("$");
    }

    pub fn add_rational(&mut self, rational: &Rational) {
        self.data.push_str(&rational.to_string());
    }

    pub fn add_text(&mut self, text: &str) {
        self.data.push_str(text);
    }

    pub fn add_bold_text(&mut self, text: &str) {
        self.data.push('*');
        self.data.push_str(text);
        self.data.push('*');
    }

    pub fn add_variable_name_to_equation(&mut self, name: &str) {
        self.data.push('"');
        self.data.push_str(name);
        self.data.push('"');
    }

    pub fn add_char(&mut self, c: char) {
        self.data.push(c);
    }

    pub fn add_parser_error(&mut self, err: Box<ParserError>) {
        self.add_header("Errors:");
            self.add_monospaced(err.to_string().as_str())
    }

    pub fn add_monospaced(&mut self, text: &str) {
        self.data.push('\n');
        self.data.push_str("```");
        self.data.push('\n');
        self.data.push_str(text);
        self.data.push_str("```");
    }

    pub fn add_variable_amount_to_equation(&mut self, name: &str, amount: &Rational) {
        self.add_rational(amount);
        self.add_variable_name_to_equation(name)
    }

    /// Adds +3/2x2 to the equation (with the sign)
    pub fn add_plus_variable_amount_to_equation(&mut self, name: &str, amount: &Rational) {
        if amount.is_positive() {
            self.add_char('+');
            self.add_variable_amount_to_equation(name, amount);
        } else {
            // For negative values, we get the - explicitly from rational to_string
            self.add_variable_amount_to_equation(name, amount);
        }
    }

    pub fn export_to_typst_source(self) -> String {
        self.data
    }

    pub fn add_parsed_mps_format(mut self, mps_model: &MpsModel) -> Self {
        self.add_header("Parsed MPS:");
        self.add_text("Model name: ");
        self.add_bold_text(mps_model.name.as_str());
        'outer: for (rhs_name, rhs) in &mps_model.rhs.rhs {
            self.add_sub_sub_header(format!("Model for RHS: {}", rhs_name).as_str());

            for (row_name, constraint) in &mps_model.rows.rows {
                self.new_line();
                self.add_bold_text(row_name.as_str());
                self.add_text(": ");
                self.start_equation();

                for (variable_name, variable_values) in &mps_model.columns.variables {
                    //Safe since we iterate over keys
                    match variable_values.get(row_name) {
                        Some(variable_value_for_row) => self.add_plus_variable_amount_to_equation(variable_name, variable_value_for_row),
                        None => self.add_plus_variable_amount_to_equation(variable_name, &Rational::zero()),
                    }
                }
                self.add_char(constraint.to_sign());
                //Constraint has right side
                if *constraint != Constraints::N {
                    match rhs.get(row_name) {
                        Some(rhs_value) => {
                            self.add_rational(rhs_value);
                            self.end_equation();
                            self.new_line();
                        }
                        None => {
                            self.end_equation();
                            self.add_parser_error(Box::new(ParserError::from_string_message(format!("RHS: {rhs_name} is missing value for non target row {row_name}"), "Each RHS must contain values for all non-target rows.")));
                            break 'outer;
                        }
                    }
                } else {
                    self.end_equation();
                }
            }

        }
        self
    }

    /// Generate Pdf from given document
    pub fn export_to_pdf(self) -> Result<Vec<u8>, Box<PdfGenerationError>> {
        let world = TypstWrapperWorld::new("".to_owned(), self.data);
        let document_compilation_result =  typst::compile(&world).output;
        let document = match document_compilation_result {
            Ok(doc) => doc,
            Err(error) => {
                let problems = error.iter().map(|source_diagnostic| source_diagnostic.message.as_str()).collect::<Vec<&str>>().join("\n");
                return Err(Box::new(PdfGenerationError::from_string_problem("Error while compiling output to PDF.", problems)));
            }
        };

        let res = typst_pdf::pdf(&document, &PdfOptions::default());
        match res {
            Ok(pdf) => Ok(pdf),
            Err(error) => {
                let problems = error.iter().map(|source_diagnostic| source_diagnostic.message.as_str()).collect::<Vec<&str>>().join("\n");
                Err(Box::new(PdfGenerationError::from_string_problem("Error while converting output to PDF.", problems)))
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::document::typst_builder::TypstDocument;
    use crate::parsers::ParserError;
    use crate::rationals::Rational;

    #[test]
    fn typst_document_builder_init_writes_intro() {
        let doc = TypstDocument::init();
        let typst_source = doc.export_to_typst_source();
        assert!(typst_source.contains("#set page(header: [
            _Simpler output. Version "));
        assert!(typst_source.contains("Generated: "));
    }
    
    #[test]
    fn typst_document_builder_add_header_succeeds() {
        let mut doc = TypstDocument::empty();
        doc.add_header("MyHeader");
        let res = doc.export_to_typst_source();
        assert_eq!(res, "\n= MyHeader\n");
    }

    #[test]
    fn typst_document_builder_add_equation_succeeds() {
        let mut doc = TypstDocument::empty();
        doc.start_equation();
        doc.end_equation();
        let res = doc.export_to_typst_source();
        assert_eq!(res, "\n$$");
    }

    #[test]
    fn typst_document_builder_add_rational_succeeds() {
        let number = &Rational::new(1, 2);
        let mut doc = TypstDocument::empty();
        doc.add_rational(number);
        let res = doc.export_to_typst_source();
        assert_eq!("1/2", res);
    }

    #[test]
    fn typst_document_builder_add_variable_amount_succeeds() {
        let mut doc = TypstDocument::empty();
        doc.start_equation();
        doc.add_variable_amount_to_equation("my_var", &Rational::new(1, 2));
        doc.end_equation();
        let res = doc.export_to_typst_source();
        assert_eq!(res, "\n$1/2\"my_var\"$");
    }

    #[test]
    fn typst_document_builder_add_monospaced_succeeds() {
        let mut doc = TypstDocument::empty();
        doc.add_monospaced("my_var");
        let res = doc.export_to_typst_source();
        assert_eq!(res, "\n```\nmy_var```");
    }

    #[test]
    fn typst_document_builder_add_parser_error_succeeds() {
        let mut doc = TypstDocument::empty();
        doc.add_parser_error(Box::new(ParserError::new("Message", "Structure")));
        let res = doc.export_to_typst_source();
        assert!(res.starts_with("\n= Errors:\n\n```\n"));
        assert!(res.ends_with("```"));
    }

}