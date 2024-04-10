#[macro_use]
extern crate tracing;

#[macro_use]
extern crate serde;

#[cfg(test)]
mod tests;

#[cfg(feature = "tls")]
mod tls;

mod logs;
mod config;
mod handler;
mod database;
mod console;
mod utils;

use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use database::Database;
use console::Console;
use config::Config;

#[cfg(feature = "tls")]
use tls::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logs::init();
    let cfg = Config::init().await?;
    Database::get().await?;

    #[cfg(feature = "rate-limits")]
    limtr::Limtr::init(32).await?;

    // on the debug mode it might be more performant to skip initializing the Resources as they are not used every time.
    #[cfg(not(debug_assertions))] {
        handler::reload_resource_map().await;
    }

    let listener = TcpListener::bind(&cfg.server_address).await?;
    info!("Running the HTTP server on: {}", cfg.server_address);

    #[cfg(feature = "cloudflare")]
    let acceptor_cf = cloudflare::TlsAcceptorCF::init()
        .expect("Failed to initialize the Cloudflare TLS!");

    #[cfg(not(feature = "tls"))]
    warn!("TLS is disabled!");

    Console::new().await?.start();

    // Main program loop.
    // todo: add graceful shutdown
    loop {
        let (stream, addr) = listener.accept().await?;
        debug!("[{}] new connection", addr);

        #[cfg(not(feature = "cloudflare"))] {
            use hyper::server::conn::http1;

            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(|req| async move {
                        handler::service(req, addr).await
                    }))
                    .await
                {
                    error!("Error serving connection: {:?}", err);
                }
            });
        }

        #[cfg(feature = "cloudflare")] {
            use hyper_util::server::conn::auto::Builder;
            use hyper_util::rt::TokioExecutor;

            let acceptor = acceptor_cf.clone();

            tokio::spawn(async move {
                let tls_stream = match acceptor.accept(stream).await {
                    Ok(tls_stream) => tls_stream,
                    Err(err) => {
                        #[cfg(debug_assertions)]
                        error!("Failed to perform a TLS handshake: {:#?}", err);

                        // to disable warning on release build
                        drop(err);

                        return;
                    }
                };

                if let Err(err) = Builder::new(TokioExecutor::new())
                    .serve_connection(TokioIo::new(tls_stream), service_fn(|req| async move {
                        handler::service(req, addr).await
                    }))
                    .await
                {
                    error!("Error serving connection: {:#?}", err);
                }
            });
        }
    }
}
