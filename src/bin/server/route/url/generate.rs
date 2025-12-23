use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use axum::{Extension, extract::State, response::IntoResponse};
use log::error;
use serde::{Deserialize, Serialize};
use server_lib::domain::auth::model::User;
use server_lib::domain::url::model::ShortUrlGenerationError;
use server_lib::domain::url::ports::UrlService;

use crate::SharedAppState;

#[derive(Deserialize, Clone, Debug)]
pub struct RegisterUrlRequest {
    url: String,
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
    let user_id = user.map(|u| u.id);

    match state.url_service.register_url(&request.url, user_id).await {
        Ok(url) => {
            let short_url = state.url_service.generate_short_url(&url).await;
            (StatusCode::OK, Json(RegisterUrlResponse { url: short_url })).into_response()
        }
        Err(e) => match e {
            ShortUrlGenerationError::InvalidOriginalUrl => {
                error!("invalid original url '{}'", request.url);
                (StatusCode::BAD_REQUEST).into_response()
            }
            _ => {
                error!("{:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
        },
    }
}
