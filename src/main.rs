mod config;
mod monitor;
mod alert;
mod dashboard;
mod models;
mod utils;

use crate::config::Config;
use crate::monitor::start_monitoring;
use crate::dashboard::start_dashboard;

#[tokio::main]
async fn main() {
    let config = Config::load("config.json").expect("Failed to load config");
    let value = config.clone();
    tokio::spawn(async move {
        start_monitoring(value.clone()).await;
    });

    start_dashboard(config).await;
}
