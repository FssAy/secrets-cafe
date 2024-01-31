use crate::database::Database;

async fn get_db() -> Database {
    Database::get()
        .await
        .expect("Couldn't load the database instance!")
}

#[tokio::test]
async fn create_post() {
    let db = get_db().await;
    db.create_post("Super cool post!").await.expect("Call failed!");
}
