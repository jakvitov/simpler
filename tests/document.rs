use simpler::document::typst_builder::TypstDocument;
use std::fs;
use std::io::Write;
use simpler::parsers::parse_mps;
// !Integration tests for Document building module!

//Turn on to generate result pdfs for visual testing
const WRITE_OUTPUTS_TO_PDF: bool = false;

#[test]
fn generate_simple_pdf_succeeds() {


    let mut document = TypstDocument::init();
    document.add_header("ahoj");
    let pdf = document.export_to_pdf();
    assert!(pdf.is_ok());
    if WRITE_OUTPUTS_TO_PDF {
        let pdf = pdf.unwrap();
        fs::write("generate_simple_pdf_succeeds_result.pdf", pdf).expect("Writing to pdf failed");
    }
}

#[test]
fn generate_pdf_from_mps_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_correct_mps")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let document = TypstDocument::init().add_parsed_mps_format(&parsed_mps).export_to_pdf();
    assert!(document.is_ok());
    if WRITE_OUTPUTS_TO_PDF {
        let pdf = document.unwrap();
        fs::write("generate_pdf_from_mps_succeeds.pdf", pdf).expect("Writing to pdf failed");
    }

}