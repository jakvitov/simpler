use std::fs;
use simpler::document::html_output::HtmlOutput;
use simpler::parsers::mps::MpsModelWithSelectedVariants;
use simpler::parsers::parse_mps;
use simpler::solvers::basic_simplex_table::BasicSimplexTable;

/// Test scope: read mps from a file, parse it to parsed mps, parse that mps to basic simplex table
/// with irrelevant bounds removal

const WRITE_OUTPUTS_TO_FILE: bool = false;
#[test]
fn parse_simple_correct_mps_to_basic_simplex_table_and_output_all_to_html() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_correct_mps")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    let mps_with_selection = MpsModelWithSelectedVariants::new(parsed_mps, None, None, None);
    let basic_simplex_table = BasicSimplexTable::try_from(&mps_with_selection).unwrap();
    html_output.add_parsed_mps(&mps_with_selection.model);
    html_output.add_parsed_basic_simplex_table(&basic_simplex_table);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("parse_simple_correct_mps_to_basic_simplex_table_and_output_all_to_html.html", html_output.to_string()).expect("Writing to html_output failed");
    }
}

#[test]
fn parse_complicated_mps_with_multiple_rhs_objectives_and_bounds_to_basic_simplex_table_output_all_to_html() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("complicated_mps_with_multiple_rhs_objectives_and_bounds")).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    let mps_with_selection = MpsModelWithSelectedVariants::new(parsed_mps, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ2".to_owned()));
    let basic_simplex_table = BasicSimplexTable::try_from(&mps_with_selection).unwrap();
    html_output.add_parsed_mps(&mps_with_selection.model);
    html_output.add_parsed_basic_simplex_table(&basic_simplex_table);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("parse_complicated_mps_with_multiple_rhs_objectives_and_bounds_to_basic_simplex_table_output_all_to_html.html", html_output.to_string()).expect("Writing to html_output failed");
    }
}