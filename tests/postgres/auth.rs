use server_lib::domain::auth::{
    model::GitlabUserInfo,
    ports::{SessionRepository, UserRepository},
};

use crate::support::{TestPostgres, user};

#[tokio::test]
async fn user_upsert_updates_profile_and_preserves_identity_and_quotas() {
    let test = TestPostgres::new().await;
    let db = &test.db;
    let original = user(db, 5_000_000_000, "alice").await;
    assert_eq!(original.max_urls_per_user, 100);
    assert_eq!(original.max_urls_per_day, 10);
    db.update_quotas(original.id, Some(200), Some(20))
        .await
        .unwrap();
    let updated = db
        .upsert(&GitlabUserInfo {
            id: original.gitlab_id,
            username: "renamed".into(),
            email: None,
            avatar_url: Some("https://example.com/avatar.png".into()),
        })
        .await
        .unwrap();
    assert_eq!(updated.id, original.id);
    assert_eq!(updated.created_at, original.created_at);
    assert_eq!(updated.username, "renamed");
    assert_eq!(updated.email, None);
    assert_eq!(
        updated.avatar_url.as_deref(),
        Some("https://example.com/avatar.png")
    );
    assert_eq!(
        (updated.max_urls_per_user, updated.max_urls_per_day),
        (200, 20)
    );
    assert_eq!(
        UserRepository::find_by_id(db, original.id).await.unwrap(),
        Some(updated)
    );
    assert_eq!(UserRepository::find_all(db).await.unwrap().len(), 1);
    assert!(db.find_by_gitlab_id(-1).await.unwrap().is_none());
}

#[tokio::test]
async fn users_are_sorted_and_partial_quota_updates_preserve_other_limit() {
    let test = TestPostgres::new().await;
    user(&test.db, 1, "zoe").await;
    let alice = user(&test.db, 2, "alice").await;
    let updated = test
        .db
        .update_quotas(alice.id, Some(12), None)
        .await
        .unwrap();
    assert_eq!(
        (updated.max_urls_per_user, updated.max_urls_per_day),
        (12, 10)
    );
    let updated = test
        .db
        .update_quotas(alice.id, None, Some(3))
        .await
        .unwrap();
    assert_eq!(
        (updated.max_urls_per_user, updated.max_urls_per_day),
        (12, 3)
    );
    let users = UserRepository::find_all(&test.db).await.unwrap();
    assert_eq!(
        users
            .iter()
            .map(|u| u.username.as_str())
            .collect::<Vec<_>>(),
        ["alice", "zoe"]
    );
    assert!(matches!(
        test.db.update_quotas(-1, Some(1), None).await,
        Err(sqlx::Error::RowNotFound)
    ));
}

#[tokio::test]
async fn sessions_round_trip_expire_and_logout_independently() {
    let test = TestPostgres::new().await;
    let db = &test.db;
    let alice = user(db, 1, "alice").await;
    let now = chrono::Utc::now().timestamp();
    let session = SessionRepository::create(db, alice.id, "expired", Some(now - 1))
        .await
        .unwrap();
    assert!(session.id > 0);
    assert_eq!(session.token, "expired");
    assert_eq!(session.user_id, alice.id);
    assert_eq!(session.expires_at, Some(now - 1));
    SessionRepository::create(db, alice.id, "boundary", Some(now))
        .await
        .unwrap();
    SessionRepository::create(db, alice.id, "active", Some(now + 3600))
        .await
        .unwrap();
    SessionRepository::create(db, alice.id, "permanent", None)
        .await
        .unwrap();
    db.update_last_used("active", now + 10).await.unwrap();
    assert_eq!(
        db.find_by_token("active")
            .await
            .unwrap()
            .unwrap()
            .last_used_at,
        now + 10
    );
    SessionRepository::delete_expired(db, now).await.unwrap();
    assert!(db.find_by_token("expired").await.unwrap().is_none());
    for token in ["boundary", "active", "permanent"] {
        assert!(db.find_by_token(token).await.unwrap().is_some());
    }
    SessionRepository::delete(db, "active").await.unwrap();
    assert!(db.find_by_token("active").await.unwrap().is_none());
    assert!(db.find_by_token("permanent").await.unwrap().is_some());
    let duplicate = SessionRepository::create(db, alice.id, "permanent", None)
        .await
        .unwrap_err();
    assert!(duplicate.as_database_error().unwrap().is_unique_violation());
    let unknown_user = SessionRepository::create(db, -1, "orphan", None)
        .await
        .unwrap_err();
    assert!(
        unknown_user
            .as_database_error()
            .unwrap()
            .is_foreign_key_violation()
    );
}
