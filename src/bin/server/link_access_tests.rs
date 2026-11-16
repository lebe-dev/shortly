use super::*;
use axum::{
    Extension,
    body::{Body, to_bytes},
    extract::ConnectInfo,
    http::{HeaderMap, Method, Request},
};
use domain::config::model::link_access::LinkAccessConfig;
use server_lib::domain::{
    auth::{model::User, ports::UserRepository},
    url::model::{Url, UrlAccess},
};
use tower::ServiceExt;

async fn setup(db: Database) -> Arc<AppState> {
    let migrations = assets::load_migrations_for_database(db.db_type()).unwrap();
    match &db {
        Database::Sqlite(db) => {
            let manager = SqliteMigrationManager::new(db.get_pool().clone(), migrations);
            manager.initialize().await.unwrap();
            manager.run_migrations().await.unwrap();
        }
        Database::Postgres(db) => {
            let manager = PostgresMigrationManager::new(db.get_pool().clone(), migrations);
            manager.initialize().await.unwrap();
            manager.run_migrations().await.unwrap();
        }
    }
    let mut config: AppConfig = config::Config::builder()
        .add_source(config::File::with_name("test-data/config.yml"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();
    config.auth.enabled = false;
    config.features.create_url.auth_only = false;
    let url_service = UrlServiceImplType::new(
        UrlServiceConfig {
            base_url: config.base_url.clone(),
            ttl_hours: 168,
            max_url_length: 2048,
            named_urls_enabled: true,
            named_url_min_length: 3,
            named_url_max_length: 50,
            reserved_names: vec![],
            max_urls_per_user: 100,
            max_urls_per_day: 10,
        },
        db.clone(),
    );
    Arc::new(AppState {
        config,
        url_service,
        auth_service: None,
        passkey_service: None,
        user_repository: db,
        start_time: 0,
        link_access: LinkAccessConfig {
            default_access: UrlAccess::Internal,
            allow_override: false,
            internal_networks: vec![
                "10.20.0.0/16".parse().unwrap(),
                "fd12:3456::/32".parse().unwrap(),
            ],
            trusted_proxies: vec!["127.0.0.1/32".parse().unwrap()],
            checks_per_minute: 30,
        },
        name_check_limiter: Arc::new(Default::default()),
    })
}

async fn sqlite_state() -> Arc<AppState> {
    setup(Database::Sqlite(
        server_lib::outbound::sqlite::init::Sqlite::new("sqlite::memory:")
            .await
            .unwrap(),
    ))
    .await
}

fn router(state: Arc<AppState>, user: Option<User>) -> Router {
    let app = Router::new()
        .route("/api/url/check", get(check_custom_name_route))
        .route("/api/url/{url_id}", get(get_short_url_by_id_route))
        .route("/api/url", post(generate_short_url_route))
        .route("/api/config", get(get_app_config_route))
        .route("/api/user/urls", get(list_user_urls_route))
        .fallback(static_handler)
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .layer(from_fn_with_state(
            state.clone(),
            route::link_access::network_middleware,
        ))
        .with_state(state);
    match user {
        Some(user) => app.layer(Extension(user)),
        None => app,
    }
}

async fn request(
    app: &Router,
    path: &str,
    ip: &str,
    forwarded: Option<&str>,
    json: Option<serde_json::Value>,
) -> (StatusCode, HeaderMap, Vec<u8>) {
    let mut request = Request::builder()
        .uri(path)
        .extension(ConnectInfo(ip.parse::<std::net::SocketAddr>().unwrap()));
    if let Some(value) = forwarded {
        request = request.header("x-forwarded-for", value);
    }
    let body = match json {
        Some(value) => {
            request = request
                .method(Method::POST)
                .header(header::CONTENT_TYPE, "application/json");
            Body::from(value.to_string())
        }
        None => Body::empty(),
    };
    let response = app
        .clone()
        .oneshot(request.body(body).unwrap())
        .await
        .unwrap();
    let (parts, body) = response.into_parts();
    (
        parts.status,
        parts.headers,
        to_bytes(body, 1024 * 1024).await.unwrap().to_vec(),
    )
}

async fn save(state: &AppState, id: &str, name: Option<&str>, access: UrlAccess) {
    state
        .user_repository
        .save(&Url {
            id: id.into(),
            custom_name: name.map(String::from),
            access,
            original_url: "https://example.com/private-target".into(),
            created: chrono::Utc::now().timestamp(),
            ttl: 0,
            user_id: None,
            last_accessed: Some(123),
        })
        .await
        .unwrap();
}

async fn access_scenarios(state: Arc<AppState>) {
    save(
        &state,
        "access-hash",
        Some("secret-name"),
        UrlAccess::Internal,
    )
    .await;
    save(
        &state,
        "public-hash",
        Some("public-name"),
        UrlAccess::Public,
    )
    .await;
    save(&state, "random-hash", None, UrlAccess::Public).await;
    let app = router(state.clone(), None);
    for path in [
        "/secret-name",
        "/SeCrEt-NaMe",
        "/access-hash",
        "/api/url/secret-name",
        "/api/url/SeCrEt-NaMe",
        "/api/url/access-hash",
    ] {
        let missing = if path.starts_with("/api/") {
            "/api/url/absent"
        } else {
            "/absent"
        };
        let expected = request(&app, missing, "198.51.100.1:1234", None, None).await;
        for (ip, forwarded) in [
            ("198.51.100.1:1234", None),
            ("198.51.100.1:1234", Some("10.20.0.1")),
            ("127.0.0.1:1234", None),
            ("127.0.0.1:1234", Some("10.20.0.1, 198.51.100.1")),
        ] {
            let denied = request(&app, path, ip, forwarded, None).await;
            assert_eq!(denied, expected, "{path} {ip} {forwarded:?}");
            assert!(!denied.1.contains_key(header::LOCATION));
            assert!(!String::from_utf8_lossy(&denied.2).contains("private-target"));
            assert_eq!(
                UrlRepository::find_by_id(&state.user_repository, "access-hash")
                    .await
                    .unwrap()
                    .unwrap()
                    .last_accessed,
                Some(123)
            );
        }
    }
    for path in [
        "/secret-name",
        "/SeCrEt-NaMe",
        "/access-hash",
        "/api/url/secret-name",
        "/api/url/SeCrEt-NaMe",
        "/api/url/access-hash",
    ] {
        for (ip, forwarded) in [
            ("10.20.0.1:1234", None),
            ("[fd12:3456::1]:1234", None),
            ("127.0.0.1:1234", Some("10.20.0.1")),
        ] {
            let allowed = request(&app, path, ip, forwarded, None).await;
            assert_eq!(allowed.1[header::CACHE_CONTROL], "private, no-store");
            if path.starts_with("/api/") {
                assert_eq!(allowed.0, StatusCode::OK);
                assert!(String::from_utf8_lossy(&allowed.2).contains("private-target"));
            } else {
                assert_eq!(allowed.0, StatusCode::FOUND);
                assert_eq!(
                    allowed.1[header::LOCATION],
                    "https://example.com/private-target"
                );
            }
        }
    }
    for path in [
        "/public-name",
        "/public-hash",
        "/random-hash",
        "/api/url/public-name",
        "/api/url/random-hash",
    ] {
        let result = request(&app, path, "198.51.100.1:1234", None, None).await;
        assert_eq!(
            result.0,
            if path.starts_with("/api/") {
                StatusCode::OK
            } else {
                StatusCode::FOUND
            }
        );
    }
    // Every repository listing preserves the stored access mode.
    assert!(
        state
            .url_service
            .list_all_urls()
            .await
            .unwrap()
            .iter()
            .any(|url| url.access == UrlAccess::Internal)
    );
}

#[tokio::test]
async fn sqlite_access_routes_hide_internal_links_and_preserve_public_links() {
    access_scenarios(sqlite_state().await).await;
}

#[tokio::test]
#[ignore = "requires TEST_POSTGRES_URL; creates and removes an isolated PostgreSQL schema"]
async fn postgres_access_routes() {
    let base_url = std::env::var("TEST_POSTGRES_URL").unwrap();
    let admin = sqlx::PgPool::connect(&base_url).await.unwrap();
    let schema = format!("link_access_{}", uuid::Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&admin)
        .await
        .unwrap();
    let mut connection_url = url::Url::parse(&base_url).unwrap();
    connection_url
        .query_pairs_mut()
        .append_pair("options", &format!("-csearch_path={schema}"));
    let db = Database::from_connection_string(connection_url.as_str())
        .await
        .unwrap();
    let Database::Postgres(ref postgres) = db else {
        unreachable!()
    };
    let test_pool = postgres.get_pool().clone();
    let mut migrations = assets::load_migrations_for_database("postgres").unwrap();
    migrations.remove("006_add_url_access.sql");
    let manager = PostgresMigrationManager::new(postgres.get_pool().clone(), migrations);
    manager.initialize().await.unwrap();
    manager.run_migrations().await.unwrap();
    sqlx::query("INSERT INTO urls (id, original_url, ttl, created, custom_name) VALUES ('legacy', 'https://example.com', 0, 0, 'legacy-name')").execute(postgres.get_pool()).await.unwrap();
    let state = setup(db).await;
    assert_eq!(
        state
            .url_service
            .find_by_id("legacy-name")
            .await
            .unwrap()
            .unwrap()
            .access,
        UrlAccess::Public
    );
    access_scenarios(state).await;
    test_pool.close().await;
    sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
        .execute(&admin)
        .await
        .unwrap();
}

async fn user(state: &AppState) -> User {
    state
        .user_repository
        .upsert(&server_lib::domain::auth::model::GitlabUserInfo {
            id: 9876,
            username: "owner".into(),
            email: None,
            avatar_url: None,
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn creation_defaults_overrides_and_owner_access() {
    let mut state = sqlite_state().await;
    let owner = user(&state).await;
    Arc::get_mut(&mut state).unwrap().config.auth.admin_users = Some(owner.username.clone());
    let app = router(state.clone(), Some(owner.clone()));
    let created = request(
        &app,
        "/api/url",
        "198.51.100.1:1234",
        None,
        Some(serde_json::json!({"url":"https://example.com", "name":"owner-link"})),
    )
    .await;
    assert_eq!(created.0, StatusCode::OK);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&created.2).unwrap()["access"],
        "internal"
    );
    let stored = state
        .url_service
        .find_by_id("owner-link")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stored.access, UrlAccess::Internal);
    assert_eq!(
        state.url_service.list_user_urls(owner.id).await.unwrap()[0].access,
        UrlAccess::Internal
    );
    assert_eq!(
        request(&app, "/api/url/owner-link", "198.51.100.1:1234", None, None)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    let override_body =
        serde_json::json!({"url":"https://example.com", "name":"override-link", "access":"public"});
    assert_eq!(
        request(
            &app,
            "/api/url",
            "198.51.100.1:1234",
            None,
            Some(override_body.clone())
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let generated = request(
        &app,
        "/api/url",
        "198.51.100.1:1234",
        None,
        Some(serde_json::json!({"url":"https://example.com"})),
    )
    .await;
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&generated.2).unwrap()["access"],
        "public"
    );
    let config = request(&app, "/api/config", "198.51.100.1:1234", None, None).await;
    let config: serde_json::Value = serde_json::from_slice(&config.2).unwrap();
    let admin_urls = config["admin"]["allUrls"].as_array().unwrap();
    assert_eq!(
        admin_urls
            .iter()
            .find(|url| url["id"] == stored.id)
            .unwrap()["access"],
        "internal"
    );
    assert_eq!(config["features"]["namedUrls"]["defaultAccess"], "internal");
    assert_eq!(
        config["features"]["namedUrls"]["allowAccessOverride"],
        false
    );
    assert!(!config.to_string().contains("10.20."));
    let listing = request(&app, "/api/user/urls", "198.51.100.1:1234", None, None).await;
    let listing: serde_json::Value = serde_json::from_slice(&listing.2).unwrap();
    assert_eq!(
        listing
            .as_array()
            .unwrap()
            .iter()
            .find(|url| url["id"] == stored.id)
            .unwrap()["access"],
        "internal"
    );
    drop(app);
    let settings = Arc::get_mut(&mut state).unwrap();
    settings.link_access.allow_override = true;
    settings.link_access.default_access = UrlAccess::Public;
    let app = router(state.clone(), Some(owner));
    assert_eq!(
        request(
            &app,
            "/api/url",
            "198.51.100.1:1234",
            None,
            Some(override_body)
        )
        .await
        .0,
        StatusCode::OK
    );
    // Changing the creation default never changes already stored links.
    assert_eq!(
        state
            .url_service
            .find_by_id("owner-link")
            .await
            .unwrap()
            .unwrap()
            .access,
        UrlAccess::Internal
    );
}

#[tokio::test]
async fn name_probes_share_a_limit_with_creation() {
    let mut state = sqlite_state().await;
    Arc::get_mut(&mut state)
        .unwrap()
        .link_access
        .checks_per_minute = 2;
    save(
        &state,
        "probe-hash",
        Some("taken-name"),
        UrlAccess::Internal,
    )
    .await;
    let owner = user(&state).await;
    let app = router(state, Some(owner));
    assert_eq!(
        request(
            &app,
            "/api/url/check?name=taken-name",
            "198.51.100.1:1234",
            None,
            None
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        request(
            &app,
            "/api/url/check?name=free-name",
            "198.51.100.1:1234",
            None,
            None
        )
        .await
        .0,
        StatusCode::OK
    );
    let blocked = request(
        &app,
        "/api/url/check?name=another-name",
        "198.51.100.1:1234",
        Some("10.20.0.1"),
        None,
    )
    .await;
    assert_eq!(blocked.0, StatusCode::TOO_MANY_REQUESTS);
    assert!(blocked.1.contains_key(header::RETRY_AFTER));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&blocked.2).unwrap()["code"],
        "name_check_rate_limited"
    );
    assert_eq!(
        request(
            &app,
            "/api/url",
            "198.51.100.1:1234",
            None,
            Some(serde_json::json!({"url":"https://example.com", "name":"yet-another"}))
        )
        .await
        .0,
        StatusCode::TOO_MANY_REQUESTS
    );
    assert_eq!(
        request(
            &app,
            "/api/url/check?name=free-name",
            "198.51.100.2:1234",
            None,
            None
        )
        .await
        .0,
        StatusCode::OK
    );
    // Ordinary generated links are not charged to the name-check budget.
    assert_eq!(
        request(
            &app,
            "/api/url",
            "198.51.100.1:1234",
            None,
            Some(serde_json::json!({"url":"https://example.com"}))
        )
        .await
        .0,
        StatusCode::OK
    );
}

#[tokio::test]
async fn migration_preserves_old_links_and_is_idempotent() {
    let db = server_lib::outbound::sqlite::init::Sqlite::new("sqlite::memory:")
        .await
        .unwrap();
    let mut migrations = assets::load_migrations_for_database("sqlite").unwrap();
    let access_migration = migrations.remove("006_add_url_access.sql").unwrap();
    let manager = SqliteMigrationManager::new(db.get_pool().clone(), migrations);
    manager.initialize().await.unwrap();
    manager.run_migrations().await.unwrap();
    sqlx::query("INSERT INTO urls (id, original_url, ttl, created, custom_name) VALUES ('legacy', 'https://example.com', 0, 0, 'legacy-name')").execute(db.get_pool()).await.unwrap();
    let manager = SqliteMigrationManager::new(
        db.get_pool().clone(),
        [("006_add_url_access.sql".into(), access_migration)].into(),
    );
    manager.run_migrations().await.unwrap();
    manager.run_migrations().await.unwrap();
    assert_eq!(
        UrlRepository::find_by_id(&db, "legacy")
            .await
            .unwrap()
            .unwrap()
            .access,
        UrlAccess::Public
    );
}

#[tokio::test]
async fn public_lookup_works_with_creation_auth_required() {
    let mut state = sqlite_state().await;
    save(
        &state,
        "restricted",
        Some("internal-name"),
        UrlAccess::Internal,
    )
    .await;
    let config = &mut Arc::get_mut(&mut state).unwrap().config;
    config.auth.enabled = true;
    config.features.create_url.auth_only = true;
    let app = router(state, None);
    let missing = request(&app, "/api/url/absent", "198.51.100.1:1234", None, None).await;
    assert_eq!(missing.0, StatusCode::BAD_REQUEST);
    assert_eq!(
        request(
            &app,
            "/api/url/internal-name",
            "198.51.100.1:1234",
            None,
            None
        )
        .await,
        missing
    );
    assert_eq!(
        request(&app, "/api/url/internal-name", "10.20.0.1:1234", None, None)
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &app,
            "/api/url",
            "10.20.0.1:1234",
            None,
            Some(serde_json::json!({"url":"https://example.com"}))
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn internal_redirect_aliases_disable_caching() {
    let state = sqlite_state().await;
    save(
        &state,
        "private-id",
        Some("private-name"),
        UrlAccess::Internal,
    )
    .await;
    let app = router(state, None);
    for path in ["/private-name", "//private-name", "///private-id"] {
        let allowed = request(&app, path, "10.20.0.1:1234", None, None).await;
        assert_eq!(allowed.0, StatusCode::FOUND, "{path}");
        assert_eq!(
            allowed.1.get(header::CACHE_CONTROL).unwrap(),
            "private, no-store",
            "{path}"
        );
        let denied = request(&app, path, "198.51.100.1:1234", None, None).await;
        assert!(!denied.1.contains_key(header::LOCATION));
        assert_eq!(
            denied.1.get(header::CACHE_CONTROL).unwrap(),
            "private, no-store"
        );
    }
}

#[tokio::test]
async fn head_lookup_uses_the_same_network_policy_as_get() {
    let mut state = sqlite_state().await;
    save(&state, "head-id", Some("head-name"), UrlAccess::Internal).await;
    save(&state, "public-id", None, UrlAccess::Public).await;
    let config = &mut Arc::get_mut(&mut state).unwrap().config;
    config.auth.enabled = true;
    config.features.create_url.auth_only = true;
    let app = router(state, None);
    for (path, ip, expected) in [
        ("/api/url/head-name", "10.20.0.1:1234", StatusCode::OK),
        (
            "/api/url/head-id",
            "198.51.100.1:1234",
            StatusCode::BAD_REQUEST,
        ),
        ("/api/url/public-id", "198.51.100.1:1234", StatusCode::OK),
        ("/api/url/absent", "10.20.0.1:1234", StatusCode::BAD_REQUEST),
        (
            "/api/url/check?name=head-name",
            "10.20.0.1:1234",
            StatusCode::UNAUTHORIZED,
        ),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::HEAD)
                    .uri(path)
                    .extension(ConnectInfo(ip.parse::<std::net::SocketAddr>().unwrap()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), expected, "{path} {ip}");
    }
}
