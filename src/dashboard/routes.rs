use axum::Json;
use serde_json::json;

pub async fn index() -> &'static str {
    "Service Health Monitor Dashboard"
}

pub async fn status() -> Json<serde_json::Value> {
    Json(json!({ "services": [] }))
}
