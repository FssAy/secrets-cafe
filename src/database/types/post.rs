use surrealdb::sql::Datetime;
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostTable {
    pub content: String,
    pub at: Datetime,
    pub state: PostState,
}
