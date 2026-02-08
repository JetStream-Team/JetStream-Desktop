use libmdns::{Responder, Service};

static SERVICE_TYPE: &str = "_jetstream._tcp";

pub fn start_mdns_responder(service_name: String, port: u16) -> Service {
    let responder = Responder::new();
    let service = responder.register(
        SERVICE_TYPE.into(),
        &service_name,
        port,
        &[]
    );
    println!("Started mDNS responder ");

    return service;
}