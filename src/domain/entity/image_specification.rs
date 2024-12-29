use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ImageSpecification {
    pub format: String,
    pub width: u32,
    pub height: u32,
}
