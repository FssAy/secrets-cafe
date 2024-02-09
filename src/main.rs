#[macro_use]
extern crate tracing;

#[macro_use]
extern crate serde;

#[cfg(test)]
mod tests;

mod logs;
mod handler;
mod database;
mod console;

use std::net::SocketAddr;
use hyper::server::conn::http1;
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

    Console::new().await?.start();

    // Main program loop.
    // todo: add graceful shutdown
    loop {
        let (stream, addr) = listener.accept().await?;
        debug!("[{}] new connection", addr);

        // todo: add TLS support
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handler::service))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}
