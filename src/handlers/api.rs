use std::time::Duration;

use axum::{response::IntoResponse, Json};
use serde_derive::Deserialize;
use tokio::time::sleep;

#[derive(Deserialize)]
pub struct LoginPayload {
    _username: String,
    _password: String,
}

// To test if middleware is catching error and putting message in a json with message field
pub async fn login(Json(_body): Json<LoginPayload>) -> impl IntoResponse {
    "Login successfull".into_response()
}

// To test if the middleware handles timeout request
pub async fn timeout_handler() -> impl IntoResponse {
    sleep(Duration::from_secs(5)).await;
    "No timeout".to_string()
}
