use serde::{Deserialize, Serialize};

use crate::domain::entity::image::Image;

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateTokenParam {
    pub image_name: String,
    pub image_format: String,
    pub image_size: usize,
    pub user_id: u32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateTokenResult {
    pub token: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VerifyTokenParam {
    pub token: String,
    pub image: Image,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VerifyTokenResult {
    pub user_id: u32
}
