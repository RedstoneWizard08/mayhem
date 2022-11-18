use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub enum ImageType {
    PNG,
    JPG,
    JPEG,
    GIF,
    ICO,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ImageMetadata {
    url: String,
    format: ImageType,
    size: Option<f32>,
}
