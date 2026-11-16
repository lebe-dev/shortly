use server_lib::domain::passkey::{
    model::{ChallengeOperation, PasskeyChallenge},
    ports::{PasskeyChallengeRepository, PasskeyCredentialRepository, WebauthnUserRepository},
};

use crate::support::{TestPostgres, user};

#[tokio::test]
async fn credentials_round_trip_update_and_enforce_ownership() {
    let test = TestPostgres::new().await;
    let db = &test.db;
    let alice = user(db, 1, "alice").await;
    let bob = user(db, 2, "bob").await;
    let credential = PasskeyCredentialRepository::create(
        db,
        alice.id,
        "credential",
        "{\"counter\":0}",
        "Laptop",
    )
    .await
    .unwrap();
    assert_eq!(credential.user_id, alice.id);
    assert_eq!(credential.credential_id, "credential");
    assert_eq!(credential.passkey, "{\"counter\":0}");
    assert_eq!(credential.name, "Laptop");
    assert_eq!(credential.last_used_at, None);
    assert_eq!(
        db.find_by_credential_id("credential").await.unwrap(),
        Some(credential.clone())
    );
    assert_eq!(
        PasskeyCredentialRepository::find_by_user_id(db, alice.id)
            .await
            .unwrap(),
        std::slice::from_ref(&credential)
    );
    assert!(
        PasskeyCredentialRepository::find_by_user_id(db, bob.id)
            .await
            .unwrap()
            .is_empty()
    );
    db.update_passkey("credential", "{\"counter\":1}", 5_000_000_000)
        .await
        .unwrap();
    let updated = db
        .find_by_credential_id("credential")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated.passkey, "{\"counter\":1}");
    assert_eq!(updated.last_used_at, Some(5_000_000_000));
    let duplicate = PasskeyCredentialRepository::create(db, bob.id, "credential", "{}", "Phone")
        .await
        .unwrap_err();
    assert!(duplicate.as_database_error().unwrap().is_unique_violation());
    assert_eq!(
        PasskeyCredentialRepository::delete(db, bob.id, credential.id)
            .await
            .unwrap(),
        0
    );
    assert_eq!(
        PasskeyCredentialRepository::count_by_user_id(db, alice.id)
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        PasskeyCredentialRepository::delete(db, alice.id, credential.id)
            .await
            .unwrap(),
        1
    );
    assert!(
        db.find_by_credential_id("credential")
            .await
            .unwrap()
            .is_none()
    );
    PasskeyCredentialRepository::create(db, alice.id, "another", "{}", "Phone")
        .await
        .unwrap();
    PasskeyCredentialRepository::create(db, bob.id, "bobs", "{}", "Phone")
        .await
        .unwrap();
    assert_eq!(db.delete_by_user_id(alice.id).await.unwrap(), 1);
    assert_eq!(
        PasskeyCredentialRepository::count_by_user_id(db, bob.id)
            .await
            .unwrap(),
        1
    );
}

#[tokio::test]
async fn challenge_is_consumed_once_under_concurrent_requests() {
    let test = TestPostgres::new().await;
    let challenge = PasskeyChallenge {
        id: "login".into(),
        user_id: None,
        operation: ChallengeOperation::Authentication,
        state: "{\"challenge\":\"state\"}".into(),
        created_at: 5_000_000_000,
        expires_at: 5_000_000_300,
    };
    PasskeyChallengeRepository::create(&test.db, &challenge)
        .await
        .unwrap();
    let (first, second) = tokio::join!(test.db.take("login"), test.db.take("login"));
    let results = [first.unwrap(), second.unwrap()];
    assert_eq!(results.iter().flatten().count(), 1);
    assert_eq!(results.into_iter().flatten().next(), Some(challenge));
    assert!(test.db.take("login").await.unwrap().is_none());
}

#[tokio::test]
async fn expired_challenges_are_removed_and_webauthn_handles_are_unique() {
    let test = TestPostgres::new().await;
    let db = &test.db;
    let alice = user(db, 1, "alice").await;
    let bob = user(db, 2, "bob").await;
    assert_eq!(db.find_webauthn_id(alice.id).await.unwrap(), None);
    db.set_webauthn_id(alice.id, "stable-handle").await.unwrap();
    assert_eq!(
        db.find_webauthn_id(alice.id).await.unwrap().as_deref(),
        Some("stable-handle")
    );
    assert_eq!(
        db.find_by_webauthn_id("stable-handle").await.unwrap(),
        Some(alice.clone())
    );
    assert!(db.find_by_webauthn_id("unknown").await.unwrap().is_none());
    let duplicate = db
        .set_webauthn_id(bob.id, "stable-handle")
        .await
        .unwrap_err();
    assert!(duplicate.as_database_error().unwrap().is_unique_violation());
    for (id, expires_at) in [("expired", 99), ("boundary", 100), ("active", 101)] {
        PasskeyChallengeRepository::create(
            db,
            &PasskeyChallenge {
                id: id.into(),
                user_id: Some(alice.id),
                operation: ChallengeOperation::Registration,
                state: "{}".into(),
                created_at: 1,
                expires_at,
            },
        )
        .await
        .unwrap();
    }
    PasskeyChallengeRepository::delete_expired(db, 100)
        .await
        .unwrap();
    assert!(db.take("expired").await.unwrap().is_none());
    for id in ["boundary", "active"] {
        let challenge = db.take(id).await.unwrap().unwrap();
        assert_eq!(challenge.user_id, Some(alice.id));
        assert_eq!(challenge.operation, ChallengeOperation::Registration);
    }
}
