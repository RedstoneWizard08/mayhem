use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::{server::channel, EChannel, EServer},
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
        ModelTrait, QueryFilter,
    },
};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use super::{ActiveMessage, ActiveMessageAction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelCreateData {
    pub name: String,
    pub server_id: i32,
    pub channel_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelsData {
    pub server_id: i32,
    pub channels: Vec<channel::Model>,
}

pub async fn on_create_channel(
    creation_data: ChannelCreateData,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let channel_data = channel::ActiveModel {
        name: Set(creation_data.name),
        server_id: Set(creation_data.server_id),
        channel_type: Set(creation_data.channel_type),

        ..Default::default()
    };

    let channel = channel_data.clone().insert(db).await.unwrap();

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::CreateChannel,
        data: channel.clone(),
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    wtx.lock().await.send(Message::Text(data)).await.unwrap();
}

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
