use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageType {
    PNG,
    JPG,
    JPEG,
    GIF,
    ICO,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageMetadata {
    url: String,
    format: ImageType,
    size: Option<f32>,
}
