use crate::dto::{MpsInput, MpsVerificationResult, MpsVerificationStatus};
use crate::parsers::parse_mps;
use axum::http::StatusCode;
use axum::Json;

/// Verify msp model endpoint function. Model is parsed and optional error returned
pub async fn verify_mps_model(Json(mps_input) : Json<MpsInput>) -> (StatusCode, Json<MpsVerificationResult>) {

    let parsed_mps = tokio::task::spawn_blocking(move || {
        parse_mps(&mps_input.data)
    }).await.unwrap();
    
    match parsed_mps {
        Ok(_) => {
            let res = MpsVerificationResult {
                status: MpsVerificationStatus::CORRECT,
                error: None
            };
            (StatusCode::OK, Json(res))
        },
        Err(error) => {
            let res = MpsVerificationResult {
                status: MpsVerificationStatus::INCORRECT,
                error:  Some(*error)
            };
            (StatusCode::OK, Json(res))
        }
    }
}