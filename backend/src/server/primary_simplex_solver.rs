use axum::http::StatusCode;
use axum::Json;
use crate::dto::{MpsInput};

/// Perform simple response on healthcheck request
pub async fn solve_primary_simplex(Json(mps_input) : Json<MpsInput>) -> (StatusCode, Json<MpsInput>) {
    (
        StatusCode::OK,
        Json(mps_input)
    )
}