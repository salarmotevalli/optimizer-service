use serde::{Deserialize, Serialize};

use crate::domain::entity::image::Image;

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateSignUrlTokenParam {
    pub image_name: String,
    pub image_ext: String,
    pub image_size: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateSignUrlTokenResult {
    pub token: String,
}

pub struct AuthorizeImageUploadParam {
    pub token: String,
    pub image: Image,
}

pub struct AuthorizeImageUploadResult {
    pub authorized: bool,
}

pub struct AuthorizeImageFormatParam {
    pub ext: String,
}

pub struct AuthorizeImageFormatResult {}
