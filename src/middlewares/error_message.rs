use crate::error::CustomError;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub async fn mw_error_response<T>(request: Request<T>, next: Next<T>) -> impl IntoResponse {
    let response = next.run(request).await;
    let (parts, body) = response.into_parts();
    if parts.status != StatusCode::OK {
        let body = match hyper::body::to_bytes(body).await {
            Ok(value) => value,
            Err(_e) => {
                return CustomError::SomethingWentWrong(None).into_response();
            }
        };
        let stringified_body = match std::str::from_utf8(&body) {
            Ok(value) => {
                if value.is_empty() {
                    return CustomError::CustomMessage(crate::error::ErrorResponse {
                        message: parts
                            .status
                            .canonical_reason()
                            .unwrap_or("Something went wrong"),
                        status_code: Some(parts.status),
                    })
                    .into_response();
                }
                value
            }
            Err(_e) => {
                return CustomError::SomethingWentWrong(None).into_response();
            }
        };
        let message = json!(
            {
                "message": stringified_body
            }
        );
        return (parts.status, Json(message)).into_response();
    }
    Response::from_parts(parts, body)
}
