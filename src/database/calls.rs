use surrealdb::sql::Thing;
use crate::database::types::{ModTier, SessionToken};
use crate::handler::api::error::{api_error, ApiError};
use super::*;

impl Database {
    pub async fn create_post(&self, content: impl AsRef<str>) -> Result<String, ApiError> {
        let post_id = self
            .query(surql::CREATE_POST)
            .bind(("content", content.as_ref()))
            .await?
            .take::<Option<Thing>>((0, "id"))?
            .ok_or_else(|| api_error!(DatabaseError))?;

        Ok(post_id.id.to_raw())
    }

    pub async fn create_mod(
        &self,
        name: impl AsRef<str>,
        password: impl AsRef<str>,
        tier: ModTier
    ) -> Result<String, ApiError> {
        let phash = bcrypt::hash(
            password.as_ref(),
            bcrypt::DEFAULT_COST
        ).unwrap();

        let mod_id = self
            .query(surql::CREATE_MOD)
            .bind(("mod_name", name.as_ref()))
            .bind(("phash", phash))
            .bind(("tier", tier as u8))
            .await?
            .take::<Option<Thing>>((0, "id"))?
            .ok_or_else(|| api_error!(DatabaseError))?;

        Ok(mod_id.id.to_raw())
    }

    // I don't feel like the session tokens are secure enough, there might be a need to improve them later.
    pub async fn create_mod_session(
        &self,
        name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<String, ApiError> {
        let mut response = self
            .query(surql::GET_MOD_LOGIN_INFO)
            .bind(("mod_name", name.as_ref()))
            .await?;

        let phash = response.take::<Option<String>>((0, "phash"))?
            .ok_or_else(|| api_error!(DatabaseError))?;

        let is_password_same = bcrypt::verify(password.as_ref(), phash.as_str())
            .unwrap_or(false);

        if !is_password_same {
            return Err(api_error!(InvalidPassword));
        }

        let mod_id = response.take::<Option<Thing>>((0, "id"))?
            .map(|thing| thing.id.to_raw())
            .ok_or_else(|| api_error!(DatabaseError))?;

        let token = SessionToken::new(mod_id)
            .sign()
            .await
            .map_err(|_| {
                error!("CRITICAL! Signing session for [{}] failed! This should never happen, check your HMAC key!", name.as_ref());
                api_error!(DatabaseError)
            })?
            .pack();

        Ok(token)
    }
}
