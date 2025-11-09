use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

mod routes;

pub async fn start_dashboard(_config: crate::config::Config) {
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/api/status", get(routes::status));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Dashboard running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
