use server_lib::domain::{
    auth::ports::UserRepository,
    url::{
        audit::{AuditEventType, AuditQueryParams, UrlAuditEvent},
        model::{ShortUrlGenerationError, UrlAccess, UserQuotas},
        ports::{UrlRepository, UrlService},
        service::{UrlServiceConfig, UrlServiceImpl},
    },
};

use crate::support::{TestPostgres, url, user};

#[tokio::test]
async fn urls_round_trip_nullable_fields_access_and_last_accessed() {
    let test = TestPostgres::new().await;
    let db = &test.db;
    let alice = user(db, 1, "alice").await;
    let anonymous = url("anonymous", None, 5_000_000_000);
    let mut named = url("named", Some(alice.id), 5_000_000_001);
    named.custom_name = Some("My-Link".into());
    named.access = UrlAccess::Internal;
    named.ttl = 0;
    for item in [&anonymous, &named] {
        db.save(item).await.unwrap();
        assert_eq!(
            UrlRepository::find_by_id(db, &item.id)
                .await
                .unwrap()
                .as_ref(),
            Some(item)
        );
    }
    assert_eq!(
        db.find_by_custom_name("mY-LINK").await.unwrap(),
        Some(named.clone())
    );
    assert!(
        UrlRepository::find_by_id(db, "missing")
            .await
            .unwrap()
            .is_none()
    );
    assert!(db.find_by_custom_name("missing").await.unwrap().is_none());
    assert_eq!(
        UrlRepository::find_by_user_id(db, alice.id).await.unwrap(),
        [named.clone()]
    );
    assert_eq!(
        UrlRepository::find_all(db).await.unwrap(),
        [named.clone(), anonymous.clone()]
    );
    named.last_accessed = Some(5_000_000_100);
    db.update_last_accessed(&named.id, named.last_accessed.unwrap())
        .await
        .unwrap();
    assert_eq!(
        db.find_by_custom_name("MY-LINK").await.unwrap(),
        Some(named)
    );
    db.delete_by_id("named").await.unwrap();
    assert!(
        UrlRepository::find_by_id(db, "named")
            .await
            .unwrap()
            .is_none()
    );
    assert_eq!(UrlRepository::find_all(db).await.unwrap(), [anonymous]);
}

#[tokio::test]
async fn duplicate_ids_and_case_insensitive_names_are_rejected() {
    let test = TestPostgres::new().await;
    let mut original = url("first", None, 1);
    original.custom_name = Some("My-Link".into());
    test.db.save(&original).await.unwrap();
    let duplicate = test.db.save(&original).await.unwrap_err();
    assert!(duplicate.as_database_error().unwrap().is_unique_violation());
    let mut collision = url("second", None, 2);
    collision.custom_name = Some("mY-lINK".into());
    let duplicate = test.db.save(&collision).await.unwrap_err();
    assert!(duplicate.as_database_error().unwrap().is_unique_violation());
    // NULL names are allowed for multiple generated links.
    for id in ["generated-one", "generated-two"] {
        test.db.save(&url(id, None, 3)).await.unwrap();
    }
    assert_eq!(UrlRepository::find_all(&test.db).await.unwrap().len(), 3);
}

#[tokio::test]
async fn cleanup_preserves_non_expiring_links_and_counts_are_scoped_by_user_and_time() {
    let test = TestPostgres::new().await;
    let db = &test.db;
    let alice = user(db, 1, "alice").await;
    let bob = user(db, 2, "bob").await;
    let now = chrono::Utc::now().timestamp();
    let expired = url("expired", Some(alice.id), now - 7200);
    let active = url("active", Some(alice.id), now);
    let mut permanent = url("permanent", Some(alice.id), now - 86400);
    permanent.ttl = 0;
    permanent.custom_name = Some("permanent".into());
    for item in [
        &expired,
        &active,
        &permanent,
        &url("bobs", Some(bob.id), now),
        &url("anon", None, now),
    ] {
        db.save(item).await.unwrap();
    }
    assert_eq!(
        UrlRepository::count_by_user_id(db, alice.id).await.unwrap(),
        3
    );
    assert_eq!(db.count_by_user_id_since(alice.id, now).await.unwrap(), 1);
    assert_eq!(
        db.count_by_user_id_since(alice.id, now + 1).await.unwrap(),
        0
    );
    UrlRepository::delete_expired(db).await.unwrap();
    assert!(
        UrlRepository::find_by_id(db, "expired")
            .await
            .unwrap()
            .is_none()
    );
    assert_eq!(
        UrlRepository::find_by_user_id(db, alice.id).await.unwrap(),
        [active, permanent]
    );
    assert_eq!(
        UrlRepository::count_by_user_id(db, bob.id).await.unwrap(),
        1
    );
}

#[tokio::test]
async fn quotas_distinguish_defaults_overrides_nulls_and_missing_users() {
    let test = TestPostgres::new().await;
    let alice = user(&test.db, 1, "alice").await;
    assert_eq!(
        test.db.find_user_quotas(alice.id).await.unwrap(),
        Some(UserQuotas {
            max_urls_per_user: Some(100),
            max_urls_per_day: Some(10),
        })
    );
    test.db
        .update_quotas(alice.id, Some(7), Some(2))
        .await
        .unwrap();
    assert_eq!(
        test.db.find_user_quotas(alice.id).await.unwrap(),
        Some(UserQuotas {
            max_urls_per_user: Some(7),
            max_urls_per_day: Some(2),
        })
    );
    sqlx::query("UPDATE users SET max_urls_per_user = NULL, max_urls_per_day = NULL WHERE id = $1")
        .bind(alice.id)
        .execute(test.db.get_pool())
        .await
        .unwrap();
    assert_eq!(
        test.db.find_user_quotas(alice.id).await.unwrap(),
        Some(UserQuotas {
            max_urls_per_user: None,
            max_urls_per_day: None,
        })
    );
    assert_eq!(test.db.find_user_quotas(-1).await.unwrap(), None);
}

#[tokio::test]
async fn url_service_enforces_database_quotas_for_named_and_generated_links() {
    let test = TestPostgres::new().await;
    let alice = user(&test.db, 1, "alice").await;
    test.db
        .update_quotas(alice.id, Some(2), Some(1))
        .await
        .unwrap();
    let service = UrlServiceImpl::new(
        UrlServiceConfig {
            base_url: "https://short.example.com".into(),
            ttl_hours: 24,
            max_url_length: 2048,
            named_urls_enabled: true,
            named_url_min_length: 3,
            named_url_max_length: 50,
            reserved_names: vec![],
            max_urls_per_user: 100,
            max_urls_per_day: 100,
        },
        test.db.clone(),
    );
    let generated = service
        .register_url("https://example.com/first", Some(alice.id), None)
        .await
        .unwrap();
    assert_eq!(
        UrlRepository::find_by_id(&test.db, &generated.id)
            .await
            .unwrap(),
        Some(generated.clone())
    );
    assert!(matches!(
        service
            .register_url(
                "https://example.com/second",
                Some(alice.id),
                Some("named-link".into())
            )
            .await,
        Err(ShortUrlGenerationError::RateLimitExceeded)
    ));
    test.db
        .update_quotas(alice.id, None, Some(10))
        .await
        .unwrap();
    let named = service
        .register_url(
            "https://example.com/second",
            Some(alice.id),
            Some("named-link".into()),
        )
        .await
        .unwrap();
    assert_eq!(
        test.db.find_by_custom_name("NAMED-LINK").await.unwrap(),
        Some(named)
    );
    assert!(matches!(
        service
            .register_url("https://example.com/third", Some(alice.id), None)
            .await,
        Err(ShortUrlGenerationError::UserLimitExceeded)
    ));
    service
        .register_url("https://example.com/anonymous", None, None)
        .await
        .unwrap();
    assert_eq!(
        UrlRepository::count_by_user_id(&test.db, alice.id)
            .await
            .unwrap(),
        2
    );
}

#[tokio::test]
async fn audit_filters_and_pagination_preserve_total_count_and_user_names() {
    let test = TestPostgres::new().await;
    let db = &test.db;
    let actor = user(db, 1, "admin").await;
    let target = user(db, 2, "alice").await;
    for (event_type, created_at, name) in [
        (AuditEventType::CreateUrl, 100, "release-one"),
        (AuditEventType::CreateUrl, 200, "release-two"),
        (AuditEventType::DeleteUrl, 300, "release-three"),
    ] {
        db.record_audit_event(&UrlAuditEvent {
            id: None,
            event_type,
            actor_user_id: actor.id,
            target_user_id: target.id,
            url_name: Some(name.into()),
            created_at,
        })
        .await
        .unwrap();
    }
    let mut params = AuditQueryParams {
        event_type: Some(AuditEventType::CreateUrl),
        actor_user_id: Some(actor.id),
        target_user_id: Some(target.id),
        url_name: Some("release".into()),
        username: Some("ali".into()),
        date_from: Some(100),
        date_to: Some(200),
        limit: 1,
        offset: 0,
    };
    let (events, total) = db.find_audit_events(&params).await.unwrap();
    assert_eq!(total, 2);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].created_at, 200);
    assert_eq!(events[0].event_type, AuditEventType::CreateUrl);
    assert_eq!(events[0].actor_user_id, actor.id);
    assert_eq!(events[0].target_user_id, target.id);
    assert_eq!(events[0].actor_username, "admin");
    assert_eq!(events[0].target_username, "alice");
    assert_eq!(events[0].url_name.as_deref(), Some("release-two"));
    params.offset = 1;
    let (events, total) = db.find_audit_events(&params).await.unwrap();
    assert_eq!(total, 2);
    assert_eq!(events[0].created_at, 100);
    params.offset = 2;
    let (events, total) = db.find_audit_events(&params).await.unwrap();
    assert_eq!(total, 2);
    assert!(events.is_empty());
    params.offset = 0;
    params.username = Some("absent".into());
    let (events, total) = db.find_audit_events(&params).await.unwrap();
    assert_eq!(total, 0);
    assert!(events.is_empty());
    let (events, total) = db
        .find_audit_events(&AuditQueryParams {
            limit: 10,
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(total, 3);
    assert_eq!(
        events.iter().map(|e| e.created_at).collect::<Vec<_>>(),
        [300, 200, 100]
    );
}
