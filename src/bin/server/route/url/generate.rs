use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse};
use log::error;
use serde::{Deserialize, Serialize};
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
    Json(request): Json<RegisterUrlRequest>,
) -> impl IntoResponse {
    match state.url_service.register_url(&request.url).await {
        Ok(url) => {
            let short_url = state.url_service.generate_short_url(&url).await;
            (StatusCode::OK, Json(RegisterUrlResponse { url: short_url })).into_response()
        }
        Err(e) => {
            error!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
