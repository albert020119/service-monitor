use reqwest::Client;
use serde_json::json;

pub async fn send_slack(text: &str, webhook_url: &str) {
    let payload = json!({ "text": text });
    let body = serde_json::to_vec(&payload).unwrap();

    let client = Client::new();
    let _ = client
        .post(webhook_url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
}
