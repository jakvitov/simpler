use simpler::document::typst_builder::TypstDocument;
use std::fs;
use std::io::Write;

// !Integration tests for Document building module!

//Turn on to generate result pdfs for visual testing
const WRITE_OUTPUTS_TO_PDF: bool = true;

#[test]
fn generate_simple_pdf_succeeds() {


    let document = TypstDocument::init().add_header("ahoj");
    let pdf = document.export_to_pdf();
    assert!(pdf.is_ok());
    if WRITE_OUTPUTS_TO_PDF {
        let pdf = pdf.unwrap();
        fs::write("generate_simple_pdf_succeeds_result.pdf", pdf).expect("Writing to pdf failed");
    }
}