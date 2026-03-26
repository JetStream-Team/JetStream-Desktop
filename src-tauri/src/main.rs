// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod mdns;
use jetstream_desktop_lib::{SERVER_NAME, PORT};

#[tokio::main]
async fn main() {
    // Setup env_logger
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("jetstream_desktop=debug")
    ).init();

    // Start mDNS responder in seperate thread
    let _service = mdns::start_mdns_responder(
        SERVER_NAME.to_string(), PORT,
    );

    // Start tauri application
    jetstream_desktop_lib::run();
}