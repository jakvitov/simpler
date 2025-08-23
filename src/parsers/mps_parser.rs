use crate::parsers::mps::{Bound, Columns, Constraints, MpsModel, Rhs, Rows, Sections};
use crate::parsers::ParserError;
use crate::rationals::Rational;
use chrono::Utc;
use log::info;
use log::LevelFilter;
use regex::Regex;
use simple_logger::SimpleLogger;
use std::error::Error;

struct MpsInParsing {
    name: Option<String>,
    rows: Option<Rows>,
    columns: Option<Columns>,
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

fn parse_rows(input: &Vec<&str>)  -> Result<Rows, Box<ParserError>> {
    let mut rows: Rows = Rows::empty();
    if input.len() < 2 {
        return Err(Box::new(ParserError::from_string_structure("Missing section ROWS in the MPS model.", input.join("\n"))));
    }
    debug_assert!(input[0].to_lowercase().trim() == "rows");
    //Skip first item in buffer (string Rows)
    for line in &input[1..] {
        let split_row = line.split_whitespace().collect::<Vec<&str>>();
        if split_row.len() != 2 {
            return Err(Box::new(ParserError::new("Incorrect format of line in section ROWS.", line)));
        }
        let Ok(constraint) = split_row[0].parse::<Constraints>() else {
            return Err(Box::new(ParserError::new("Incorrect format of constraint in line", line)));
        };
        if rows.rows.contains_key(split_row[1]) {
            return Err(Box::new(ParserError::new("Row with the given name already exists.", line)));
        }
        rows.rows.insert(split_row[1].to_string(), constraint);
    }
    Ok(rows)
}

fn parse_columns(input: &Vec<&str>)  -> Result<Columns, Box<ParserError>> {
    let mut res = Columns::empty();

    if input.len() < 2 {
        return Err(Box::new(ParserError::from_string_structure("Column section is incorrect", input.join("\n"))))
    }
    debug_assert!(input[0].to_lowercase().trim() == "columns");

    //The first line contains "COLUMNS" string
    for line in input[1..].iter() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        //Number of arguments must be odd because we get variable_name and var value pairs
        if parts.len() % 2 != 1 {
            return Err(Box::new(ParserError::new("Line in COLUMNS section is incorrect.\n Invalid number of arguments.", line)));
        }
        let var_name = parts[0];

        //Safe step by, because the number of parts is checked before to be even
        //We start by one, because in the split line, the first string is the variable name
        for i in (1..parts.len()).step_by(2) {
            let variable_values = res.variables.get_mut(var_name);
            match variable_values {
                Some(values) => {
                    let variable_amount = Rational::from_str(parts[i+1])?;
                    values.push((parts[i].to_string(), variable_amount));
                },
                None => {
                    let variable_amount = Rational::from_str(parts[i+1])?;
                    let variable_values = vec![(parts[i].to_string(), variable_amount)];
                    res.variables.insert(var_name.to_string(), variable_values);
                }
            }
        }
    }
    Ok(res)
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
    use crate::parsers::mps::Constraints;
    use crate::parsers::mps_parser::{parse_columns, parse_name, parse_rows};
    use crate::rationals::Rational;

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
        assert_eq!(rows.rows.len(), 4);

        assert_eq!(*rows.rows.get("CONST").unwrap(), Constraints::N);
        assert_eq!(*rows.rows.get("LIM1").unwrap(), Constraints::L);
        assert_eq!(*rows.rows.get("LIM2").unwrap(), Constraints::G);
        assert_eq!(*rows.rows.get("MYEQN").unwrap(), Constraints::E);
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

    #[test]
    fn parse_rows_with_empty_input_fails() {
        let input: Vec <&str> = Vec::new();
        let parse_res = parse_rows(&input);
        assert!(parse_res.is_err());
    }

    #[test]
    fn parse_columns_succeeds() {
        let input = "COLUMNS     \n\t    XONE      COST                 1/2   LIM1                 -5/9\n\tXONE      LIM2                 2/5\n\tYTWO      COST                 4   LIM1                 1\n\tYTWO      MYEQN               -1\n\tZTHREE    COST                 9   LIM2                 1\n\tZTHREE    MYEQN                1"
            .split("\n").collect();
        let parse_res = parse_columns(&input);
        assert!(parse_res.is_ok());
        let columns = parse_res.unwrap();
        assert_eq!(columns.variables.len(), 3);

        let xone_variable = columns.variables.get("XONE").unwrap();
        assert!(xone_variable.len() == 3);
        assert_eq!(xone_variable[0], (String::from("COST"), Rational::new(1,2)));
        assert_eq!(xone_variable[1], (String::from("LIM1"), Rational::new(-5,9)));
        assert_eq!(xone_variable[2], (String::from("LIM2"), Rational::new(2,5)));

        let ytwo_variable = columns.variables.get("YTWO").unwrap();
        assert!(ytwo_variable.len() == 3);
        assert_eq!(ytwo_variable[0], (String::from("COST"), Rational::new(4,1)));
        assert_eq!(ytwo_variable[1], (String::from("LIM1"), Rational::new(1,1)));
        assert_eq!(ytwo_variable[2], (String::from("MYEQN"), Rational::new(-1,1)));

        let zthree_variable = columns.variables.get("ZTHREE").unwrap();
        assert_eq!(zthree_variable[0], (String::from("COST"), Rational::new(9,1)));
        assert_eq!(zthree_variable[1], (String::from("LIM2"), Rational::new(1,1)));
        assert_eq!(zthree_variable[2], (String::from("MYEQN"), Rational::new(1,1)));
    }

    #[test]
    fn parse_columns_with_empty_variable_row_succeeds() {
        let input = "COLUMNS     \n\t    XONE      COST                 1/2   LIM1                 -5/9\n\tXONE      LIM2                 2/5\n\tXONE    "
            .split("\n").collect();
        let parse_res = parse_columns(&input);
        assert!(parse_res.is_ok());
        let columns = parse_res.unwrap();
        assert_eq!(columns.variables.len(), 1);
        let xone_variable = columns.variables.get("XONE").unwrap();
        assert!(xone_variable.len() == 3);
        assert_eq!(xone_variable[0], (String::from("COST"), Rational::new(1,2)));
        assert_eq!(xone_variable[1], (String::from("LIM1"), Rational::new(-5,9)));
        assert_eq!(xone_variable[2], (String::from("LIM2"), Rational::new(2,5)));
    }

    #[test]
    fn parse_columns_without_variable_intendation_succeeds() {
        let input = "COLUMNS     \nXONE      COST                 1/2   LIM1                 -5/9\n\tXONE      LIM2                 2/5"
            .split("\n").collect();
        let parse_res = parse_columns(&input);
        assert!(parse_res.is_ok());
        let columns = parse_res.unwrap();
        let xone_variable = columns.variables.get("XONE").unwrap();
        assert!(xone_variable.len() == 3);
        assert_eq!(xone_variable[0], (String::from("COST"), Rational::new(1,2)));
        assert_eq!(xone_variable[1], (String::from("LIM1"), Rational::new(-5,9)));
        assert_eq!(xone_variable[2], (String::from("LIM2"), Rational::new(2,5)));

    }

    #[test]
    fn parse_columns_without_variable_name_fails() {
        let input = "COLUMNS     \n\t    XONE      COST                 1/2   LIM1                 -5/9\n\t      LIM2                 2/5\n\tXONE    "
            .split("\n").collect();
        let parse_res = parse_columns(&input);
        assert!(parse_res.is_err());
    }

    #[test]
    fn parse_columns_without_variable_value_fails() {
        let input = "COLUMNS     \nXONE      COST               LIM1                 -5/9\n\tXONE      LIM2                 2/5"
            .split("\n").collect();
        let parse_res = parse_columns(&input);
        assert!(parse_res.is_err());
    }




}