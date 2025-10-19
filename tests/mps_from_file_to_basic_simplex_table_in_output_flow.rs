use std::fs;
use simpler::document::html_output::HtmlOutput;
use simpler::parsers::mps::{CroppedMpsModel, MpsModelWithSelectedVariants};
use simpler::parsers::parse_mps;
use simpler::solvers::basic_simplex_table_data::{BasicSimplexTable, OptimizationType};
use simpler::utils::tests::CorrectMps::CORRECT;

/// Test scope: read mps from a file, parse it to parsed mps, parse that mps to basic simplex table
/// with irrelevant bounds removal

const WRITE_OUTPUTS_TO_FILE: bool = false;
#[test]
fn parse_simple_correct_mps_to_basic_simplex_table_and_output_all_to_html() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_mps", CORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    let mps_with_selection = MpsModelWithSelectedVariants::new(parsed_mps, None, None, None, OptimizationType::MIN);

    mps_with_selection.verify_mps_model().unwrap();
    let mut cropped_model = CroppedMpsModel::from(mps_with_selection);
    cropped_model.optimise_bounds().unwrap();
    cropped_model.convert_initially_unfeasible_rhs_constraints_and_bounds().unwrap();

    let basic_simplex_table = BasicSimplexTable::try_from(&cropped_model).unwrap();
    html_output.add_parsed_mps(&cropped_model.model);
    html_output.add_parsed_basic_simplex_table(&basic_simplex_table);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("parse_simple_correct_mps_to_basic_simplex_table_and_output_all_to_html.html", html_output.to_string()).expect("Writing to html_output failed");
    }
}

/// Verify: No errors, mps is shown with 2 rhs, 2 objective rows, 2 bounds. Simplex table has x1, x2, s1-s5, a1-a3, base s1,a1,a2,s3,a3,s5
#[test]
fn parse_complicated_mps_with_multiple_rhs_objectives_and_bounds_to_basic_simplex_table_output_all_to_html() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("complicated_mps_with_multiple_rhs_objectives_and_bounds", CORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    let mps_with_selection = MpsModelWithSelectedVariants::new(parsed_mps, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ2".to_owned()), OptimizationType::MIN);

    mps_with_selection.verify_mps_model().unwrap();
    let mut cropped_model = CroppedMpsModel::from(mps_with_selection);
    cropped_model.optimise_bounds().unwrap();
    cropped_model.convert_initially_unfeasible_rhs_constraints_and_bounds().unwrap();

    let basic_simplex_table = BasicSimplexTable::try_from(&cropped_model).unwrap();
    html_output.add_parsed_mps(&cropped_model.model);
    html_output.add_parsed_basic_simplex_table(&basic_simplex_table);
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("parse_complicated_mps_with_multiple_rhs_objectives_and_bounds_to_basic_simplex_table_output_all_to_html.html", html_output.to_string()).expect("Writing to html_output failed");
    }
}