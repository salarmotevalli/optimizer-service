use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageSpecification {
    pub format: String,
    pub quality: f32,
    pub filter: String,
    pub quantization: Option<u8>,
    pub dithering: Option<f32>,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

impl std::default::Default for ImageSpecification {
    fn default() -> Self {
        Self { 
            format: Default::default(),
            quality: 75.0,
            filter: "lanczos3".to_string(), 
            quantization: None,
            dithering: None,        
            width: None, 
            height: None,
        }
    }
}
