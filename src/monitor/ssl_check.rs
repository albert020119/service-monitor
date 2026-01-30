use crate::models::service::{CheckConfig, Service};
use crate::state::AppState;
use crate::utils::net::normalize_host_port;
use native_tls::TlsConnector as NativeTlsConnector;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_native_tls::TlsConnector;

pub async fn run(service: &Service, check: &CheckConfig, state: &AppState) {
    let start = Instant::now();

    let (host, port) = normalize_host_port(&service.url, 443);
    let addr = format!("{}:{}", host, port);

    let connect_future = TcpStream::connect(addr);
    let stream_result = timeout(
        std::time::Duration::from_millis(check.timeout_ms),
        connect_future,
    )
    .await;
    let elapsed = start.elapsed().as_millis() as u64;

    match stream_result {
        Ok(Ok(stream)) => {
            // perform TLS handshake
            match NativeTlsConnector::new() {
                Ok(native) => {
                    let connector = TlsConnector::from(native);
                    match connector.connect(&host, stream).await {
                        Ok(_tls_stream) => {
                            let message = format!("TLS handshake succeeded for {}", host);
                            println!("{} SSL OK", service.name);

                            state
                                .update_check_status(
                                    service.name.clone(),
                                    service.url.clone(),
                                    "SSL".to_string(),
                                    true,
                                    Some(elapsed),
                                    message,
                                    check.interval_seconds,
                                )
                                .await;
                        }
                        Err(e) => {
                            let message = format!("TLS handshake failed: {}", e);
                            println!("{} SSL FAILED: {}", service.name, e);

                            state
                                .update_check_status(
                                    service.name.clone(),
                                    service.url.clone(),
                                    "SSL".to_string(),
                                    false,
                                    Some(elapsed),
                                    message,
                                    check.interval_seconds,
                                )
                                .await;
                        }
                    }
                }
                Err(e) => {
                    let message = format!("TLS connector error: {}", e);
                    println!("{} SSL FAILED: {}", service.name, e);

                    state
                        .update_check_status(
                            service.name.clone(),
                            service.url.clone(),
                            "SSL".to_string(),
                            false,
                            Some(elapsed),
                            message,
                            check.interval_seconds,
                        )
                        .await;
                }
            }
        }
        Ok(Err(e)) => {
            let message = format!("Connection error: {}", e);
            println!("{} SSL FAILED: {}", service.name, e);

            state
                .update_check_status(
                    service.name.clone(),
                    service.url.clone(),
                    "SSL".to_string(),
                    false,
                    Some(elapsed),
                    message,
                    check.interval_seconds,
                )
                .await;
        }
        Err(_) => {
            let message = "Timed out".to_string();
            println!("{} SSL TIMEOUT", service.name);

            state
                .update_check_status(
                    service.name.clone(),
                    service.url.clone(),
                    "SSL".to_string(),
                    false,
                    Some(elapsed),
                    message,
                    check.interval_seconds,
                )
                .await;
        }
    }
}
