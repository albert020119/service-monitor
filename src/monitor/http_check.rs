use reqwest::Client;
use crate::models::service::Service;

pub async fn run(service: &Service) {
    let client = Client::new();
    let result = client
        .get(&service.url)
        .timeout(std::time::Duration::from_millis(service.timeout_ms))
        .send()
        .await;

    match result {
        Ok(resp) => println!("{} OK ({})", service.name, resp.status()),
        Err(e)   => println!("{} FAILED: {}", service.name, e),
    }
}
