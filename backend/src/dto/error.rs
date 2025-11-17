use serde::Serialize;

/// General error that is returned in case, the server runs into panic
#[derive(Serialize)]
pub struct GeneralPanicError {
    pub error: String,
    pub message: String,
    pub request_id: Option<String>,
}