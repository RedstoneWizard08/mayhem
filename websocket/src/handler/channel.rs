use mayhem_db::{
    models::{server::channel, Channel, Server},
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
    client: &Client,
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

    if let Some(sender) = &client.sender {
        sender.send(Ok(Message::text(data))).unwrap();
    } else {
        warn(
            format!(
                "Unable to send data for new channel {} to client {}!",
                channel.id, client.client_id
            )
            .as_str(),
        );

        return;
    }
}

pub async fn on_get_channel(
    server_id: i32,
    channel_id: i32,
    db: &DatabaseConnection,
    client: &Client,
) {
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
        let channel_res = server
            .find_related(Channel)
            .filter(channel::Column::Id.eq(channel_id))
            .one(db)
            .await;

        if let Err(err) = &channel_res {
            if let Some(sender) = &client.sender {
                sender.send(Ok(Message::text(err.to_string()))).unwrap();
            } else {
                warn(format!("Unable to send error log to client {}!", client_id).as_str());
            }

            return;
        }

        let channel_opt = channel_res.unwrap();

        if let Some(channel) = channel_opt {
            let data_struct = ActiveMessage {
                action: ActiveMessageAction::GetChannelInfo,
                data: channel,
            };

            let data = serde_json::to_string(&data_struct).unwrap();

            if let Some(sender) = &client.sender {
                sender.send(Ok(Message::text(data))).unwrap();
            } else {
                warn(
                    format!(
                        "Unable to send channel info for {} to client {}!",
                        channel_id, client_id
                    )
                    .as_str(),
                );

                return;
            }
        } else {
            if let Some(sender) = &client.sender {
                sender
                    .send(Ok(Message::text(
                        "Could not get the channel from the database!",
                    )))
                    .unwrap();
            } else {
                warn(format!("Unable to send error log to client {}!", client_id).as_str());
            }

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

pub async fn on_get_channels(server_id: i32, db: &DatabaseConnection, client: &Client) {
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
        let channel_res = server.find_related(Channel).all(db).await;

        if let Err(err) = &channel_res {
            if let Some(sender) = &client.sender {
                sender.send(Ok(Message::text(err.to_string()))).unwrap();
            } else {
                warn(format!("Unable to send error log to client {}!", client_id).as_str());
            }

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

        if let Some(sender) = &client.sender {
            sender.send(Ok(Message::text(data))).unwrap();
        } else {
            warn(
                format!(
                    "Unable to send channels for server {} to client {}!",
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
