use serde::{Deserialize, Serialize};

use crate::domain::constant::file_status::FileStatus;

#[derive(Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: u64,
    pub full_name: String,
    pub status: FileStatus,
    pub size: usize,
}

// TODO: fix file status
impl Default for Image {
    fn default() -> Self {
        Self {
            id: 0,
            full_name: String::new(),
            status: FileStatus::Processed,
            size: 0,
        }
    }
}

impl Image {
    pub fn ext(&self) -> String {
        self.full_name.split('.').last().unwrap_or("").to_string()
    }

    pub fn processed(&mut self) {
        self.status = FileStatus::Processed
    }

    pub fn reject(&mut self) {
        self.status = FileStatus::Rejected
    }
}
