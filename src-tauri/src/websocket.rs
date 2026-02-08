use crate::certs;

pub fn start_server() {
    let (cert_pem, key_pem) = certs::get_cert();
    println!("{cert_pem}");
    println!("{key_pem}");
}