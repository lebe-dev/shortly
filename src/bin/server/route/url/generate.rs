use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use axum::{Extension, extract::State, response::IntoResponse};
use log::{error, info};
use serde::{Deserialize, Serialize};
use server_lib::domain::auth::model::User;
use server_lib::domain::url::model::ShortUrlGenerationError;
use server_lib::domain::url::ports::UrlService;

use crate::SharedAppState;

/// Machine-readable code returned when the daily URL quota is exhausted
pub const DAILY_LIMIT_EXCEEDED_CODE: &str = "daily_limit_exceeded";
/// Machine-readable code returned when the total URL quota is exhausted
pub const TOTAL_LIMIT_EXCEEDED_CODE: &str = "total_limit_exceeded";

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUrlRequest {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    access: Option<server_lib::domain::url::model::UrlAccess>,
}

#[derive(Serialize, Clone, Debug)]
pub struct RegisterUrlResponse {
    pub url: String,
    pub access: server_lib::domain::url::model::UrlAccess,
}

pub async fn generate_short_url_route(
    State(state): State<Arc<SharedAppState>>,
    user: Option<Extension<User>>,
    axum::Extension(client): axum::Extension<crate::route::link_access::ClientNetwork>,
    Json(request): Json<RegisterUrlRequest>,
) -> impl IntoResponse {
    info!("generate short url request..");
    let user_id = user.as_ref().map(|u| u.id);
    let access = match state
        .link_access
        .creation_access(request.name.is_some(), request.access)
    {
        Ok(access) => access,
        Err(message) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": message})),
            )
                .into_response();
        }
    };

    if request.name.is_some() {
        if let Err(retry) = state
            .name_check_limiter
            .check(client.ip, state.link_access.checks_per_minute)
        {
            return crate::route::link_access::too_many_checks(retry);
        }
        if !state.config.features.named_urls.enabled {
            error!("named URLs feature is disabled");
            return (StatusCode::BAD_REQUEST, "Named URLs feature is disabled").into_response();
        }

        if user.is_none() {
            error!("attempted to create named URL without authentication");
            return (
                StatusCode::UNAUTHORIZED,
                "Authentication required for named URLs",
            )
                .into_response();
        }
    }

    match state
        .url_service
        .register_url_with_access(&request.url, user_id, request.name, access)
        .await
    {
        Ok(url) => {
            let short_url = state.url_service.generate_short_url(&url).await;
            (
                StatusCode::OK,
                Json(RegisterUrlResponse {
                    url: short_url,
                    access: url.access,
                }),
            )
                .into_response()
        }
        Err(e) => match e {
            ShortUrlGenerationError::InvalidOriginalUrl => {
                error!("invalid original URL");
                (StatusCode::BAD_REQUEST, "Invalid URL").into_response()
            }
            ShortUrlGenerationError::InvalidCustomName(msg) => {
                error!("invalid custom name: {}", msg);
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": msg })),
                )
                    .into_response()
            }
            ShortUrlGenerationError::CustomNameExists => {
                error!("custom name already exists");
                (StatusCode::CONFLICT, "Custom name already exists").into_response()
            }
            ShortUrlGenerationError::CustomNameReserved => {
                error!("custom name is reserved");
                (StatusCode::CONFLICT, "Custom name is reserved").into_response()
            }
            ShortUrlGenerationError::RateLimitExceeded => {
                error!("daily URL limit exceeded");
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(serde_json::json!({
                        "code": DAILY_LIMIT_EXCEEDED_CODE,
                        "error": "Rate limit exceeded: too many URLs created today",
                    })),
                )
                    .into_response()
            }
            ShortUrlGenerationError::UserLimitExceeded => {
                error!("total URL limit exceeded");
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(serde_json::json!({
                        "code": TOTAL_LIMIT_EXCEEDED_CODE,
                        "error": "User URL limit exceeded",
                    })),
                )
                    .into_response()
            }
            _ => {
                error!("failed to register URL");
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
        },
    }
}
