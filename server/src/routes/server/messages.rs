use crate::{errors::conflict::BasicResponseError, state::AppState};
use axum::{
    debug_handler,
    extract::{Path, State},
    http::{HeaderMap, Response, StatusCode},
};
use mayhem_db::{
    models::{ChatMessage, EChannel, EChatMessage},
    sea_orm::{DatabaseConnection, EntityTrait, ModelTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessagesData {
    pub channel_id: i32,
    pub messages: Vec<ChatMessage>,
}

#[debug_handler]
pub async fn messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((_, channel_id)): Path<(i32, i32)>,
) -> Response<String> {
    let token_header = headers.get("Authorization");

    if let Some(token) = token_header {
        let _token_str = token.to_str().unwrap().to_string();

        let client = &state.client;
        let db: &DatabaseConnection = &client.client.clone();

        let channel_res = EChannel::find_by_id(channel_id).one(db).await;

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

        let channel_opt = channel_res.unwrap();

        if let Some(channel) = channel_opt {
            let messages_res = channel.find_related(EChatMessage).all(db).await;

            if let Err(err) = &messages_res {
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

            let messages = messages_res.unwrap();
            let messages = messages.iter().rev().take(20).rev().cloned().collect();

            let data_struct = MessagesData {
                channel_id,
                messages,
            };

            let data = serde_json::to_string(&data_struct).unwrap();

            return Response::new(data);
        } else {
            let mut response = Response::new(
                serde_json::to_string(&BasicResponseError {
                    code: 500,
                    message: "Could not get the channel from the database!".to_string(),
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
