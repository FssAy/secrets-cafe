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
async fn moderator() {
    let db = get_db().await;

    let mod1_id = db.create_mod("mod1", "2137", ModTier::Dev).await.expect("Call failed!");

    assert!(
        db.create_mod("mod1", "1123581321", ModTier::Admin).await.is_err(),
        "Created a second mod with the same name!",
    );

    db.create_mod_session("mod1", "2137").await.expect("Failed to create mod session!");

    db.update_mod_tier("mod1", ModTier::Verifier).await.expect("Failed to update mod tier!");
    assert!(
        db.update_mod_tier(&mod1_id, ModTier::Dev).await.is_err(),
        "Updated mod by it's ID without the table name!"
    );
    db.update_mod_tier(format!("mod:{}", mod1_id), ModTier::Dev).await.expect("Failed to update mod tier!");

    db.update_mod_password("mod1", "2137", "1234").await.expect("Failed to update mod password!");
    // todo: check here if previously created session will be valid (it will be for the 0.X releases)
    db.create_mod_session("mod1", "1234").await.expect("Failed to create mod session after password change!");
}
