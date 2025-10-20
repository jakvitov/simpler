use std::fs;
use simpler::document::html_output::HtmlOutput;
use simpler::parsers::mps::{CroppedMpsModel, MpsModelWithSelectedVariants};
use simpler::parsers::parse_mps;
use simpler::rationals::Rational;
use simpler::solvers;
use simpler::solvers::basic_simplex_table_data::{BasicSimplexTable, OptimizationType};
use simpler::utils::tests::CorrectMps::CORRECT;

const WRITE_OUTPUTS_TO_FILE: bool = false;

#[test]
fn solve_basic_simplex_two_iterations_from_mps_file_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("basic_simplex_two_iterations_mps", CORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    html_output.add_parsed_mps(&parsed_mps);

    let mps_with_selection = MpsModelWithSelectedVariants::new(parsed_mps, None, None, None, OptimizationType::MIN);

    mps_with_selection.verify_mps_model().unwrap();
    let mut cropped_model = CroppedMpsModel::from(mps_with_selection);
    cropped_model.optimise_bounds().unwrap();
    cropped_model.convert_initially_unfeasible_rhs_constraints_and_bounds().unwrap();

    let mut basic_simplex_table = BasicSimplexTable::try_from(&cropped_model).unwrap();
    html_output.add_parsed_basic_simplex_table(&basic_simplex_table);
    let res = solvers::solve_basic_simplex(&mut basic_simplex_table, &mut html_output);

    assert!(res.is_ok());
    let res = res.unwrap();
    assert_eq!(res.unwrap(), Rational::from_integer(10));
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("solve_basic_simplex_two_iterations_from_mps_file_succeeds.html", html_output.to_string()).expect("Writing to html_output failed");
    }
}

#[test]
fn solve_basic_simplex_unbounded_two_iterations_from_mps_file_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("basic_unbounded_simplex_two_iterations_mps", CORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    let mut html_output = HtmlOutput::with_application_header();
    html_output.add_parsed_mps(&parsed_mps);

    let mps_with_selection = MpsModelWithSelectedVariants::new(parsed_mps, None, None, None, OptimizationType::MIN);

    mps_with_selection.verify_mps_model().unwrap();
    let mut cropped_model = CroppedMpsModel::from(mps_with_selection);
    cropped_model.optimise_bounds().unwrap();
    cropped_model.convert_initially_unfeasible_rhs_constraints_and_bounds().unwrap();
    let mut basic_simplex_table = BasicSimplexTable::try_from(&cropped_model).unwrap();

    html_output.add_parsed_basic_simplex_table(&basic_simplex_table);
    let res = solvers::solve_basic_simplex(&mut basic_simplex_table, &mut html_output);

    assert!(res.is_ok());
    let res = res.unwrap();
    if WRITE_OUTPUTS_TO_FILE {
        fs::write("solve_basic_simplex_unbounded_two_iterations_from_mps_file_succeeds.html", html_output.to_string()).expect("Writing to html_output failed");
    }
    assert!(res.is_none());

}