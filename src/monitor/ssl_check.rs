use crate::models::service::{CheckConfig, Service};
use crate::state::AppState;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_native_tls::TlsConnector;
use native_tls::TlsConnector as NativeTlsConnector;
use std::time::Instant;

fn normalize_host_port(raw: &str, default_port: u16) -> (String, u16) {
	let mut s = raw.to_string();
	if let Some(pos) = s.find("://") {
		s = s.split_at(pos + 3).1.to_string();
	}
	if let Some(pos) = s.find('/') {
		s = s[..pos].to_string();
	}

	if let Some(pos) = s.rfind(':') {
		if let Ok(p) = s[pos+1..].parse::<u16>() {
			let host = s[..pos].to_string();
			return (host, p);
		}
	}

	(s, default_port)
}

pub async fn run(service: &Service, check: &CheckConfig, state: &AppState) {
	let start = Instant::now();

	let (host, port) = normalize_host_port(&service.url, 443);
	let addr = format!("{}:{}", host, port);

	let connect_future = TcpStream::connect(addr);
	let stream_result = timeout(std::time::Duration::from_millis(check.timeout_ms), connect_future).await;
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

							state.update_check_status(
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

							state.update_check_status(
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

					state.update_check_status(
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

			state.update_check_status(
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

			state.update_check_status(
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
