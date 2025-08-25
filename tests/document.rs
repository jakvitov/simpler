use simpler::document::typst_builder::TypstDocument;
use std::fs;
use std::io::Write;

//Integration tests for Document building module

const WRITE_OUTPUTS_TO_PDF: bool = false;

#[test]
fn generate_simple_pdf_succeeds() {


    let document = TypstDocument::empty();
    let pdf = document.export_to_pdf();
    assert!(pdf.is_ok());
    if WRITE_OUTPUTS_TO_PDF {
        let pdf = pdf.unwrap();
        fs::write("generate_simple_pdf_succeeds_result.pdf", pdf).expect("Writing to pdf failed");
    }
}