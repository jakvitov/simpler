use crate::parsers::mps::{Bound, Column, MpsModel, Row, Sections, Rhs};
use crate::parsers::ParserError;
use chrono::Utc;
use log::info;
use log::LevelFilter;
use regex::Regex;
use simple_logger::SimpleLogger;
use std::error::Error;

struct MpsInParsing {
    name: Option<String>,
    rows: Option<Vec<Row>>,
    columns: Option<Vec<Column>>,
    rhs: Option<Vec<Rhs>>,
    bounds: Option<Vec<Bound>>
}

impl MpsInParsing {
    fn empty() -> Self {
        MpsInParsing { name: None, rows: None, columns: None, rhs: None, bounds: None }
    }
}

fn parse_name(lowercase_input: &str) -> Result<String, Box<ParserError>> {
    let Some(first_line) = lowercase_input.lines().next() else {
        return Err(Box::new(ParserError::from_string_structure("No section NAME found in input", lowercase_input.to_string())));
    };
    if first_line.len() < 5 {
        return Err(Box::new(ParserError::from_string_structure("Incorrect NAME section in MPS", lowercase_input.to_string())));
    }

    if first_line[0..4].to_string() != "name" {
        return Err(Box::new(ParserError::from_string_structure("MPS model does not start with NAME section", lowercase_input.to_string())));
    }

    let res = first_line[4..].trim().to_string();
    if res.is_empty() {
        return Err(Box::new(ParserError::from_string_structure("MPS model name cannot contain only whitespaces", lowercase_input.to_string())));
    }
    Ok(res)
}

fn parse_rows(input: &Vec<&str>)  -> Result<Vec<Row>, Box<ParserError>> {
    Ok(Vec::new())
}

fn parse_columns(input: &Vec<&str>)  -> Result<Vec<Column>, Box<ParserError>> {
    Ok(Vec::new())
}

fn parse_bounds(input: &Vec<&str>)  -> Result<Vec<Bound>, Box<ParserError>> {
    Ok(Vec::new())
}

fn parse_rhs(input: &Vec<&str>)  -> Result<Vec<Rhs>, Box<ParserError>> {
    Ok(Vec::new())
}



fn parse_mps(input: String, logger: &SimpleLogger) -> Result<(), Box<ParserError>> {
    info!("Started parsing MPS input.");
    let start_timestamp = Utc::now();

    if input.is_empty() {
        return Err(Box::new(ParserError::from_string_structure("Input MPS is empty!", input)));
    }

    let lowercase_input = input.to_ascii_lowercase().to_string();
    let lines = lowercase_input.lines();
    let mut state = Sections::NAME;
    let mut buffer = Vec::<&str>::new();
    let mut mps_in_parsing = MpsInParsing::empty();
    while let Some(line) = lines.clone().next() {
        match state {
            Sections::NAME => {
                mps_in_parsing.name = Some(parse_name(line)?);
                state = Sections::ROWS;
            },
            Sections::ROWS => {
                let line_string = line.to_string();
                if (line_string == "COLUMNS") {
                    state = Sections::COLUMNS;
                    let parsed_rows = parse_rows(&buffer)?;
                    mps_in_parsing.rows = Some(parsed_rows);
                    buffer = Vec::new()
                } else {
                    buffer.push(line);
                }
            },
            Sections::COLUMNS => {
                let line_string = line.to_string();
                if (line_string == "RHS") {
                    state = Sections::RHS;
                    let parsed_columns = parse_columns(&buffer)?;
                    mps_in_parsing.columns = Some(parsed_columns);
                    buffer = Vec::new()
                } else {
                    buffer.push(line);
                }
            },
            Sections::RHS => {
                if (line == "BOUNDS") {
                    state = Sections::BOUNDS;
                    let parsed_rhs = parse_rhs(&buffer)?;
                    mps_in_parsing.rhs = Some(parsed_rhs);
                    buffer = Vec::new()
                } else {
                    buffer.push(line);
                }
            },
            Sections::BOUNDS => {
                let line_string = line.to_string();
                if (line_string == "ENDATA") {
                    state = Sections::ENDATA;
                    let parsed_bounds = parse_bounds(&buffer)?;
                    mps_in_parsing.bounds = Some(parsed_bounds);
                    buffer = Vec::new()
                } else {
                    buffer.push(line);
                }
            },
            Sections::ENDATA => {
                break;
            }
        }
    }



    let end_timestamp = Utc::now();
    info!("Finished parsing MPS input in {} milliseconds.", (end_timestamp - start_timestamp).num_milliseconds());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parsers::mps_parser::parse_name;

    #[test]
    fn parse_name_succeeds() {
        let input = "name         my_model\n";
        let parsing_result = parse_name(&input.to_string());
        assert!(parsing_result.is_ok());
        let Ok(res) = parsing_result else {
            panic!("Parsing failed!");
        };
        assert_eq!(res, "my_model");
    }

    #[test]
    fn parse_name_with_empty_input_fails() {
        let input = "";
        let parse_res = parse_name(&input.to_string());
        assert!(parse_res.is_err());
    }

    #[test]
    fn parse_name_shorter_than_necessary_fails() {
        let input = "nam";
        let parse_res = parse_name(&input.to_string());
        assert!(parse_res.is_err());
    }

    #[test]
    fn parse_name_with_only_whitespace_fails() {
        let input = "name     ";
        let parse_res = parse_name(&input.to_string());
        assert!(parse_res.is_err());
    }

    #[test]
    fn parse_name_without_name_section_fails(){
        let input = "namjdjd";
        let parse_res = parse_name(&input.to_string());
        assert!(parse_res.is_err());
    }
}