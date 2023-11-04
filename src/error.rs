use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

pub struct ErrorResponse {
    pub message: &'static str,
    pub status_code: Option<StatusCode>,
}

pub enum CustomError {
    CustomMessage(ErrorResponse),
    SomethingWentWrong(Option<StatusCode>),
}
impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (message, status_code) = match self {
            Self::SomethingWentWrong(e) => match e {
                Some(statuscode) => ("Something went wrong", statuscode),
                None => ("Something went wrong", StatusCode::INTERNAL_SERVER_ERROR),
            },
            Self::CustomMessage(e) => (
                e.message,
                e.status_code.unwrap_or(StatusCode::EXPECTATION_FAILED),
            ),
        };
        (status_code, Json(json!({"message": message}))).into_response()
    }
}
