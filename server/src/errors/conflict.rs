use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicResponseError {
    pub code: i32,
    pub message: String,
}
