use either::Either;
use rocket::serde::{Deserialize, Serialize};

use super::icon::ImageMetadata;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ServerResource {
    pub name: String,
    pub id: f32,
    pub is_owner: bool,
    pub icon: Either<String, ImageMetadata>,
}
