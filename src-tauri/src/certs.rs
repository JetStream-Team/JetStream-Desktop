use log::debug;
use rcgen;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

#[allow(dead_code)]
pub fn generate_cert() -> (String, String) {
    let subject_alt_names = vec![
        "jetstream".to_string(),
        "localhost".to_string(),
        "127.0.0.1".to_string(),
        "0.0.0.0".to_string(),
    ];
    let cert = rcgen::generate_simple_self_signed(subject_alt_names).unwrap();
    let cert_pem = cert.cert.pem();
    let key_pem = cert.signing_key.serialize_pem();

    let config_dir = dirs::config_dir().unwrap().join("jetstream");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("cert.pem"), &cert_pem).unwrap();
    std::fs::write(config_dir.join("key.pem"), &key_pem).unwrap();

    return (cert_pem, key_pem);
}

#[allow(dead_code)]
pub fn get_cert() -> (Vec<CertificateDer<'static>>, PrivateKeyDer<'static>) {
    let config_dir = dirs::config_dir().unwrap().join("jetstream");
    if !config_dir.join("cert.pem").exists() || !config_dir.join("key.pem").exists() {
        debug!("Certificate or key not found, generating new ones");
        generate_cert();
    }
    let cert_pem = std::fs::read_to_string(config_dir.join("cert.pem"))
        .expect("Failed to read certificate PEM from disk");
    let key_pem = std::fs::read_to_string(config_dir.join("key.pem"))
        .expect("Failed to read key PEM from disk");

    // Convert raw cert and key to rustls compatible format
    let certs = rustls_pemfile::certs(&mut cert_pem.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse certificate PEM")
        .into_iter()
        .collect::<Vec<_>>();
    let key = rustls_pemfile::private_key(&mut key_pem.as_bytes())
        .expect("Failed to parse key PEM")
        .expect("No private key found in PEM file");

    return (certs, key);
}
