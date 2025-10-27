use super::mps::{BoundType, Constraints, MpsModelWithSelectedVariants};
use super::ParserError;
use crate::document::html_convertible_error::HtmlConvertibleError;
use crate::rationals::Rational;
use crate::solvers::simplex_error::SimplexError;
use indexmap::IndexMap;
use std::collections::HashSet;
use log::{debug, log, warn};
use crate::utils::env_parameters::ApplicationEnvParameter;
use crate::utils::env_parameters::ApplicationEnvParameter::MaxVariableLength;

impl MpsModelWithSelectedVariants {

    pub fn verify_mps_model(&self) -> Result<(), Box<dyn HtmlConvertibleError>>{
        let row_names_set = self.model.rows.rows.iter().map(|(row_name, _)| row_name).collect::<HashSet<_>>();

        //Verify selected RHS
        let selected_rhs;
        if self.selected_rhs.is_none() && self.model.rhs.rhs.len() == 1{
            selected_rhs = &self.model.rhs.rhs[0];
        } else if let Some(selected_rhs_name) = &self.selected_rhs {
            if let Some(selected_rhs_from_model) = self.model.rhs.rhs.get(selected_rhs_name) {
                selected_rhs = selected_rhs_from_model;
            } else {
                return Err(Box::new(ParserError::from_string_structure("Selected RHS not found in model RHSs.", format!("Selected: {}. Present in model: {}", selected_rhs_name, self.model.rhs.rhs.keys().map(|x| x.to_owned()).collect::<Vec<String>>().join(", ")))));
            }
        } else {
            return Err(Box::new(ParserError::from_string_structure("None RHS was selected, but multiple were found in model.", format!("Found RHS in model: {}", self.model.rhs.rhs.keys().map(|x| x.to_owned()).collect::<Vec<String>>().join(", ")))));
        }

        //Each row has RHS
        for (row_name, constraint) in &self.model.rows.rows {
            if *constraint == Constraints::N {
                continue;
            }
            if !selected_rhs.contains_key(row_name) {
                return Err(Box::new(ParserError::from_string_structure("Selected RHS does not contain value for ROW!", format!("Selected RHS: {}. ROW: {}", self.selected_rhs.as_ref().unwrap_or(&"DEFAULT".to_owned()).clone(), row_name))));
            }
        }

        //Each rhs has row
        for row_name in selected_rhs.keys() {
            if !row_names_set.contains(&row_name) {
                return Err(Box::new(ParserError::from_string_structure("Selected RHS contains not defined ROW.", format!("RHS: {}, ROW: {}.", self.selected_rhs.as_ref().unwrap_or(&"DEFAULT".to_owned()), row_name))));
            }
        }

        //Column variables don't have none existent rows and are legal
        for (variable_name, variable_values) in &self.model.columns.variables {
            if !is_variable_name_legal(variable_name) {
                return Err(Box::new(ParserError::from_string_structure("Variable name is illegal. Letters A,a,S,s followed by numbers are reserved for slack, surplus and artificial variable.", format!("Failing variable name: {}.", variable_name))));
            }
            for (row_name, _) in variable_values {
                if !row_names_set.contains(row_name) {
                    return Err(Box::new(ParserError::from_string_structure("COLUMN specifies variable for non-existent row", format!("Variable name {}. Non existent row name {}.", variable_name, row_name))))
                }
            }
        }

        //Selected objective row exists (if selected), if not, only one objective row has to be present
        let objective_row_names = self.model.rows.rows.iter().filter(|(row_name, constraint)| **constraint == Constraints::N)
            .map(|(row_name, constraint)| row_name).collect::<HashSet<_>>();

        if objective_row_names.is_empty() {
            return Err(Box::new(ParserError::new("No objective rows found in model!", "Model does not contain row with specifier N.")));
        }

        if objective_row_names.len() == 1 && self.selected_opt_row_name.is_none() {
            //OK
        } else if let Some(selected_row_name) = &self.selected_opt_row_name {
            if !objective_row_names.contains(selected_row_name) {
                return Err(Box::new(ParserError::from_string_structure("Selected objective row name not found among defined objective rows!", format!("Selected: {}, model contains: {}.", selected_row_name, objective_row_names.iter().map(|x| x.to_owned().to_owned()).collect::<Vec<String>>().join(", ")))));
            }
        } else {
            return Err(Box::new(ParserError::from_string_structure("No objective row selected and multiple found int he model!. Select one for optimisation.", format!("Objective rows found {}.", objective_row_names.iter().map(|x| x.to_owned().to_owned()).collect::<Vec<String>>().join(", ")))));
        }

        //Bounds contain defined variables
        let defined_variables = self.model.columns.variables.keys().collect::<HashSet<_>>();
        for (bound_name, variables) in &self.model.bounds.bounds {
            for (variable_name, _, _) in variables {
                if !defined_variables.contains(variable_name) {
                    return Err(Box::new(ParserError::from_string_structure("Bound contains not defined variable.", format!("BOUND name: {}, non defined variable: {}.", bound_name, variable_name))));
                }
            }
        }


        //Selected bounds are present
        if (self.model.bounds.bounds.len() == 1 || self.model.bounds.bounds.is_empty()) && self.selected_bounds.is_none() {
            //OK
        } else if let Some(selected_bound_name) = &self.selected_bounds {
            if !self.model.bounds.bounds.contains_key(selected_bound_name) {
                return Err(Box::new(ParserError::from_string_structure("Selected BOUNDS are not present in the model!", format!("Selected BOUNDS: {}, present bounds: {}.", selected_bound_name, self.model.bounds.bounds.keys().map(|x| x.to_owned()).collect::<Vec<String>>().join(", ")))));
            }
        } else {
            return Err(Box::new(ParserError::from_string_structure("No BOUNDS were specified, but multiple are present in the model! Select one!", format!("Present bounds: {}.", self.model.bounds.bounds.keys().map(|x| x.to_owned()).collect::<Vec<String>>().join(", ")))));
        }

        Ok(())
    }

    /// Return selected bounds from the model. If none are selected, return none
    /// Return error explaining why, if no bounds are chosen
    pub fn get_selected_bounds_from_the_model(self: &MpsModelWithSelectedVariants) -> Result<Option<&Vec<(String, Rational, BoundType)>>, Box<SimplexError>> {
        if self.selected_bounds.is_none() {
            Ok(None)
        } else {
            let selected_bounds_name = self.selected_bounds.as_ref().unwrap();
            let Some(bounds) = self.model.bounds.bounds.get(selected_bounds_name) else {
                return Err(Box::new(SimplexError::from_string_reason(format!("Selected bounds {selected_bounds_name} were not found among the ones defined in the model.\nPlease select defined bounds."))));
            };
            Ok(Some(bounds))
        }
    }
}

/// Return true if variable name is legal
/// Illegal variable names are S\d+, s\d+ , A\d+, a\d+
fn is_variable_name_legal(variable_name: &String) -> bool {
    let max_var_length = ApplicationEnvParameter::get_as_usize(&MaxVariableLength);
    debug_assert!(max_var_length.is_some());
    warn!("Max variable length could not be parsed as usize!");
    if let Some(max_var_length) = max_var_length {
       if variable_name.len() > max_var_length {
           warn!("Variable name is too long!. Variable {variable_name}");
           return false;
       }
    }

    let mut all_numeric = true;
    if variable_name.to_lowercase().starts_with("s") || variable_name.to_lowercase().starts_with("a"){
        for i in variable_name[1..].chars() {
            if !i.is_numeric() {
                all_numeric = false;
            }
        }
        if all_numeric {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::super::mps::test_utils::create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives;
    use crate::parsers::mps::{BoundType, MpsModelWithSelectedVariants};
    use crate::parsers::mps_with_selected_variants_operations::is_variable_name_legal;
    use crate::rationals::Rational;
    use crate::solvers::basic_simplex_table_data::OptimizationType;
    use crate::utils::env_parameters::ApplicationEnvParameter;
    // #[test]
    // fn convert_initially_unfeasible_rhs_constraints_and_bounds_succeeds() {
    //     let mut model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
    //     let rhs1 = model.rhs.rhs.get_mut("RHS1").unwrap();
    //     // RHS1 for row2 is -4
    //     rhs1.get_mut("ROW1").unwrap().negate_mut();
    //     //Leave only one <= bound and negate it, so it can be converted
    //     model.bounds.bounds.get_mut("BND1").unwrap().remove(0);
    //     model.bounds.bounds.get_mut("BND1").unwrap().remove(4);
    //     model.bounds.bounds.get_mut("BND1").unwrap()[0].1.negate_mut();
    //
    //     // 2x1 + x2 <= -6   ROW1
    //     // x1 + x2 = 4    ROW2
    //     // x1 - x2 >= 1    ROW3
    //     // 3x1 + x2 -> N   OBJ1
    //     // 2x1 + 8x2 -> N  OBJ2
    //     // x1 <= -20    BND1
    //     // x2 >= 5     BND1
    //     // x2 ≥ 10     BND1
    //
    //     let mut mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
    //
    //     mps_model_with_selected_variants.convert_initially_unfeasible_rhs_constraints_and_bounds();
    //     assert_eq!(mps_model_with_selected_variants.model.rows.rows[0], Constraints::G);
    //     assert_eq!(mps_model_with_selected_variants.model.rows.rows[1], Constraints::E);
    //     assert_eq!(mps_model_with_selected_variants.model.rows.rows[2], Constraints::G);
    //
    //
    // }

    #[test]
    fn verify_mps_succeeds() {
        let model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_ok());
    }

    #[test]
    fn verify_mps_fails_for_no_selected_rhs() {
        let model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, None, Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_undefined_selected_rhs() {
        let model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("kdljfkldj".to_owned()), Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_rhs_with_undefined_row() {
        let mut model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        model.rhs.rhs.get_mut(&"RHS1".to_owned()).unwrap().insert("KJDF".to_owned(), Rational::zero());
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_row_with_undefined_rhs() {
        let mut model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        model.rhs.rhs.get_mut(&"RHS1".to_owned()).unwrap().remove(&"ROW1".to_owned());
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_variable_with_undefined_row() {
        let mut model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        model.columns.variables.get_mut("x1").unwrap().insert("kjdfld".to_owned(), Rational::zero());
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_illegal_variable_name() {
        let mut model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        let x1_values = model.columns.variables.get("x1").unwrap().clone();
        model.columns.variables.swap_remove("x1");
        model.columns.variables.insert("S1".to_owned(), x1_values);
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_none_obj_functions() {
        let mut model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        model.rows.rows.swap_remove("OBJ1");
        model.rows.rows.swap_remove("OBJ2");
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_obj_function_name_not_found() {
        let model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJDF2221".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_no_obj_function_defined_and_multiple_present() {
        let mut model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), None, OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_undefined_variable_in_bounds() {
        let mut model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        model.bounds.bounds.get_mut("BND1").unwrap().push(("dfdf".to_owned(), Rational::zero(), BoundType::LO));
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BND1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_missing_selected_bounds() {
        let model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, Some("RHS1".to_owned()), Some("BNDDFDF1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }

    #[test]
    fn verify_mps_fails_for_none_bounds_selected_when_more_present() {
        let model = create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives();
        let mps_model_with_selected_variants = MpsModelWithSelectedVariants::new(model, None, Some("BNDDFDF1".to_owned()), Some("OBJ1".to_owned()), OptimizationType::MAX);
        assert!(mps_model_with_selected_variants.verify_mps_model().is_err());
    }


    #[test]
    fn is_variable_name_legal_succeeds_for_legal() {
        let legal = "AHOJ".to_owned();
        assert!(is_variable_name_legal(&legal));
    }

    #[test]
    fn is_variable_name_legal_fails_for_illegal() {
        let illegal = "A12".to_owned();
        let illegal2 = "S3".to_owned();
        let illegal3 = illegal.to_lowercase();
        let illegal4 = illegal2.to_lowercase();
        assert!(!is_variable_name_legal(&illegal));
        assert!(!is_variable_name_legal(&illegal2));
        assert!(!is_variable_name_legal(&illegal3));
        assert!(!is_variable_name_legal(&illegal4));
    }

    #[test]
    fn is_variable_name_legal_fails_for_too_long() {
        env::set_var(ApplicationEnvParameter::MaxVariableLength.to_string(), "2");
        let too_long = "AHOJ".to_owned();
        assert!(!is_variable_name_legal(&too_long));
        env::set_var(ApplicationEnvParameter::MaxVariableLength.to_string(), ApplicationEnvParameter::MaxVariableLength.get_or_default());
    }
}