use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::EServer,
    sea_orm::{DatabaseConnection, EntityTrait, ModelTrait},
};
use tokio::sync::Mutex;

use crate::{
    util::serde::SerdeDeleteResult,
    ws::handlers::{ActiveMessage, ActiveMessageAction},
};

pub async fn on_delete_server(
    server_id: i32,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let server_res = EServer::find_by_id(server_id).one(db).await;

    if let Err(err) = &server_res {
        wtx.lock()
            .await
            .send(Message::Text(err.to_string()))
            .await
            .unwrap();

        return;
    }

    let server_opt = server_res.unwrap();

    if let Some(server) = server_opt {
        let res = server.delete(db).await;

        if let Ok(res) = res {
            let data_struct = ActiveMessage {
                action: ActiveMessageAction::DeleteServer,
                data: SerdeDeleteResult::from(res),
            };

            let data = serde_json::to_string(&data_struct).unwrap();

            wtx.lock().await.send(Message::Text(data)).await.unwrap();
        } else {
            let err = res.unwrap_err();

            wtx.lock()
                .await
                .send(Message::Text(err.to_string()))
                .await
                .unwrap();
        }
    } else {
        wtx.lock()
            .await
            .send(Message::Text(
                "Could not delete the server from the database!".to_string(),
            ))
            .await
            .unwrap();
    }
}
