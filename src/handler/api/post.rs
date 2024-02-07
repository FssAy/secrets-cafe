use http_body_util::BodyExt;
use hyper::Method;
use crate::database::Database;
use crate::database::types::{PostState, SessionToken, TokenPack};
use crate::handler::api::error::ApiError;
use super::*;

const DEFAULT_REJECTION_REASON: &str = "Not provided.";

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
            &Method::GET => {
                let db: Database = Database::get().await.unwrap();

                let headers = req.headers();

                let post_code = headers
                    .get("post-code")
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| api_error!(InvalidHeader))?
                    .ok_or_else(|| api_error!(InvalidHeader))?;

                let mut post_table = db.get_post(post_code).await?;

                // todo: on Rejected or ForDeletion state return error with reason for this state and mod's name.
                if post_table.state != PostState::Approved {
                    post_table.content = String::from("--REDACTED--");
                }

                Ok(Res::new(Full::new(Bytes::from(
                    serde_json::to_string(&post_table).unwrap()
                ))))
            }
            &Method::PATCH => {
                let headers = req.headers();

                let session = headers
                    .get("session")
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| api_error!(InvalidHeader))?
                    .ok_or_else(|| api_error!(InvalidHeader))?;

                let action = headers
                    .get("action")
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| api_error!(InvalidHeader))?
                    .ok_or_else(|| api_error!(InvalidHeader))?;

                let post_id = headers
                    .get("post_id")
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| api_error!(InvalidHeader))?
                    .ok_or_else(|| api_error!(InvalidHeader))?;

                let db: Database = Database::get().await.unwrap();

                let token = SessionToken::verify(
                    TokenPack::unpack(session.to_string())?
                ).await?;

                match action {
                    "approve" => {
                        db.verify_post(token.user_id, post_id).await?;
                    }
                    "reject" => {
                        let reason = headers
                            .get("reason")
                            .map(|value|
                                value.to_str().unwrap_or_else(|_| DEFAULT_REJECTION_REASON)
                            )
                            .unwrap_or_else(|| DEFAULT_REJECTION_REASON);

                        db.reject_post(token.user_id, post_id, reason).await?;
                    }
                    "delete" => {}
                    _ => return Err(api_error!(InvalidHeader)),
                }

                // todo: return some other data
                Ok(Res::new(Full::new(Bytes::from("{}"))))
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
