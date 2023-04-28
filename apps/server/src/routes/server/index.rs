use crate::{
    errors::conflict::BasicResponseError, state::AppState, ws::handlers::server::ServersData,
};

use axum::{
    debug_handler,
    extract::State,
    http::{HeaderMap, Response, StatusCode},
};

use mayhem_db::{
    models::{EServer, EUser},
    sea_orm::{DbConn, EntityTrait, ModelTrait},
};

#[debug_handler]
pub async fn index(State(state): State<AppState>, headers: HeaderMap) -> Response<String> {
    let token_header = headers.get("Authorization");

    if let Some(token) = token_header {
        let token_str = token
            .to_str()
            .unwrap()
            .replace("Bearer", "")
            .trim()
            .to_string();

        let client = &state.client;
        let user_res = client.query.user.find_user_by_token(token_str).await;

        if let Err(err) = &user_res {
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

        let user_opt = user_res.unwrap();

        if let Some(user) = user_opt {
            let user = EUser::find_by_id(user.id)
                .one(&client.client as &DbConn)
                .await
                .unwrap()
                .unwrap();

            let server_res = user
                .find_related(EServer)
                .all(&client.client as &DbConn)
                .await;

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

            let servers = server_res.unwrap();

            let data_struct = ServersData {
                user_id: user.id,
                servers,
            };

            let data = serde_json::to_string(&data_struct).unwrap();

            return Response::new(data);
        } else {
            let mut response = Response::new(
                serde_json::to_string(&BasicResponseError {
                    code: 500,
                    message: "Could not get the user from the database!".to_string(),
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
