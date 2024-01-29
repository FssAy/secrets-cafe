use std::pin::pin;
use super::*;

#[derive(Serialize)]
struct PingResponse {
    method: String,
    msg: &'static str,
}

pub struct Ping;

impl API for Ping {
    fn handle(&self, req: Req) -> ResFuture {
        let fut = async move {
            let response = PingResponse {
                method: req.method().to_string(),
                msg: "pong",
            };

            Res::new(Full::new(Bytes::from(
                serde_json::to_string(&response).unwrap()
            )))
        };

        ResFuture {
            handler: Box::pin(fut),
        }
    }
}
