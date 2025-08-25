use std::fs;
use typst_pdf::PdfOptions;
use crate::document::typst_wrapper_world::TypstWrapperWorld;

struct TypstDocument {
    data: String
}


impl TypstDocument {
    fn new() -> Self {
        return TypstDocument {data: String::new()};
    }

    fn to_string(&self) -> String {
        String::from(r#"
#set page(paper: "a4", margin: 2cm)
#set text(size: 14pt)
= Simpler output
"#)
    }

    fn export_to_pdf(&self) -> () {
        let world = TypstWrapperWorld::new("".to_owned(), self.to_string());
        let document =  typst::compile(&world).output.expect("Compilation of PDF failed.");
        let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
        fs::write("./output.pdf", pdf).expect("Error writing PDF.");
    }

}

#[cfg(test)]
mod tests {
    use crate::document::typst_builder::TypstDocument;

    #[test]
    fn generate_empty_document() {
        TypstDocument::new().export_to_pdf();
    }

}