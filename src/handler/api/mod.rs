use super::*;

pub async fn handle_api_endpoint(api_path: &str, req: Req) -> Res {
    Response::new(
        Full::new(
            Bytes::from("{}")
        )
    )
}
