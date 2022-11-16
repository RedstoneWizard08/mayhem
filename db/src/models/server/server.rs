use async_trait::async_trait;
use rocket::serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, Row, Statement};

use crate::traits::access::PostgresAccessible;

use super::{channels::ServerChannel, member::ServerMember, role::ServerRole};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ServerData {
    pub id: i32,
    pub name: String,
    pub roles: Vec<ServerRole>,
    pub members: Vec<ServerMember>,
    pub channels: Vec<ServerChannel>,
    pub owner: i32,
}

#[async_trait]
impl PostgresAccessible for ServerData {
    async fn from_postgres(row: Row, client: &Client) -> Self {
        let id = row.get("id");
        let name = row.get("name");
        let role_ids: Vec<i32> = row.get("roles");
        let member_ids: Vec<i32> = row.get("members");
        let channel_ids: Vec<i32> = row.get("channels");
        let owner = row.get("owner");

        let mut roles: Vec<ServerRole> = Vec::new();
        let mut members: Vec<ServerMember> = Vec::new();
        let mut channels: Vec<ServerChannel> = Vec::new();

        for id in role_ids {
            roles.push(ServerRole::from_id(id, client).await.unwrap());
        }

        for id in member_ids {
            members.push(ServerMember::from_id(id, client).await.unwrap());
        }

        for id in channel_ids {
            channels.push(ServerChannel::from_id(id, client).await.unwrap());
        }

        return Self {
            id,
            name,
            roles,
            members,
            channels,
            owner,
        };
    }

    async fn update(&self, client: &Client) -> Result<Vec<Row>, Error> {
        let statement_raw = include_str!("../../sql/insert_server.sql");
        let statement_result = client.prepare(statement_raw).await;
        let statement_out: Statement;

        match statement_result {
            Ok(statement) => statement_out = statement,
            Err(err) => return Err(err),
        };

        let mut roles: Vec<i32> = Vec::new();
        let mut members: Vec<i32> = Vec::new();
        let mut channels: Vec<i32> = Vec::new();

        for role in self.roles.clone() {
            roles.push(role.id);
        }

        for member in self.members.clone() {
            members.push(member.id);
        }

        for channel in self.channels.clone() {
            channels.push(channel.id);
        }

        let query_result = client
            .query(
                &statement_out,
                &[&self.name, &roles, &members, &channels, &self.owner],
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

impl ServerData {
    pub async fn from_id(id: i32, client: &Client) -> Result<ServerData, Error> {
        let statement_raw = include_str!("../../sql/get_server.sql");

        println!("Preparing server statement...");
        let statement_result = client.prepare(&statement_raw).await;
        let statement_out: Statement;

        match statement_result {
            Ok(statement) => statement_out = statement,
            Err(err) => return Err(err),
        };

        println!("Running server query...");
        let row =
            client.query(&statement_out, &[&id])
            .await.unwrap().pop().unwrap();

        return Ok(ServerData::from_postgres(row, &client).await);
    }
}
