use crate::models::service::{CheckConfig, Service};
use crate::state::AppState;
use trust_dns_resolver::TokioAsyncResolver;
use std::time::Instant;

fn normalize_host(raw: &str) -> String {
    let mut s = raw.to_string();
    if let Some(pos) = s.find("://") {
        s = s.split_at(pos + 3).1.to_string();
    }
    if let Some(pos) = s.find('/') {
        s = s[..pos].to_string();
    }
    if let Some(pos) = s.rfind(':') {
        // strip a numeric :port suffix
        if s[pos + 1..].parse::<u16>().is_ok() {
            s = s[..pos].to_string();
        }
    }
    s
}

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
            
            state.update_check_status(
                service.name.clone(),
                service.url.clone(),
                "DNS".to_string(),
                true,
                Some(elapsed),
                message,
                check.interval_seconds,
            )
            .await;
        },
        Err(e) => {
            let message = format!("Error: {}", e);
            println!("{} DNS FAILED: {}", service.name, e);
            
            state.update_check_status(
                service.name.clone(),
                service.url.clone(),
                "DNS".to_string(),
                false,
                Some(elapsed),
                message,
                check.interval_seconds,
            )
            .await;
        },
    }
}
