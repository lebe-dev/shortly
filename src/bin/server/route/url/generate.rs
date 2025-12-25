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

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUrlRequest {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct RegisterUrlResponse {
    pub url: String,
}

pub async fn generate_short_url_route(
    State(state): State<Arc<SharedAppState>>,
    user: Option<Extension<User>>,
    Json(request): Json<RegisterUrlRequest>,
) -> impl IntoResponse {
    info!("generate short url request..");
    let user_id = user.as_ref().map(|u| u.id);

    if request.name.is_some() {
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
        .register_url(&request.url, user_id, request.name)
        .await
    {
        Ok(url) => {
            let short_url = state.url_service.generate_short_url(&url).await;
            (StatusCode::OK, Json(RegisterUrlResponse { url: short_url })).into_response()
        }
        Err(e) => match e {
            ShortUrlGenerationError::InvalidOriginalUrl => {
                error!("invalid original url '{}'", request.url);
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
                error!("rate limit exceeded");
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    "Rate limit exceeded: too many URLs created today",
                )
                    .into_response()
            }
            ShortUrlGenerationError::UserLimitExceeded => {
                error!("user URL limit exceeded");
                (StatusCode::TOO_MANY_REQUESTS, "User URL limit exceeded").into_response()
            }
            _ => {
                error!("{:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
        },
    }
}
