use anyhow::Context;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn new(url: &str) -> Result<Postgres, anyhow::Error> {
        let pool = PgPool::connect(url)
            .await
            .with_context(|| format!("Failed to connect to PostgreSQL at {}", url))?;

        Ok(Postgres { pool })
    }
}
