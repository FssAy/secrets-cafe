use hyper::Method;
use crate::database::Database;
use crate::database::types::{SessionToken, TokenPack};
use crate::handler::api::error::ApiError;
use super::*;

pub struct Moderator;

impl Moderator {
    async fn handler(req: Req) -> Result<Res, ApiError> {
        match req.method() {
            &Method::GET => {
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
    fn handle(&self, req: Req) -> ResFuture {
        let fut = async move {
            Self::handler(req).await.unwrap_or_else(|err| err.into())
        };

        ResFuture {
            handler: Box::pin(fut)
        }
    }
}
