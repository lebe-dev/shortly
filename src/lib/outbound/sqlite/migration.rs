use anyhow::{Result, anyhow};
use log::{info, warn};
use sqlx::sqlite::SqlitePool;
use std::collections::BTreeMap;

pub struct MigrationManager {
    pool: SqlitePool,
    migrations: BTreeMap<String, String>,
}

impl MigrationManager {
    pub fn new(pool: SqlitePool, migrations: BTreeMap<String, String>) -> Self {
        Self { pool, migrations }
    }

    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing migration system");

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS _migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to create migrations table: {}", e))?;

        Ok(())
    }

    pub async fn run_migrations(&self) -> Result<()> {
        info!("Starting database migrations");

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

            sqlx::query(sql_content)
                .execute(&mut *tx)
                .await
                .map_err(|e| anyhow!("Failed to execute migration {}: {}", name, e))?;

            // Record that migration was applied
            sqlx::query("INSERT INTO _migrations (name) VALUES (?1)")
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
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM _migrations WHERE name = ?1")
                .bind(name)
                .fetch_one(&self.pool)
                .await?;

        Ok(result > 0)
    }
}

/// Load migrations from filesystem (for tests)
pub fn load_migrations_from_fs() -> Result<BTreeMap<String, String>> {
    use std::fs;
    use std::path::Path;

    let migrations_dir = Path::new("migrations");
    if !migrations_dir.exists() {
        return Err(anyhow!(
            "Migrations directory does not exist: {}",
            migrations_dir.display()
        ));
    }

    let mut migration_files: BTreeMap<String, String> = BTreeMap::new();

    for entry in fs::read_dir(migrations_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            let file_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow!("Invalid file name: {}", path.display()))?;

            let content = fs::read_to_string(&path)
                .map_err(|e| anyhow!("Failed to read migration file {}: {}", path.display(), e))?;

            migration_files.insert(file_name.to_string(), content);
        }
    }

    Ok(migration_files)
}
