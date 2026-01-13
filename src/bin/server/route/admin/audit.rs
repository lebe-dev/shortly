use axum::{
    Extension, Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::error;
use serde::{Deserialize, Serialize};
use server_lib::domain::auth::model::User;
use server_lib::domain::url::audit::{AuditEventType, AuditQueryParams as DomainAuditQueryParams};
use server_lib::domain::url::ports::UrlService;
use std::sync::Arc;

use crate::AppState;
use crate::route::config::is_user_admin;

#[derive(Deserialize)]
pub struct AuditQueryRequest {
    #[serde(default = "default_page")]
    pub page: i64,

    #[serde(default = "default_per_page")]
    pub per_page: i64,

    pub event_type: Option<String>,
    pub user_id: Option<i64>,
    pub url_name: Option<String>,
    pub username: Option<String>,
    pub date_from: Option<i64>,
    pub date_to: Option<i64>,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditResponse {
    pub events: Vec<AuditEventDto>,
    pub total_count: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventDto {
    pub id: i64,
    pub event_type: String,
    pub actor_user_id: i64,
    pub actor_username: String,
    pub target_user_id: i64,
    pub target_username: String,
    pub url_name: Option<String>,
    pub created_at: i64,
}

pub async fn list_audit_events_route(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Query(params): Query<AuditQueryRequest>,
) -> impl IntoResponse {
    if !is_user_admin(&user.username, &state.config) {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "Forbidden",
                "message": "Admin access required"
            })),
        )
            .into_response();
    }

    let page = params.page.max(1);
    let per_page = params.per_page.clamp(1, 100);
    let offset = (page - 1) * per_page;

    let event_type = params.event_type.and_then(|et| match et.as_str() {
        "create_url" => Some(AuditEventType::CreateUrl),
        "delete_url" => Some(AuditEventType::DeleteUrl),
        "user_login" => Some(AuditEventType::UserLogin),
        "user_logout" => Some(AuditEventType::UserLogout),
        "user_quota_update" => Some(AuditEventType::UserQuotaUpdate),
        _ => None,
    });

    let query_params = DomainAuditQueryParams {
        event_type,
        actor_user_id: params.user_id,
        target_user_id: params.user_id,
        url_name: params.url_name,
        username: params.username,
        date_from: params.date_from,
        date_to: params.date_to,
        limit: per_page,
        offset,
    };

    match state.url_service.find_audit_events(query_params).await {
        Ok((events, total_count)) => {
            let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

            let dto_events: Vec<AuditEventDto> = events
                .into_iter()
                .map(|e| AuditEventDto {
                    id: e.id,
                    event_type: e.event_type.to_string(),
                    actor_user_id: e.actor_user_id,
                    actor_username: e.actor_username,
                    target_user_id: e.target_user_id,
                    target_username: e.target_username,
                    url_name: e.url_name,
                    created_at: e.created_at,
                })
                .collect();

            (
                StatusCode::OK,
                Json(AuditResponse {
                    events: dto_events,
                    total_count,
                    page,
                    per_page,
                    total_pages,
                }),
            )
                .into_response()
        }
        Err(e) => {
            error!("Failed to fetch audit events: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Internal Server Error",
                    "message": "Failed to fetch audit events"
                })),
            )
                .into_response()
        }
    }
}
