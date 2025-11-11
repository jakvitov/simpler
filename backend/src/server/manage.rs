use axum::http::StatusCode;
use axum::Json;
use crate::dto::HealthResponse;

/// Perform simple response on healthcheck request
pub async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse{
            status: "UP".to_string()
        })
    )
}