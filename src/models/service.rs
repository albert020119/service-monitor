use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum CheckType {
    Http,
    Tcp,
    Dns,
    Ssl,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CheckConfig {
    pub check_type: CheckType,
    pub interval_seconds: u64,
    pub timeout_ms: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub url: String,
    pub checks: Vec<CheckConfig>,
}
