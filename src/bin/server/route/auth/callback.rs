use axum::{
    extract::{Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::Deserialize;
use server_lib::domain::auth::ports::AuthService;
use std::sync::Arc;
use time::Duration;

use crate::AppState;

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
    state: String,
}

pub async fn callback_route(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CallbackQuery>,
) -> impl IntoResponse {
    if !state.config.auth.enabled {
        return (StatusCode::NOT_FOUND, "Authentication not enabled").into_response();
    }

    let auth_service = match &state.auth_service {
        Some(service) => service,
        None => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    match auth_service.complete_oauth(&query.code, &query.state).await {
        Ok((_user, session)) => {
            let cookie = Cookie::build(("session_token", session.token))
                .path("/")
                .max_age(Duration::days(30))
                .same_site(SameSite::Lax)
                .http_only(true)
                .secure(false) // Set to true in production with HTTPS
                .build();

            (
                StatusCode::FOUND,
                [
                    (header::SET_COOKIE, cookie.to_string()),
                    (header::LOCATION, "/".to_string()),
                ],
            )
                .into_response()
        }
        Err(e) => {
            log::error!("OAuth callback failed: {:?}", e);
            Redirect::temporary("/login?error=auth_failed").into_response()
        }
    }
}
