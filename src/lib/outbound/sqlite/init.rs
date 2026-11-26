use std::str::FromStr;

use anyhow::anyhow;
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
                .map_err(|_| anyhow!("Invalid SQLite connection string"))?
                .pragma("foreign_keys", "ON")
                .pragma("journal_mode", "WAL"),
        )
        .await
        .map_err(|_| anyhow!("Failed to open SQLite database"))?;

        Ok(Sqlite { pool })
    }
}

#[cfg(test)]
mod tests {
    use super::Sqlite;

    #[tokio::test]
    async fn invalid_connection_string_omits_secrets() {
        let path = "sqlite://temporary.db?mode=sqlite-test-secret";
        let error = Sqlite::new(path).await.unwrap_err();

        assert_eq!(format!("{error:#}"), "Invalid SQLite connection string");
    }
}
