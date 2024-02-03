use http_body_util::BodyExt;
use hyper::Method;
use crate::database::Database;
use crate::handler::api::error::ApiError;
use super::*;

#[derive(Serialize)]
struct PostResponse {
    code: String,
}

pub struct Post;

impl Post {
    async fn handler(req: Req) -> Result<Res, ApiError> {
        match req.method() {
            &Method::POST => {
                let db: Database = Database::get().await.unwrap();

                // todo: check body length
                // might be possible to crash the server with huge body request
                let body_bytes = req
                    .into_body()
                    .collect()
                    .await
                    .map_err(|_| api_error!(NetworkError))?
                    .to_bytes();

                let body_parsed = String::from_utf8(body_bytes.to_vec())
                    .map_err(|_| api_error!(InvalidBody))?;

                let response = PostResponse {
                    code: db.create_post(body_parsed).await?
                };

                Ok(Res::new(Full::new(Bytes::from(
                    serde_json::to_string(&response).unwrap()
                ))))
            }
            _ => Err(api_error!(MethodNotSupported))
        }
    }
}

impl API for Post {
    fn handle(&self, req: Req) -> ResFuture {
        let fut = async move {
            Self::handler(req).await.unwrap_or_else(|err| err.into())
        };

        ResFuture {
            handler: Box::pin(fut)
        }
    }
}
