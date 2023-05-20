use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::{EChannel, EServer},
    sea_orm::{DatabaseConnection, EntityTrait, ModelTrait},
};
use tokio::sync::Mutex;

use crate::ws::handlers::{ActiveMessage, ActiveMessageAction};

use super::ChannelsData;

pub async fn on_get_channels(
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
        let channel_res = server.find_related(EChannel).all(db).await;

        if let Err(err) = &channel_res {
            wtx.lock()
                .await
                .send(Message::Text(err.to_string()))
                .await
                .unwrap();

            return;
        }

        let channels = channel_res.unwrap();

        let data_struct = ActiveMessage {
            action: ActiveMessageAction::GetChannelInfo,
            data: ChannelsData {
                server_id,
                channels,
            },
        };

        let data = serde_json::to_string(&data_struct).unwrap();

        wtx.lock().await.send(Message::Text(data)).await.unwrap();
    } else {
        wtx.lock()
            .await
            .send(Message::Text(
                "Could not get the server from the database!".to_string(),
            ))
            .await
            .unwrap();
    }
}
