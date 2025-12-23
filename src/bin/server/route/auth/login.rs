use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use server_lib::domain::auth::ports::AuthService;
use std::sync::Arc;

use crate::AppState;

pub async fn login_route(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if !state.config.auth.enabled {
        return (StatusCode::NOT_FOUND, "Authentication not enabled").into_response();
    }

    let auth_service = match &state.auth_service {
        Some(service) => service,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Auth service not available",
            )
                .into_response();
        }
    };

    match auth_service.generate_oauth_url().await {
        Ok((auth_url, _state)) => {
            // In production, could store state in a cache or signed cookie for validation
            // For now, we rely on GitLab returning it and SameSite cookies for CSRF protection
            Redirect::temporary(&auth_url).into_response()
        }
        Err(e) => {
            log::error!("Failed to generate OAuth URL: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to initiate login",
            )
                .into_response()
        }
    }
}
