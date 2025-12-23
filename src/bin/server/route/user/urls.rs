use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use log::error;
use serde::Serialize;
use server_lib::domain::auth::model::User;
use server_lib::domain::url::ports::UrlService;

use crate::SharedAppState;

#[derive(Serialize)]
pub struct UserUrlResponse {
    pub id: String,
    pub url: String,
    pub original_url: String,
    pub created: i64,
    pub ttl: u32,
}

pub async fn list_user_urls_route(
    State(state): State<Arc<SharedAppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    match state.url_service.list_user_urls(user.id).await {
        Ok(urls) => {
            let response: Vec<UserUrlResponse> = urls
                .into_iter()
                .map(|url| UserUrlResponse {
                    id: url.id.clone(),
                    url: format!("{}/{}", state.config.base_url, url.id),
                    original_url: url.original_url,
                    created: url.created,
                    ttl: url.ttl,
                })
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            error!("failed to list user urls: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
