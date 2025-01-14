use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::entity::image::Image;
use crate::domain::entity::image_specification::ImageSpecification;

#[derive(Clone, Serialize, Deserialize)]
pub struct StoreImageInfoParam {
    pub image: Image,
    pub specification: ImageSpecification,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StoreImageInfoResult {}

#[derive(Clone, Serialize, Deserialize)]
pub struct OptimizeImageParam {
    pub image: Image,
    pub specification: ImageSpecification
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OptimizeImageResult {}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProcessResultParam {
    pub image: Image,
    pub err: Option<HashMap<String, String>>
}
