use http_body_util::Full;
use hyper::body::Bytes;
use serde::Serialize;
use crate::handler::Res;

/// Converts self into a HTTP JSON Response.
pub trait AsRes: Serialize {
    fn as_res(&self) -> Res {
        Res::new(Full::new(Bytes::from(
            serde_json::to_string(self).unwrap()
        )))
    }
}

#[derive(Serialize)]
pub(super) struct PingResponse {
    pub method: String,
    pub msg: &'static str,
}

#[derive(Serialize)]
pub(super) struct PostResponse {
    pub code: String,
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub(super) enum SingleResponse {
    Ok,
}

impl AsRes for PingResponse {}
impl AsRes for PostResponse {}
impl AsRes for SingleResponse {}
