use axum::{
    Extension, Json,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use log::error;
use serde::{Deserialize, Serialize};
use server_lib::domain::auth::model::User;
use server_lib::domain::passkey::model::{PasskeyCredential, PasskeyError};
use server_lib::domain::passkey::ports::PasskeyService;
use server_lib::domain::url::audit::{AuditEventType, UrlAuditEvent};
use server_lib::domain::url::ports::UrlService;
use std::sync::Arc;
use time::Duration;
use webauthn_rs::prelude::{PublicKeyCredential, RegisterPublicKeyCredential};

use crate::AppState;
use crate::route::config::is_user_admin;

const SESSION_COOKIE_DAYS: i64 = 30;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishRegistrationRequest {
    pub challenge_id: String,
    pub name: Option<String>,
    pub credential: RegisterPublicKeyCredential,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinishAuthenticationRequest {
    pub challenge_id: String,
    pub credential: PublicKeyCredential,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasskeyCredentialDto {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub username: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedPasskeysResponse {
    pub deleted: u64,
}

impl From<PasskeyCredential> for PasskeyCredentialDto {
    fn from(credential: PasskeyCredential) -> Self {
        PasskeyCredentialDto {
            id: credential.id,
            name: credential.name,
            created_at: credential.created_at,
            last_used_at: credential.last_used_at,
        }
    }
}

/// Begin a passwordless login. Available without a session.
pub async fn passkey_login_start_route(State(state): State<Arc<AppState>>) -> Response {
    let Some(service) = &state.passkey_service else {
        return passkey_disabled_response();
    };

    match service.start_authentication().await {
        Ok(start) => (StatusCode::OK, Json(start)).into_response(),
        Err(e) => error_response(e),
    }
}

/// Complete a passwordless login and open a session for the existing account.
pub async fn passkey_login_finish_route(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<FinishAuthenticationRequest>,
) -> Response {
    let Some(service) = &state.passkey_service else {
        return passkey_disabled_response();
    };

    let (user, session) = match service
        .finish_authentication(&payload.challenge_id, &payload.credential)
        .await
    {
        Ok(result) => result,
        Err(e) => return error_response(e),
    };

    record_audit_event(&state, AuditEventType::UserLogin, user.id).await;

    let cookie = Cookie::build(("session_token", session.token))
        .path("/")
        .max_age(Duration::days(SESSION_COOKIE_DAYS))
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(state.config.base_url.starts_with("https://"))
        .build();

    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie.to_string())],
        Json(LoginResponse {
            username: user.username,
        }),
    )
        .into_response()
}

/// List the passkeys of the current user.
pub async fn list_passkeys_route(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Response {
    let Some(service) = &state.passkey_service else {
        return passkey_disabled_response();
    };

    match service.list_credentials(user.id).await {
        Ok(credentials) => {
            let credentials: Vec<PasskeyCredentialDto> = credentials
                .into_iter()
                .map(PasskeyCredentialDto::from)
                .collect();

            (StatusCode::OK, Json(credentials)).into_response()
        }
        Err(e) => error_response(e),
    }
}

/// Begin registration of a new passkey for the current user.
pub async fn passkey_register_start_route(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> Response {
    let Some(service) = &state.passkey_service else {
        return passkey_disabled_response();
    };

    match service.start_registration(&user).await {
        Ok(start) => (StatusCode::OK, Json(start)).into_response(),
        Err(e) => error_response(e),
    }
}

/// Store the passkey created by the browser.
pub async fn passkey_register_finish_route(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<FinishRegistrationRequest>,
) -> Response {
    let Some(service) = &state.passkey_service else {
        return passkey_disabled_response();
    };

    match service
        .finish_registration(
            &user,
            &payload.challenge_id,
            &payload.credential,
            payload.name,
        )
        .await
    {
        Ok(credential) => {
            record_audit_event(&state, AuditEventType::PasskeyRegister, user.id).await;

            (
                StatusCode::CREATED,
                Json(PasskeyCredentialDto::from(credential)),
            )
                .into_response()
        }
        Err(e) => error_response(e),
    }
}

/// Delete one passkey of the current user.
pub async fn delete_passkey_route(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(credential_id): Path<i64>,
) -> Response {
    let Some(service) = &state.passkey_service else {
        return passkey_disabled_response();
    };

    match service.delete_credential(user.id, credential_id).await {
        Ok(()) => {
            record_audit_event(&state, AuditEventType::PasskeyDelete, user.id).await;

            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => error_response(e),
    }
}

/// Delete every passkey of a user. Administrators only.
pub async fn delete_user_passkeys_route(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(user_id): Path<i64>,
) -> Response {
    let Some(service) = &state.passkey_service else {
        return passkey_disabled_response();
    };

    if !is_user_admin(&user.username, &state.config) {
        return json_error(
            StatusCode::FORBIDDEN,
            "forbidden",
            "Administrator access is required",
        );
    }

    match service.delete_user_credentials(user_id).await {
        Ok(deleted) => {
            record_audit_event_for(&state, AuditEventType::PasskeyDelete, user.id, user_id).await;

            (StatusCode::OK, Json(DeletedPasskeysResponse { deleted })).into_response()
        }
        Err(e) => error_response(e),
    }
}

async fn record_audit_event(state: &Arc<AppState>, event_type: AuditEventType, user_id: i64) {
    record_audit_event_for(state, event_type, user_id, user_id).await;
}

async fn record_audit_event_for(
    state: &Arc<AppState>,
    event_type: AuditEventType,
    actor_user_id: i64,
    target_user_id: i64,
) {
    let event = UrlAuditEvent {
        id: None,
        event_type,
        actor_user_id,
        target_user_id,
        url_name: None,
        created_at: chrono::Utc::now().timestamp(),
    };

    if let Err(e) = state.url_service.record_audit_event(&event).await {
        error!("failed to record passkey audit event: {:?}", e);
    }
}

fn passkey_disabled_response() -> Response {
    json_error(
        StatusCode::NOT_FOUND,
        "passkey_disabled",
        "Passkey authentication is not enabled",
    )
}

fn error_response(error: PasskeyError) -> Response {
    let (status, code) = match error {
        PasskeyError::Disabled => (StatusCode::NOT_FOUND, "passkey_disabled"),
        PasskeyError::ChallengeNotFound | PasskeyError::ChallengeMismatch => {
            (StatusCode::BAD_REQUEST, "challenge_expired")
        }
        PasskeyError::UnknownUser => (StatusCode::UNAUTHORIZED, "unknown_account"),
        PasskeyError::CredentialNotFound => (StatusCode::NOT_FOUND, "passkey_not_found"),
        PasskeyError::CredentialAlreadyRegistered => {
            (StatusCode::CONFLICT, "passkey_already_registered")
        }
        PasskeyError::Webauthn(_) => (StatusCode::BAD_REQUEST, "webauthn_failed"),
        PasskeyError::DatabaseError(_) | PasskeyError::Unknown(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
        }
    };

    if status == StatusCode::INTERNAL_SERVER_ERROR {
        error!("passkey request failed: {}", error);
    }

    json_error(status, code, &error.to_string())
}

fn json_error(status: StatusCode, code: &str, message: &str) -> Response {
    (
        status,
        Json(serde_json::json!({
            "code": code,
            "message": message,
        })),
    )
        .into_response()
}
