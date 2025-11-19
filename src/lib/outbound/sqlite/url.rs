use chrono::Utc;
use log::info;
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::domain::url::{model::Url, ports::UrlRepository};

use super::init::Sqlite;

impl UrlRepository for Sqlite {
    async fn save(&self, url: &Url) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO urls (
                id,
                original_url,
                ttl,
                created
            ) VALUES ($1, $2, $3, $4);
            "#,
        )
        .bind(&url.id)
        .bind(&url.original_url)
        .bind(&url.ttl)
        .bind(&url.created)
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
            })
            .fetch_optional(self.get_pool())
            .await?;

        Ok(url)
    }

    async fn delete_expired(&self) -> Result<(), sqlx::Error> {
        let query =
            sqlx::query("DELETE FROM urls WHERE created + ttl < $1").bind(Utc::now().timestamp());

        query.execute(self.get_pool()).await?;

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
        };

        // Create a valid URL (created now with 1 week TTL)
        let valid_url = Url {
            id: "val789".to_string(),
            original_url: "https://example.com/valid".to_string(),
            ttl: 604800, // 1 week in seconds
            created: Utc::now().timestamp(),
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
}
