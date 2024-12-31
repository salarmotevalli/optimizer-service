use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageSpecification {
    pub format: String,
    pub width: u32,
    pub height: u32,
}
