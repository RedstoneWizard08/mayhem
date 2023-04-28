use axum::{debug_handler, http::Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: i64,
    pub message: String,
}

#[debug_handler]
pub async fn not_found() -> Response<String> {
    let text = serde_json::to_string(&ErrorInfo {
        code: 404,
        message: "Route not found!".to_string(),
    })
    .unwrap();

    let mut response = Response::new(text);

    let s = response.status_mut();
    *s = StatusCode::NOT_FOUND;

    return response;
}
