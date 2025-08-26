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
fn generate_pdf_from_simple_correct_mps_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_correct_mps")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut document = TypstDocument::init();
    let res = document.add_parsed_mps_format(&parsed_mps);
    assert!(res.is_ok());
    let export = document.export_to_pdf();
    assert!(export.is_ok());
    if WRITE_OUTPUTS_TO_PDF {
        let pdf = export.unwrap();
        fs::write("generate_pdf_from_mps_succeeds.pdf", pdf).expect("Writing to pdf failed");
    }
}

#[test]
fn generate_pdf_from_simple_correct_mps_with_two_rhs_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_correct_mps_with_two_rhs")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut document = TypstDocument::init();
    let res = document.add_parsed_mps_format(&parsed_mps);
    assert!(res.is_ok());
    let export = document.export_to_pdf();
    assert!(export.is_ok());
    if WRITE_OUTPUTS_TO_PDF {
        let pdf = export.unwrap();
        fs::write("generate_pdf_from_simple_correct_mps_with_two_rhs_succeeds.pdf", pdf).expect("Writing to pdf failed");
    }
}

#[test]
fn generate_pdf_from_simple_incorrect_mps_missing_row_in_rhs_fails() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_incorrect_mps_missing_row_in_rhs")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut document = TypstDocument::init();
    let res = document.add_parsed_mps_format(&parsed_mps);
    assert!(res.is_err());
}


#[test]
fn generate_pdf_from_simple_incorrect_mps_with_undefined_variable_in_bounds_fails() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_incorrect_mps_with_undefined_variable_in_bounds")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut document = TypstDocument::init();
    let res = document.add_parsed_mps_format(&parsed_mps);
    assert!(res.is_err());
}