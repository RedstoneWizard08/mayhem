use super::{user::UserWithId, user_settings::UserSettingsWithId};
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FullUser<'r> {
    pub id: i64,
    pub first_name: &'r str,
    pub last_name: &'r str,
    pub email: &'r str,
    pub username: &'r str,
    pub password: &'r str,

    pub settings: UserSettingsWithId,
}

impl<'r> FullUser<'r> {
    pub fn new(user: UserWithId<'r>, settings: UserSettingsWithId) -> FullUser<'r> {
        let u: FullUser<'r> = FullUser {
            id: user.id,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            username: user.username.clone(),
            password: user.password.clone(),

            settings,
        };

        return u;
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct RealFullUser {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,

    pub settings: UserSettingsWithId,
}

impl RealFullUser {
    pub fn from<'r>(user: FullUser<'r>) -> RealFullUser {
        return RealFullUser {
            id: user.id,
            first_name: user.first_name.clone().to_string(),
            last_name: user.last_name.clone().to_string(),
            email: user.email.clone().to_string(),
            username: user.username.clone().to_string(),
            password: user.password.clone().to_string(),

            settings: user.settings.clone(),
        };
    }
}
