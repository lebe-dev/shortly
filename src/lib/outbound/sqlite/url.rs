use chrono::Utc;
use log::info;
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::domain::url::{
    audit::{AuditEventType, AuditEventWithUser, UrlAuditEvent},
    model::Url,
    ports::UrlRepository,
};

use super::init::Sqlite;

impl UrlRepository for Sqlite {
    async fn save(&self, url: &Url) -> Result<(), sqlx::Error> {
        sqlx::query(
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
        .bind(&url.ttl)
        .bind(&url.created)
        .bind(&url.user_id)
        .bind(&url.custom_name)
        .bind(&url.last_accessed)
        .execute(self.get_pool())
        .await?;

        info!("url '{}' has been saved", url.id);

        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Url>, sqlx::Error> {
        let select_query = sqlx::query("SELECT * FROM urls WHERE id=$1").bind(id);

        let url = select_query
            .map(|row: SqliteRow| Url {
                id: row.get("id"),
                original_url: row.get("original_url"),
                ttl: row.get("ttl"),
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
        let select_query = sqlx::query("SELECT * FROM urls WHERE custom_name = $1 COLLATE NOCASE")
            .bind(custom_name);

        let url = select_query
            .map(|row: SqliteRow| Url {
                id: row.get("id"),
                original_url: row.get("original_url"),
                ttl: row.get("ttl"),
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
        let query = sqlx::query("DELETE FROM urls WHERE ttl > 0 AND created + ttl < $1")
            .bind(Utc::now().timestamp());

        query.execute(self.get_pool()).await?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: i64) -> Result<Vec<Url>, sqlx::Error> {
        let query = sqlx::query("SELECT * FROM urls WHERE user_id = $1 ORDER BY created DESC")
            .bind(user_id);

        let urls = query
            .map(|row: SqliteRow| Url {
                id: row.get("id"),
                original_url: row.get("original_url"),
                ttl: row.get("ttl"),
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
        sqlx::query("DELETE FROM urls WHERE id = $1")
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
        let query = sqlx::query("SELECT * FROM urls ORDER BY created DESC");

        let urls = query
            .map(|row: SqliteRow| Url {
                id: row.get("id"),
                original_url: row.get("original_url"),
                ttl: row.get("ttl"),
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
        event_type: Option<AuditEventType>,
        actor_user_id: Option<i64>,
        target_user_id: Option<i64>,
        url_name: Option<String>,
        username: Option<String>,
        date_from: Option<i64>,
        date_to: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<AuditEventWithUser>, i64), sqlx::Error> {
        let mut where_clauses = Vec::new();

        if event_type.is_some() {
            where_clauses.push("ua.event_type = ?");
        }
        if actor_user_id.is_some() {
            where_clauses.push("ua.actor_user_id = ?");
        }
        if target_user_id.is_some() {
            where_clauses.push("ua.target_user_id = ?");
        }
        if url_name.is_some() {
            where_clauses.push("ua.url_name LIKE ?");
        }
        if username.is_some() {
            where_clauses.push("(actor.username LIKE ? OR target.username LIKE ?)");
        }
        if date_from.is_some() {
            where_clauses.push("ua.created_at >= ?");
        }
        if date_to.is_some() {
            where_clauses.push("ua.created_at <= ?");
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

        let main_query = format!(
            "SELECT ua.id, ua.event_type, ua.actor_user_id, actor.username as actor_username, \
             ua.target_user_id, target.username as target_username, ua.url_name, ua.created_at \
             FROM url_audit ua \
             INNER JOIN users actor ON ua.actor_user_id = actor.id \
             INNER JOIN users target ON ua.target_user_id = target.id{} \
             ORDER BY ua.created_at DESC \
             LIMIT ? OFFSET ?",
            where_clause
        );

        let mut count_q = sqlx::query_scalar(&count_query);
        if let Some(ref et) = event_type {
            count_q = count_q.bind(et.to_string());
        }
        if let Some(actor_id) = actor_user_id {
            count_q = count_q.bind(actor_id);
        }
        if let Some(target_id) = target_user_id {
            count_q = count_q.bind(target_id);
        }
        if let Some(ref name) = url_name {
            count_q = count_q.bind(format!("%{}%", name));
        }
        if let Some(ref uname) = username {
            let pattern = format!("%{}%", uname);
            count_q = count_q.bind(pattern.clone()).bind(pattern);
        }
        if let Some(df) = date_from {
            count_q = count_q.bind(df);
        }
        if let Some(dt) = date_to {
            count_q = count_q.bind(dt);
        }
        let total_count: i64 = count_q.fetch_one(self.get_pool()).await?;

        let mut main_q = sqlx::query(&main_query);
        if let Some(ref et) = event_type {
            main_q = main_q.bind(et.to_string());
        }
        if let Some(actor_id) = actor_user_id {
            main_q = main_q.bind(actor_id);
        }
        if let Some(target_id) = target_user_id {
            main_q = main_q.bind(target_id);
        }
        if let Some(ref name) = url_name {
            main_q = main_q.bind(format!("%{}%", name));
        }
        if let Some(ref uname) = username {
            let pattern = format!("%{}%", uname);
            main_q = main_q.bind(pattern.clone()).bind(pattern);
        }
        if let Some(df) = date_from {
            main_q = main_q.bind(df);
        }
        if let Some(dt) = date_to {
            main_q = main_q.bind(dt);
        }
        main_q = main_q.bind(limit).bind(offset);

        let events = main_q
            .map(|row: SqliteRow| {
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
        sqlx::query("UPDATE urls SET last_accessed = $1 WHERE id = $2")
            .bind(timestamp)
            .bind(url_id)
            .execute(self.get_pool())
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::database::get_in_memory_db;

    #[tokio::test]
    async fn test_delete_expired_empty() {
        let db = get_in_memory_db().await;

        let result = db.delete_expired().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_expired_removes_expired_urls() {
        let db = get_in_memory_db().await;

        // Create an expired URL (created 2 hours ago with 1 hour TTL)
        let expired_url = Url {
            id: "exp123".to_string(),
            original_url: "https://example.com/expired".to_string(),
            ttl: 3600,                              // 1 hour in seconds
            created: Utc::now().timestamp() - 7200, // 2 hours ago
            user_id: None,
            custom_name: None,
            last_accessed: Some(Utc::now().timestamp() - 7200),
        };

        db.save(&expired_url).await.unwrap();

        // Verify URL exists before deletion
        let url_before = db.find_by_id("exp123").await.unwrap();
        assert!(url_before.is_some());

        // Delete expired URLs
        let result = db.delete_expired().await;
        assert!(result.is_ok());

        // Verify URL was deleted
        let url_after = db.find_by_id("exp123").await.unwrap();
        assert!(url_after.is_none());
    }

    #[tokio::test]
    async fn test_delete_expired_keeps_valid_urls() {
        let db = get_in_memory_db().await;

        // Create an expired URL (created 2 hours ago with 1 hour TTL)
        let expired_url = Url {
            id: "exp456".to_string(),
            original_url: "https://example.com/expired".to_string(),
            ttl: 3600,                              // 1 hour
            created: Utc::now().timestamp() - 7200, // 2 hours ago
            user_id: None,
            custom_name: None,
            last_accessed: Some(Utc::now().timestamp() - 7200),
        };

        // Create a valid URL (created now with 1 week TTL)
        let valid_url = Url {
            id: "val789".to_string(),
            original_url: "https://example.com/valid".to_string(),
            ttl: 604800, // 1 week in seconds
            created: Utc::now().timestamp(),
            user_id: None,
            custom_name: None,
            last_accessed: Some(Utc::now().timestamp()),
        };

        db.save(&expired_url).await.unwrap();
        db.save(&valid_url).await.unwrap();

        // Delete expired URLs
        let result = db.delete_expired().await;
        assert!(result.is_ok());

        // Verify expired URL was deleted
        let expired_after = db.find_by_id("exp456").await.unwrap();
        assert!(expired_after.is_none());

        // Verify valid URL still exists
        let valid_after = db.find_by_id("val789").await.unwrap();
        assert!(valid_after.is_some());
        assert_eq!(valid_after.unwrap().id, "val789");
    }

    #[tokio::test]
    async fn test_delete_expired_keeps_named_urls() {
        let db = get_in_memory_db().await;

        // Create an expired regular URL
        let expired_url = Url {
            id: "exp123".to_string(),
            original_url: "https://example.com/expired".to_string(),
            ttl: 3600,                              // 1 hour
            created: Utc::now().timestamp() - 7200, // 2 hours ago
            user_id: Some(1),
            custom_name: None,
            last_accessed: Some(Utc::now().timestamp() - 7200),
        };

        // Create a named URL with ttl=0 (should never expire)
        let named_url = Url {
            id: "hash456".to_string(),
            original_url: "https://example.com/named".to_string(),
            ttl: 0,                                        // Named URLs have no expiration
            created: Utc::now().timestamp() - 86400 * 365, // 1 year ago
            user_id: Some(1),
            custom_name: Some("my-link".to_string()),
            last_accessed: Some(Utc::now().timestamp() - 86400 * 365),
        };

        db.save(&expired_url).await.unwrap();
        db.save(&named_url).await.unwrap();

        // Delete expired URLs
        db.delete_expired().await.unwrap();

        // Verify expired regular URL was deleted
        assert!(db.find_by_id("exp123").await.unwrap().is_none());

        // Verify named URL still exists
        let found_named = db.find_by_id("hash456").await.unwrap();
        assert!(found_named.is_some());
        assert_eq!(
            found_named.unwrap().custom_name,
            Some("my-link".to_string())
        );
    }
}
