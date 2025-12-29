use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::error;
use serde::Serialize;

use crate::SharedAppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
}

pub async fn health_route(State(state): State<Arc<SharedAppState>>) -> impl IntoResponse {
    let result = sqlx::query("SELECT COUNT(*) FROM _migrations")
        .fetch_one(state.user_repository.get_pool())
        .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(HealthResponse {
                status: "healthy".to_string(),
                database: "ok".to_string(),
            }),
        )
            .into_response(),
        Err(e) => {
            error!("health check failed - database error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HealthResponse {
                    status: "unhealthy".to_string(),
                    database: "error".to_string(),
                }),
            )
                .into_response()
        }
    }
}
