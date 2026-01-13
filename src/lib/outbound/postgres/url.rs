use chrono::Utc;
use log::info;
use sqlx::postgres::PgRow;
use sqlx::{Postgres as SqlxPostgres, Row};

use crate::domain::url::{
    audit::{AuditEventType, AuditEventWithUser, AuditQueryParams, UrlAuditEvent},
    model::Url,
    ports::UrlRepository,
};

use super::init::Postgres;

impl UrlRepository for Postgres {
    async fn save(&self, url: &Url) -> Result<(), sqlx::Error> {
        sqlx::query::<SqlxPostgres>(
            r#"
            INSERT INTO urls (
                id,
                original_url,
                ttl,
                created,
                user_id,
                custom_name,
                last_accessed
            ) VALUES ($1, $2, $3, $4, $5, $6, $7);
            "#,
        )
        .bind(&url.id)
        .bind(&url.original_url)
        .bind(url.ttl as i32)
        .bind(url.created)
        .bind(url.user_id)
        .bind(&url.custom_name)
        .bind(url.last_accessed)
        .execute(self.get_pool())
        .await?;

        info!("url '{}' has been saved", url.id);

        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Url>, sqlx::Error> {
        let select_query = sqlx::query::<SqlxPostgres>("SELECT * FROM urls WHERE id=$1").bind(id);

        let url = select_query
            .map(|row: PgRow| Url {
                id: row.get("id"),
                original_url: row.get("original_url"),
                ttl: row.get::<i32, _>("ttl") as u32,
                created: row.get("created"),
                user_id: row.get("user_id"),
                custom_name: row.get("custom_name"),
                last_accessed: row.get("last_accessed"),
            })
            .fetch_optional(self.get_pool())
            .await?;

        Ok(url)
    }

    async fn find_by_custom_name(&self, custom_name: &str) -> Result<Option<Url>, sqlx::Error> {
        let select_query =
            sqlx::query::<SqlxPostgres>("SELECT * FROM urls WHERE LOWER(custom_name) = LOWER($1)")
                .bind(custom_name);

        let url = select_query
            .map(|row: PgRow| Url {
                id: row.get("id"),
                original_url: row.get("original_url"),
                ttl: row.get::<i32, _>("ttl") as u32,
                created: row.get("created"),
                user_id: row.get("user_id"),
                custom_name: row.get("custom_name"),
                last_accessed: row.get("last_accessed"),
            })
            .fetch_optional(self.get_pool())
            .await?;

        Ok(url)
    }

    async fn delete_expired(&self) -> Result<(), sqlx::Error> {
        let query =
            sqlx::query::<SqlxPostgres>("DELETE FROM urls WHERE ttl > 0 AND created + ttl < $1")
                .bind(Utc::now().timestamp());

        query.execute(self.get_pool()).await?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: i64) -> Result<Vec<Url>, sqlx::Error> {
        let query = sqlx::query::<SqlxPostgres>(
            "SELECT * FROM urls WHERE user_id = $1 ORDER BY created DESC",
        )
        .bind(user_id);

        let urls = query
            .map(|row: PgRow| Url {
                id: row.get("id"),
                original_url: row.get("original_url"),
                ttl: row.get::<i32, _>("ttl") as u32,
                created: row.get("created"),
                user_id: row.get("user_id"),
                custom_name: row.get("custom_name"),
                last_accessed: row.get("last_accessed"),
            })
            .fetch_all(self.get_pool())
            .await?;

        Ok(urls)
    }

    async fn delete_by_id(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query::<SqlxPostgres>("DELETE FROM urls WHERE id = $1")
            .bind(id)
            .execute(self.get_pool())
            .await?;

        info!("url '{}' has been deleted", id);

        Ok(())
    }

    async fn count_by_user_id(&self, user_id: i64) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM urls WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(self.get_pool())
            .await?;

        Ok(count)
    }

    async fn count_by_user_id_since(
        &self,
        user_id: i64,
        since_timestamp: i64,
    ) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM urls WHERE user_id = $1 AND created >= $2",
        )
        .bind(user_id)
        .bind(since_timestamp)
        .fetch_one(self.get_pool())
        .await?;

        Ok(count)
    }

    async fn record_audit_event(&self, event: &UrlAuditEvent) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO url_audit (
                event_type,
                actor_user_id,
                target_user_id,
                url_name,
                created_at
            ) VALUES ($1, $2, $3, $4, $5);
            "#,
        )
        .bind(event.event_type.to_string())
        .bind(event.actor_user_id)
        .bind(event.target_user_id)
        .bind(&event.url_name)
        .bind(event.created_at)
        .execute(self.get_pool())
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Url>, sqlx::Error> {
        let query = sqlx::query::<SqlxPostgres>("SELECT * FROM urls ORDER BY created DESC");

        let urls = query
            .map(|row: PgRow| Url {
                id: row.get("id"),
                original_url: row.get("original_url"),
                ttl: row.get::<i32, _>("ttl") as u32,
                created: row.get("created"),
                user_id: row.get("user_id"),
                custom_name: row.get("custom_name"),
                last_accessed: row.get("last_accessed"),
            })
            .fetch_all(self.get_pool())
            .await?;

        Ok(urls)
    }

    async fn find_audit_events(
        &self,
        params: &AuditQueryParams,
    ) -> Result<(Vec<AuditEventWithUser>, i64), sqlx::Error> {
        let mut where_clauses = Vec::new();
        let mut param_idx = 1usize;

        if params.event_type.is_some() {
            where_clauses.push(format!("ua.event_type = ${}", param_idx));
            param_idx += 1;
        }
        if params.actor_user_id.is_some() {
            where_clauses.push(format!("ua.actor_user_id = ${}", param_idx));
            param_idx += 1;
        }
        if params.target_user_id.is_some() {
            where_clauses.push(format!("ua.target_user_id = ${}", param_idx));
            param_idx += 1;
        }
        if params.url_name.is_some() {
            where_clauses.push(format!("ua.url_name LIKE ${}", param_idx));
            param_idx += 1;
        }
        if params.username.is_some() {
            where_clauses.push(format!(
                "(actor.username LIKE ${} OR target.username LIKE ${})",
                param_idx,
                param_idx + 1
            ));
            param_idx += 2;
        }
        if params.date_from.is_some() {
            where_clauses.push(format!("ua.created_at >= ${}", param_idx));
            param_idx += 1;
        }
        if params.date_to.is_some() {
            where_clauses.push(format!("ua.created_at <= ${}", param_idx));
            param_idx += 1;
        }

        let where_clause = if !where_clauses.is_empty() {
            format!(" WHERE {}", where_clauses.join(" AND "))
        } else {
            String::new()
        };

        let count_query = format!(
            "SELECT COUNT(*) FROM url_audit ua \
             INNER JOIN users actor ON ua.actor_user_id = actor.id \
             INNER JOIN users target ON ua.target_user_id = target.id{}",
            where_clause
        );

        let limit_placeholder = format!("${}", param_idx);
        param_idx += 1;
        let offset_placeholder = format!("${}", param_idx);

        let main_query = format!(
            "SELECT ua.id, ua.event_type, ua.actor_user_id, actor.username as actor_username, \
             ua.target_user_id, target.username as target_username, ua.url_name, ua.created_at \
             FROM url_audit ua \
             INNER JOIN users actor ON ua.actor_user_id = actor.id \
             INNER JOIN users target ON ua.target_user_id = target.id{} \
             ORDER BY ua.created_at DESC \
             LIMIT {} OFFSET {}",
            where_clause, limit_placeholder, offset_placeholder
        );

        let mut count_q = sqlx::query_scalar(&count_query);
        if let Some(ref et) = params.event_type {
            count_q = count_q.bind(et.to_string());
        }
        if let Some(actor_id) = params.actor_user_id {
            count_q = count_q.bind(actor_id);
        }
        if let Some(target_id) = params.target_user_id {
            count_q = count_q.bind(target_id);
        }
        if let Some(ref name) = params.url_name {
            count_q = count_q.bind(format!("%{}%", name));
        }
        if let Some(ref uname) = params.username {
            let pattern = format!("%{}%", uname);
            count_q = count_q.bind(pattern.clone()).bind(pattern);
        }
        if let Some(df) = params.date_from {
            count_q = count_q.bind(df);
        }
        if let Some(dt) = params.date_to {
            count_q = count_q.bind(dt);
        }
        let total_count: i64 = count_q.fetch_one(self.get_pool()).await?;

        let mut main_q = sqlx::query(&main_query);
        if let Some(ref et) = params.event_type {
            main_q = main_q.bind(et.to_string());
        }
        if let Some(actor_id) = params.actor_user_id {
            main_q = main_q.bind(actor_id);
        }
        if let Some(target_id) = params.target_user_id {
            main_q = main_q.bind(target_id);
        }
        if let Some(ref name) = params.url_name {
            main_q = main_q.bind(format!("%{}%", name));
        }
        if let Some(ref uname) = params.username {
            let pattern = format!("%{}%", uname);
            main_q = main_q.bind(pattern.clone()).bind(pattern);
        }
        if let Some(df) = params.date_from {
            main_q = main_q.bind(df);
        }
        if let Some(dt) = params.date_to {
            main_q = main_q.bind(dt);
        }
        main_q = main_q.bind(params.limit).bind(params.offset);

        let events = main_q
            .map(|row: PgRow| {
                let event_type_str: String = row.get("event_type");
                let event_type = match event_type_str.as_str() {
                    "create_url" => AuditEventType::CreateUrl,
                    "delete_url" => AuditEventType::DeleteUrl,
                    "user_login" => AuditEventType::UserLogin,
                    "user_logout" => AuditEventType::UserLogout,
                    "user_quota_update" => AuditEventType::UserQuotaUpdate,
                    _ => AuditEventType::UserLogin, // Default fallback
                };

                AuditEventWithUser {
                    id: row.get("id"),
                    event_type,
                    actor_user_id: row.get("actor_user_id"),
                    actor_username: row.get("actor_username"),
                    target_user_id: row.get("target_user_id"),
                    target_username: row.get("target_username"),
                    url_name: row.get("url_name"),
                    created_at: row.get("created_at"),
                }
            })
            .fetch_all(self.get_pool())
            .await?;

        Ok((events, total_count))
    }

    async fn update_last_accessed(&self, url_id: &str, timestamp: i64) -> Result<(), sqlx::Error> {
        sqlx::query::<SqlxPostgres>("UPDATE urls SET last_accessed = $1 WHERE id = $2")
            .bind(timestamp)
            .bind(url_id)
            .execute(self.get_pool())
            .await?;

        Ok(())
    }
}
