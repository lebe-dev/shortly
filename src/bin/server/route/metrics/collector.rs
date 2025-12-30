use sqlx::SqlitePool;

pub async fn collect_url_metrics(pool: &SqlitePool) -> Result<UrlMetrics, sqlx::Error> {
    let now = chrono::Utc::now().timestamp();
    let yesterday = now - 86400;

    // Total URLs
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM urls")
        .fetch_one(pool)
        .await?;

    // Last created
    let last_created: (Option<i64>,) = sqlx::query_as("SELECT MAX(created) FROM urls")
        .fetch_one(pool)
        .await?;

    // Custom named
    let custom_named: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM urls WHERE custom_name IS NOT NULL")
            .fetch_one(pool)
            .await?;

    // Expired
    let expired: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM urls WHERE ttl > 0 AND ttl < ?")
        .bind(now)
        .fetch_one(pool)
        .await?;

    // Last accessed
    let last_accessed: (Option<i64>,) = sqlx::query_as("SELECT MAX(last_accessed) FROM urls")
        .fetch_one(pool)
        .await?;

    // Deleted last 24h
    let deleted_24h: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM url_audit WHERE event_type = 'DeleteUrl' AND created_at >= ?",
    )
    .bind(yesterday)
    .fetch_one(pool)
    .await?;

    Ok(UrlMetrics {
        total: total.0,
        last_created: last_created.0,
        custom_named: custom_named.0,
        expired: expired.0,
        last_accessed: last_accessed.0,
        deleted_24h: deleted_24h.0,
    })
}

pub async fn collect_user_metrics(pool: &SqlitePool) -> Result<UserMetrics, sqlx::Error> {
    let now = chrono::Utc::now().timestamp();

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    let active_sessions: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM sessions WHERE expires_at IS NULL OR expires_at > ?")
            .bind(now)
            .fetch_one(pool)
            .await?;

    let last_login: (Option<i64>,) =
        sqlx::query_as("SELECT MAX(created_at) FROM url_audit WHERE event_type = 'UserLogin'")
            .fetch_one(pool)
            .await?;

    Ok(UserMetrics {
        total: total.0,
        active_sessions: active_sessions.0,
        last_login: last_login.0.unwrap_or(0),
    })
}

pub async fn collect_audit_metrics(pool: &SqlitePool) -> Result<Vec<AuditMetric>, sqlx::Error> {
    let rows: Vec<(String, i64, Option<i64>)> = sqlx::query_as(
        "SELECT event_type, COUNT(*) as count, MAX(created_at) as last_timestamp
         FROM url_audit
         GROUP BY event_type",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(event_type, count, last_ts)| AuditMetric {
            event_type,
            count,
            last_timestamp: last_ts.unwrap_or(0),
        })
        .collect())
}

pub async fn collect_ttl_values(pool: &SqlitePool) -> Result<Vec<i64>, sqlx::Error> {
    let rows: Vec<(i64,)> = sqlx::query_as("SELECT ttl FROM urls WHERE ttl > 0")
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter().map(|(ttl,)| ttl).collect())
}

pub struct UrlMetrics {
    pub total: i64,
    pub last_created: Option<i64>,
    pub custom_named: i64,
    pub expired: i64,
    pub last_accessed: Option<i64>,
    pub deleted_24h: i64,
}

pub struct UserMetrics {
    pub total: i64,
    pub active_sessions: i64,
    pub last_login: i64,
}

pub struct AuditMetric {
    pub event_type: String,
    pub count: i64,
    pub last_timestamp: i64,
}
