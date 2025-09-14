use std::fs;
use std::os::unix::raw::nlink_t;
use simpler::document::html_output::HtmlOutput;
use simpler::parsers::parse_mps;

/// Module with integration tests
/// Integration of MPS parser with HTML document builder


const WRITE_OUTPUTS_TO_FILE: bool = true;

#[test]
fn generate_pdf_from_simple_correct_mps_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_correct_mps")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    html_output.add_parsed_mps(&parsed_mps);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("generate_pdf_from_mps_succeeds.html", html_output.to_string()).expect("Writing to pdf failed");
    }
}