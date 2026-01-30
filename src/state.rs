use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckStatus {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub url: String,
    pub status: HealthStatus,
    pub last_check: DateTime<Utc>,
    pub response_time_ms: Option<u64>,
    pub uptime_percentage: f64,
    pub total_checks: u64,
    pub successful_checks: u64,
    pub message: String,
    pub checks: Vec<CheckStatus>,
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

    pub async fn update_check_status(
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
        
        let service = services.entry(name.clone()).or_insert(ServiceStatus {
            name: name.clone(),
            url: url.clone(),
            status: HealthStatus::Unknown,
            last_check: Utc::now(),
            response_time_ms: None,
            uptime_percentage: 0.0,
            total_checks: 0,
            successful_checks: 0,
            message: String::new(),
            checks: Vec::new(),
        });

        // keep URL up to date in case config changed
        service.url = url;

        let now = Utc::now();

        let check = match service.checks.iter_mut().find(|c| c.check_type == check_type) {
            Some(existing) => existing,
            None => {
                service.checks.push(CheckStatus {
                    check_type: check_type.clone(),
                    status: HealthStatus::Unknown,
                    last_check: now,
                    response_time_ms: None,
                    uptime_percentage: 0.0,
                    total_checks: 0,
                    successful_checks: 0,
                    message: String::new(),
                    interval_seconds,
                });
                service
                    .checks
                    .iter_mut()
                    .find(|c| c.check_type == check_type)
                    .expect("just inserted")
            }
        };

        check.total_checks += 1;
        if is_success {
            check.successful_checks += 1;
            check.status = HealthStatus::Up;
        } else {
            check.status = HealthStatus::Down;
        }
        
        check.last_check = now;
        check.response_time_ms = response_time_ms;
        check.message = message;
        check.interval_seconds = interval_seconds;
        check.uptime_percentage = if check.total_checks > 0 {
            (check.successful_checks as f64 / check.total_checks as f64) * 100.0
        } else {
            0.0
        };

        recompute_service_aggregate(service);
    }

    pub async fn get_all_services(&self) -> Vec<ServiceStatus> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }
}

fn recompute_service_aggregate(service: &mut ServiceStatus) {
    if service.checks.is_empty() {
        service.status = HealthStatus::Unknown;
        service.last_check = Utc::now();
        service.response_time_ms = None;
        service.total_checks = 0;
        service.successful_checks = 0;
        service.uptime_percentage = 0.0;
        service.message.clear();
        return;
    }

    let any_down = service.checks.iter().any(|c| c.status == HealthStatus::Down);
    let all_up = service.checks.iter().all(|c| c.status == HealthStatus::Up);

    service.status = if any_down {
        HealthStatus::Down
    } else if all_up {
        HealthStatus::Up
    } else {
        HealthStatus::Unknown
    };

    service.last_check = service
        .checks
        .iter()
        .map(|c| c.last_check)
        .max()
        .unwrap_or_else(Utc::now);

    service.total_checks = service.checks.iter().map(|c| c.total_checks).sum();
    service.successful_checks = service.checks.iter().map(|c| c.successful_checks).sum();
    service.uptime_percentage = if service.total_checks > 0 {
        (service.successful_checks as f64 / service.total_checks as f64) * 100.0
    } else {
        0.0
    };

    let response_times: Vec<u64> = service
        .checks
        .iter()
        .filter_map(|c| c.response_time_ms)
        .collect();
    service.response_time_ms = if response_times.is_empty() {
        None
    } else {
        Some((response_times.iter().sum::<u64>() as f64 / response_times.len() as f64).round() as u64)
    };

    if service.status == HealthStatus::Down {
        let parts: Vec<String> = service
            .checks
            .iter()
            .filter(|c| c.status == HealthStatus::Down)
            .map(|c| format!("{}: {}", c.check_type, c.message))
            .collect();
        service.message = parts.join(" | ");
    } else {
        service.message.clear();
    }
}

