use surrealdb::sql::{Datetime, Thing};
use crate::handler::api::AsRes;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostTable {
    pub code: String,
    pub content: String,
    pub at: Datetime,
    pub state: PostState,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostTableDB {
    pub id: Thing,
    pub content: String,
    pub at: Datetime,
    pub state: PostState,
}

impl Into<PostTable> for PostTableDB {
    fn into(self) -> PostTable {
        PostTable {
            code: self.id.id.to_string(),
            content: self.content,
            at: self.at,
            state: self.state,
        }
    }
}

impl AsRes for PostTable {}
