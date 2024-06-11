use mayhem_db::{
    models::{user::ActiveModel, user_settings::Model as UserSettings},
    util::{CompleteServer, CompleteUser},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordlessUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub servers: Vec<CompleteServer>,
    pub settings: Option<UserSettings>,
}

impl PasswordlessUser {
    pub fn from_complete(complete: CompleteUser) -> PasswordlessUser {
        let CompleteUser {
            id,
            first_name,
            last_name,
            email,
            username,
            servers,
            settings,
            password: _,
            token: _,
        } = complete;

        return PasswordlessUser {
            id,
            first_name,
            last_name,
            email,
            username,
            servers,
            settings,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordlessActiveUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub token: Option<String>,
}

impl PasswordlessActiveUser {
    pub fn from_active(active: ActiveModel) -> PasswordlessActiveUser {
        let ActiveModel {
            id,
            first_name,
            last_name,
            email,
            username,
            token,
            password: _,
        } = active;

        return PasswordlessActiveUser {
            id: id.unwrap(),
            first_name: first_name.unwrap(),
            last_name: last_name.unwrap(),
            email: email.unwrap(),
            username: username.unwrap(),
            token: token.unwrap(),
        };
    }
}
