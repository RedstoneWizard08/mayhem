use crate::errors::conflict::BasicResponseError;
use axum::{
    debug_handler,
    http::{HeaderMap, Response, StatusCode},
};

#[debug_handler]
pub async fn index(headers: HeaderMap) -> Response<String> {
    let token_header = headers.get("Authorization");

    if let Some(token) = token_header {
        let token_str = token.to_str().unwrap().to_string();

        return Response::new(token_str);
    }

    let mut response = Response::new(
        serde_json::to_string(&BasicResponseError {
            code: 400,
            message: "Missing authorization header!".to_string(),
        })
        .unwrap(),
    );

    let s = response.status_mut();
    *s = StatusCode::BAD_REQUEST;

    return response;
}
