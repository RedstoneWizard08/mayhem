use async_trait::async_trait;
use rocket::serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error, Row, Statement};

use crate::traits::access::PostgresAccessible;

use super::{server::server::ServerData, user_settings::UserSettings};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub servers: Vec<ServerData>,
    pub settings: UserSettings,
}

impl User {
    pub async fn from_postgres_ref(row: &Row, client: &Client) -> Result<Self, Error> {
        println!("Getting props...");
        let id = row.get("id");
        let first_name = row.get("first_name");
        let last_name = row.get("last_name");
        let email = row.get("email");
        let username = row.get("username");
        let password = row.get("password");
        let server_ids: Vec<i32> = row.get("servers");
        let settings_id = row.get("settings");

        println!("Getting servers...");
        let servers_result = User::get_servers(client, server_ids).await;

        println!("Getting settings...");
        let settings_result = User::get_settings(client, settings_id).await;

        let servers: Vec<ServerData>;
        let settings: UserSettings;

        match servers_result {
            Ok(val) => servers = val,
            Err(err) => return Err(err),
        };

        match settings_result {
            Ok(val) => settings = val,
            Err(err) => return Err(err),
        };

        return Ok(Self {
            id,
            first_name,
            last_name,
            email,
            username,
            password,
            servers,
            settings,
        });
    }

    async fn get_servers(client: &Client, server_ids: Vec<i32>) -> Result<Vec<ServerData>, Error> {
        let mut servers: Vec<ServerData> = Vec::new();

        for id in server_ids {
            let result = ServerData::from_id(id, &client).await;

            match result {
                Ok(val) => servers.push(val),
                Err(err) => return Err(err),
            };
        }

        return Ok(servers);
    }

    async fn get_settings(client: &Client, id: i32) -> Result<UserSettings, Error> {
        let statement_raw = include_str!("../sql/get_settings.sql");

        println!("Preparing settings statement...");
        let statement_result = client.prepare(statement_raw).await;
        let statement_out: Statement;

        match statement_result {
            Ok(statement) => statement_out = statement,
            Err(err) => return Err(err),
        };

        println!("Running settings query...");
        let query_result = client
            .query(&statement_out, &[&id])
            .await
            .unwrap()
            .pop()
            .unwrap();

        return Ok(UserSettings::from_postgres(query_result, client).await);
    }
}

#[async_trait]
impl PostgresAccessible for User {
    async fn from_postgres(row: Row, client: &Client) -> Self {
        let id = row.get("id");
        let first_name = row.get("first_name");
        let last_name = row.get("last_name");
        let email = row.get("email");
        let username = row.get("username");
        let password = row.get("password");
        let server_ids: Vec<i32> = row.get("servers");
        let settings_id = row.get("settings");

        let servers = User::get_servers(client, server_ids).await.unwrap();
        let settings = User::get_settings(client, settings_id).await.unwrap();

        return Self {
            id,
            first_name,
            last_name,
            email,
            username,
            password,
            servers,
            settings,
        };
    }

    async fn update(&self, client: &Client) -> Result<Vec<Row>, Error> {
        let statement_raw = include_str!("../sql/insert_user.sql");
        let statement_result = client.prepare(statement_raw).await;
        let statement_out: Statement;

        match statement_result {
            Ok(statement) => statement_out = statement,
            Err(err) => return Err(err),
        };

        let query_result = client
            .query(
                &statement_out,
                &[
                    &self.first_name,
                    &self.last_name,
                    &self.email,
                    &self.username,
                    &self.password,
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
