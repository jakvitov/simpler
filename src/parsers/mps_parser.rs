use super::mps::{BoundType, Bounds, Columns, Constraints, MpsModel, Rhs, Rows, Sections};
use super::ParserError;
use crate::rationals::Rational;
use chrono::Utc;
use log::info;
use std::collections::HashMap;
use std::str::FromStr;

pub(super) struct MpsInParsing {
    pub(super) name: Option<String>,
    pub(super) rows: Option<Rows>,
    pub(super) columns: Option<Columns>,
    pub(super) rhs: Option<Rhs>,
    pub(super) bounds: Option<Bounds>
}

impl MpsInParsing {

    #[allow(dead_code)]
    fn empty() -> Self {
        MpsInParsing { name: None, rows: None, columns: None, rhs: None, bounds: None }
    }

    /// Return () if mps in parsing is fully filled
    /// Return parser error with message about which section has not been filled otherwise
    pub(super) fn is_filled(&self) -> Result<(), Box<ParserError>> {
        if self.name.is_none() {
            return Err(Box::new(ParserError::new("Parsed MPS error: model misses field NAME. Possible cause: NAME or ROWS is missing.", "")))
        } else if self.rows.is_none() {
            return Err(Box::new(ParserError::new("Parsed MPS error: model misses field ROWS. Possible cause: ROWS or COLUMNS is missing.", "")))
        } else if self.columns.is_none() {
            return Err(Box::new(ParserError::new("Parsed MPS error: model misses field COLUMNS. Possible cause: COLUMNS or RHS is missing.", "")))
        } else if self.rhs.is_none() {
            return Err(Box::new(ParserError::new("Parsed MPS error: model misses field RHS. Possible cause: RHS or BOUNDS  is missing.", "")))
        } else if self.bounds.is_none() {
            return Err(Box::new(ParserError::new("Parsed MPS error: model misses field BOUNDS. Possible cause: BOUNDS or ENDATA is missing.", "")))
        }
        Ok(())
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
    if input.len() < 2 {
        return Err(Box::new(ParserError::from_string_structure("Missing section ROWS in the MPS model.", input.join("\n"))));
    }
    debug_assert!(input[0].to_lowercase().trim() == "rows");

    let mut rows: Rows = Rows::empty();
    //We keep the objective rows separately and add them as last to be at the bottom
    let mut obj_rows: Vec<(String, Constraints)> = Vec::new();

    //Skip first item in buffer (string Rows)
    for line in &input[1..] {
        let split_row = line.split_whitespace().collect::<Vec<&str>>();
        if split_row.len() != 2 {
            return Err(Box::new(ParserError::new("Incorrect format of line in section ROWS.", line)));
        }
        let Ok(constraint) = split_row[0].parse::<Constraints>() else {
            return Err(Box::new(ParserError::new("Incorrect format of constraint in line", line)));
        };
        let row_name = split_row[1].to_string();
        if rows.rows.contains_key(&row_name) {
            return Err(Box::new(ParserError::new("Row with the given name already exists.", line)));
        }
        if constraint == Constraints::N {
            obj_rows.push((row_name, constraint));
        } else {
            rows.rows.insert(row_name, constraint);
        }
    }
    for (row_name, constraint) in obj_rows {
        rows.rows.insert(row_name, constraint);
    }
    Ok(rows)
}

fn parse_columns(input: &Vec<&str>)  -> Result<Columns, Box<ParserError>> {

    if input.len() < 2 {
        return Err(Box::new(ParserError::from_string_structure("Column section is incorrect", input.join("\n"))))
    }
    debug_assert!(input[0].to_lowercase().trim() == "columns");

    let mut res = Columns::empty();

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
                    let row_name = parts[i].to_string();
                    if values.contains_key(&row_name) {
                        return Err(Box::new(ParserError::from_string_message(format!("Section COLUMNS: Row {row_name} has already assigned value for variable {var_name}."), line)));
                    }
                    //todo what if given variable already contains row_name with a value?
                    // + add test for this
                    let variable_amount = Rational::from_str(parts[i+1])?;
                    values.insert(parts[i].to_string(), variable_amount);
                },
                None => {
                    let variable_amount = Rational::from_str(parts[i+1])?;
                    let mut variable_values = HashMap::new();
                    variable_values.insert(parts[i].to_string(), variable_amount);
                    res.variables.insert(var_name.to_string(), variable_values);
                }
            }
        }
    }
    Ok(res)
}
fn parse_bounds(input: &Vec<&str>)  -> Result<Bounds, Box<ParserError>> {
    if input.len() == 1 && input[0] == "bounds" {
        return Ok(Bounds::empty());
    }
    if input.len() < 2 {
        return Err(Box::new(ParserError::from_string_structure("Bounds section is incorrect", input.join("\n"))))
    }
    debug_assert!(input[0].to_lowercase().trim() == "bounds");
    let mut res = Bounds::empty();

    //We iterate from 1, because line 0 is "bounds"
    for line in input[1..].iter() {
        let line_split = line.split_whitespace().collect::<Vec<&str>>();
        if line_split.len() != 4 {
            return Err(Box::new(ParserError::new("Bound in input has incorrect length.\nRequired are 3 elements.", line)));
        }
        let Ok(bound_type) = line_split[0].parse::<BoundType>() else {
            return Err(Box::new(ParserError::new("Incorrect bound type.", line)));
        };
        let value = Rational::from_str(line_split[3])?;
        let bound_name = line_split[1];
        match res.bounds.get_mut(bound_name) {
            Some(bound_list) => {
                bound_list.push((String::from(line_split[2]), value, bound_type));
            },
            None => {
                let bound_list = vec![(String::from(line_split[2]), value, bound_type)];
                res.bounds.insert(bound_name.to_string(), bound_list);
            }
        }
    }
    Ok(res)
}

fn parse_rhs(input: &Vec<&str>)  -> Result<Rhs, Box<ParserError>> {
    if input.len() < 2 {
        return Err(Box::new(ParserError::from_string_structure("RHS section empty.", input.join("\n").to_string())));
    }

    debug_assert!(input[0].to_lowercase().trim() == "rhs");
    let mut res = Rhs::empty();
    //We skip first line, because it contains "rhs" string
    for line in input[1..].iter() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() % 2 != 1 {
            return Err(Box::new(ParserError::new("Incorrect number of arguments for RHS line.", line)));
        }

        let rhs_name = parts[0];
        //We iterate over pairs of row_name(i) + value(i+1)
        for i in (1..parts.len()).step_by(2) {
            let row_name = parts[i];
            let value = Rational::from_str(parts[i+1])?;
            match res.rhs.get_mut(rhs_name) {
                Some(rhs) => {
                    if rhs.contains_key(row_name)  {
                        return Err(Box::new(ParserError::new("RHS with given name already contains this row with a value.", line)));
                    }
                    rhs.insert(row_name.to_string(), value);
                },
                None => {
                    let mut row_value = HashMap::new();
                    row_value.insert(row_name.to_string(), value);
                    res.rhs.insert(rhs_name.to_string(), row_value);
                }
            }
        }
    }
    Ok(res)
}


#[allow(dead_code)]
pub fn parse_mps(input: &String) -> Result<MpsModel, Box<ParserError>> {
    info!("Started parsing MPS input.");
    let start_timestamp = Utc::now();

    if input.is_empty() {
        return Err(Box::new(ParserError::from_string_structure("Input MPS is empty!", input.clone())));
    }

    let lowercase_input = input.to_ascii_lowercase().to_string();
    let mut lines = lowercase_input.lines().filter(|l| !l.trim().is_empty());
    let mut state = Sections::NAME;
    let mut buffer = Vec::<&str>::new();
    let mut mps_in_parsing = MpsInParsing::empty();
    while let Some(line) = lines.next() {
        match state {
            Sections::NAME => {
                mps_in_parsing.name = Some(parse_name(line)?);
                state = Sections::ROWS;
            },
            Sections::ROWS => {
                if line == "columns" {
                    state = Sections::COLUMNS;
                    let parsed_rows = parse_rows(&buffer)?;
                    mps_in_parsing.rows = Some(parsed_rows);
                    buffer = vec![line]
                } else {
                    buffer.push(line);
                }
            },
            Sections::COLUMNS => {
                if line == "rhs" {
                    state = Sections::RHS;
                    let parsed_columns = parse_columns(&buffer)?;
                    mps_in_parsing.columns = Some(parsed_columns);
                    buffer = vec![line]
                } else {
                    buffer.push(line);
                }
            },
            Sections::RHS => {
                if line == "bounds" {
                    state = Sections::BOUNDS;
                    let parsed_rhs = parse_rhs(&buffer)?;
                    mps_in_parsing.rhs = Some(parsed_rhs);
                    buffer = vec![line]
                } else {
                    buffer.push(line);
                }
            },
            Sections::BOUNDS => {
                if line == "endata" {
                    state = Sections::ENDATA;
                    let parsed_bounds = parse_bounds(&buffer)?;
                    mps_in_parsing.bounds = Some(parsed_bounds);
                    buffer = vec![line]
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
    let res = mps_in_parsing.try_into()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::parsers::mps::{BoundType, Bounds, Columns, Constraints, Rhs, Rows};
    use crate::parsers::mps_parser::{parse_bounds, parse_columns, parse_name, parse_rhs, parse_rows, MpsInParsing};
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
    fn parse_rows_adds_objective_rows_at_the_end() {
        let input = "ROWS\n N CONST\n N CONST2\n L LIM1\n G LIM2\n E MYEQN".split("\n").collect();
        let parse_res = parse_rows(&input);
        assert!(parse_res.is_ok());

        let rows = parse_res.unwrap();
        assert_eq!(rows.rows.len(), 5);
        assert_eq!(rows.rows[3], Constraints::N);
        assert_eq!(rows.rows[4], Constraints::N)
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
        assert_eq!(*xone_variable.get("COST").unwrap(), Rational::new(1,2));
        assert_eq!(*xone_variable.get("LIM1").unwrap(), Rational::new(-5,9));
        assert_eq!(*xone_variable.get("LIM2").unwrap(), Rational::new(2,5));

        let ytwo_variable = columns.variables.get("YTWO").unwrap();
        assert!(ytwo_variable.len() == 3);
        assert_eq!(*ytwo_variable.get("COST").unwrap(), Rational::new(4,1));
        assert_eq!(*ytwo_variable.get("LIM1").unwrap(), Rational::new(1,1));
        assert_eq!(*ytwo_variable.get("MYEQN").unwrap(), Rational::new(-1,1));

        let zthree_variable = columns.variables.get("ZTHREE").unwrap();
        assert_eq!(*zthree_variable.get("COST").unwrap(), Rational::new(9,1));
        assert_eq!(*zthree_variable.get("LIM2").unwrap(), Rational::new(1,1));
        assert_eq!(*zthree_variable.get("MYEQN").unwrap(), Rational::new(1,1));
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
        assert_eq!(*xone_variable.get("COST").unwrap(), Rational::new(1,2));
        assert_eq!(*xone_variable.get("LIM1").unwrap(), Rational::new(-5,9));
        assert_eq!(*xone_variable.get("LIM2").unwrap(), Rational::new(2,5));
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
        assert_eq!(*xone_variable.get("COST").unwrap(), Rational::new(1,2));
        assert_eq!(*xone_variable.get("LIM1").unwrap(), Rational::new(-5,9));
        assert_eq!(*xone_variable.get("LIM2").unwrap(), Rational::new(2,5));
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

    #[test]
    fn parse_columns_variable_having_two_values_for_the_same_column_fails() {
        let input = "COLUMNS     \n\t    XONE      COST                 1/2   LIM1                 -5/9\n\tXONE      LIM2                 2/5    COST 2/5\n\tYTWO      COST                 4   LIM1                 1\n\tYTWO      MYEQN               -1\n\tZTHREE    COST                 9   LIM2                 1\n\tZTHREE    MYEQN                1"
            .split("\n").collect();
        let parse_res = parse_columns(&input);
        assert!(parse_res.is_err());
    }

    #[test]
    fn parse_bounds_with_one_bound_name_succeeds() {
        let input = "BOUNDS     \n\tUP BND1      XONE                 4/3\n\tLO BND1      YTWO                -1/2\n\tUP BND1      YTWO                 1"
            .split("\n").collect();
        let bounds_parse_res = parse_bounds(&input);
        assert!(bounds_parse_res.is_ok());
        let bounds = bounds_parse_res.unwrap();

        assert_eq!(bounds.bounds.keys().len(), 1);
        let bnd1 = bounds.bounds.get("BND1").unwrap();

        assert_eq!(bnd1[0], (String::from("XONE"), Rational::new(4,3), BoundType::UP));
        assert_eq!(bnd1[1], (String::from("YTWO"), Rational::new(-1,2), BoundType::LO));
        assert_eq!(bnd1[2], (String::from("YTWO"), Rational::new(1,1), BoundType::UP));
    }

    #[test]
    fn parse_bounds_with_two_bound_names_succeeds() {
        let input = "BOUNDS     \n\tUP BND1      XONE                 4/3\n\tLO BND1      YTWO                -1/2\n\tUP BND1      YTWO                 1\n\tUP BND2    YTWO                 1"
            .split("\n").collect();
        let bounds_parse_res = parse_bounds(&input);
        assert!(bounds_parse_res.is_ok());
        let bounds = bounds_parse_res.unwrap();

        assert_eq!(bounds.bounds.keys().len(), 2);
        let bnd1 = bounds.bounds.get("BND1").unwrap();

        assert_eq!(bnd1[0], (String::from("XONE"), Rational::new(4,3), BoundType::UP));
        assert_eq!(bnd1[1], (String::from("YTWO"), Rational::new(-1,2), BoundType::LO));
        assert_eq!(bnd1[2], (String::from("YTWO"), Rational::new(1,1), BoundType::UP));

        let bnd2 = bounds.bounds.get("BND2").unwrap();
        assert_eq!(bnd2[0], (String::from("YTWO"), Rational::new(1,1), BoundType::UP));


    }

    #[test]
    fn parse_bounds_without_bound_intendation_succeeds() {
        let input = "BOUNDS     \nUP BND1      XONE                 1"
            .split("\n").collect();
        let bounds_parse_res = parse_bounds(&input);
        assert!(bounds_parse_res.is_ok());
        let bounds = bounds_parse_res.unwrap();

        assert_eq!(bounds.bounds.keys().len(), 1);
        let bnd1 = bounds.bounds.get("BND1").unwrap();

        assert_eq!(bnd1[0], (String::from("XONE"), Rational::new(1,1), BoundType::UP));

    }

    #[test]
    fn parse_bounds_with_empty_bound_fails() {
        let input = "BOUNDS     \n\tUP BND1      XONE                 1\n\tLO BND1      YTWO                1\n\tUP"
            .split("\n").collect();
        let bounds_parse_res = parse_bounds(&input);
        assert!(bounds_parse_res.is_err());
    }

    #[test]
    fn parse_bounds_with_invalid_rational_value_fails() {
        let input = "BOUNDS     \n\tUP BND1      XONE                 wrong_value\n\tLO BND1      YTWO                1\n\tUP BND1      YTWO                 1"
            .split("\n").collect();
        let bounds_parse_res = parse_bounds(&input);
        assert!(bounds_parse_res.is_err());
    }

    #[test]
    fn parse_rhs_with_one_rhs_name_suceeds() {
        let input = "RHS   \n\tRHS1      LIM1                 -5/2   LIM2                10\n\tRHS1      MYEQN                -7"
            .split("\n").collect();
        let parsed_rhs = parse_rhs(&input);
        assert!(parsed_rhs.is_ok());
        let rhs = parsed_rhs.unwrap();

        assert_eq!(rhs.rhs.len(), 1);
        let rhs1 = rhs.rhs.get("RHS1").unwrap();
        assert_eq!(rhs1.len(), 3);
        assert_eq!(*rhs1.get("LIM1").unwrap(), Rational::new(-5,2));
        assert_eq!(*rhs1.get("LIM2").unwrap(), Rational::new(10,1));
        assert_eq!(*rhs1.get("MYEQN").unwrap(), Rational::new(-7,1));
    }

    #[test]
    fn parse_rhs_with_two_rhs_names_succeeds() {
        let input = "RHS   \n\tRHS1      LIM1                 -5/2   LIM2                10\n\tRHS1      MYEQN                -7\n\tRHS2      LIM1                 -5/2   LIM2                10\n\tRHS2      MYEQN                -7"
            .split("\n").collect();
        let parsed_rhs = parse_rhs(&input);
        assert!(parsed_rhs.is_ok());
        let rhs = parsed_rhs.unwrap();

        assert_eq!(rhs.rhs.len(), 2);
        let rhs1 = rhs.rhs.get("RHS1").unwrap();
        assert_eq!(rhs1.len(), 3);
        assert_eq!(*rhs1.get("LIM1").unwrap(), Rational::new(-5,2));
        assert_eq!(*rhs1.get("LIM2").unwrap(), Rational::new(10,1));
        assert_eq!(*rhs1.get("MYEQN").unwrap(), Rational::new(-7,1));

        let rhs2 = rhs.rhs.get("RHS2").unwrap();
        assert_eq!(rhs2.len(), 3);
        assert_eq!(*rhs2.get("LIM1").unwrap(), Rational::new(-5,2));
        assert_eq!(*rhs2.get("LIM2").unwrap(), Rational::new(10,1));
        assert_eq!(*rhs2.get("MYEQN").unwrap(), Rational::new(-7,1));
    }

    ///RHS
    ///for example MY_RHS ROW1 1 ROW1 1 -> Fails
    #[test]
    fn parse_rhs_with_two_same_variables_under_same_rhs_name_fails() {
        let input = "RHS   \n\tRHS1      LIM1                 -5/2   LIM2                10\n\tRHS1      LIM2                -7"
            .split("\n").collect();
        let parsed_rhs = parse_rhs(&input);
        assert!(parsed_rhs.is_err());
    }

    #[test]
    fn parse_rhs_without_data_fails() {
        let input = "RHS   \n".split("\n").collect();
        let parsed_rhs = parse_rhs(&input);
        assert!(parsed_rhs.is_err());
    }

    #[test]
    fn parse_rhs_row_without_value_fails() {
        let input = "RHS   \n\tRHS1      LIM1                    LIM2                10\n\tRHS1      MYEQN                -7"
            .split("\n").collect();
        let parsed_rhs = parse_rhs(&input);
        assert!(parsed_rhs.is_err());
    }

    #[test]
    fn parse_rhs_without_row_name_fails() {
        let input = "RHS   \n\tRHS1      LIM1                 -5/2   LIM2                10\n\tRHS1                     -7"
            .split("\n").collect();
        let parsed_rhs = parse_rhs(&input);
        assert!(parsed_rhs.is_err());
    }

    #[test]
    fn parse_rhs_with_invalid_rational_fails() {
        let input = "RHS   \n\tRHS1      LIM1                 wrong_rational   LIM2                10\n\tRHS1      MYEQN                -7\n\tRHS2      LIM1                 -5/2   LIM2                10\n\tRHS2      MYEQN                -7"
            .split("\n").collect();
        let parsed_rhs = parse_rhs(&input);
        assert!(parsed_rhs.is_err());
    }

    #[test]
    fn mps_in_parsing_is_full_works_successfully_for_full_struct() {
        let mut mps_in_parsing = MpsInParsing::empty();
        mps_in_parsing.rows = Some(Rows::empty());
        mps_in_parsing.columns = Some(Columns::empty());
        mps_in_parsing.name = Some(String::from("test_name"));
        mps_in_parsing.rhs = Some(Rhs::empty());
        mps_in_parsing.bounds = Some(Bounds::empty());
        assert!(mps_in_parsing.is_filled().is_ok())
    }

    #[test]
    fn mps_in_parsing_is_returns_err_when_name_is_missing() {
        let mut mps_in_parsing = MpsInParsing::empty();
        mps_in_parsing.rows = Some(Rows::empty());
        mps_in_parsing.name = Some(String::from("test_name"));
        mps_in_parsing.rhs = Some(Rhs::empty());
        mps_in_parsing.bounds = Some(Bounds::empty());
        assert!(mps_in_parsing.is_filled().is_err());
    }

    #[test]
    fn mps_in_parsing_is_returns_err_when_rows_is_missing() {
        let mut mps_in_parsing = MpsInParsing::empty();
        mps_in_parsing.name = Some(String::from("test_name"));
        mps_in_parsing.columns = Some(Columns::empty());
        mps_in_parsing.rhs = Some(Rhs::empty());
        mps_in_parsing.bounds = Some(Bounds::empty());
        assert!(mps_in_parsing.is_filled().is_err());
    }

    #[test]
    fn mps_in_parsing_is_returns_err_when_column_is_missing() {
        let mut mps_in_parsing = MpsInParsing::empty();
        mps_in_parsing.name = Some(String::from("test_name"));
        mps_in_parsing.rows = Some(Rows::empty());
        mps_in_parsing.rhs = Some(Rhs::empty());
        mps_in_parsing.bounds = Some(Bounds::empty());
        assert!(mps_in_parsing.is_filled().is_err());
    }

    #[test]
    fn mps_in_parsing_is_returns_err_when_rhs_is_missing() {
        let mut mps_in_parsing = MpsInParsing::empty();
        mps_in_parsing.name = Some(String::from("test_name"));
        mps_in_parsing.columns = Some(Columns::empty());
        mps_in_parsing.rows = Some(Rows::empty());
        mps_in_parsing.bounds = Some(Bounds::empty());
        assert!(mps_in_parsing.is_filled().is_err());
    }

    #[test]
    fn mps_in_parsing_is_returns_err_when_bounds_is_missing() {
        let mut mps_in_parsing = MpsInParsing::empty();
        mps_in_parsing.name = Some(String::from("test_name"));
        mps_in_parsing.columns = Some(Columns::empty());
        mps_in_parsing.rows = Some(Rows::empty());
        mps_in_parsing.rhs = Some(Rhs::empty());
        assert!(mps_in_parsing.is_filled().is_err());
    }

}