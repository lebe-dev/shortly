use crate::SharedAppState;
use axum::Json;
use axum::http::StatusCode;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use server_lib::domain::url::ports::UrlService;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CheckNameQuery {
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckNameResponse {
    available: bool,
}

pub async fn check_custom_name_route(
    State(state): State<Arc<SharedAppState>>,
    Query(query): Query<CheckNameQuery>,
) -> impl IntoResponse {
    match state
        .url_service
        .check_custom_name_available(&query.name)
        .await
    {
        Ok(available) => {
            if available {
                info!("custom name '{}' is available", query.name);
                (StatusCode::OK, Json(CheckNameResponse { available: true })).into_response()
            } else {
                info!("custom name '{}' is not available", query.name);
                (
                    StatusCode::CONFLICT,
                    Json(CheckNameResponse { available: false }),
                )
                    .into_response()
            }
        }
        Err(e) => {
            error!("error checking custom name: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
