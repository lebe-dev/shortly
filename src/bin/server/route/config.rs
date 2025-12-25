use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::http::StatusCode;
use axum::{Extension, extract::State, response::IntoResponse};
use log::error;

use crate::SharedAppState;
use crate::domain::config::model::dto::AppConfigDto;
use server_lib::domain::auth::model::User;
use server_lib::domain::url::ports::UrlService;

pub async fn get_app_config_route(
    State(state): State<Arc<SharedAppState>>,
    user: Option<Extension<User>>,
) -> impl IntoResponse {
    let mut dto: AppConfigDto = state.config.clone().into();

    // If user is authenticated, add consumption data
    if let Some(ref user_ext) = user {
        let user_id = user_ext.id;
        error!("Config requested by user_id: {}", user_id);

        // Calculate timestamp for 24 hours ago
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() as i64,
            Err(e) => {
                error!("Failed to get current timestamp: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                    .into_response();
            }
        };
        let day_ago = now - 86400;

        // Fetch total count
        match state.url_service.count_named_urls_by_user(user_id).await {
            Ok(total_count) => {
                error!("Total count for user {}: {}", user_id, total_count);
                dto.features.create_url.current_urls = Some(total_count as u32);
            }
            Err(e) => {
                error!(
                    "Failed to fetch total URL count for user {}: {:?}",
                    user_id, e
                );
                // Continue without total count - graceful degradation
            }
        }

        // Fetch daily count
        match state
            .url_service
            .count_named_urls_by_user_since(user_id, day_ago)
            .await
        {
            Ok(daily_count) => {
                error!("Daily count for user {}: {}", user_id, daily_count);
                dto.features.create_url.current_urls_today = Some(daily_count as u32);
            }
            Err(e) => {
                error!(
                    "Failed to fetch daily URL count for user {}: {:?}",
                    user_id, e
                );
                // Continue without daily count - graceful degradation
            }
        }
    }

    (StatusCode::OK, Json(dto)).into_response()
}
