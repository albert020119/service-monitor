use crate::models::service::{CheckConfig, Service};
use crate::state::AppState;
use crate::utils::net::normalize_host_port;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub async fn run(service: &Service, check: &CheckConfig, state: &AppState) {
    let start = Instant::now();

    let (host, port) = normalize_host_port(&service.url, 80);
    let addr = format!("{}:{}", host, port);

    let connect_future = TcpStream::connect(addr);
    let result = timeout(
        std::time::Duration::from_millis(check.timeout_ms),
        connect_future,
    )
    .await;
    let elapsed = start.elapsed().as_millis() as u64;

    match result {
        Ok(Ok(_stream)) => {
            let message = format!("Connected to {}:{}", host, port);
            println!("{} TCP OK ({})", service.name, message);

            state
                .update_check_status(
                    service.name.clone(),
                    service.url.clone(),
                    "TCP".to_string(),
                    true,
                    Some(elapsed),
                    message,
                    check.interval_seconds,
                )
                .await;
        }
        Ok(Err(e)) => {
            let message = format!("Error: {}", e);
            println!("{} TCP FAILED: {}", service.name, e);

            state
                .update_check_status(
                    service.name.clone(),
                    service.url.clone(),
                    "TCP".to_string(),
                    false,
                    Some(elapsed),
                    message,
                    check.interval_seconds,
                )
                .await;
        }
        Err(_) => {
            let message = "Timed out".to_string();
            println!("{} TCP TIMEOUT", service.name);

            state
                .update_check_status(
                    service.name.clone(),
                    service.url.clone(),
                    "TCP".to_string(),
                    false,
                    Some(elapsed),
                    message,
                    check.interval_seconds,
                )
                .await;
        }
    }
}
