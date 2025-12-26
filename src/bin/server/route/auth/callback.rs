use axum::{
    extract::{Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::Deserialize;
use server_lib::domain::auth::ports::AuthService;
use server_lib::domain::url::audit::{AuditEventType, UrlAuditEvent};
use server_lib::domain::url::ports::UrlService;
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
        return (StatusCode::BAD_REQUEST, "Authentication not enabled").into_response();
    }

    let auth_service = match &state.auth_service {
        Some(service) => service,
        None => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    match auth_service.complete_oauth(&query.code, &query.state).await {
        Ok((user, session)) => {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let audit_event = UrlAuditEvent {
                id: None,
                event_type: AuditEventType::UserLogin,
                actor_user_id: user.id,
                target_user_id: user.id,
                url_name: None,
                created_at: timestamp,
            };

            if let Err(e) = state.url_service.record_audit_event(&audit_event).await {
                log::error!("Failed to record login audit event: {:?}", e);
            }

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
