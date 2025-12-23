use axum::{
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use axum_extra::extract::cookie::Cookie;
use server_lib::domain::auth::ports::AuthService;
use std::sync::Arc;

use crate::{AppState, route::middleware::extract_session_token};

pub async fn logout_route(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !state.config.auth.enabled {
        return StatusCode::NOT_FOUND.into_response();
    }

    let auth_service = match &state.auth_service {
        Some(service) => service,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if let Some(token) = extract_session_token(&headers) {
        let _ = auth_service.logout(&token).await;
    }

    // Clear cookie
    let cookie = Cookie::build(("session_token", ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build();

    (StatusCode::OK, [(header::SET_COOKIE, cookie.to_string())]).into_response()
}
