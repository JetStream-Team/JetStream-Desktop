#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("jetstream_desktop=debug")
    ).init();

    let _service = jetstream_desktop_lib::mdns::start_mdns_responder(
        jetstream_desktop_lib::SERVER_NAME.to_string(),
        jetstream_desktop_lib::PORT,
    );

    jetstream_desktop_lib::run();
}