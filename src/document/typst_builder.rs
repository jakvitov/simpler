use crate::document::typst_wrapper_world::TypstWrapperWorld;
use chrono::Utc;
use typst_pdf::PdfOptions;
use crate::document::pdf_generation_error::PdfGenerationError;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct TypstDocument {
    data: String
}

impl TypstDocument {
    pub fn empty() -> Self {
        return TypstDocument {data: String::new()};
    }

    pub fn create_header() -> String {
        format!(r#"
            #set page(header: [
            _Simpler output. Version {}_
            #h(1fr)
            Generated: {}
           ])
        "#,VERSION, Utc::now().to_string())
    }

    pub fn export_to_typst_source(&self) -> String {
        Self::create_header() + &self.data
    }

    /// Generate Pdf from given document
    pub fn export_to_pdf(&self) -> Result<Vec<u8>, Box<PdfGenerationError>> {
        let world = TypstWrapperWorld::new("".to_owned(), self.export_to_typst_source());
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


}