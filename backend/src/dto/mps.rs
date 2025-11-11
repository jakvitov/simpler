use serde::{Deserialize, Serialize};
use crate::solvers::basic_simplex_table_data::OptimizationType;

/// Incoming MPS to rest endpoint
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MpsInput {
    data: String,
    optimization_type: OptimizationType
}