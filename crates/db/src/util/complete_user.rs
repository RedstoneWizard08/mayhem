use serde::{Deserialize, Serialize};

use super::CompleteServer;
use crate::models::user_settings::Model as UserSettings;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompleteUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub servers: Vec<CompleteServer>,
    pub settings: Option<UserSettings>,
    pub token: Option<String>,
}
