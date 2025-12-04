use crate::models::service::Service;
use crate::state::AppState;
use std::time::Instant;

pub async fn run(service: &Service, state: &AppState) {
    let start = Instant::now();
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(service.timeout_ms))
        .build()
        .unwrap();
    
    let result = client.get(&service.url).send().await;
    let elapsed = start.elapsed().as_millis() as u64;
    
    match result {
        Ok(resp) => {
            let message = format!("SSL connection successful ({})", resp.status());
            println!("{} SSL OK", service.name);
            
            state.update_service_status(
                service.name.clone(),
                service.url.clone(),
                "SSL".to_string(),
                true,
                Some(elapsed),
                message,
                service.interval_seconds,
            ).await;
        },
        Err(e) => {
            let message = format!("SSL error: {}", e);
            println!("{} SSL FAILED: {}", service.name, e);
            
            state.update_service_status(
                service.name.clone(),
                service.url.clone(),
                "SSL".to_string(),
                false,
                Some(elapsed),
                message,
                service.interval_seconds,
            ).await;
        },
    }
}
