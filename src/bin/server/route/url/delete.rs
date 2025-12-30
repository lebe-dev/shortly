use std::sync::Arc;

use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::error;
use server_lib::domain::auth::model::User;
use server_lib::domain::url::model::DeleteUrlError;
use server_lib::domain::url::ports::UrlService;

use crate::SharedAppState;

pub async fn delete_url_route(
    State(state): State<Arc<SharedAppState>>,
    Extension(user): Extension<User>,
    Path(url_id): Path<String>,
) -> impl IntoResponse {
    let is_admin = is_user_admin(&user.username, &state.config);

    match state
        .url_service
        .delete_url(&url_id, user.id, is_admin)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(e) => match e {
            DeleteUrlError::NotFound => {
                error!("URL not found: {}", url_id);
                StatusCode::NOT_FOUND
            }
            DeleteUrlError::Unauthorized => {
                error!("Unauthorized deletion attempt by user {}", user.id);
                StatusCode::FORBIDDEN
            }
            _ => {
                error!("Delete error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        },
    }
}

fn is_user_admin(username: &str, config: &crate::domain::config::model::config::AppConfig) -> bool {
    config
        .auth
        .admin_users
        .as_ref()
        .map(|admins| {
            admins
                .split(',')
                .map(|s| s.trim())
                .any(|admin| admin == username)
        })
        .unwrap_or(false)
}
