use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateTokenParam {
    pub expire_time: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerateTokenResult {
    pub token: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VerifyTokenParam {
    pub token: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VerifyTokenResult {}
