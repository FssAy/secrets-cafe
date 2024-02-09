use super::*;

pub struct Ping;

impl API for Ping {
    fn handle(&self, req: Req) -> ResFuture {
        let fut = async move {
            let response = PingResponse {
                method: req.method().to_string(),
                msg: "pong",
            };

            response.as_res()
        };

        ResFuture {
            handler: Box::pin(fut),
        }
    }
}
