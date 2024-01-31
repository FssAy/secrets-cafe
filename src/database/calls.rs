use surrealdb::sql::Thing;
use crate::database::types::ModTier;
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
}
