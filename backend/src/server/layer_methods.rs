use crate::dto::MpsVerificationResult;
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use log::error;
use std::any::Any;

/// Custom panic handler
/// Used, so that the server does not crash on panics
pub fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic occurred".to_string()
    };

    error!("Handler panicked: {}", details);

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        details,
    ).into_response()
}