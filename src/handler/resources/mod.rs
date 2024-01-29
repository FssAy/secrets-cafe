use super::*;

pub async fn handle_resource_endpoint(_res_path: &str, _req: Req) -> Res {
    Response::new(
        Full::new(
            Bytes::from("Hello World!")
        )
    )
}
