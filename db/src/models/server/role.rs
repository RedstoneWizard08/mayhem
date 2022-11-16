use async_trait::async_trait;
use rocket::serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, Row, Statement};

use crate::traits::access::PostgresAccessible;

use super::{
    member::ServerMember,
    permissions::{deserialize_permissions, PermissionAccessType, ServerPermission},
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ServerRole {
    pub id: i32,
    pub name: String,
    pub permissions: Vec<(ServerPermission, PermissionAccessType)>,
    pub members: Vec<ServerMember>,
}

#[async_trait]
impl PostgresAccessible for ServerRole {
    async fn from_postgres(row: Row, client: &Client) -> Self {
        let id = row.get("id");
        let name = row.get("name");
        let permission_strings: Vec<String> = row.get("permissions");
        let member_ids: Vec<i32> = row.get("members");

        let mut members: Vec<ServerMember> = Vec::new();
        let permissions: Vec<(ServerPermission, PermissionAccessType)> =
            deserialize_permissions(permission_strings);

        for member in member_ids {
            members.push(ServerMember::from_id(member, client).await.unwrap());
        }

        return Self {
            id,
            name,
            permissions,
            members,
        };
    }

    async fn update(&self, client: &Client) -> Result<Vec<Row>, Error> {
        let statement_raw = include_str!("../../sql/insert_role.sql");
        let statement_result = client.prepare(statement_raw).await;
        let statement_out: Statement;

        match statement_result {
            Ok(statement) => statement_out = statement,
            Err(err) => return Err(err),
        };

        let mut permissions: Vec<String> = Vec::new();
        let mut members: Vec<i32> = Vec::new();

        for permission in self.permissions.clone() {
            permissions.push(format!("{}:{}", permission.0 as u8, permission.1 as u8));
        }

        for member in self.members.clone() {
            members.push(member.id);
        }

        let query_result = client
            .query(&statement_out, &[&self.name, &permissions, &members])
            .await;

        let query_out: Vec<Row>;

        match query_result {
            Ok(rows) => query_out = rows,
            Err(err) => return Err(err),
        };

        return Ok(query_out);
    }
}

impl ServerRole {
    pub async fn from_id(id: i32, client: &Client) -> Result<ServerRole, Error> {
        let statement_raw = include_str!("../../sql/get_role.sql");
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

        return Ok(ServerRole::from_postgres(query_result, client).await);
    }
}
