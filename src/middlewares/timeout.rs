use axum::{http::StatusCode, BoxError};
use hyper::{Method, Uri};

pub async fn mw_timeout_response(method: Method, uri: Uri, _err: BoxError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("{} request timeout at route: {}", method, uri),
    )
}
