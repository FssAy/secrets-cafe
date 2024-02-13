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

/// Initializes the `ResourceMap`.
///
/// This might be dangerous if resources are not initialized at the start of the program and accessed at the same time.
///
/// # Errors
/// Will return an error message if resources couldn't be loaded.
///
/// # Panics
/// This function will panic if called for a second time.
async fn init_resource_map() -> anyhow::Result<&'static RwLock<ResourceMap>> {
    let resource_map = loader::ResourceSettings::from_file()
        .await?
        .into_resource_map()
        .await?;

    RESOURCES.set(RwLock::new(resource_map)).unwrap();
    Ok(RESOURCES.get().unwrap())
}

/// Reloads the resource map.
///
/// This will load all the files from the disk and create a new `ResourceMap`.
/// It's public to allow resource reload from other parts of the system like console or API.
///
/// # Errors
/// Will return an error message when it couldn't load the resources.
pub(crate) async fn reload_resource_map() -> anyhow::Result<()> {
    let resources = if let Some(resources) = RESOURCES.get() {
        resources
    } else {
        init_resource_map().await?;
        return Ok(());
    };

    let resource_map = loader::ResourceSettings::from_file()
        .await?
        .into_resource_map()
        .await?;

    let mut lock = resources.write().await;
    *lock = resource_map;
    drop(lock);

    Ok(())
}

// todo: add proper 404 page
/// Creates a new 404 page response.
fn page_404() -> Res {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::from(
            "Page not found!"
        )))
        .unwrap()
}

/// Handles each resource based request.
///
/// This includes requests to ex:
/// - `/`
/// - `/scripts/scr.js`
/// - `/img/image.png`
///
/// ... etc
pub async fn handle_resource_endpoint(mut resource_path: &str, _req: Req) -> Res {
    let resources = if let Some(resources) = RESOURCES.get() {
        resources
    } else {
        if let Ok(resources) = init_resource_map().await {
            resources
        } else {
            return page_404();
        }
    };

    if resource_path == "/index" {
        resource_path = "/";
    }

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
        page_404()
    }
}
