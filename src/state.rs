use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub url: String,
    pub check_type: String,
    pub status: HealthStatus,
    pub last_check: DateTime<Utc>,
    pub response_time_ms: Option<u64>,
    pub uptime_percentage: f64,
    pub total_checks: u64,
    pub successful_checks: u64,
    pub message: String,
    pub interval_seconds: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Up,
    Down,
    Unknown,
}

#[derive(Clone)]
pub struct AppState {
    pub services: Arc<RwLock<HashMap<String, ServiceStatus>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn update_service_status(
        &self,
        name: String,
        url: String,
        check_type: String,
        is_success: bool,
        response_time_ms: Option<u64>,
        message: String,
        interval_seconds: u64,
    ) {
        let mut services = self.services.write().await;
        
        let status = services.entry(name.clone()).or_insert(ServiceStatus {
            name: name.clone(),
            url,
            check_type,
            status: HealthStatus::Unknown,
            last_check: Utc::now(),
            response_time_ms: None,
            uptime_percentage: 0.0,
            total_checks: 0,
            successful_checks: 0,
            message: String::new(),
            interval_seconds,
        });

        status.total_checks += 1;
        if is_success {
            status.successful_checks += 1;
            status.status = HealthStatus::Up;
        } else {
            status.status = HealthStatus::Down;
        }
        
        status.last_check = Utc::now();
        status.response_time_ms = response_time_ms;
        status.message = message;
        status.uptime_percentage = if status.total_checks > 0 {
            (status.successful_checks as f64 / status.total_checks as f64) * 100.0
        } else {
            0.0
        };
    }

    pub async fn get_all_services(&self) -> Vec<ServiceStatus> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }
}

