use simpler::document::html_output::HtmlOutput;
use simpler::parsers::parse_mps;
use std::fs;
use simpler::parsers::mps::MpsModelWithSelectedVariants;
use simpler::solvers::basic_simplex_table::BasicSimplexTable;

/// Module with integration tests
/// Integration of MPS parser with HTML document builder


const WRITE_OUTPUTS_TO_FILE: bool = false;

#[test]
fn generate_html_output_from_simple_correct_mps_parsing_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_correct_mps")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    html_output.add_parsed_mps(&parsed_mps);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("generate_html_output_from_mps_succeeds.html", html_output.to_string()).expect("Writing to html_output failed");
    }
}

#[test]
fn generate_html_output_from_simple_correct_mps_parsing_with_two_rhs_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_correct_mps_with_two_rhs")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut document = HtmlOutput::with_application_header();
    document.add_parsed_mps(&parsed_mps);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("generate_html_output_from_simple_correct_mps_with_two_rhs_succeeds.html", document.to_string()).expect("Writing to html_output failed");
    }
}

/// Missing RHS row should be written in output as missing, but the method must succeed
#[test]
fn generate_html_output_from_simple_incorrect_mps_parsing_missing_row_in_rhs_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_incorrect_mps_missing_row_in_rhs")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut document = HtmlOutput::with_application_header();
    document.add_parsed_mps(&parsed_mps);
    if WRITE_OUTPUTS_TO_FILE {
            fs::write("generate_html_output_from_simple_incorrect_mps_missing_row_in_rhs_succeeds.html", document.to_string()).expect("Writing to html_output failed");
    }
}

/// Undefined variable in bounds should be included in MPS user output
/// Simplex parser fails on this, but not MPS
#[test]
fn generate_html_output_from_simple_incorrect_mps_parsing_with_undefined_variable_in_bounds_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_incorrect_mps_with_undefined_variable_in_bounds")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut document = HtmlOutput::with_application_header();
    document.add_parsed_mps(&parsed_mps);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("generate_html_output_from_simple_incorrect_mps_with_undefined_variable_in_bounds_succeeds.html", document.to_string()).expect("Writing to html_output failed");
    }
}

/// Parse MPS and add result of Simplex parsing table to the result
/// Should not contain any errors
#[test]
fn generate_html_output_with_simplex_table_from_simple_correct_mps(){
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_correct_mps")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    let mps_with_selection = MpsModelWithSelectedVariants::new(parsed_mps, None, None, None);
    let basic_simplex_table = BasicSimplexTable::try_from(&mps_with_selection).unwrap();
    html_output.add_parsed_basic_simplex_table(&basic_simplex_table);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("generate_html_output_from_mps_succeeds.html", html_output.to_string()).expect("Writing to html_output failed");
    }
}

#[test]
fn generate_html_output_with_simplex_table_complicated_correct_mps(){
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("complicated_mps_with_multiple_rhs_objectives_and_bounds")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    let mps_with_selection = MpsModelWithSelectedVariants::new(parsed_mps, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ2".to_owned()));
    let basic_simplex_table = BasicSimplexTable::try_from(&mps_with_selection).unwrap();
    html_output.add_parsed_basic_simplex_table(&basic_simplex_table);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("generate_html_output_with_simplex_table_complicated_correct_mps.html", html_output.to_string()).expect("Writing to html_output failed");
    }
}

// Todo add failing tests for mps parsings