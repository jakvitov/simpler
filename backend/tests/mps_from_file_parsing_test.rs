use simpler::parsers::mps::{BoundType, Constraints};
use simpler::parsers::parse_mps;
use simpler::rationals::Rational;
use simpler::utils::tests::CorrectMps::{CORRECT, INCORRECT};
// Integration tests for the parsers module
use std::fs;
use std::path::PathBuf;

/// Test scope: read mps string from a file and parse it to MpsModel
/// Both error and success variants

fn setup_path(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../data/mps");
    path.push(file_name);
    path
}

#[test]
fn parsing_simple_correct_mps_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_mps", CORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    //Name
    assert_eq!(parsed_mps.name, "TESTPROB");
    //Rows
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("COST").unwrap(), Constraints::N);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("LIM1").unwrap(), Constraints::L);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("LIM2").unwrap(), Constraints::G);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("MYEQN").unwrap(), Constraints::E);
    //Columns
    let column_variables = parsed_mps.columns.get_variables_clone();
    assert_eq!(column_variables.len(), 3);
    let xone = column_variables.get("XONE").unwrap();
    assert_eq!(*xone.get("COST").unwrap(), Rational::new(1, 3));
    assert_eq!(*xone.get("LIM1").unwrap(), Rational::new(5, 9));
    assert_eq!(*xone.get("LIM2").unwrap(), Rational::new(-1, 8));
    let ytwo = column_variables.get("YTWO").unwrap();
    assert_eq!(*ytwo.get("COST").unwrap(), Rational::new(4, 1));
    assert_eq!(*ytwo.get("LIM1").unwrap(), Rational::new(1, 1));
    assert_eq!(*ytwo.get("MYEQN").unwrap(), Rational::new(-1, 1));
    let zthree = column_variables.get("ZTHREE").unwrap();
    assert_eq!(*zthree.get("COST").unwrap(), Rational::new(9, 1));
    assert_eq!(*zthree.get("LIM2").unwrap(), Rational::new(1, 1));
    assert_eq!(*zthree.get("MYEQN").unwrap(), Rational::new(1, 1));
    //RHS
    let rhs = parsed_mps.rhs.get_rhs_clone();
    assert_eq!(rhs.len(), 1);
    let rhs1 = rhs.get("RHS1").unwrap();
    assert_eq!(*rhs1.get("LIM1").unwrap(), Rational::new(5, 2));
    assert_eq!(*rhs1.get("LIM2").unwrap(), Rational::new(10, 2));
    assert_eq!(*rhs1.get("MYEQN").unwrap(), Rational::new(7, 2));
    //Bounds
    let bounds = parsed_mps.bounds.get_bounds_clone();
    assert_eq!(bounds.len(), 1);
    let bnd1 = bounds.get("BND1").unwrap();
    assert_eq!(bnd1[0], (String::from("XONE"), Rational::new(4, 1), BoundType::UP));
    assert_eq!(bnd1[1], (String::from("YTWO"), Rational::new(-1, 1), BoundType::LO));
    assert_eq!(bnd1[2], (String::from("YTWO"), Rational::new(1, 1), BoundType::UP));
}

#[test]
fn parsing_simple_correct_mps_with_blank_lines_succeeds() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_mps_with_blank_lines", CORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    //Name
    assert_eq!(parsed_mps.name, "TESTPROB");
    //Rows
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("COST").unwrap(), Constraints::N);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("LIM1").unwrap(), Constraints::L);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("LIM2").unwrap(), Constraints::G);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("MYEQN").unwrap(), Constraints::E);
    //Columns
    let column_variables = parsed_mps.columns.get_variables_clone();
    assert_eq!(column_variables.len(), 3);
    let xone = column_variables.get("XONE").unwrap();
    assert_eq!(*xone.get("COST").unwrap(), Rational::new(1, 3));
    assert_eq!(*xone.get("LIM1").unwrap(), Rational::new(5, 9));
    assert_eq!(*xone.get("LIM2").unwrap(), Rational::new(-1, 8));
    let ytwo = column_variables.get("YTWO").unwrap();
    assert_eq!(*ytwo.get("COST").unwrap(), Rational::new(4, 1));
    assert_eq!(*ytwo.get("LIM1").unwrap(), Rational::new(1, 1));
    assert_eq!(*ytwo.get("MYEQN").unwrap(), Rational::new(-1, 1));
    let zthree = column_variables.get("ZTHREE").unwrap();
    assert_eq!(*zthree.get("COST").unwrap(), Rational::new(9, 1));
    assert_eq!(*zthree.get("LIM2").unwrap(), Rational::new(1, 1));
    assert_eq!(*zthree.get("MYEQN").unwrap(), Rational::new(1, 1));
    //RHS
    let rhs = parsed_mps.rhs.get_rhs_clone();
    assert_eq!(rhs.len(), 1);
    let rhs1 = rhs.get("RHS1").unwrap();
    assert_eq!(*rhs1.get("LIM1").unwrap(), Rational::new(5, 2));
    assert_eq!(*rhs1.get("LIM2").unwrap(), Rational::new(10, 2));
    assert_eq!(*rhs1.get("MYEQN").unwrap(), Rational::new(7, 2));
    //Bounds
    let bounds = parsed_mps.bounds.get_bounds_clone();
    assert_eq!(bounds.len(), 1);
    let bnd1 = bounds.get("BND1").unwrap();
    assert_eq!(bnd1[0], (String::from("XONE"), Rational::new(4, 1), BoundType::UP));
    assert_eq!(bnd1[1], (String::from("YTWO"), Rational::new(-1, 1), BoundType::LO));
    assert_eq!(bnd1[2], (String::from("YTWO"), Rational::new(1, 1), BoundType::UP));
}

#[test]
fn parsing_simple_mps_without_columns_fails() {
    let mps_file = fs::read_to_string(simpler::utils::tests::setup_path_to_mps("simple_mps_without_columns", INCORRECT)).unwrap();
    let parsed_mps = parse_mps(&mps_file);
    assert!(parsed_mps.is_err());
}