use async_trait::async_trait;
use rocket::serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, Row, Statement};

use crate::traits::access::PostgresAccessible;

use super::role::ServerRole;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ServerMember {
    pub id: i32,
    pub name: String,
    pub nick: String,
    pub roles: Vec<ServerRole>,
}

#[async_trait]
impl PostgresAccessible for ServerMember {
    async fn from_postgres(row: Row, client: &Client) -> Self {
        let id = row.get("id");
        let name = row.get("name");
        let nick = row.get("nick");
        let role_ids: Vec<i32> = row.get("roles");

        let mut roles: Vec<ServerRole> = Vec::new();

        for role in role_ids {
            roles.push(ServerRole::from_id(role, client).await.unwrap());
        }

        return ServerMember {
            id,
            name,
            nick,
            roles,
        };
    }

    async fn update(&self, client: &Client) -> Result<Vec<Row>, Error> {
        let statement_raw = include_str!("../../sql/insert_member.sql");
        let statement_result = client.prepare(statement_raw).await;
        let statement_out: Statement;

        match statement_result {
            Ok(statement) => statement_out = statement,
            Err(err) => return Err(err),
        };

        let mut roles: Vec<i32> = Vec::new();

        for role in self.roles.clone() {
            roles.push(role.id);
        }

        let query_result = client
            .query(&statement_out, &[&self.name, &self.nick, &roles])
            .await;

        let query_out: Vec<Row>;

        match query_result {
            Ok(rows) => query_out = rows,
            Err(err) => return Err(err),
        };

        return Ok(query_out);
    }
}

impl ServerMember {
    pub async fn from_id(id: i32, client: &Client) -> Result<ServerMember, Error> {
        let statement_raw = include_str!("../../sql/get_member.sql");
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

        return Ok(ServerMember::from_postgres(query_result, client).await);
    }
}
