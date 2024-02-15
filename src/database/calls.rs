use rand::Rng;
use surrealdb::sql::{Id, Thing};
use crate::database::types::{ModTier, PostState, PostTable, PostTableDB, SessionToken};
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

    pub async fn get_post(&self, code: impl ToString) -> Result<PostTable, ApiError> {
        let post_table_id = Thing {
            tb: "post".into(),
            id: Id::String(code.to_string())
        };

        let post_table_db = self
            .query(surql::GET_POST)
            .bind(("post_id", post_table_id))
            .await?
            .take::<Option<PostTableDB>>(0)?
            .ok_or_else(|| api_error!(PostNotFound))?;

        Ok(post_table_db.into())
    }

    pub async fn get_post_random(&self) -> Result<PostTable, ApiError> {
        let position_current = self
            .query(surql::GET_POSITION_CURRENT)
            .await?
            .take::<Option<i64>>((0, "val"))?
            .ok_or_else(|| api_error!(DatabaseError))?;

        let position = if position_current.is_negative() {
            return Err(api_error!(NoPostsLeft));
        } else if position_current == 0 {
            0
        } else {
            rand::thread_rng().gen_range(0..position_current+1)
        };

        let post_table_db = self
            .query(surql::GET_POST_RANDOM)
            .bind(("position", position))
            .await?
            .take::<Option<PostTableDB>>(0)?
            .ok_or_else(|| api_error!(PostNotFound))?;

        Ok(post_table_db.into())
    }

    pub async fn get_post_unverified(&self) -> Result<PostTable, ApiError> {
        self.query(surql::GET_POST_UNVERIFIED)
            .bind(("unverified_state", PostState::Awaiting))
            .await?
            .take::<Option<PostTableDB>>(0)?
            .ok_or_else(|| api_error!(NoPostsLeft))
            .map(|post_table_db| post_table_db.into())
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

    // The session tokens are not secure enough, there might be a need to improve them later.
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

    pub async fn update_mod_tier(
        &self,
        input: impl AsRef<str>,
        new_tier: ModTier,
    ) -> Result<(), ApiError> {
        let mut query = self
            .query(surql::UPDATE_MOD_TIER)
            .bind(("tier", new_tier as u8));

        query = if let Some((table, id)) = input.as_ref().split_once(':') {
            query.bind(("input", Thing {
                tb: table.to_string(),
                id: Id::String(id.to_string()),
            } ))
        } else {
            query.bind(("input", input.as_ref()))
        };

        let mod_id = query
            .await?
            .take::<Option<Thing>>((0, "id"))?;

        match mod_id {
            None => Err(api_error!(ModNotFound)),
            Some(_) => Ok(())
        }
    }

    pub async fn update_mod_password(
        &self,
        id: impl ToString,
        password: impl AsRef<str>,
        password_new: impl AsRef<str>,
    ) -> Result<(), ApiError> {
        let mod_table_id = Thing {
            tb: "mod".to_string(),
            id: Id::String(id.to_string()),
        };

        let mut response = self
            .query(surql::GET_MOD_PHASH)
            .bind(("mod_id", &mod_table_id))
            .await?;

        let phash = response
            .take::<Option<String>>((0, "phash"))?
            .ok_or_else(|| api_error!(DatabaseError))?;

        let is_password_same = bcrypt::verify(password.as_ref(), phash.as_str())
            .unwrap_or(false);

        if !is_password_same {
            return Err(api_error!(InvalidPassword));
        }

        let phash_new = bcrypt::hash(
            password_new.as_ref(),
            bcrypt::DEFAULT_COST
        ).unwrap();

        let _ = self
            .query(surql::UPDATE_MOD_PHASH)
            .bind(("mod_id", mod_table_id))
            .bind(("phash", phash_new))
            .await?;

        Ok(())
    }

    pub async fn verify_post(
        &self,
        mod_id: impl ToString,
        post_id: impl ToString,
    ) -> Result<(), ApiError> {
        let mod_table_id = Thing {
            tb: "mod".into(),
            id: Id::String(mod_id.to_string()),
        };

        let post_table_id = Thing {
            tb: "post".into(),
            id: Id::String(post_id.to_string()),
        };

        let _ = self
            .query(surql::RELATE_MOD_VERIFIED)
            .bind(("mod_id", mod_table_id))
            .bind(("post_id", post_table_id))
            .bind(("verifier_tier", ModTier::Verifier as u8))
            .bind(("verified_state", PostState::Approved))
            .bind(("unverified_state", PostState::Awaiting))
            .await?
            .check()?;

        Ok(())
    }

    pub async fn reject_post(
        &self,
        mod_id: impl ToString,
        post_id: impl ToString,
        reason: impl AsRef<str>,
    ) -> Result<(), ApiError> {
        let mod_table_id = Thing {
            tb: "mod".into(),
            id: Id::String(mod_id.to_string()),
        };

        let post_table_id = Thing {
            tb: "post".into(),
            id: Id::String(post_id.to_string()),
        };

        let _ = self
            .query(surql::RELATE_MOD_REJECTED)
            .bind(("mod_id", mod_table_id))
            .bind(("post_id", post_table_id))
            .bind(("reason", reason.as_ref()))
            .bind(("verifier_tier", ModTier::Verifier as u8))
            .bind(("rejected_state", PostState::Rejected))
            .bind(("unverified_state", PostState::Awaiting))
            .await?
            .check()?;

        Ok(())
    }
}
