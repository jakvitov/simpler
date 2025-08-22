use crate::parsers::mps::{Bound, Column, Constraints, MpsModel, Rhs, Row, Sections};
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

fn parse_name(name_line: &str) -> Result<String, Box<ParserError>> {
    if name_line.len() < 5 {
        return Err(Box::new(ParserError::from_string_structure("Incorrect NAME section in MPS", name_line.to_string())));
    }

    if name_line[0..4].to_string() != "name" {
        return Err(Box::new(ParserError::from_string_structure("MPS model does not start with NAME section", name_line.to_string())));
    }

    let res = name_line[4..].trim().to_string();
    if res.is_empty() {
        return Err(Box::new(ParserError::from_string_structure("MPS model name cannot contain only whitespaces", name_line.to_string())));
    }
    Ok(res)
}

fn parse_rows(input: &Vec<&str>)  -> Result<Vec<Row>, Box<ParserError>> {
    let mut rows: Vec<Row> = Vec::new();
    debug_assert!(input[0].to_lowercase() == "rows");
    //Skip first item in buffer (string Rows)
    for line in &input[1..] {
        let split_row = line.split_whitespace().collect::<Vec<&str>>();
        if split_row.len() != 2 {
            return Err(Box::new(ParserError::new("Incorrect format of line in section ROWS.", line)));
        }
        let Ok(constraint) = split_row[0].parse::<Constraints>() else {
            return Err(Box::new(ParserError::new("Incorrect format of constraint in line", line)));
        };
        rows.push(Row::new(constraint, String::from(split_row[1])));
    }
    Ok(rows)
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



fn parse_mps(input: &String, logger: &SimpleLogger) -> Result<(), Box<ParserError>> {
    info!("Started parsing MPS input.");
    let start_timestamp = Utc::now();

    if input.is_empty() {
        return Err(Box::new(ParserError::from_string_structure("Input MPS is empty!", input.clone())));
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
    use crate::parsers::mps::{Constraints, Row};
    use crate::parsers::mps_parser::{parse_name, parse_rows};

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

    #[test]
    fn parse_rows_succeeds() {
        let input = "ROWS\n N CONST\n L LIM1\n G LIM2\n E MYEQN".split("\n").collect();
        let parse_res = parse_rows(&input);
        assert!(parse_res.is_ok());

        let rows = parse_res.unwrap();
        assert_eq!(rows.len(), 4);

        assert_eq!(rows[0], Row::new(Constraints::N, String::from("CONST")));
        assert_eq!(rows[1], Row::new(Constraints::L, String::from("LIM1")));
        assert_eq!(rows[2], Row::new(Constraints::G, String::from("LIM2")));
        assert_eq!(rows[3], Row::new(Constraints::E, String::from("MYEQN")));
    }

    #[test]
    fn parse_rows_with_invalid_constraint_fails() {
        let input = "ROWS\n I CONST\n L LIM1\n G LIM2\n E MYEQN".split("\n").collect();
        let parse_res = parse_rows(&input);
        assert!(parse_res.is_err());
    }

    #[test]
    fn parse_rows_with_three_row_arguments_fails() {
        let input = "ROWS\n L CONST LIM1\n L LIM1\n G LIM2\n E MYEQN".split("\n").collect();
        let parse_res = parse_rows(&input);
        assert!(parse_res.is_err());
    }

    #[test]
    fn parse_rows_without_row_name_fails() {
        let input = "ROWS\n N \n L LIM1\n G LIM2\n E MYEQN".split("\n").collect();
        let parse_res = parse_rows(&input);
        assert!(parse_res.is_err());
    }


}