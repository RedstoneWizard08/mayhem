use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::{server::channel, EChannel, EServer},
    sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter},
};
use tokio::sync::Mutex;

use crate::ws::handlers::{ActiveMessage, ActiveMessageAction};

pub async fn on_get_channel(
    server_id: i32,
    channel_id: i32,
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
        let channel_res = server
            .find_related(EChannel)
            .filter(channel::Column::Id.eq(channel_id))
            .one(db)
            .await;

        if let Err(err) = &channel_res {
            wtx.lock()
                .await
                .send(Message::Text(err.to_string()))
                .await
                .unwrap();

            return;
        }

        let channel_opt = channel_res.unwrap();

        if let Some(channel) = channel_opt {
            let data_struct = ActiveMessage {
                action: ActiveMessageAction::GetChannelInfo,
                data: channel,
            };

            let data = serde_json::to_string(&data_struct).unwrap();

            wtx.lock().await.send(Message::Text(data)).await.unwrap();
        } else {
            wtx.lock()
                .await
                .send(Message::Text(
                    "Could not get the channel from the database!".to_string(),
                ))
                .await
                .unwrap();
        }
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
