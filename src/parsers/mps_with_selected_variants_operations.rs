use std::collections::HashSet;
use std::env::var;
use super::mps::{BoundType, Constraints, MpsModelWithSelectedVariants};
use super::ParserError;
use crate::document::html_convertible_error::HtmlConvertibleError;
use crate::rationals::Rational;
use crate::solvers::simplex_error::SimplexError;
use indexmap::IndexMap;

impl MpsModelWithSelectedVariants {

    fn verify_mps_model(&self) -> Result<(), Box<dyn HtmlConvertibleError>>{

        //Verify selected RHS
        let mut selected_rhs;
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

        //Column variables don't have none existent rows
        let row_names_set = self.model.rows.rows.iter().map(|(row_name, constraint)| row_name).collect::<HashSet<_>>();
        for (variable_name, variable_values) in &self.model.columns.variables {
            for (row_name, value) in variable_values {
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
        if self.model.bounds.bounds.len() == 1 && self.selected_bounds.is_none() {
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


    /// Given MPS model, convert initially unfeasible constrains, such as:
    /// x1 <= -5 -> x1 >= 5
    /// x1 >= -10 -> x1 <= 10
    /// !!Remove all the unselected RHS and BOUNDS (conflicts can occur after optimisation)!!
    // fn convert_initially_unfeasible_rhs_constraints_and_bounds(&mut self) -> Result<(), Box<dyn HtmlConvertibleError>>{
    //     //Remove unselected RHS
    //     if self.model.rhs.rhs.len() > 1 {
    //         let Some(selected_rhs) = self.selected_rhs.as_ref() else {
    //             return Err(Box::new(ParserError::from_string_structure("More than one RHS encountered, but none were selected as primary.", format!("Found RHS: {}.", self.model.rhs.rhs.keys().collect::<Vec<_>>().len()))));
    //         };
    //         self.model.rhs.rhs.retain(|rhs_name, _| *rhs_name == *selected_rhs);
    //     }
    //     // Remove unselected bounds
    //     if self.model.bounds.bounds.len() > 1 {
    //         let Some(selected_bounds) = self.selected_bounds.as_ref() else {
    //             return Err(Box::new(ParserError::from_string_structure("More than one BOUNDS encountered, but none were selected as primary.", format!("Found BOUNDS: {}.", self.model.bounds.bounds.keys().collect::<Vec<_>>().len()))));
    //         };
    //         self.model.bounds.bounds.retain(|bound_name, _| *bound_name == *selected_bounds)
    //     }
    //
    //     let Some((selected_rhs_name, selected_rhs_values)) = self.model.rhs.rhs.first_mut() else {
    //         return Err(Box::new(ParserError::from_string_structure("No RHS found to optimise on.", format!("Selected RHS not found among the MPS RHSs."))))
    //     };
    //
    //     let objective_rows_count = self.model.rows.rows.iter().filter(|(_, constraints)| **constraints == Constraints::N).count();
    //
    //     if objective_rows_count == 0 {
    //         return Err(Box::new(ParserError::new("No objective rows found in model. LP cannot optimise!", "No rows with type N found.")));
    //     }
    //     if objective_rows_count > 1 && self.selected_opt_row_name.is_none() {
    //         return Err(Box::new(ParserError::from_string_structure("Multiple objective rows found. But none was selected as primary.", format!("Objective rows: {}", self.model.rows.rows.iter().filter(|(_, constraints)| **constraints == Constraints::N).map(|(row_name, _)| row_name.to_owned()).collect::<Vec<_>>().join(", ")))));
    //     }
    //
    //     // Remove non selected objective rows
    //     self.model.rows.rows.retain(|row_name, constraint| {
    //         if *constraint != Constraints::N {
    //             return true;
    //         }
    //
    //         if objective_rows_count > 1 &&
    //
    //     });
    //
    //     let mut rows_to_invert = Vec::new();
    //     for (i, (row_name, constraint)) in self.model.rows.rows.iter().enumerate() {
    //         if *constraint == Constraints::N {
    //             continue;
    //         }
    //         let Some(rhs_for_row) = selected_rhs_values.get(row_name) else {
    //             return Err(Box::new(ParserError::from_string_structure("No value for row found in selected RHS.", format!("ROW: {row_name}, RHS: {selected_rhs_name}"))));
    //         };
    //         if (*constraint == Constraints::L || *constraint == Constraints::G) && rhs_for_row.is_negative() {
    //             rows_to_invert.push(i);
    //         }
    //     }
    //
    //     //Invert rows and RHS
    //     for i in rows_to_invert {
    //         self.model.rows.rows[i].invert_mut();
    //         let row_name = &self.model.rows.rows.keys()[i];
    //         for (_, variable_values) in &mut self.model.columns.variables {
    //             if let Some(variable_value) = variable_values.get_mut(row_name) {
    //                 variable_value.negate_mut();
    //             }
    //         }
    //         let Some(rhs_value) = selected_rhs_values.get_mut(row_name) else {
    //             return Err(Box::new(ParserError::from_string_structure("No value for row found in selected RHS.", format!("ROW: {row_name}, RHS: {selected_rhs_name}"))));
    //         };
    //         rhs_value.negate_mut();
    //     }
    //
    //     //Optimise and invert bounds
    //     if self.model.bounds.bounds.is_empty() {
    //         return Ok(());
    //     }
    //
    //     let mut optimised_bounds = self.get_optimised_bounds_from_model().map_err(|e| e as Box<dyn HtmlConvertibleError>)?;
    //     let mut bounds_to_invert = Vec::new();
    //     for (index, ((_, _), value)) in optimised_bounds.iter().enumerate() {
    //         if value.is_negative() {
    //             bounds_to_invert.push(index);
    //         }
    //     }
    //
    //     //Iterate over bounds to invert, create new bound as inverted and insert it in  the original place
    //     for i in bounds_to_invert {
    //         let mut bound_key  = optimised_bounds.keys()[i].clone();
    //         optimised_bounds.swap_remove(&bound_key);
    //         bound_key.1.invert_mut();
    //
    //         let mut negated_value = optimised_bounds[i];
    //         negated_value.negate_mut();
    //         optimised_bounds.shift_insert(i, bound_key, negated_value);
    //     }
    //
    //     //Insert optimised bounds back to model
    //     self.model.bounds.bounds.clear();
    //     let optimised_bounds_name = "OPTIMISED_BOUNDS".to_owned();
    //     self.model.bounds.bounds.insert(optimised_bounds_name.clone(), Vec::new());
    //     for ((row_name,bound_type), value) in optimised_bounds {
    //         let bound_vec = self.model.bounds.bounds.get_mut(&optimised_bounds_name).unwrap();
    //         bound_vec.push((row_name, value, bound_type));
    //     }
    //     Ok(())
    // }

    /// Return names of initially unfeasible rows and bounds, that are caused by negative right hands
    /// (unfeasible_row_names, unfeasible_bound_names)
    fn get_mps_model_contain_initially_unfeasible_rhs_rows(&self) -> (Vec<String>, Vec<String>) {
        let res = (Vec::new(), Vec::new());
        // for (row_name, constraint) in &self.rows.rows {
        //     if *constraint == Constraints::N {
        //         continue;
        //     }
        // }
        res
    }


    /// Obtain and optimise bounds obtained from the model. Return empty vec if none are selected
    /// Return Error explaining why, if that is not possible
    pub fn get_optimised_bounds_from_model(self: &MpsModelWithSelectedVariants) -> Result<IndexMap<(String, BoundType), Rational>, Box<SimplexError>> {
        let Some(selected_bounds) = self.get_selected_bounds_from_the_model()? else {
            return Ok(IndexMap::new());
        };
        //Variable_name, bound_type, Rational
        let mut variable_bounds: IndexMap<(String, BoundType), Rational> = IndexMap::new();
        for (variable_name, value, bound_type) in selected_bounds {
            //todo optimise these heap copies
            let found_val = variable_bounds.get(&(variable_name.to_owned(), bound_type.to_owned()));
            if let Some(current_bound_value) = found_val {
                let current_bound_value = found_val.unwrap();
                match bound_type {
                    BoundType::UP => {
                        if value < current_bound_value {
                            variable_bounds.insert((variable_name.to_owned(), BoundType::UP), value.to_owned());
                        }
                    },
                    BoundType::LO => {
                        if value > current_bound_value {
                            variable_bounds.insert((variable_name.to_owned(), BoundType::LO), value.to_owned());
                        }
                    }
                }
            } else {
                variable_bounds.insert((variable_name.to_owned(), bound_type.to_owned()), value.to_owned());
            }

        }
        Ok(variable_bounds)
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
    use super::super::mps::test_utils::create_simple_mps_model_for_test_multiple_bounds_multiple_rhs_multiple_objectives;
    use crate::parsers::mps::{Constraints, MpsModelWithSelectedVariants};
    use crate::parsers::mps_with_selected_variants_operations::is_variable_name_legal;
    use crate::solvers::basic_simplex_table_data::OptimizationType;

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
        let res = mps_model_with_selected_variants.verify_mps_model();
        res.expect("Test failed.");
        assert!(mps_model_with_selected_variants.verify_mps_model().is_ok());
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

}