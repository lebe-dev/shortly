use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::error;
use serde::Serialize;
use server_lib::outbound::database::DatabasePool;

use crate::SharedAppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
}

pub async fn health_route(State(state): State<Arc<SharedAppState>>) -> impl IntoResponse {
    let pool = state.user_repository.get_pool();

    let is_ok = match pool {
        DatabasePool::Sqlite(ref p) => sqlx::query("SELECT COUNT(*) FROM _migrations")
            .fetch_one(p)
            .await
            .is_ok(),
        DatabasePool::Postgres(ref p) => sqlx::query("SELECT COUNT(*) FROM _migrations")
            .fetch_one(p)
            .await
            .is_ok(),
    };

    if is_ok {
        (
            StatusCode::OK,
            Json(HealthResponse {
                status: "healthy".to_string(),
                database: "ok".to_string(),
            }),
        )
            .into_response()
    } else {
        error!("health check failed - database error");
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
