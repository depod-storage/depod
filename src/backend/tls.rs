use std::fs::File;
use std::io::BufReader;

use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{certs, private_key};

pub fn load_tls_config(cert_path: &str, key_path: &str) -> std::io::Result<ServerConfig> {
    let mut cert_reader = BufReader::new(File::open(cert_path)?);
    let mut key_reader = BufReader::new(File::open(key_path)?);

    let cert_chain: Vec<CertificateDer<'static>> =
        certs(&mut cert_reader).collect::<Result<_, _>>()?;

    let key: PrivateKeyDer<'static> = private_key(&mut key_reader)?.expect("No private key found!");

    Ok(ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .expect("Failed to create TLS"))
}
