use crate::{
    errors::conflict::BasicResponseError, state::AppState, ws::handlers::channel::ChannelsData,
};
use axum::{
    debug_handler,
    extract::{Path, State},
    http::{HeaderMap, Response, StatusCode},
};
use mayhem_db::{
    models::{EChannel, EServer},
    sea_orm::{DatabaseConnection, EntityTrait, ModelTrait},
};

#[debug_handler]
pub async fn channels(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(server_id): Path<i32>,
) -> Response<String> {
    let token_header = headers.get("Authorization");

    if let Some(token) = token_header {
        let _token_str = token.to_str().unwrap().to_string();

        let client = &state.client;
        let db: &DatabaseConnection = &client.client.clone();

        let server_res = EServer::find_by_id(server_id).one(db).await;

        if let Err(err) = &server_res {
            let mut response = Response::new(
                serde_json::to_string(&BasicResponseError {
                    code: 500,
                    message: err.to_string(),
                })
                .unwrap(),
            );

            let s = response.status_mut();
            *s = StatusCode::from_u16(500).unwrap();

            return response;
        }

        let server_opt = server_res.unwrap();

        if let Some(server) = server_opt {
            let channel_res = server.find_related(EChannel).all(db).await;

            if let Err(err) = &channel_res {
                let mut response = Response::new(
                    serde_json::to_string(&BasicResponseError {
                        code: 500,
                        message: err.to_string(),
                    })
                    .unwrap(),
                );

                let s = response.status_mut();
                *s = StatusCode::from_u16(500).unwrap();

                return response;
            }

            let channels = channel_res.unwrap();

            let data_struct = ChannelsData {
                server_id,
                channels,
            };

            let data = serde_json::to_string(&data_struct).unwrap();

            return Response::new(data);
        } else {
            let mut response = Response::new(
                serde_json::to_string(&BasicResponseError {
                    code: 500,
                    message: "Could not get the server from the database!".to_string(),
                })
                .unwrap(),
            );

            let s = response.status_mut();
            *s = StatusCode::from_u16(500).unwrap();

            return response;
        }
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
