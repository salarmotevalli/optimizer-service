use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum FileStatus {
    Processed,
    Rejected,
}
