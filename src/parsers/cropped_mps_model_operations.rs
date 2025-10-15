use std::error::Error;
use crate::document::html_convertible_error::HtmlConvertibleError;
use crate::utils::ApplicationError;
use super::mps::Constraints;
use super::mps::CroppedMpsModel;

impl CroppedMpsModel {

    ///Convert initially unfeasible rows and bounds such as
    /// x1 >= -5
    /// x1 <= -5
    /// BND UP -5
    /// BND LO -5
    /// by inverting them to feasible starters.
    fn convert_initially_unfeasible_rhs_constraints_and_bounds(&mut self) -> Result<(), Box<dyn HtmlConvertibleError>> {

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
        let mut bounds_to_invert: Vec<(usize)> = Vec::new();
        if let Some((_, model_bounds)) = self.model.bounds.bounds.first() {
            for (index, (_, value, _)) in model_bounds.iter().enumerate() {
                if value.is_negative() {
                    bounds_to_invert.push((index));
                }
            }
        } else {
            return Err(Box::new(ApplicationError::with_reason("None BOUNDS were found in cropped MPS model.")));
        }

        for i in bounds_to_invert {
            if let Some(model_bounds) = &mut self.model.bounds.bounds.first_mut() {
                //Unwrap is safe, since we checked for one bounds above
                model_bounds.1[i].2.invert_mut();
                model_bounds.1[i].1.negate_mut();
            } else {
                return Err(Box::new(ApplicationError::with_reason("None BOUNDS were found in cropped MPS model.")));
            }
        }

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use crate::parsers::mps::{BoundType, Constraints};
    use crate::rationals::Rational;
    use super::super::mps::test_utils::create_cropped_mps_model_with_initially_unfeasible_rhs_and_bounds;

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
}