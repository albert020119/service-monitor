use crate::models::service::{CheckConfig, Service};
use crate::state::AppState;
use crate::utils::net::normalize_host;
use std::time::Instant;
use trust_dns_resolver::TokioAsyncResolver;

pub async fn run(service: &Service, check: &CheckConfig, state: &AppState) {
    let start = Instant::now();
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();
    let host = normalize_host(service.url.as_str());
    let result = resolver.lookup_ip(host.as_str()).await;
    let elapsed = start.elapsed().as_millis() as u64;

    match result {
        Ok(response) => {
            let ips: Vec<String> = response.iter().map(|ip| ip.to_string()).collect();
            let message = format!("Resolved to: {}", ips.join(", "));
            println!("{} DNS OK", service.name);

            state
                .update_check_status(
                    service.name.clone(),
                    service.url.clone(),
                    "DNS".to_string(),
                    true,
                    Some(elapsed),
                    message,
                    check.interval_seconds,
                )
                .await;
        }
        Err(e) => {
            let message = format!("Error: {}", e);
            println!("{} DNS FAILED: {}", service.name, e);

            state
                .update_check_status(
                    service.name.clone(),
                    service.url.clone(),
                    "DNS".to_string(),
                    false,
                    Some(elapsed),
                    message,
                    check.interval_seconds,
                )
                .await;
        }
    }
}
