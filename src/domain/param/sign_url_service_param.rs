use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateSignUrlParam {
    pub image_name: String,
    pub image_ext: String,
    pub image_size: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateSignUrlResult {
    pub url: String,
}
