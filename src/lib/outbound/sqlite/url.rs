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
}
