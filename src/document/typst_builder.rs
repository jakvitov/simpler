use crate::document::typst_wrapper_world::TypstWrapperWorld;
use chrono::Utc;
use typst_pdf::PdfOptions;
use crate::parsers::ParserError;
use crate::document::pdf_generation_error::PdfGenerationError;
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

    pub fn add_header(mut self, header: &str) -> Self {
        self.data.push_str("\n= ");
        self.data.push_str(header);
        self.data.push('\n');
        self
    }

    pub fn new_line(mut self) -> Self {
        self.data.push_str("\\\n");
        self
    }

    pub fn start_equation(mut self) -> Self {
        self.data.push_str("\n$");
        self
    }

    pub fn end_equation(mut self) -> Self {
        self.data.push_str("$");
        self
    }

    pub fn add_rational(mut self, rational: Rational) -> Self {
        self.data.push_str(&rational.to_string());
        self
    }

    pub fn add_text(mut self, text: &str) -> Self {
        self.data.push_str(text);
        self
    }

    pub fn add_bold_text(mut self, text: &str) -> Self {
        self.data.push('*');
        self.data.push_str(text);
        self.data.push('*');
        self
    }

    pub fn add_variable_name_to_equation(mut self, name: &str) -> Self {
        self.data.push('"');
        self.data.push_str(name);
        self.data.push('"');
        self
    }

    pub fn add_char(mut self, c: char) -> Self {
        self.data.push(c);
        self
    }

    pub fn add_parser_error(mut self, err: Box<ParserError>) -> Self {
        self.add_header("Errors:").add_monospaced(err.to_string().as_str())
    }

    pub fn add_monospaced(mut self, text: &str) -> Self {
        self.data.push('\n');
        self.data.push_str("```");
        self.data.push('\n');
        self.data.push_str(text);
        self.data.push_str("```");
        self
    }

    pub fn add_variable_amount_to_equation(mut self, name: &str, amount: Rational) -> Self {
        let s = self.add_rational(amount);
        s.add_variable_name_to_equation(name)
    }

    pub fn export_to_typst_source(self) -> String {
        self.data
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
        let doc = TypstDocument::empty().add_header("MyHeader");
        let res = doc.export_to_typst_source();
        assert_eq!(res, "\n= MyHeader\n");
    }

    #[test]
    fn typst_document_builder_add_equation_succeeds() {
        let doc = TypstDocument::empty().start_equation().end_equation().export_to_typst_source();
        assert_eq!(doc, "\n$$");
    }

    #[test]
    fn typst_document_builder_add_rational_succeeds() {
        let number = Rational::new(1, 2);
        let doc = TypstDocument::empty().add_rational(number).export_to_typst_source();
        assert_eq!("1/2", doc);
    }

    #[test]
    fn typst_document_builder_add_variable_amount_succeeds() {
        let doc = TypstDocument::empty().start_equation()
            .add_variable_amount_to_equation("my_var", Rational::new(1, 2)).end_equation().export_to_typst_source();
        assert_eq!(doc, "\n$1/2\"my_var\"$");
    }

    #[test]
    fn typst_document_builder_add_monospaced_succeeds() {
        let doc = TypstDocument::empty().add_monospaced("my_var").export_to_typst_source();
        assert_eq!(doc, "\n```\nmy_var```");
    }

    #[test]
    fn typst_document_builder_add_parser_error_succeeds() {
        let doc = TypstDocument::empty()
            .add_parser_error(Box::new(ParserError::new("Message", "Structure")))
            .export_to_typst_source();
        assert!(doc.starts_with("\n= Errors:\n\n```\n"));
        assert!(doc.ends_with("```"));
    }



}