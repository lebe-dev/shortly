use axum::{
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use axum_extra::extract::cookie::Cookie;
use server_lib::domain::auth::ports::AuthService;
use server_lib::domain::url::audit::{AuditEventType, UrlAuditEvent};
use server_lib::domain::url::ports::UrlService;
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
        if let Ok(user) = auth_service.validate_session(&token).await {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let audit_event = UrlAuditEvent {
                id: None,
                event_type: AuditEventType::UserLogout,
                actor_user_id: user.id,
                target_user_id: user.id,
                url_name: None,
                created_at: timestamp,
            };

            if let Err(e) = state.url_service.record_audit_event(&audit_event).await {
                log::error!("Failed to record logout audit event: {:?}", e);
            }
        }

        let _ = auth_service.logout(&token).await;
    }

    let cookie = Cookie::build(("session_token", ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build();

    (StatusCode::OK, [(header::SET_COOKIE, cookie.to_string())]).into_response()
}
