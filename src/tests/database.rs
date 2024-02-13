use super::*;
use crate::database::Database;
use crate::database::types::{ModTier, SessionToken, TokenPack};

/// A simple function wrapper to get the database instance.
///
/// On the `test` build this function creates new in-memory instance each time it is called.
async fn get_db() -> Database {
    Database::get()
        .await
        .expect("Couldn't load the database instance!")
}

/// Tests basic post functionality.
#[tokio::test]
async fn post() {
    let db = get_db().await;
    let post_content = "Super cool post!";

    // check what happens when getting an unverified post on an empty database.
    assert!(db.get_post_unverified().await.is_err());

    // check if it is possible to create a post and get its content.
    let post_code = db.create_post(post_content).await.expect("Call failed!");
    let post = db.get_post(post_code).await.expect("Failed to get the post table!");
    assert_eq!(post.content, post_content, "Post content mismatch!");

    // check if we can get the post we've created by getting an unverified post.
    assert!(db.get_post_unverified().await.is_ok(), "Failed to get an unverified post even after creating one!");
}

/// Tests moderator accounts and everything it's related to.
#[tokio::test]
async fn moderator() {
    let db = get_db().await;
    let mod_name = "mod1";
    let mod_password = "2137";
    let mod_password_new = "1234";

    // check account creation.
    let mod1_id = db.create_mod(mod_name, mod_password, ModTier::Dev).await.expect("Call failed!");
    assert!(
        db.create_mod(mod_name, "1123581321", ModTier::Admin).await.is_err(),
        "Created a second mod with the same name!",
    );

    // check logging-in functionality.
    let session = db.create_mod_session(mod_name, mod_password).await.expect("Failed to create mod session!");
    SessionToken::verify(
        TokenPack::unpack(session).expect("Failed to unpack the session!")
    ).await.expect("Failed to verify the session token!");

    // check if it's possible to change the mod's tier.
    db.update_mod_tier(mod_name, ModTier::Verifier).await.expect("Failed to update mod tier!");
    assert!(
        db.update_mod_tier(&mod1_id, ModTier::Dev).await.is_err(),
        "Updated mod by it's ID without the table name!"
    );
    db.update_mod_tier(format!("mod:{}", mod1_id), ModTier::Dev).await.expect("Failed to update mod tier!");

    // check account "security"
    db.update_mod_password(&mod1_id, mod_password, mod_password_new).await.expect("Failed to update mod password!");
    // todo: check here if previously created session will be valid (it will be for the 0.X releases)
    db.create_mod_session(mod_name, mod_password_new).await.expect("Failed to create mod session after password change!");
}

/// Tests everything about managing the posts by a moderator.
#[test]
fn manage_posts() {
    with_stack_size(async {
        let db = get_db().await;

        let mod_none = db.create_mod("none", "123", ModTier::None).await.expect("Failed to create a mod!");
        let mod_verifier = db.create_mod("verifier", "123", ModTier::Verifier).await.expect("Failed to create a mod!");
        let mut post_code: String;

        // check what happens when we get a random verified post on an empty database.
        assert!(db.get_post_random().await.is_err());

        // check post rejection.
        post_code = db.create_post("Malicious Post!").await.expect("Failed to create a post!");
        db.reject_post(&mod_none, &post_code, "Post is malicious.").await.expect_err("Mod with tier None rejected a post!");
        db.reject_post(&mod_verifier, &post_code, "Post is malicious.").await.expect("Mod with tier Verifier failed to reject a post!");
        db.reject_post(&mod_verifier, &post_code, "Post is malicious.").await.expect_err("Rejected post that has been already rejected!");

        // check if rejected post can be returned by getting a random post.
        assert!(db.get_post_random().await.is_err());

        // check post verification.
        post_code = db.create_post("Simple Post!").await.expect("Failed to create a post!");
        db.verify_post(&mod_none, &post_code).await.expect_err("Mod with tier None verified a post!");
        db.verify_post(&mod_verifier, &post_code).await.expect("Mod with tier Verifier failed to verify a post!");
        db.verify_post(&mod_verifier, &post_code).await.expect_err("Verified post that has been already verified!");

        // check if verified post can be returned by getting a random post.
        db.get_post_random().await.expect("Failed to get a random post!");
    });
}
