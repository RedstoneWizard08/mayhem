use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use mayhem_db::{
    models::{server::server, user_server, EServer, EUser, EUserServer},
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
        ModelTrait, QueryFilter,
    },
};
use tokio::sync::Mutex;

use super::{ActiveMessage, ActiveMessageAction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerCreateData {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServersData {
    pub user_id: i32,
    pub servers: Vec<server::Model>,
}

pub async fn on_create_server(
    creation_data: ServerCreateData,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let server_data = server::ActiveModel {
        name: Set(creation_data.name),
        ..Default::default()
    };

    let server = server_data.clone().insert(db).await.unwrap();

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::CreateServer,
        data: server.clone(),
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    wtx.lock().await.send(Message::Text(data)).await.unwrap();
}

pub async fn on_get_server(
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
        let data_struct = ActiveMessage {
            action: ActiveMessageAction::GetServerInfo,
            data: server,
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

pub async fn on_join_server(
    user_id: i32,
    server_id: i32,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let user_server_data = user_server::ActiveModel {
        server_id: Set(server_id.clone()),
        user_id: Set(user_id.clone()),

        ..Default::default()
    };

    let user_server = user_server_data.clone().insert(db).await.unwrap();

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::JoinServer,
        data: user_server,
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    wtx.lock().await.send(Message::Text(data)).await.unwrap();
}

pub async fn on_leave_server(
    user_id: i32,
    server_id: i32,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let user_server = EUserServer::find()
        .filter(user_server::Column::ServerId.eq(server_id))
        .filter(user_server::Column::UserId.eq(user_id))
        .one(db)
        .await
        .unwrap()
        .unwrap();

    user_server.delete(db).await.unwrap();

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::LeaveServer,
        data: "success",
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    wtx.lock().await.send(Message::Text(data)).await.unwrap();
}

pub async fn on_get_servers(
    user_id: i32,
    db: &DatabaseConnection,
    wtx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
) {
    let user_res = EUser::find_by_id(user_id).one(db).await;

    if let Err(err) = &user_res {
        wtx.lock()
            .await
            .send(Message::Text(err.to_string()))
            .await
            .unwrap();

        return;
    }

    let user_opt = user_res.unwrap();

    if let Some(user) = user_opt {
        let server_res = user.find_related(EServer).all(db).await;

        if let Err(err) = &server_res {
            wtx.lock()
                .await
                .send(Message::Text(err.to_string()))
                .await
                .unwrap();

            return;
        }

        let servers = server_res.unwrap();

        let data_struct = ActiveMessage {
            action: ActiveMessageAction::GetServersForUser,
            data: ServersData { user_id, servers },
        };

        let data = serde_json::to_string(&data_struct).unwrap();

        wtx.lock().await.send(Message::Text(data)).await.unwrap();
    } else {
        wtx.lock()
            .await
            .send(Message::Text(
                "Could not get the user from the database!".to_string(),
            ))
            .await
            .unwrap();
    }
}
