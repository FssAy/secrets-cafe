use super::*;

impl Database {
    // todo: add error handling
    // todo: return id
    pub async fn create_post(&self, content: String) {
        let _ = self
            .query(surql::CREATE_POST)
            .bind(("content", content))
            .await
            .unwrap()
            .check()
            .unwrap()
    }
}
