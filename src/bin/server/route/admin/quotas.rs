use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use server_lib::domain::auth::model::User;
use server_lib::domain::auth::ports::UserRepository;
use server_lib::domain::url::audit::{AuditEventType, UrlAuditEvent};
use server_lib::domain::url::ports::UrlService;
use std::sync::Arc;

use crate::{AppState, route::config::is_user_admin};

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserQuotasRequest {
    /// Optional: new maximum URLs per user
    pub max_urls_per_user: Option<i32>,
    /// Optional: new maximum URLs per day
    pub max_urls_per_day: Option<i32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserQuotasResponse {
    pub id: i64,
    pub username: String,
    pub max_urls_per_user: i32,
    pub max_urls_per_day: i32,
    pub updated_at: i64,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

// ============================================================================
// ROUTE HANDLER
// ============================================================================

pub async fn update_user_quotas_route(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateUserQuotasRequest>,
) -> impl IntoResponse {
    if !is_user_admin(&user.username, &state.config) {
        return (
            StatusCode::FORBIDDEN,
            Json(ErrorResponse {
                error: "Forbidden".to_string(),
                message: "Admin access required".to_string(),
            }),
        )
            .into_response();
    }

    if payload.max_urls_per_user.is_none() && payload.max_urls_per_day.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Bad Request".to_string(),
                message: "At least one quota field must be provided".to_string(),
            }),
        )
            .into_response();
    }

    if let Some(val) = payload.max_urls_per_user {
        if val < 0 {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: "max_urls_per_user must be non-negative".to_string(),
                }),
            )
                .into_response();
        }
    }

    if let Some(val) = payload.max_urls_per_day {
        if val < 0 {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Bad Request".to_string(),
                    message: "max_urls_per_day must be non-negative".to_string(),
                }),
            )
                .into_response();
        }
    }

    match state
        .user_repository
        .update_quotas(user_id, payload.max_urls_per_user, payload.max_urls_per_day)
        .await
    {
        Ok(updated_user) => {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let audit_event = UrlAuditEvent {
                id: None,
                event_type: AuditEventType::UserQuotaUpdate,
                actor_user_id: user.id, // Admin who performed the action
                target_user_id: updated_user.id, // User whose quotas were changed
                url_name: None,
                created_at: timestamp,
            };

            if let Err(e) = state.url_service.record_audit_event(&audit_event).await {
                error!("Failed to record quota update audit event: {:?}", e);
            }

            info!(
                "Admin user {} updated quotas for user {} (per_user: {:?}, per_day: {:?})",
                user.id, user_id, payload.max_urls_per_user, payload.max_urls_per_day
            );

            (
                StatusCode::OK,
                Json(UpdateUserQuotasResponse {
                    id: updated_user.id,
                    username: updated_user.username,
                    max_urls_per_user: updated_user.max_urls_per_user,
                    max_urls_per_day: updated_user.max_urls_per_day,
                    updated_at: updated_user.updated_at,
                }),
            )
                .into_response()
        }
        Err(sqlx::Error::RowNotFound) => {
            error!("User {} not found for quota update", user_id);
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Not Found".to_string(),
                    message: format!("User with id {} not found", user_id),
                }),
            )
                .into_response()
        }
        Err(e) => {
            error!("Failed to update user quotas: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal Server Error".to_string(),
                    message: "Failed to update user quotas".to_string(),
                }),
            )
                .into_response()
        }
    }
}
