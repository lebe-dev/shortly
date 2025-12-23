use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use server_lib::domain::auth::ports::AuthService;
use std::sync::Arc;

use crate::{AppState, route::middleware::extract_session_token};

#[derive(Serialize)]
pub struct SessionResponse {
    authenticated: bool,
    user: Option<UserInfo>,
}

#[derive(Serialize)]
pub struct UserInfo {
    username: String,
    email: Option<String>,
    avatar_url: Option<String>,
}

pub async fn session_route(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !state.config.auth.enabled {
        return (
            StatusCode::OK,
            Json(SessionResponse {
                authenticated: false,
                user: None,
            }),
        )
            .into_response();
    }

    let auth_service = match &state.auth_service {
        Some(service) => service,
        None => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let token = match extract_session_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::OK,
                Json(SessionResponse {
                    authenticated: false,
                    user: None,
                }),
            )
                .into_response();
        }
    };

    match auth_service.validate_session(&token).await {
        Ok(user) => (
            StatusCode::OK,
            Json(SessionResponse {
                authenticated: true,
                user: Some(UserInfo {
                    username: user.username,
                    email: user.email,
                    avatar_url: user.avatar_url,
                }),
            }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::OK,
            Json(SessionResponse {
                authenticated: false,
                user: None,
            }),
        )
            .into_response(),
    }
}
