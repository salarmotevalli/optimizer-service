use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: u64,
    pub user_id: u32,
    pub full_name: String,
    pub size: usize,
}

impl Image {
    pub fn ext(&self) -> String {
        self.full_name.split('.').last().unwrap_or("").to_string()
    }
}
