use http_body_util::BodyExt;
use hyper::Method;
use crate::config::Config;
use crate::database::Database;
use crate::database::types::{PostState, PostTable, SessionToken, TokenPack};
use crate::handler::api::error::ApiError;
use super::*;

const DEFAULT_REJECTION_REASON: &str = "Not provided.";

pub struct Post;

impl Post {
    async fn handler(req: Req, addr: SocketAddr) -> Result<Res, ApiError> {
        match req.method() {
            &Method::POST => {
                let body_max_size = Config::get().await.unwrap().body_max_size;
                let mut body = Vec::new();
                let mut body_stream = req.into_body();
                while let Some(frame) = body_stream.frame().await {
                    let frame = frame.map_err(|_| ApiError::InvalidBody)?;
                    if let Ok(data) = frame.into_data() {
                        if body.len() + data.len() > body_max_size {
                            return Err(ApiError::InvalidBody);
                        }
                        body.extend(data.to_vec());
                    }
                }

                #[cfg(feature = "rate-limits")]
                {
                    let ratelimit = Limtr::update_limit(
                        addr.ip(),
                        FeatureAPI::PostUpload,
                        1800,  // 30 minutes
                        2,
                    ).await?;

                    if ratelimit != 0 {
                        return Err(ApiError::TooManyRequests {
                            limit: ratelimit,
                        });
                    }
                }

                let body_parsed = String::from_utf8(body)
                    .map_err(|_| ApiError::InvalidBody)?;

                let db: Database = Database::get().await.unwrap();

                Ok(PostResponse {
                    code: db.create_post(body_parsed).await?
                }.as_res())
            }
            &Method::GET => {
                let db: Database = Database::get().await.unwrap();

                let headers = req.headers();

                // Get a post for verification
                if let Some(session) = headers
                    .get("session")
                    .map(|value| value.to_str().map_err(|_| ApiError::InvalidHeader("session".into())))
                {
                    let session = session?;
                    let _ = SessionToken::verify(
                        TokenPack::unpack(session.to_string())?
                    ).await?;

                    let post_table: PostTable = db
                        .get_post_unverified()
                        .await?;

                    return Ok(post_table.as_res())
                }

                let post_code = headers
                    .get("post_code")
                    .map(|value| value.to_str().unwrap_or_default())
                    .unwrap_or_default();

                let mut post_table = if post_code.is_empty() || post_code == "random" {
                    db.get_post_random().await?
                } else {
                    db.get_post(post_code).await?
                };

                // todo: on Rejected or ForDeletion state return error with reason for this state and mod's name.
                if post_table.state != PostState::Approved {
                    post_table.content = String::from("--REDACTED--");
                }

                Ok(post_table.as_res())
            }
            &Method::PATCH => {
                let headers = req.headers();

                let mut header = "session";
                let session = headers
                    .get(header)
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?;

                header = "action";
                let action = headers
                    .get(header)
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?;

                header = "post_code";
                let post_code = headers
                    .get(header)
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?;

                let db: Database = Database::get().await.unwrap();

                let token = SessionToken::verify(
                    TokenPack::unpack(session.to_string())?
                ).await?;

                match action {
                    "approve" => {
                        db.verify_post(token.user_id, post_code).await?;
                    }
                    "reject" => {
                        let reason = headers
                            .get("reason")
                            .map(|value|
                                value.to_str().unwrap_or_else(|_| DEFAULT_REJECTION_REASON)
                            )
                            .unwrap_or_else(|| DEFAULT_REJECTION_REASON);

                        db.reject_post(token.user_id, post_code, reason).await?;
                    }
                    "delete" => {}
                    _ => return Err(ApiError::InvalidHeader(action.to_string().into())),
                }

                Ok(SingleResponse::Ok.as_res())
            }
            _ => Err(ApiError::MethodNotSupported)
        }
    }
}

impl API for Post {
    fn handle(&self, req: Req, addr: SocketAddr) -> ResFuture {
        let fut = async move {
            Self::handler(req, addr).await.unwrap_or_else(|err| err.into())
        };

        ResFuture {
            handler: Box::pin(fut)
        }
    }
}
