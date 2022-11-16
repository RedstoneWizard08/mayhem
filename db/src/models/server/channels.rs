use async_trait::async_trait;
use num_derive::FromPrimitive;
use rocket::serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, Row, Statement};

use crate::traits::access::PostgresAccessible;

use super::{permissions::PermissionAccessType, role::ServerRole};

pub type ChannelPermissions = Vec<(ChannelPermission, Vec<(ServerRole, PermissionAccessType)>)>;

pub type ChannelUserPermissions = Vec<(ChannelPermission, Vec<(i32, PermissionAccessType)>)>;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ServerChannel {
    pub id: i32,
    pub name: String,
    pub role_permissions: Vec<(ServerRole, PermissionAccessType)>,
    pub channel_permissions: ChannelPermissions,
    pub channel_user_permissions: ChannelUserPermissions,
    pub server: i32,
}

#[async_trait]
impl PostgresAccessible for ServerChannel {
    async fn from_postgres(row: Row, client: &Client) -> Self {
        let id = row.get("id");
        let name = row.get("name");
        let role_permission_strings: Vec<String> = row.get("role_permissions");
        let channel_permission_strings: Vec<String> = row.get("channel_permissions");
        let channel_user_permission_strings: Vec<String> = row.get("channel_user_permissions");
        let server: i32 = row.get("server");

        let role_permissions =
            ServerChannel::deserialize_role_permissions(role_permission_strings, client).await;
        
        let channel_permissions =
            ServerChannel::deserialize_channel_permissions(channel_permission_strings, &client).await;
        
        let channel_user_permissions: ChannelUserPermissions =
            ServerChannel::deserialize_channel_user_permissions(channel_user_permission_strings);

        return Self {
            id,
            name,
            role_permissions,
            channel_permissions,
            channel_user_permissions,
            server,
        };
    }

    async fn update(&self, client: &Client) -> Result<Vec<Row>, Error> {
        let statement_raw = include_str!("../../sql/insert_channel.sql");
        let statement_result = client.prepare(statement_raw).await;
        let statement_out: Statement;

        match statement_result {
            Ok(statement) => statement_out = statement,
            Err(err) => return Err(err),
        };

        let mut role_permissions: Vec<String> = Vec::new();
        let mut channel_permissions: Vec<String> = Vec::new();
        let mut channel_user_permissions: Vec<String> = Vec::new();

        for role in self.role_permissions.clone() {
            role_permissions.push(format!("{}:{}", role.0.id, role.1 as u8));
        }

        for member in self.channel_permissions.clone() {
            for perm in member.1 {
                channel_permissions
                    .push(format!("{}:{}:{}", member.0 as u8, perm.0.id, perm.1 as u8));
            }
        }

        for channel in self.channel_user_permissions.clone() {
            for perm in channel.1 {
                channel_user_permissions
                    .push(format!("{}:{}:{}", channel.0 as u8, perm.0, perm.1 as u8));
            }
        }

        let query_result = client
            .query(
                &statement_out,
                &[
                    &self.name,
                    &role_permissions,
                    &channel_permissions,
                    &channel_user_permissions,
                    &self.server,
                ],
            )
            .await;

        let query_out: Vec<Row>;

        match query_result {
            Ok(rows) => query_out = rows,
            Err(err) => return Err(err),
        };

        return Ok(query_out);
    }
}

impl ServerChannel {
    pub fn deserialize_role_permission(text: String) -> (i32, PermissionAccessType) {
        let mut split = text.split(":");

        let id = split.nth(0).unwrap().parse::<i32>().unwrap();
        let access = split.nth(1).unwrap().parse::<u32>().unwrap();
        let access_type: PermissionAccessType = num::FromPrimitive::from_u32(access).unwrap();

        return (id, access_type);
    }

    pub fn deserialize_channel_permission(
        text: String,
    ) -> (ChannelPermission, i32, PermissionAccessType) {
        let mut split = text.split(":");

        let permission_id = split.nth(0).unwrap().parse::<u32>().unwrap();
        let id = split.nth(1).unwrap().parse::<i32>().unwrap();
        let access = split.nth(2).unwrap().parse::<u32>().unwrap();

        let permission: ChannelPermission = num::FromPrimitive::from_u32(permission_id).unwrap();
        let access_type: PermissionAccessType = num::FromPrimitive::from_u32(access).unwrap();

        return (permission, id, access_type);
    }

    pub async fn deserialize_channel_permissions(
        strings: Vec<String>,
        client: &Client,
    ) -> ChannelPermissions {
        let mut vec: ChannelPermissions = Vec::new();

        for string in strings {
            let (perm, id, access) = ServerChannel::deserialize_channel_permission(string);

            let clone_1 = vec.clone();
            let found = clone_1.iter().find(|val| val.0 == perm);

            match found {
                Some(val) => {
                    let mut vec_val = val.1.clone();
                    vec_val.push((ServerRole::from_id(id, &client).await.unwrap(), access));

                    let index = vec.clone().iter().position(|r| r.0 == val.0).unwrap();
                    vec[index] = (val.0.clone(), vec_val);
                }

                None => vec.push((
                    perm,
                    vec![(ServerRole::from_id(id, &client).await.unwrap(), access)],
                )),
            };
        }

        return vec;
    }

    pub fn deserialize_channel_user_permissions(
        strings: Vec<String>,
    ) -> ChannelUserPermissions {
        let mut vec: ChannelUserPermissions = Vec::new();

        for string in strings {
            let (perm, id, access) = ServerChannel::deserialize_channel_permission(string);

            let clone_1 = vec.clone();
            let found = clone_1.iter().find(|val| val.0 == perm);

            match found {
                Some(val) => {
                    let mut vec_val = val.1.clone();
                    vec_val.push((id, access));

                    let index = vec.clone().iter().position(|r| r.0 == val.0).unwrap();
                    vec[index] = (val.0.clone(), vec_val);
                }

                None => vec.push((
                    perm,
                    vec![(id, access)],
                )),
            };
        }

        return vec;
    }

    pub async fn from_id(id: i32, client: &Client) -> Result<ServerChannel, Error> {
        let statement_raw = include_str!("../../sql/get_channel.sql");
        let statement_result = client.prepare(statement_raw).await;
        let statement_out: Statement;

        match statement_result {
            Ok(statement) => statement_out = statement,
            Err(err) => return Err(err),
        };

        let query_result = client
            .query(&statement_out, &[&id])
            .await
            .unwrap()
            .pop()
            .unwrap();

        return Ok(ServerChannel::from_postgres(query_result, client).await);
    }

    pub async fn deserialize_role_permissions(strings: Vec<String>, client: &Client) -> Vec<(ServerRole, PermissionAccessType)> {
        let mut vec: Vec<(ServerRole, PermissionAccessType)> = Vec::new();
        
        for item in strings {
            let mut split = item.split(":");

            let role_id = split.nth(0).unwrap().parse::<i32>().unwrap();
            let access = split.nth(1).unwrap().parse::<u32>().unwrap();

            let role = ServerRole::from_id(role_id, client).await.unwrap();
            let access_type: PermissionAccessType = num::FromPrimitive::from_u32(access).unwrap();

            vec.push((role, access_type));
        }

        return vec;
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, FromPrimitive, PartialEq, Copy)]
#[serde(crate = "rocket::serde")]
pub enum ChannelPermission {
    SendMessage,
    ReadMessage,
    EditMessage,
    DeleteMessage,

    LoadOldMessages,
    AtMention,
    AtEveryone,

    JoinVoice,
    VoiceUnmute,
    VoiceUndeafen,

    DeleteOtherMessage,
    UseCommands,
}
