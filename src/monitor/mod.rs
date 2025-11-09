use crate::config::Config;
use crate::models::service::{Service, CheckType};

pub mod http_check;
pub mod tcp_check;
pub mod dns_check;
pub mod ssl_check;

pub async fn start_monitoring(config: Config) {
    for service in config.services {
        tokio::spawn(async move {
            loop {
                match service.check_type {
                    CheckType::Http => http_check::run(&service).await,
                    CheckType::Tcp  => tcp_check::run(&service).await,
                    CheckType::Dns  => dns_check::run(&service).await,
                    CheckType::Ssl  => ssl_check::run(&service).await,
                }
                tokio::time::sleep(std::time::Duration::from_secs(service.interval_seconds)).await;
            }
        });
    }
}
