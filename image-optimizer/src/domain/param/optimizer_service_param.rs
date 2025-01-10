use serde::{Deserialize, Serialize};

use crate::domain::entity::{image::Image, image_specification::ImageSpecification};

#[derive(Clone, Serialize, Deserialize)]
pub struct ProcessParam {
    pub image: Image,
    pub specification: ImageSpecification,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub data: Vec<u8>
}
