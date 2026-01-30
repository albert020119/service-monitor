use crate::config::Config;
use crate::models::service::{CheckType, CheckConfig, Service};
use crate::state::AppState;

pub mod http_check;
pub mod tcp_check;
pub mod dns_check;
pub mod ssl_check;

pub async fn start_monitoring(config: Config, state: AppState) {
    for service in config.services {
        for check in service.checks.clone() {
            let state_clone = state.clone();
            let service_clone = service.clone();
            tokio::spawn(async move {
                loop {
                    run_check(&service_clone, &check, &state_clone).await;
                    tokio::time::sleep(std::time::Duration::from_secs(check.interval_seconds)).await;
                }
            });
        }
    }
}

async fn run_check(service: &Service, check: &CheckConfig, state: &AppState) {
    match &check.check_type {
        CheckType::Http => http_check::run(service, check, state).await,
        CheckType::Tcp => tcp_check::run(service, check, state).await,
        CheckType::Dns => dns_check::run(service, check, state).await,
        CheckType::Ssl => ssl_check::run(service, check, state).await,
    }
}
