#[cfg(debug_assertions)]
mod ping;

pub mod error;
mod post;
mod moderator;
mod responses;

pub use responses::AsRes;

use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use lazy_static::lazy_static;
use responses::*;
use limtr::Feature;
use crate::handler::api::error::api_error;
use super::*;

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
enum FeatureAPI {
    PostUpload,
}

impl Feature for FeatureAPI {
    fn into_feature(self) -> u16 {
        self as u16
    }
}

/// Future wrapper for the request's Response.
///
/// Used by the `API` trait.
struct ResFuture {
    pub handler: Pin<Box<dyn Future<Output=Res> + Send>>,
}

impl Future for ResFuture {
    type Output = Res;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut pinned = std::pin::pin!(&mut self.handler);
        pinned.as_mut().poll(cx)
    }
}

/// Trait implemented for every API endpoint handler.
///
/// More functionality might come, but idk.
trait API where Self: 'static {
    /// Creates a handler future that will return a Response to the Request.
    ///
    /// The future is wrapped with `ResFuture` struct as
    /// when trying to return a raw future the compiler bitched around.
    fn handle(&self, req: Req, addr: SocketAddr) -> ResFuture;

    /// Converts itself into trait object.
    fn into_obj(self) -> Box<dyn API + Sync> where Self: Sized, Self: Sync {
        Box::new(self)
    }
}

lazy_static! {
    /// Lazy loaded map of API endpoint handlers.
    ///
    /// It could be a good idea to force the initialization on startup,
    /// or somehow replace it with different technique.
    static ref API_ENDPOINTS: HashMap<&'static str, Box<dyn API + Sync>> = {
        let mut map = HashMap::new();

        #[cfg(debug_assertions)]
        map.insert("/ping", ping::Ping.into_obj());
        map.insert("/post", post::Post.into_obj());
        map.insert("/mod", moderator::Moderator.into_obj());

        map
    };
}

/// This function will call a specific API endpoint handler based on the given `api_path`.
pub async fn handle_api_endpoint(api_path: &str, req: Req, addr: SocketAddr) -> Res {
    if let Some(api_endpoint) = API_ENDPOINTS.get(api_path) {
        api_endpoint.handle(req, addr).await
    } else {
        debug!("Not Found");
        let err = api_error!(InvalidEndpoint);
        Response::builder()
            .status(err.code)
            .body(Full::new(Bytes::from(
                serde_json::to_string(&err).unwrap()
            )))
            .unwrap()
    }
}
