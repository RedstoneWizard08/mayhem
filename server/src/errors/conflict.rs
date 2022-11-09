use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BasicResponseError<'r> {
    pub code: i32,
    pub message: &'r str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RealConflictError {
    pub code: i32,
    pub message: String,
}

impl RealConflictError {
    pub fn from<'r>(err: BasicResponseError<'r>) -> RealConflictError {
        return RealConflictError {
            code: err.code,
            message: err.message.clone().to_string(),
        };
    }
}
