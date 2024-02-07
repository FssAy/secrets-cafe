use super::*;
use crate::database::Database;
use crate::database::types::ModTier;

async fn get_db() -> Database {
    Database::get()
        .await
        .expect("Couldn't load the database instance!")
}

#[tokio::test]
async fn post() {
    let db = get_db().await;

    let post_content = "Super cool post!";

    let post_code = db.create_post(post_content).await.expect("Call failed!");
    let post = db.get_post(post_code).await.expect("Failed to get the post table!");
    assert_eq!(post.content, post_content, "Post content mismatch!");
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

#[test]
fn manage_posts() {
    with_stack_size(async {
        let db = get_db().await;

        let mod_none = db.create_mod("none", "123", ModTier::None).await.expect("Failed to create a mod!");
        let mod_verifier = db.create_mod("verifier", "123", ModTier::Verifier).await.expect("Failed to create a mod!");
        let mut post_code: String;

        assert!(db.get_post_random().await.is_err());

        post_code = db.create_post("Malicious Post!").await.expect("Failed to create a post!");
        db.reject_post(&mod_none, &post_code, "Post is malicious.").await.expect_err("Mod with tier None rejected a post!");
        db.reject_post(&mod_verifier, &post_code, "Post is malicious.").await.expect("Mod with tier Verifier failed to reject a post!");
        db.reject_post(&mod_verifier, &post_code, "Post is malicious.").await.expect_err("Rejected post that has been already rejected!");

        assert!(db.get_post_random().await.is_err());

        post_code = db.create_post("Simple Post!").await.expect("Failed to create a post!");
        db.verify_post(&mod_none, &post_code).await.expect_err("Mod with tier None verified a post!");
        db.verify_post(&mod_verifier, &post_code).await.expect("Mod with tier Verifier failed to verify a post!");
        db.verify_post(&mod_verifier, &post_code).await.expect_err("Verified post that has been already verified!");

        db.get_post_random().await.expect("Failed to get a random post!");
    });
}
