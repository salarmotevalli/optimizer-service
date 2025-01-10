use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageSpecification {
    pub format: String,
    pub quality: f32,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

impl std::default::Default for ImageSpecification {
    fn default() -> Self {
        Self {
            format: Default::default(),
            quality: 75.0,
            width: None,
            height: None,
        }
    }
}
