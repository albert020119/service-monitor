use crate::models::service::{CheckConfig, Service};
use crate::state::AppState;
use reqwest::Client;
use std::time::Instant;

pub async fn run(service: &Service, check: &CheckConfig, state: &AppState) {
    let client = Client::new();
    let start = Instant::now();

    let result = client
        .get(&service.url)
        .timeout(std::time::Duration::from_millis(check.timeout_ms))
        .send()
        .await;

    let elapsed = start.elapsed().as_millis() as u64;

    match result {
        Ok(resp) => {
            let status_code = resp.status();
            let is_success = status_code.is_success();
            let message = format!("HTTP {}", status_code);

            println!("{} OK ({})", service.name, status_code);

            state
                .update_check_status(
                    service.name.clone(),
                    service.url.clone(),
                    "HTTP".to_string(),
                    is_success,
                    Some(elapsed),
                    message,
                    check.interval_seconds,
                )
                .await;
        }
        Err(e) => {
            let message = format!("Error: {}", e);
            println!("{} FAILED: {}", service.name, e);

            state
                .update_check_status(
                    service.name.clone(),
                    service.url.clone(),
                    "HTTP".to_string(),
                    false,
                    Some(elapsed),
                    message,
                    check.interval_seconds,
                )
                .await;
        }
    }
}
