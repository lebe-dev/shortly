use std::str::FromStr;

use anyhow::Context;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: SqlitePool,
}

impl Sqlite {
    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub async fn new(path: &str) -> Result<Sqlite, anyhow::Error> {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path {}", path))?
                .pragma("foreign_keys", "ON")
                .pragma("journal_mode", "WAL"),
        )
        .await
        .with_context(|| format!("failed to open database at {}", path))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS entities (
            path varchar,
            filename varchar,
            size integer,
            optimized integer,
            optimization_date integer,
            UNIQUE(path)
            );"#,
        )
        .execute(&pool)
        .await?;

        Ok(Sqlite { pool })
    }
}
