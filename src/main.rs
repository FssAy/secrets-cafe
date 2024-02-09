#[macro_use]
extern crate tracing;

#[macro_use]
extern crate serde;

#[cfg(test)]
mod tests;

#[cfg(feature = "tls")]
mod tls;

mod logs;
mod handler;
mod database;
mod console;

use std::net::SocketAddr;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use database::Database;
use console::Console;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logs::init();
    Database::get().await?;

    let server_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // on the debug mode it might be more performant to skip initializing the Resources as they are not used every time.
    #[cfg(not(debug_assertions))] {
        handler::reload_resource_map().await;
    }

    let listener = TcpListener::bind(&server_addr).await?;
    info!("Running the HTTP server on: {}", server_addr);

    #[cfg(feature = "tls")]
    let acceptor = tls::init().expect("Failed to initialize the TLS!");

    #[cfg(not(feature = "tls"))]
    warn!("TLS is disabled!");

    Console::new().await?.start();

    let service = service_fn(handler::service);

    // Main program loop.
    // todo: add graceful shutdown
    loop {
        let (stream, addr) = listener.accept().await?;
        debug!("[{}] new connection", addr);

        #[cfg(not(feature = "tls"))] {
            use hyper::server::conn::http1;

            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service)
                    .await
                {
                    error!("Error serving connection: {:?}", err);
                }
            });
        }

        #[cfg(feature = "tls")] {
            use hyper_util::server::conn::auto::Builder;
            use hyper_util::rt::TokioExecutor;

            let acceptor = acceptor.clone();

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
                    .serve_connection(TokioIo::new(tls_stream), service)
                    .await
                {
                    error!("Error serving connection: {:#?}", err);
                }
            });
        }
    }
}
