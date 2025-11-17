use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse};

use crate::SharedAppState;
use crate::domain::config::model::dto::AppConfigDto;

pub async fn get_app_config_route(State(state): State<Arc<SharedAppState>>) -> impl IntoResponse {
    let dto: AppConfigDto = state.config.clone().into();
    (StatusCode::OK, Json(dto)).into_response()
}
