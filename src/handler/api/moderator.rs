use hyper::Method;
use crate::database::Database;
use crate::database::types::{SessionToken, TokenPack};
use crate::handler::api::error::ApiError;
use super::*;

pub struct Moderator;

impl Moderator {
    async fn handler(req: Req, addr: SocketAddr) -> Result<Res, ApiError> {
        match req.method() {
            &Method::GET => {
                #[cfg(feature = "rate-limits")]
                {
                    let ratelimit = limtr::Limtr::update_limit(
                        addr.ip(),
                        FeatureAPI::ModLogin,
                        300,  // 5 minute
                        3,
                    ).await?;

                    if ratelimit != 0 {
                        return Err(ApiError::TooManyRequests {
                            limit: ratelimit,
                        });
                    }
                }

                let headers = req.headers();

                let mut header = "login";
                let login = headers
                    .get(header)
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?;

                header = "pass";
                let pass = headers
                    .get(header)
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?;

                let db: Database = Database::get().await.unwrap();
                let session = db.create_mod_session(login, pass).await?;

                Ok(Res::new(Full::new(Bytes::from(
                    serde_json::to_string(&session).unwrap()
                ))))
            }
            &Method::PATCH => {
                let headers = req.headers();

                let mut header = "session";
                let session = headers
                    .get(header)
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?;

                header = "pass";
                let pass = headers
                    .get(header)
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?;

                header = "new_pass";
                let new_pass = headers
                    .get(header)
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?
                    .ok_or_else(|| ApiError::InvalidHeader(header.into()))?;

                let token = SessionToken::verify(
                    TokenPack::unpack(session.to_string())?
                ).await?;

                let db: Database = Database::get().await.unwrap();
                db.update_mod_password(
                    token.user_id,
                    pass,
                    new_pass
                ).await?;

                Ok(SingleResponse::Ok.as_res())
            }
            _ => Err(ApiError::MethodNotSupported)
        }
    }
}

impl API for Moderator {
    fn handle(&self, req: Req, addr: SocketAddr) -> ResFuture {
        let fut = async move {
            Self::handler(req, addr).await.unwrap_or_else(|err| err.into())
        };

        ResFuture {
            handler: Box::pin(fut)
        }
    }
}
