use axum::{debug_handler, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: i64,
    pub message: String,
}

#[debug_handler]
pub async fn not_found() -> Json<ErrorInfo> {
    return Json(ErrorInfo {
        code: 404,
        message: "Route not found!".to_string(),
    });
}
