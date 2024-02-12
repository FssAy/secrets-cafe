use std::ops::Deref;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio_rustls::TlsAcceptor;

#[cfg(feature = "cloudflare")]
pub mod cloudflare;

trait InnerTLS {
    fn load_certs() -> std::io::Result<Vec<CertificateDer<'static>>>;
    fn load_private_key() -> std::io::Result<PrivateKeyDer<'static>>;
}

pub trait TLS: InnerTLS + Deref<Target=TlsAcceptor> + Clone {
    fn init() -> Result<Self, Box<dyn std::error::Error>>;
}
