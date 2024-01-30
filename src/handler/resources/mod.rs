mod loader;

use std::collections::HashMap;
use hyper::header::{CONTENT_TYPE, HeaderValue};
use hyper::StatusCode;
use once_cell::sync::OnceCell;
use tokio::sync::RwLock;
use super::*;

type ResourceMap = HashMap<String, ResourceEndpoint>;

static RESOURCES: OnceCell<RwLock<ResourceMap>> = OnceCell::new();

#[derive(Debug, Clone)]
struct ResourceEndpoint {
    pub blob: Vec<u8>,
    pub mime: HeaderValue,
}

// todo: add error handling
async fn init_resource_map() -> &'static RwLock<ResourceMap> {
    let resource_map = loader::ResourceSettings::from_file()
        .await
        .into_resource_map()
        .await;

    RESOURCES.set(RwLock::new(resource_map)).unwrap();
    RESOURCES.get().unwrap()
}

pub async fn handle_resource_endpoint(resource_path: &str, _req: Req) -> Res {
    let resources = if let Some(resources) = RESOURCES.get() {
        resources
    } else {
        init_resource_map().await
    };

    if let Some(endpoint) = resources
        .read()
        .await
        .get(resource_path)
    {
        Response::builder()
            .header(CONTENT_TYPE, &endpoint.mime)
            .body(Full::new(Bytes::copy_from_slice(
                &endpoint.blob
            )))
            .unwrap()
    } else {
        // todo: add proper 404 page
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from(
                "Page not found!"
            )))
            .unwrap()
    }
}
