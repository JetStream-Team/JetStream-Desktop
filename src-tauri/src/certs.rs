use rcgen;

pub fn generate_cert() -> (String, String) {

    let subject_alt_names = vec!["jetstream".to_string()];
    let cert = rcgen::generate_simple_self_signed(subject_alt_names).unwrap();
    let cert_pem = cert.cert.pem();
    let key_pem = cert.signing_key.serialize_pem();

    let config_dir = dirs::config_dir().unwrap().join("jetstream");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("cert.pem"), &cert_pem).unwrap();
    std::fs::write(config_dir.join("key.pem"), &key_pem).unwrap();

    return (cert_pem, key_pem);
}

pub fn get_cert() -> (String, String) {
    let config_dir = dirs::config_dir().unwrap().join("jetstream");
    if !config_dir.join("cert.pem").exists()
    || !config_dir.join("key.pem").exists() {
        return generate_cert();
    }
    let cert_pem = std::fs::read_to_string(config_dir.join("cert.pem")).unwrap();
    let key_pem = std::fs::read_to_string(config_dir.join("key.pem")).unwrap();
    return (cert_pem, key_pem);
}