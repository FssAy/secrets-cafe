pub mod api;
mod resources;

use std::convert::Infallible;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};

pub(crate) use resources::reload_resource_map;

pub type Req = Request<hyper::body::Incoming>;
pub type Res = Response<Full<Bytes>>;

const HTML_FILE_EXTENSION: &str = ".html";

/// Main HTTP service. Each request made to the server lands here.
///
/// # Errors
/// This function should always return a Response in an `Ok` variant!
pub async fn service(req: Req) -> Result<Res, Infallible> {
    let req_path = req.uri().path().to_owned();
    let mut path = req_path.as_str();

    // This will help support local HTML links
    if path.ends_with(HTML_FILE_EXTENSION) {
        path = path.rsplit_once(HTML_FILE_EXTENSION).unwrap().0;
    }

    let mut res: Res;

    // API based requests
    if path.starts_with("/api/") {
        let api_path = path.split_once("/api").unwrap().1;
        res = api::handle_api_endpoint(api_path, req).await;

        res.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        return Ok(res);
    }

    // Resource based requests
    res = resources::handle_resource_endpoint(path, req).await;

    Ok(res)
}
