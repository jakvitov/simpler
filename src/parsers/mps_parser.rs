use crate::parsers::mps::MpsModel;
use crate::parsers::ParserError;
use chrono::Utc;
use log::info;
use log::LevelFilter;
use regex::Regex;
use simple_logger::SimpleLogger;
use std::error::Error;

fn parse_name(lowercase_input: &String) -> Result<String, Box<ParserError>> {
    let Some(first_line) = lowercase_input.lines().next() else {
        return Err(Box::new(ParserError::from_string_structure("No section NAME found in input", lowercase_input.clone())));
    };
    if first_line.len() < 5 {
        return Err(Box::new(ParserError::from_string_structure("Incorrect NAME section in MPS", lowercase_input.clone())));
    }

    if first_line[0..4].to_string() != "name" {
        return Err(Box::new(ParserError::from_string_structure("MPS model does not start with NAME section", lowercase_input.clone())));
    }

    let res = first_line[4..].trim().to_string();
    if res.is_empty() {
        return Err(Box::new(ParserError::from_string_structure("MPS model name cannot contain only whitespaces", lowercase_input.clone())));
    }
    Ok(res)
}

fn parse_mps(input: String, logger: &SimpleLogger) -> Result<(), Box<ParserError>> {
    info!("Started parsing MPS input.");
    let start_timestamp = Utc::now();

    if input.is_empty() {
        return Err(Box::new(ParserError::from_string_structure("Input MPS is empty!", input)));
    }

    let lowercase_input = input.to_ascii_lowercase().to_string();



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