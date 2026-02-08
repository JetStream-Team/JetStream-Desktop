// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mdns;

fn main() {
    // Start the mdns service
    let _service = mdns::start_mdns_responder("JetStream".to_string(), 8080);

    // Start the jetstream server

    // Run the Tauri application
    // jetstream_desktop_lib::run()
}
