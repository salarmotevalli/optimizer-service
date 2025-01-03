use serde::{Deserialize, Serialize};

use crate::domain::entity::image::Image;
use crate::domain::entity::image_specification::ImageSpecification;

#[derive(Serialize, Deserialize)]
pub struct OptImgParam {
    pub image_path: String,
    pub specification: ImageSpecification,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OptImgResult {
    pub image: Image,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StoreImageInfoParam {
    pub image: Image,
    pub specification: ImageSpecification,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StoreImageInfoResult {}
