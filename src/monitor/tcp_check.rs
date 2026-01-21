use crate::models::service::Service;
use crate::state::AppState;
use tokio::net::TcpStream;
use tokio::time::timeout;
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

pub async fn run(service: &Service, state: &AppState) {
	let start = Instant::now();

	let (host, port) = normalize_host_port(&service.url, 80);
	let addr = format!("{}:{}", host, port);

	let connect_future = TcpStream::connect(addr);
	let result = timeout(std::time::Duration::from_millis(service.timeout_ms), connect_future).await;
	let elapsed = start.elapsed().as_millis() as u64;

	match result {
		Ok(Ok(_stream)) => {
			let message = format!("Connected to {}:{}", host, port);
			println!("{} TCP OK ({})", service.name, message);

			state.update_service_status(
				service.name.clone(),
				service.url.clone(),
				"TCP".to_string(),
				true,
				Some(elapsed),
				message,
				service.interval_seconds,
			).await;
		}
		Ok(Err(e)) => {
			let message = format!("Error: {}", e);
			println!("{} TCP FAILED: {}", service.name, e);

			state.update_service_status(
				service.name.clone(),
				service.url.clone(),
				"TCP".to_string(),
				false,
				Some(elapsed),
				message,
				service.interval_seconds,
			).await;
		}
		Err(_) => {
			let message = "Timed out".to_string();
			println!("{} TCP TIMEOUT", service.name);

			state.update_service_status(
				service.name.clone(),
				service.url.clone(),
				"TCP".to_string(),
				false,
				Some(elapsed),
				message,
				service.interval_seconds,
			).await;
		}
	}
}
