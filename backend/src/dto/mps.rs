use serde::{Deserialize, Serialize};
use crate::parsers::ParserError;
use crate::solvers::basic_simplex_table_data::OptimizationType;

/// Incoming MPS to rest endpoint
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MpsInput {
    pub data: String,
    pub optimization_type: OptimizationType
}

#[derive(Serialize, Clone, Debug)]
pub enum MpsVerificationStatus {
    CORRECT,
    INCORRECT
}

#[derive(Serialize, Debug)]
pub struct MpsVerificationResult {
    pub status: MpsVerificationStatus,
    pub error: Option<ParserError>
}