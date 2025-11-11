use super::mps::{Bounds, Constraints};
use super::mps::CroppedMpsModel;
use crate::document::html_convertible_error::HtmlConvertibleError;
use crate::parsers::mps::BoundType::{LO, UP};
use crate::rationals::Rational;
use crate::utils::ApplicationError;
use std::collections::HashMap;

impl CroppedMpsModel {

    ///Convert initially unfeasible rows and bounds such as
    /// x1 >= -5
    /// x1 <= -5
    /// BND UP -5
    /// BND LO -5
    /// by inverting them to feasible starters.
    pub fn convert_initially_unfeasible_rhs_constraints_and_bounds(&mut self) -> Result<(), Box<ApplicationError>> {

        //Convert ROWS
        let mut rows_to_convert: Vec<String> = Vec::new();
        for (row_name, constraint) in &self.model.rows.rows {
            if *constraint == Constraints::N {
                continue;
            }

            if self.model.rhs.rhs.len() > 1 {
                return Err(Box::new(ApplicationError::with_reason("Cropped MPS model contains more than one RHS.")));
            }
            if let Some((rhs_name, rhs_values)) = self.model.rhs.rhs.first() {
                if let Some(rhs_value_for_row) = rhs_values.get(row_name) {
                    if rhs_value_for_row.is_negative() {
                        rows_to_convert.push(row_name.clone());
                    }
                } else {
                    return Err(Box::new(ApplicationError::from_string_details("No RHS value found for row in cropped MPS model.", format!("RHS: {}, ROW: {}.", rhs_name, row_name))));
                }
            }
            else {
                return Err(Box::new(ApplicationError::with_reason("No RHS found in cropped MPS model.")));
            }
        }

        for row_name in rows_to_convert {
            //Unwrap is safe, since we stored the values from the keys
            self.model.rows.rows.get_mut(&row_name).unwrap().invert_mut();

            if let Some((rhs_name, rhs_values)) = self.model.rhs.rhs.first_mut() {
                if let Some(rhs_value_for_row) = rhs_values.get_mut(&row_name) {
                    rhs_value_for_row.negate_mut();
                } else {
                    return Err(Box::new(ApplicationError::from_string_details("No RHS value found for row in cropped MPS model.", format!("RHS: {}, ROW: {}.", rhs_name, row_name))));
                }
            }
            else {
                return Err(Box::new(ApplicationError::with_reason("No RHS found in cropped MPS model.")));
            }
        }

        // Invert BOUNDS
        if self.model.bounds.bounds.len() > 1 {
            return Err(Box::new(ApplicationError::with_reason("Multiple BOUNDS found in cropped MPS model.")));
        }
        let mut bounds_to_invert: Vec<usize> = Vec::new();
        if let Some((_, model_bounds)) = self.model.bounds.bounds.first() {
            for (index, (_, value, _)) in model_bounds.iter().enumerate() {
                if value.is_negative() {
                    bounds_to_invert.push(index);
                }
            }
        }

        for i in bounds_to_invert {
            if let Some(model_bounds) = &mut self.model.bounds.bounds.first_mut() {
                //Unwrap is safe, since we checked for one bounds above
                model_bounds.1[i].2.invert_mut();
                model_bounds.1[i].1.negate_mut();
            }
        }

        Ok(())
    }
    
    //todo implement optimise cropped model bounds
    ///Optimise models bounds removing the redundant ones
    pub fn optimise_bounds(&mut self) -> Result<(), Box<ApplicationError>> {
        if self.model.bounds.bounds.len() > 1 {
            return Err(Box::new(ApplicationError::with_reason("Multiple BOUNDS found in cropped MPS model.")));
        } else if self.model.bounds.bounds.is_empty() {
            return Ok(());
        }

        // var_name -> UP, LO (var_name -> <=, >=)
        let mut optimal_bounds: HashMap<String, (Option<Rational>, Option<Rational>)> = HashMap::new();
        if let Some((_, model_bounds)) = self.model.bounds.bounds.first() {
            for (variable_name, value, bound_type) in model_bounds {
                if let Some((upper_bound, lower_bound)) = optimal_bounds.get(variable_name)  {
                    if *bound_type == UP {
                        if let Some(bound_value) = *upper_bound{
                            if *value < bound_value {
                                //todo remove these hideous heap copies
                                optimal_bounds.insert(variable_name.to_owned(), (Some(value.to_owned()), *lower_bound));
                            }
                        } else {
                            optimal_bounds.insert(variable_name.to_owned(), (Some(value.to_owned()), *lower_bound));
                        }
                    } else if *bound_type == LO {
                        if let Some(bound_value) = *lower_bound {
                            if *value > bound_value {
                                optimal_bounds.insert(variable_name.to_owned(), (*upper_bound, Some(value.to_owned())));
                            }
                        } else {
                            optimal_bounds.insert(variable_name.to_owned(), (*upper_bound, Some(value.to_owned())));
                        }
                    }
                } else if *bound_type == UP {
                    optimal_bounds.insert(variable_name.to_owned(), (Some(value.to_owned()), None ));
                } else if *bound_type == LO {
                    optimal_bounds.insert(variable_name.to_owned(), (None, Some(value.to_owned())));
                }
            }
        } else {
            return Err(Box::new(ApplicationError::with_reason("None BOUNDS were found in cropped MPS model.")));
        }


        self.model.bounds.bounds[0].retain(|(variable_name, value, bound_type)| {
            if let Some(variable_optimised_bounds) = optimal_bounds.get(variable_name) {
                if *bound_type == UP {
                    if let Some(optimal_upperbound) = variable_optimised_bounds.0 {
                        *value == optimal_upperbound
                    } else {
                        true
                    }
                }
                else if *bound_type == LO {
                    if let Some(optimal_lowerbound) = variable_optimised_bounds.1 {
                        *value == optimal_lowerbound
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            else {
                true
            }
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::mps::test_utils::create_cropped_mps_model_with_initially_unfeasible_rhs_and_bounds;
    use crate::parsers::mps::{BoundType, Constraints};
    use crate::parsers::mps::test_utils::create_rich_cropped_mps_model_for_test;
    use crate::rationals::Rational;

    #[test]
    fn convert_initially_unfeasible_rhs_constraints_and_bounds_succeeds() {
        let mut model = create_cropped_mps_model_with_initially_unfeasible_rhs_and_bounds();
        dbg!(model.model.rows.rows.get("ROW1").unwrap());
        dbg!(*model.model.rhs.rhs.get("RHS1").unwrap().get("ROW1").unwrap());

        model.convert_initially_unfeasible_rhs_constraints_and_bounds().expect("Test failed");
        dbg!(model.model.rows.rows.get("ROW1").unwrap());
        dbg!(*model.model.rhs.rhs.get("RHS1").unwrap().get("ROW1").unwrap());
        assert_eq!(*model.model.rows.rows.get("ROW1").unwrap(), Constraints::G);
        assert_eq!(*model.model.rhs.rhs.get("RHS1").unwrap().get("ROW1").unwrap(), Rational::from_integer(6));

        assert_eq!(model.model.bounds.bounds.get("BND1").unwrap()[0].2, BoundType::LO);
        assert_eq!(model.model.bounds.bounds.get("BND1").unwrap()[0].1, Rational::from_integer(20));
    }

    #[test]
    fn optimise_bounds_succeeds() {
        let mut model = create_rich_cropped_mps_model_for_test();
        model.optimise_bounds().expect("Test failed");
        /// x1 <= 10    BND1
        /// x2 ≥ 10     BND1
        /// x2 <= 20    BND1
        assert_eq!(model.model.bounds.bounds.first().unwrap().1.len(), 3);

        assert_eq!(model.model.bounds.bounds.first().unwrap().1[0].1, Rational::from_integer(10));
        assert_eq!(model.model.bounds.bounds.first().unwrap().1[1].1, Rational::from_integer(10));
        assert_eq!(model.model.bounds.bounds.first().unwrap().1[2].1, Rational::from_integer(20));
    }

    #[test]
    fn optimise_bounds_succeeds_for_model_without_bounds() {
        let mut model = create_rich_cropped_mps_model_for_test();
        model.model.bounds.bounds.clear();
        model.optimise_bounds().expect("Test failed");
    }
}