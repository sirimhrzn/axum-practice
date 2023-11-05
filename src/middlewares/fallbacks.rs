use hyper::{StatusCode, Uri};

pub async fn fallback_root_api(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Route {} not found", uri))
}
