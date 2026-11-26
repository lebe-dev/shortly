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
            .context("Failed to connect to PostgreSQL")?;

        Ok(Postgres { pool })
    }
}

#[cfg(test)]
mod tests {
    use super::Postgres;

    #[tokio::test]
    async fn connection_error_omits_credentials() {
        let url =
            "postgresql://probe:userinfo-secret@127.0.0.1:invalid/probe?password=query-secret";
        let error = Postgres::new(url).await.unwrap_err();
        let message = format!("{error:#}");

        assert!(message.starts_with("Failed to connect to PostgreSQL"));
        assert!(!message.contains("userinfo-secret"));
        assert!(!message.contains("query-secret"));
    }
}
