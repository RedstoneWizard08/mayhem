use mayhem_db::{
    models::user_settings::Model as UserSettings,
    util::{CompleteServer, CompleteUser},
};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
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
