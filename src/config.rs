use serde::{Serialize, Deserialize};
use std::fs;

use crate::models::service::Service;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub services: Vec<Service>,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }
}
