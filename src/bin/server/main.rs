use std::{path::Path, sync::Arc};

use anyhow::anyhow;

use axum::{
    Router,
    extract::State,
    http::{StatusCode, Uri, header},
    middleware::from_fn_with_state,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use domain::config::model::{config::AppConfig, ports::AppConfigService};
use log::{debug, error, info, warn};
use logger::get_logging_config;
use outbound::config::file::AppConfigServiceImpl;
use route::{
    auth::{
        callback::callback_route, login::login_route, logout::logout_route, session::session_route,
    },
    config::get_app_config_route,
    health::health_route,
    metrics::metrics_route,
    middleware::auth_middleware,
    version::get_version_route,
};
use rust_embed::Embed;
use server_lib::domain::url::service::BASE_RESERVED_NAMES;
use server_lib::{
    VERSION,
    domain::{
        auth::{ports::AuthService, service::AuthServiceImpl},
        url::{ports::UrlService, service::UrlServiceImpl as UrlServiceImplType},
    },
    outbound::{
        oauth::gitlab::GitlabOAuthClient,
        sqlite::{init::Sqlite, migration::MigrationManager},
    },
};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::route::{
    admin::{audit::list_audit_events_route, quotas::update_user_quotas_route},
    url::{
        check::check_custom_name_route, delete::delete_url_route,
        generate::generate_short_url_route, get::get_short_url_by_id_route,
    },
    user::urls::list_user_urls_route,
};

pub mod assets;
pub mod domain;
pub mod logger;
pub mod outbound;
pub mod route;

pub type SharedAppState = AppState;

#[derive(Clone)]
pub struct AppState {
    config: AppConfig,
    url_service: UrlServiceImplType<Sqlite>,
    auth_service: Option<AuthServiceImpl<Sqlite, Sqlite, GitlabOAuthClient>>,
    user_repository: Sqlite,
    start_time: i64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_service = AppConfigServiceImpl;

    let config_file = Path::new("config.yml");

    match config_service.load_from_file(&config_file) {
        Ok(app_config) => {
            let logging_config = get_logging_config(&app_config.log_level, &app_config.log_target);
            log4rs::init_config(logging_config).expect("unable to init logging configuration");

            info!("config: {}", app_config);

            match Sqlite::new(&app_config.db_cnn).await {
                Ok(db_pool) => {
                    let migrations = assets::load_migrations_from_assets()
                        .expect("failed to load migrations from assets");

                    let migration_manager =
                        MigrationManager::new(db_pool.get_pool().clone(), migrations);
                    if let Err(e) = migration_manager.initialize().await {
                        eprintln!("failed to initialize migration system: {}", e);
                        return Err(anyhow!("migration initialization error"));
                    }
                    if let Err(e) = migration_manager.run_migrations().await {
                        eprintln!("failed to run migrations: {}", e);
                        return Err(anyhow!("migration error"));
                    }

                    let url_service = UrlServiceImplType::new(
                        &app_config.base_url,
                        app_config.short_url.ttl,
                        app_config.short_url.max_length,
                        db_pool.clone(),
                        app_config.features.named_urls.enabled,
                        app_config.features.named_urls.min_length,
                        app_config.features.named_urls.max_length,
                        app_config.features.named_urls.reserved_names.clone(),
                        app_config.features.create_url.max_per_user,
                        app_config.features.create_url.max_per_day,
                    );

                    let auth_service = if app_config.auth.enabled {
                        let redirect_uri = format!("{}/api/auth/callback", app_config.base_url);

                        let oauth_client = GitlabOAuthClient::new(
                            app_config.auth.providers.gitlab.base_url.clone(),
                            app_config.auth.providers.gitlab.application_id.clone(),
                            app_config.auth.providers.gitlab.secret.clone(),
                            redirect_uri.clone(),
                        );

                        let auth_service = AuthServiceImpl::new(
                            db_pool.clone(),
                            db_pool.clone(),
                            oauth_client,
                            app_config.auth.providers.gitlab.base_url.clone(),
                            app_config.auth.providers.gitlab.application_id.clone(),
                            redirect_uri,
                            Some(30), // 30 day session TTL
                        );

                        Some(auth_service)
                    } else {
                        None
                    };

                    let app_state = AppState {
                        config: app_config.clone(),
                        url_service: url_service.clone(),
                        auth_service: auth_service.clone(),
                        user_repository: db_pool.clone(),
                        start_time: chrono::Utc::now().timestamp(),
                    };

                    // SCHEDULER - START

                    let sched = JobScheduler::new()
                        .await
                        .expect("scheduler initialization error");

                    let service = url_service.clone();

                    sched
                        .add(
                            Job::new_async(
                                app_config.scheduler.cleanup_expired_urls.clone().as_str(),
                                move |_, _| {
                                    let service = service.clone();

                                    Box::pin(async move {
                                        let _ = service.cleanup_expired_urls().await;
                                    })
                                },
                            )
                            .expect("unable to create cronjob for cleanup expired urls"),
                        )
                        .await
                        .expect("unable to add cleanup expired urls cronjob to scheduler");

                    if let Some(ref auth_svc) = auth_service {
                        let auth_svc_clone = auth_svc.clone();
                        sched
                            .add(
                                Job::new_async("0 0 * * * *", move |_, _| {
                                    // Every hour
                                    let service = auth_svc_clone.clone();
                                    Box::pin(async move {
                                        let _ = service.cleanup_expired_sessions().await;
                                    })
                                })
                                .expect("unable to create cronjob for cleanup expired sessions"),
                            )
                            .await
                            .expect("unable to add cleanup expired sessions cronjob to scheduler");
                    }

                    sched.start().await?;

                    // SCHEDULER - END

                    let app_state_arc = Arc::new(app_state);

                    let app = Router::new()
                        .route("/api/version", get(get_version_route))
                        .route("/api/health", get(health_route))
                        .route("/api/metrics", get(metrics_route))
                        .route("/api/config", get(get_app_config_route))
                        .route("/api/auth/login", get(login_route))
                        .route("/api/auth/callback", get(callback_route))
                        .route("/api/auth/session", get(session_route))
                        .route("/api/auth/logout", post(logout_route))
                        .route("/api/url/{url_id}", get(get_short_url_by_id_route))
                        .route("/api/url/{url_id}", axum::routing::delete(delete_url_route))
                        .route("/api/url", post(generate_short_url_route))
                        .route("/api/url/check", get(check_custom_name_route))
                        .route("/api/user/urls", get(list_user_urls_route))
                        .route("/api/user/{user_id}/quotas", post(update_user_quotas_route))
                        .route("/api/admin/audit", get(list_audit_events_route))
                        .fallback(static_handler)
                        .layer(from_fn_with_state(app_state_arc.clone(), auth_middleware))
                        .with_state(app_state_arc);

                    let bind = format!("{}", &app_config.bind);

                    let listener = tokio::net::TcpListener::bind(&bind)
                        .await
                        .expect("unable to bind tcp socket");

                    println!("SHORTLY v{}", VERSION);
                    println!("Bind: http://{bind}");
                    println!("URL: {}", app_config.base_url);

                    axum::serve(listener, app)
                        .await
                        .expect("unable to start web server");

                    Ok(())
                }
                Err(e) => {
                    eprintln!("database error: {}", e);
                    Err(anyhow!("database error"))
                }
            }
        }
        Err(e) => {
            eprintln!("failed to load configuration: {}", e);
            std::process::exit(1);
        }
    }
}

static INDEX_HTML: &str = "index.html";

// Use the same base reserved names as the URL service to ensure consistency
static RESERVED_FRONTEND_ROUTES: &[&str] = BASE_RESERVED_NAMES;

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => {
            warn!("index.html not found in embedded assets (dev mode?)");

            let html = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Shortly</title>
    <script>
        // In development, frontend runs on port 4200
        window.location.href = 'http://localhost:4200' + window.location.pathname + window.location.search;
    </script>
</head>
<body>
    <p>Redirecting to development server...</p>
    <p>If not redirected, <a href="http://localhost:4200">click here</a></p>
</body>
</html>"#;

            (StatusCode::OK, Html(html)).into_response()
        }
    }
}

async fn static_handler(State(state): State<Arc<SharedAppState>>, uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    debug!("static_handler: path = '{}'", path);

    if !RESERVED_FRONTEND_ROUTES.contains(&path)
        && !path.is_empty()
        && path != INDEX_HTML
        && !path.contains('.')
        && !path.contains('/')
    {
        info!("checking database for potential short URL: '{}'", path);

        match state.url_service.find_by_id(path).await {
            Ok(Some(url)) => {
                info!("redirecting short URL '{}' to '{}'", path, url.original_url);

                return (
                    StatusCode::FOUND, // 302 Temporary Redirect
                    [(header::LOCATION, url.original_url)],
                )
                    .into_response();
            }
            Ok(None) => {
                info!("short URL '{}' not found in database", path);
                return index_html().await;
            }
            Err(e) => {
                error!("error looking up short URL '{}': {:?}", path, e);
                return index_html().await;
            }
        }
        // If URL not found or error - continue with normal static file handling
    } else {
        if RESERVED_FRONTEND_ROUTES.contains(&path) {
            debug!(
                "path '{}' is a reserved frontend route, skipping short URL lookup",
                path
            );
        } else {
            debug!("path '{}' doesn't match short URL criteria", path);
        }
    }

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

#[derive(Embed)]
#[folder = "static/"]
struct Assets;
