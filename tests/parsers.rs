// Integration tests for the parsers module
use std::fs;
use std::path::PathBuf;
use simple_logger::SimpleLogger;
use simpler::parsers::parse_mps;

#[test]
fn parsing_simple_correct_mps_succeeds() {
    let file_name = "simple_correct_mps";
    SimpleLogger::new().init().unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/mps");
    path.push(file_name);

    let mps_file = fs::read_to_string(&path).unwrap();
    let parsed_mps = parse_mps(&mps_file).unwrap();

}