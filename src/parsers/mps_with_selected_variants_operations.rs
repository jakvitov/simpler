use indexmap::IndexMap;
use crate::rationals::Rational;
use crate::solvers::simplex_error::SimplexError;
use super::mps::{BoundType, Constraints, MpsModelWithSelectedVariants};
use super::mps::MpsModel;

impl MpsModelWithSelectedVariants {

    /// Given MPS model, convert initially unfeasible constrains, such as:
    /// x1 <= -5 -> x1 >= 5
    /// x1 >= -10 -> x1 <= 10
    /// remove all the unselected RHS and BOUNDS (conflicts can occur after optimisation)
    fn convert_initially_unfeasible_rhs_constraints_and_bounds(&mut self) {

    }

    /// Return names of initially unfeasible rows and bounds, that are caused by negative right hands
    /// (unfeasible_row_names, unfeasible_bound_names)
    fn get_mps_model_contain_initially_unfeasible_rhs_rows(&self) -> (Vec<String>, Vec<String>) {
        // let rhs = 
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