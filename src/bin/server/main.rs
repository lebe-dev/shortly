use std::{path::Path, sync::Arc};

use axum::{
    Router,
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use domain::config::model::{config::AppConfig, ports::AppConfigService};
use logger::get_logging_config;
use outbound::config::file::AppConfigServiceImpl;
use route::{config::get_app_config_route, version::get_version_route};
use rust_embed::Embed;
use server_lib::VERSION;

pub mod domain;
pub mod logger;
pub mod outbound;
pub mod route;

#[cfg(test)]
pub mod tests;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_service = AppConfigServiceImpl;

    let config_file = Path::new("config.yml");

    match config_service.load_from_file(&config_file) {
        Ok(app_config) => {
            let logging_config = get_logging_config(&app_config.log_level, &app_config.log_target);
            log4rs::init_config(logging_config).expect("unable to init logging configuration");

            let app_state = AppState {
                config: app_config.clone(),
            };

            let app = Router::new()
                .route("/api/version", get(get_version_route))
                .route("/api/config", get(get_app_config_route))
                .fallback(static_handler)
                //.layer(from_fn(auth_middleware))
                .with_state(Arc::new(app_state));

            let bind = format!("{}", &app_config.bind);

            let listener = tokio::net::TcpListener::bind(&bind)
                .await
                .expect("unable to bind tcp socket");

            println!("MY APP v{}", VERSION);
            println!("URL: http://{bind}");

            axum::serve(listener, app)
                .await
                .expect("unable to start web server");

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    }
}

static INDEX_HTML: &str = "index.html";

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

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
