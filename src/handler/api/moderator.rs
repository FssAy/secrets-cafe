use hyper::Method;
use limtr::Limtr;
use crate::database::Database;
use crate::database::types::{SessionToken, TokenPack};
use crate::handler::api::error::ApiError;
use super::*;

pub struct Moderator;

impl Moderator {
    async fn handler(req: Req, addr: SocketAddr) -> Result<Res, ApiError> {
        match req.method() {
            &Method::GET => {
                let ratelimit = Limtr::update_limit(
                    addr.ip(),
                    FeatureAPI::ModLogin,
                    300,  // 5 minute
                    3,
                ).await?;

                if ratelimit != 0 {
                    // todo: return the ratelimit value
                    return Err(api_error!(TooManyRequests));
                }

                let headers = req.headers();

                let login = headers
                    .get("login")
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| api_error!(InvalidHeader))?
                    .ok_or_else(|| api_error!(InvalidHeader))?;

                let pass = headers
                    .get("pass")
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| api_error!(InvalidHeader))?
                    .ok_or_else(|| api_error!(InvalidHeader))?;

                let db: Database = Database::get().await.unwrap();
                let session = db.create_mod_session(login, pass).await?;

                Ok(Res::new(Full::new(Bytes::from(
                    serde_json::to_string(&session).unwrap()
                ))))
            }
            &Method::PATCH => {
                let headers = req.headers();

                let session = headers
                    .get("session")
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| api_error!(InvalidHeader))?
                    .ok_or_else(|| api_error!(InvalidHeader))?;

                let pass = headers
                    .get("pass")
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| api_error!(InvalidHeader))?
                    .ok_or_else(|| api_error!(InvalidHeader))?;

                let new_pass = headers
                    .get("new_pass")
                    .map(|value| value.to_str().ok())
                    .ok_or_else(|| api_error!(InvalidHeader))?
                    .ok_or_else(|| api_error!(InvalidHeader))?;

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
            _ => Err(api_error!(MethodNotSupported))
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
