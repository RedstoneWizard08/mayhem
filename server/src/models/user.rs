use rocket::serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[serde(crate = "rocket::serde")]
#[pg_mapper(table = "users")]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

impl Clone for User {
    fn clone(&self) -> Self {
        return Self {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
        };
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct RawUserCreation<'r> {
    pub first_name: &'r str,
    pub last_name: &'r str,
    pub email: &'r str,
    pub username: &'r str,
    pub password: &'r str,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[serde(crate = "rocket::serde")]
#[pg_mapper(table = "users")]
pub struct UserCreation {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl UserCreation {
    pub fn from<'r>(raw: RawUserCreation<'r>) -> UserCreation {
        return UserCreation {
            first_name: raw.first_name.clone().to_string(),
            last_name: raw.last_name.clone().to_string(),
            email: raw.email.clone().to_string(),
            username: raw.username.clone().to_string(),
            password: raw.password.clone().to_string(),
        };
    }
}

impl Clone for UserCreation {
    fn clone(&self) -> Self {
        return Self {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
        };
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserWithId<'r> {
    pub id: i64,
    pub first_name: &'r str,
    pub last_name: &'r str,
    pub email: &'r str,
    pub username: &'r str,
    pub password: &'r str,
}

#[derive(Deserialize, PostgresMapper, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[pg_mapper(table = "users")]
pub struct RealUserWithId {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}
