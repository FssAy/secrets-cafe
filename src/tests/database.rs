use crate::database::Database;
use crate::database::types::ModTier;

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

#[tokio::test]
async fn create_mod() {
    let db = get_db().await;

    db.create_mod("mod1", "2137", ModTier::Dev).await.expect("Call failed!");

    // todo: fix, name uniques doesn't work
    // assert!(
    //     db.create_mod("mod1", "1123581321", ModTier::Admin).await.is_err(),
    //     "Created a second mod with the same name!",
    // );
}
