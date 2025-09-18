use simpler::document::html_output::HtmlOutput;
use simpler::parsers::parse_mps;
use std::fs;
use simpler::utils::tests::CorrectMps::{CORRECT, INCORRECT};

///Test scope: read mps from file, parse it to MpsModel and add the parsed mps model to html output, export output to html

const WRITE_OUTPUTS_TO_FILE: bool = false;

#[test]
fn generate_html_output_from_simple_correct_mps_parsing_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_mps", CORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    html_output.add_parsed_mps(&parsed_mps);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("generate_html_output_from_simple_correct_mps_parsing_succeeds.html", html_output.to_string()).expect("Writing to html_output failed");
    }
}

#[test]
fn generate_html_output_from_simple_correct_mps_parsing_with_two_rhs_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_mps_with_two_rhs", CORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut document = HtmlOutput::with_application_header();
    document.add_parsed_mps(&parsed_mps);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("generate_html_output_from_simple_correct_mps_parsing_with_two_rhs_succeeds.html", document.to_string()).expect("Writing to html_output failed");
    }
}

/// Missing RHS row should be written in output as missing, but the method must succeed
#[test]
fn generate_html_output_from_simple_incorrect_mps_parsing_missing_row_in_rhs_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_mps_missing_row_in_rhs", INCORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut document = HtmlOutput::with_application_header();
    document.add_parsed_mps(&parsed_mps);
    if WRITE_OUTPUTS_TO_FILE {
            fs::write("generate_html_output_from_simple_incorrect_mps_parsing_missing_row_in_rhs_succeeds.html", document.to_string()).expect("Writing to html_output failed");
    }
}

/// Undefined variable in bounds should be included in MPS user output
/// Simplex parser fails on this, but not MPS
#[test]
fn generate_html_output_from_simple_incorrect_mps_parsing_with_undefined_variable_in_bounds_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_mps_with_undefined_variable_in_bounds", INCORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut document = HtmlOutput::with_application_header();
    document.add_parsed_mps(&parsed_mps);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("generate_html_output_from_simple_incorrect_mps_parsing_with_undefined_variable_in_bounds_succeeds.html", document.to_string()).expect("Writing to html_output failed");
    }
}

/// Generate html output from MPS missing columns
/// Verify: Error on output
#[test]
fn generate_html_output_from_simple_incorrect_mps_without_columns() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_mps_without_columns", INCORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).err().unwrap();
    let mut document = HtmlOutput::with_application_header();
    document.add_html_convertible_error(parsed_mps);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("generate_html_output_from_simple_incorrect_mps_without_columns.html", document.to_string()).expect("Writing to html_output failed");
    }
}

// todo add test for incorrect number in mps file (like b/5)