// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mdns;
mod websocket;
mod certs;
mod protobuf_message;
mod message_handlers;

// const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SERVER_NAME: &str = "Mathew's JetStream";
pub const PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("jetstream_desktop=debug")).init();

    // Start the mdns service
    let _service = mdns::start_mdns_responder(SERVER_NAME.to_string(), PORT);

    // Start the jetstream server
    tokio::spawn(async move {
        websocket::start_server(PORT).await;
    });

    // Run the Tauri application
    // jetstream_desktop_lib::run();

    loop {}
}