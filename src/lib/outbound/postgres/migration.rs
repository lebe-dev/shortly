use anyhow::{Result, anyhow};
use log::{info, warn};
use sqlx::PgPool;
use std::collections::BTreeMap;

/// Splits a SQL file content into individual statements
/// Filters out comments (lines starting with --) and empty statements
fn split_sql_statements(sql: &str) -> Vec<String> {
    sql.lines()
        .filter(|line| !line.trim().starts_with("--"))
        .collect::<Vec<_>>()
        .join("\n")
        .split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub struct PostgresMigrationManager {
    pool: PgPool,
    migrations: BTreeMap<String, String>,
}

impl PostgresMigrationManager {
    pub fn new(pool: PgPool, migrations: BTreeMap<String, String>) -> Self {
        Self { pool, migrations }
    }

    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing PostgreSQL migration system");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS _migrations (
                id BIGSERIAL PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to create migrations table: {}", e))?;

        Ok(())
    }

    pub async fn run_migrations(&self) -> Result<()> {
        info!("Starting PostgreSQL database migrations");

        if self.migrations.is_empty() {
            warn!("No migration files provided");
            return Ok(());
        }

        for (name, sql_content) in &self.migrations {
            if self.is_migration_applied(name).await? {
                info!("Migration {} already applied, skipping", name);
                continue;
            }

            info!("Applying migration: {}", name);

            let mut tx = self.pool.begin().await?;

            for statement in split_sql_statements(sql_content) {
                sqlx::query(&statement)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| anyhow!("Failed to execute migration {}: {}", name, e))?;
            }

            sqlx::query("INSERT INTO _migrations (name) VALUES ($1)")
                .bind(name)
                .execute(&mut *tx)
                .await
                .map_err(|e| anyhow!("Failed to record migration {}: {}", name, e))?;

            tx.commit().await?;
            info!("Successfully applied migration: {}", name);
        }

        info!("All migrations completed");
        Ok(())
    }

    async fn is_migration_applied(&self, name: &str) -> Result<bool> {
        let result =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM _migrations WHERE name = $1")
                .bind(name)
                .fetch_one(&self.pool)
                .await?;

        Ok(result > 0)
    }
}
