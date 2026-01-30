use crate::state::AppState;
use axum::routing::get;
use axum::{Extension, Router};
use tokio::net::TcpListener;

mod routes;

pub async fn start_dashboard(_config: crate::config::Config, state: AppState) {
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/api/status", get(routes::status))
        .layer(Extension(state));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Dashboard running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
