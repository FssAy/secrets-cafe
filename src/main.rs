#[macro_use]
extern crate tracing;

#[macro_use]
extern crate serde;

mod logs;
mod handler;

use std::net::SocketAddr;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logs::init();

    let server_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(&server_addr).await?;
    info!("Running the HTTP server on: {}", server_addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        debug!("[{}] new connection", addr);

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
