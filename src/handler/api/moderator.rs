use hyper::Method;
use crate::database::Database;
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
