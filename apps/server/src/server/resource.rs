use either::Either;
use serde::{Deserialize, Serialize};

use super::icon::ImageMetadata;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerResource {
    pub name: String,
    pub id: f32,
    pub is_owner: bool,
    pub icon: Either<String, ImageMetadata>,
}
