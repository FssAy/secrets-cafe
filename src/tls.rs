use std::io::BufReader;
use std::sync::Arc;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;

static TLS_CRT: &[u8] = include_bytes!("secrets/server.crt");
static TLS_KEY: &[u8] = include_bytes!("secrets/server.key");

fn load_certs() -> std::io::Result<Vec<CertificateDer<'static>>> {
    let mut reader = BufReader::new(TLS_CRT);
    rustls_pemfile::certs(&mut reader).collect()
}

fn load_private_key() -> std::io::Result<PrivateKeyDer<'static>> {
    let mut reader = BufReader::new(TLS_KEY);
    rustls_pemfile::private_key(&mut reader).map(|key| key.unwrap())
}

pub(super) fn init() -> Result<TlsAcceptor, Box<dyn std::error::Error>> {
    let certs = load_certs()?;
    let key = load_private_key()?;

    let mut server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    server_config.alpn_protocols = vec![
        b"h2".to_vec(),
        b"http/1.1".to_vec(),
        b"http/1.0".to_vec(),
    ];

    let tls_acceptor = TlsAcceptor::from(
        Arc::new(server_config)
    );

    info!("TLS initialized!");

    Ok(tls_acceptor)
}
