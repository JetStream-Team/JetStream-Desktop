// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mdns;
mod websocket;
mod certs;

// const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Start the mdns service
    let _service = mdns::start_mdns_responder("Mathew's JetStream".to_string(), PORT);

    // Start the jetstream server
    websocket::start_server(PORT).await;

    // Run the Tauri application
    // jetstream_desktop_lib::run();

    loop {}
}
