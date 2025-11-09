use crate::models::service::Service;
use tokio::net::TcpStream;
use std::time::Duration;

pub async fn run(service: &Service) {
    let timeout = Duration::from_millis(service.timeout_ms);
    let result = tokio::time::timeout(timeout, TcpStream::connect(&service.url)).await;

    match result {
        Ok(Ok(_)) => println!("{} TCP OK", service.name),
        _ => println!("{} TCP FAILED", service.name),
    }
}
