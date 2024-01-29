use super::*;

pub async fn handle_resource_endpoint(res_path: &str, req: Req) -> Res {
    Response::new(
        Full::new(
            Bytes::from("Hello World!")
        )
    )
}
