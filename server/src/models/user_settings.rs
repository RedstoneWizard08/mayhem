use rocket::serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[pg_mapper(table = "user_settings")]
pub struct UserSettings {
    pub user_id: i64,
}

impl Clone for UserSettings {
    fn clone(&self) -> Self {
        return Self {
            user_id: self.user_id,
        };
    }
}

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
#[pg_mapper(table = "user_settings")]
pub struct UserSettingsWithId {
    #[serde(skip_deserializing, skip_serializing)]
    pub id: i64,
    pub user_id: i64,
}

impl Clone for UserSettingsWithId {
    fn clone(&self) -> Self {
        return Self {
            id: self.id,
            user_id: self.user_id,
        };
    }
}
