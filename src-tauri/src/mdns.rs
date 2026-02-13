use libmdns::{Responder, Service};
use log::{self, info};

static SERVICE_TYPE: &str = "_jetstream._tcp";

pub fn start_mdns_responder(service_name: String, port: u16) -> Service {
    let responder = Responder::new();
    let service = responder.register(
        SERVICE_TYPE.into(),
        &service_name,
        port,
        &[]
    );
    info!("Started mDNS responder for '{service_name}' on port {port}");

    return service;
}