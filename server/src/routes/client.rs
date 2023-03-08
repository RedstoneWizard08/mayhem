use axum::{
    debug_handler,
    extract::State,
    http::{uri::Uri, Request, Response},
};

use hyper::Body;

use crate::state::AppState;

#[debug_handler]
pub async fn handle_client_proxy(
    State(state): State<AppState>,
    mut req: Request<Body>,
) -> Response<Body> {
    let client_port = state.config.clone().port + 1;
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("http://{}:{}{}", state.config.host, client_port, path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    return state.request_client.request(req).await.unwrap();
}
