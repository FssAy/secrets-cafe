mod loader;

use std::collections::HashMap;
use hyper::header::{CONTENT_TYPE, HeaderValue};
use hyper::StatusCode;
use once_cell::sync::OnceCell;
use tokio::sync::RwLock;
use super::*;

/// Hashmap of resource endpoints.
/// - key: resource URI path
/// - value: resource response data
type ResourceMap = HashMap<String, ResourceEndpoint>;

/// Static `RwLock` pointer to the `ResourceMap`.
///
/// The resources are not dynamic, but might be changed on an update.
/// To prevent shutting down the server, `RwLock` has been implemented.
static RESOURCES: OnceCell<RwLock<ResourceMap>> = OnceCell::new();

/// Resource response data.
#[derive(Debug, Clone)]
struct ResourceEndpoint {
    /// Content of the resource.
    /// It can be HTML file, JS script, image, or anything.
    pub blob: Vec<u8>,
    /// MIME type that will be added to the response header for this resource.
    pub mime: HeaderValue,
}

// todo: add error handling
/// Initializes the `ResourceMap`.
///
/// # Panics
/// This function will panic if called for a second time or if there was an error while loading the resources.
async fn init_resource_map() -> &'static RwLock<ResourceMap> {
    let resource_map = loader::ResourceSettings::from_file()
        .await
        .into_resource_map()
        .await;

    RESOURCES.set(RwLock::new(resource_map)).unwrap();
    RESOURCES.get().unwrap()
}

// todo: add error handling
/// Reloads the resource map.
///
/// This will load all the files from the disk and create a new `ResourceMap`.
/// It's public to allow resource reload from other parts of the system like console or API.
///
/// # Panics
/// Will panic on any IO issue.
pub(crate) async fn reload_resource_map() {
    let resources = if let Some(resources) = RESOURCES.get() {
        resources
    } else {
        init_resource_map().await;
        return;
    };

    let resource_map = loader::ResourceSettings::from_file()
        .await
        .into_resource_map()
        .await;

    let mut lock = resources.write().await;
    *lock = resource_map;
    drop(lock);
}

/// Handles each resource based request.
///
/// This includes requests to ex:
/// - `/`
/// - `/scripts/scr.js`
/// - `/img/image.png`
///
/// ... etc
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
