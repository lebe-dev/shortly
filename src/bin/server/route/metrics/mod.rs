mod collector;
mod registry;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use prometheus::Encoder;
use server_lib::VERSION;
use std::sync::{Arc, Once};

use crate::AppState;

static INIT: Once = Once::new();

pub async fn metrics_route(
    State(state): State<Arc<AppState>>,
) -> Result<Response, (StatusCode, String)> {
    INIT.call_once(|| {
        registry::register_metrics();
    });

    if let Err(e) = collect_all_metrics(&state).await {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to collect metrics: {}", e),
        ));
    }

    let encoder = prometheus::TextEncoder::new();
    let metric_families = registry::REGISTRY.gather();
    let mut buffer = Vec::new();

    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to encode metrics: {}", e),
        ));
    }

    Ok((
        StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        buffer,
    )
        .into_response())
}

async fn collect_all_metrics(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let pool = state.user_repository.get_pool();

    let url_metrics = collector::collect_url_metrics(pool).await?;
    registry::URLS_TOTAL.set(url_metrics.total as f64);
    registry::URLS_LAST_CREATED.set(url_metrics.last_created as f64);
    registry::URLS_CUSTOM_NAMED.set(url_metrics.custom_named as f64);
    registry::URLS_EXPIRED.set(url_metrics.expired as f64);
    registry::URLS_CREATED_24H.set(url_metrics.created_24h as f64);
    registry::URLS_DELETED_24H.set(url_metrics.deleted_24h as f64);

    if state.auth_service.is_some() {
        let user_metrics = collector::collect_user_metrics(pool).await?;
        registry::USERS_TOTAL.set(user_metrics.total as f64);
        registry::USERS_ACTIVE_SESSIONS.set(user_metrics.active_sessions as f64);
        registry::USERS_LAST_LOGIN.set(user_metrics.last_login as f64);
    }

    let audit_metrics = collector::collect_audit_metrics(pool).await?;
    for metric in audit_metrics {
        registry::AUDIT_EVENTS_TOTAL
            .with_label_values(&[&metric.event_type])
            .inc_by(metric.count as f64);
        registry::AUDIT_LAST_EVENT
            .with_label_values(&[&metric.event_type])
            .set(metric.last_timestamp as f64);
    }

    registry::DB_POOL_SIZE.set(pool.size() as f64);
    registry::DB_POOL_IDLE.set(pool.num_idle() as f64);

    let now = chrono::Utc::now().timestamp();
    let uptime = now - state.start_time;
    registry::UPTIME.set(uptime as f64);
    registry::VERSION_INFO
        .with_label_values(&[VERSION])
        .set(1.0);

    let ttl_values = collector::collect_ttl_values(pool).await?;
    for ttl in ttl_values {
        let hours_remaining = (ttl - now) as f64 / 3600.0;
        if hours_remaining > 0.0 {
            registry::URLS_TTL_HISTOGRAM.observe(hours_remaining);
        }
    }

    Ok(())
}
