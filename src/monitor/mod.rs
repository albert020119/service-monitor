use crate::config::Config;
use crate::models::service::CheckType;
use crate::state::AppState;

pub mod http_check;
pub mod tcp_check;
pub mod dns_check;
pub mod ssl_check;

pub async fn start_monitoring(config: Config, state: AppState) {
    for service in config.services {
        let state_clone = state.clone();
        tokio::spawn(async move {
            loop {
                match service.check_type {
                    CheckType::Http => http_check::run(&service, &state_clone).await,
                    CheckType::Tcp  => tcp_check::run(&service, &state_clone).await,
                    CheckType::Dns  => dns_check::run(&service, &state_clone).await,
                    CheckType::Ssl  => ssl_check::run(&service, &state_clone).await,
                }
                tokio::time::sleep(std::time::Duration::from_secs(service.interval_seconds)).await;
            }
        });
    }
}
