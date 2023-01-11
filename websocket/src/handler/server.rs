use mayhem_db::{
    models::{server::server, user_server, Server, UserServer},
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
        ModelTrait, QueryFilter,
    },
};
use serde::{Deserialize, Serialize};
use warp::ws::Message;

use crate::{logging::warn, Client};

use super::{ActiveMessage, ActiveMessageAction};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerCreateData {
    pub name: String,
}

pub async fn on_create_server(
    creation_data: ServerCreateData,
    db: &DatabaseConnection,
    client: &Client,
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

    if let Some(sender) = &client.sender {
        sender.send(Ok(Message::text(data))).unwrap();
    } else {
        warn(
            format!(
                "Unable to send data for new server {} to client {}!",
                server.id, client.client_id
            )
            .as_str(),
        );

        return;
    }
}

pub async fn on_get_server(server_id: i32, db: &DatabaseConnection, client: &Client) {
    let client_id = client.clone().client_id;
    let server_res = Server::find_by_id(server_id).one(db).await;

    if let Err(err) = &server_res {
        if let Some(sender) = &client.sender {
            sender.send(Ok(Message::text(err.to_string()))).unwrap();
        } else {
            warn(format!("Unable to send error log to client {}!", client_id).as_str());
        }

        return;
    }

    let server_opt = server_res.unwrap();

    if let Some(server) = server_opt {
        let data_struct = ActiveMessage {
            action: ActiveMessageAction::GetServerInfo,
            data: server,
        };

        let data = serde_json::to_string(&data_struct).unwrap();

        if let Some(sender) = &client.sender {
            sender.send(Ok(Message::text(data))).unwrap();
        } else {
            warn(
                format!(
                    "Unable to send info for server {} to client {}!",
                    server_id, client_id
                )
                .as_str(),
            );

            return;
        }
    } else {
        if let Some(sender) = &client.sender {
            sender
                .send(Ok(Message::text(
                    "Could not get the server from the database!",
                )))
                .unwrap();
        } else {
            warn(format!("Unable to send error log to client {}!", client_id).as_str());
        }

        return;
    }
}

pub async fn on_join_server(
    user_id: i32,
    server_id: i32,
    db: &DatabaseConnection,
    client: &Client,
) {
    let user_server_data = user_server::ActiveModel {
        server_id: Set(server_id.clone()),
        user_id: Set(user_id.clone()),

        ..Default::default()
    };

    let client_id = client.clone().client_id;
    let user_server = user_server_data.clone().insert(db).await.unwrap();

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::JoinServer,
        data: user_server,
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    if let Some(sender) = &client.sender {
        sender.send(Ok(Message::text(data))).unwrap();
    } else {
        warn(
            format!(
                "Unable to send info for server join (server {}, user {}) to client {}!",
                server_id, user_id, client_id
            )
            .as_str(),
        );

        return;
    }
}

pub async fn on_leave_server(
    user_id: i32,
    server_id: i32,
    db: &DatabaseConnection,
    client: &Client,
) {
    let user_server = UserServer::find()
        .filter(user_server::Column::ServerId.eq(server_id))
        .filter(user_server::Column::UserId.eq(user_id))
        .one(db)
        .await
        .unwrap()
        .unwrap();

    user_server.delete(db).await.unwrap();

    let client_id = client.clone().client_id;

    let data_struct = ActiveMessage {
        action: ActiveMessageAction::LeaveServer,
        data: "success",
    };

    let data = serde_json::to_string(&data_struct).unwrap();

    if let Some(sender) = &client.sender {
        sender.send(Ok(Message::text(data))).unwrap();
    } else {
        warn(
            format!(
                "Unable to send info for server leave (server {}, user {}) to client {}!",
                server_id, user_id, client_id
            )
            .as_str(),
        );

        return;
    }
}
