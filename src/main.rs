mod config;
mod monitor;
mod alert;
mod dashboard;
mod models;
mod utils;
mod state;

use crate::config::Config;
use crate::monitor::start_monitoring;
use crate::dashboard::start_dashboard;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    println!("Starting Service Health Monitor...");
    
    let config = Config::load("config.json").expect("Failed to load config");
    let state = AppState::new();
    
    let monitor_state = state.clone();
    let monitor_config = config.clone();
    tokio::spawn(async move {
        start_monitoring(monitor_config, monitor_state).await;
    });

    start_dashboard(config, state).await;
}
