// Integration tests for the parsers module
use std::fs;
use std::path::PathBuf;
use simple_logger::SimpleLogger;
use simpler::parsers::parse_mps;
use simpler::rationals::Rational;
use simpler::parsers::mps::{ BoundType, Constraints};

#[test]
fn parsing_simple_correct_mps_succeeds() {
    let file_name = "simple_correct_mps";
    SimpleLogger::new().init().unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/mps");
    path.push(file_name);

    let mps_file = fs::read_to_string(&path).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    //Name
    assert_eq!(parsed_mps.name, "testprob");
    //Rows
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("cost").unwrap(), Constraints::N);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("lim1").unwrap(), Constraints::L);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("lim2").unwrap(), Constraints::G);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("myeqn").unwrap(), Constraints::E);
    //Columns
    let column_variables = parsed_mps.columns.get_variables_clone();
    assert_eq!(column_variables.len(), 3);
    let xone = column_variables.get("xone").unwrap();
    assert_eq!(*xone.get("cost").unwrap(), Rational::new(1, 3));
    assert_eq!(*xone.get("lim1").unwrap(), Rational::new(5, 9));
    assert_eq!(*xone.get("lim2").unwrap(), Rational::new(-1, 8));
    let ytwo = column_variables.get("ytwo").unwrap();
    assert_eq!(*ytwo.get("cost").unwrap(), Rational::new(4, 1));
    assert_eq!(*ytwo.get("lim1").unwrap(), Rational::new(1, 1));
    assert_eq!(*ytwo.get("myeqn").unwrap(), Rational::new(-1, 1));
    let zthree = column_variables.get("zthree").unwrap();
    assert_eq!(*zthree.get("cost").unwrap(), Rational::new(9, 1));
    assert_eq!(*zthree.get("lim2").unwrap(), Rational::new(1, 1));
    assert_eq!(*zthree.get("myeqn").unwrap(), Rational::new(1, 1));
    //RHS
    let rhs = parsed_mps.rhs.get_rhs_clone();
    assert_eq!(rhs.len(), 1);
    let rhs1 = rhs.get("rhs1").unwrap();
    assert_eq!(*rhs1.get("lim1").unwrap(), Rational::new(5, 2));
    assert_eq!(*rhs1.get("lim2").unwrap(), Rational::new(10, 2));
    assert_eq!(*rhs1.get("myeqn").unwrap(), Rational::new(7, 2));
    //Bounds
    let bounds = parsed_mps.bounds.get_bounds_clone();
    assert_eq!(bounds.len(), 1);
    let bnd1 = bounds.get("bnd1").unwrap();
    assert_eq!(bnd1[0], (String::from("xone"), Rational::new(4, 1), BoundType::UP));
    assert_eq!(bnd1[1], (String::from("ytwo"), Rational::new(-1, 1), BoundType::LO));
    assert_eq!(bnd1[2], (String::from("ytwo"), Rational::new(1, 1), BoundType::UP));
}

#[test]
fn parsing_simple_correct_mps_with_blank_lines_succeeds() {
    let file_name = "simple_correct_mps_with_blank_lines";
    SimpleLogger::new().init().unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/mps");
    path.push(file_name);

    let mps_file = fs::read_to_string(&path).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();
    //Name
    assert_eq!(parsed_mps.name, "testprob");
    //Rows
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("cost").unwrap(), Constraints::N);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("lim1").unwrap(), Constraints::L);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("lim2").unwrap(), Constraints::G);
    assert_eq!(*parsed_mps.rows.get_constraint_by_row_name("myeqn").unwrap(), Constraints::E);
    //Columns
    let column_variables = parsed_mps.columns.get_variables_clone();
    assert_eq!(column_variables.len(), 3);
    let xone = column_variables.get("xone").unwrap();
    assert_eq!(*xone.get("cost").unwrap(), Rational::new(1, 3));
    assert_eq!(*xone.get("lim1").unwrap(), Rational::new(5, 9));
    assert_eq!(*xone.get("lim2").unwrap(), Rational::new(-1, 8));
    let ytwo = column_variables.get("ytwo").unwrap();
    assert_eq!(*ytwo.get("cost").unwrap(), Rational::new(4, 1));
    assert_eq!(*ytwo.get("lim1").unwrap(), Rational::new(1, 1));
    assert_eq!(*ytwo.get("myeqn").unwrap(), Rational::new(-1, 1));
    let zthree = column_variables.get("zthree").unwrap();
    assert_eq!(*zthree.get("cost").unwrap(), Rational::new(9, 1));
    assert_eq!(*zthree.get("lim2").unwrap(), Rational::new(1, 1));
    assert_eq!(*zthree.get("myeqn").unwrap(), Rational::new(1, 1));
    //RHS
    let rhs = parsed_mps.rhs.get_rhs_clone();
    assert_eq!(rhs.len(), 1);
    let rhs1 = rhs.get("rhs1").unwrap();
    assert_eq!(*rhs1.get("lim1").unwrap(), Rational::new(5, 2));
    assert_eq!(*rhs1.get("lim2").unwrap(), Rational::new(10, 2));
    assert_eq!(*rhs1.get("myeqn").unwrap(), Rational::new(7, 2));
    //Bounds
    let bounds = parsed_mps.bounds.get_bounds_clone();
    assert_eq!(bounds.len(), 1);
    let bnd1 = bounds.get("bnd1").unwrap();
    assert_eq!(bnd1[0], (String::from("xone"), Rational::new(4, 1), BoundType::UP));
    assert_eq!(bnd1[1], (String::from("ytwo"), Rational::new(-1, 1), BoundType::LO));
    assert_eq!(bnd1[2], (String::from("ytwo"), Rational::new(1, 1), BoundType::UP));
}