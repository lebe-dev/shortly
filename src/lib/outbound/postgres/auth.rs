use log::info;
use sqlx::Postgres as SqlxPostgres;

use crate::domain::auth::{
    model::{GitlabUserInfo, Session, User},
    ports::{SessionRepository, UserRepository},
};

use super::init::Postgres;

impl UserRepository for Postgres {
    async fn find_by_gitlab_id(&self, gitlab_id: i64) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, gitlab_id, username, email, avatar_url, created_at, updated_at,
                    max_urls_per_user, max_urls_per_day
             FROM users WHERE gitlab_id = $1",
        )
        .bind(gitlab_id)
        .fetch_optional(self.get_pool())
        .await?;

        Ok(user)
    }

    async fn upsert(&self, gitlab_user: &GitlabUserInfo) -> Result<User, sqlx::Error> {
        let current_time = chrono::Utc::now().timestamp();

        if let Some(mut existing_user) = self.find_by_gitlab_id(gitlab_user.id).await? {
            sqlx::query(
                "UPDATE users SET username = $1, email = $2, avatar_url = $3, updated_at = $4
                 WHERE gitlab_id = $5",
            )
            .bind(&gitlab_user.username)
            .bind(&gitlab_user.email)
            .bind(&gitlab_user.avatar_url)
            .bind(current_time)
            .bind(gitlab_user.id)
            .execute(self.get_pool())
            .await?;

            existing_user.username = gitlab_user.username.clone();
            existing_user.email = gitlab_user.email.clone();
            existing_user.avatar_url = gitlab_user.avatar_url.clone();
            existing_user.updated_at = current_time;

            info!("updated user: {}", existing_user.username);
            return Ok(existing_user);
        }

        let _result = sqlx::query(
            "INSERT INTO users (gitlab_id, username, email, avatar_url, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(gitlab_user.id)
        .bind(&gitlab_user.username)
        .bind(&gitlab_user.email)
        .bind(&gitlab_user.avatar_url)
        .bind(current_time)
        .bind(current_time)
        .execute(self.get_pool())
        .await?;

        let user = self
            .find_by_gitlab_id(gitlab_user.id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;

        info!("Created new user: {}", user.username);
        Ok(user)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, gitlab_id, username, email, avatar_url, created_at, updated_at,
                    max_urls_per_user, max_urls_per_day
             FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.get_pool())
        .await?;

        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, gitlab_id, username, email, avatar_url, created_at, updated_at,
                    max_urls_per_user, max_urls_per_day
             FROM users
             ORDER BY username ASC",
        )
        .fetch_all(self.get_pool())
        .await?;

        Ok(users)
    }

    async fn update_quotas(
        &self,
        user_id: i64,
        max_urls_per_user: Option<i32>,
        max_urls_per_day: Option<i32>,
    ) -> Result<User, sqlx::Error> {
        let current_time = chrono::Utc::now().timestamp();

        let user = self
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;

        let new_max_urls_per_user = max_urls_per_user.unwrap_or(user.max_urls_per_user);
        let new_max_urls_per_day = max_urls_per_day.unwrap_or(user.max_urls_per_day);

        sqlx::query(
            "UPDATE users
             SET max_urls_per_user = $1, max_urls_per_day = $2, updated_at = $3
             WHERE id = $4",
        )
        .bind(new_max_urls_per_user)
        .bind(new_max_urls_per_day)
        .bind(current_time)
        .bind(user_id)
        .execute(self.get_pool())
        .await?;

        self.find_by_id(user_id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)
    }
}

impl SessionRepository for Postgres {
    async fn create(
        &self,
        user_id: i64,
        token: &str,
        expires_at: Option<i64>,
    ) -> Result<Session, sqlx::Error> {
        let current_time = chrono::Utc::now().timestamp();

        let session = sqlx::query_as::<_, Session>(
            "INSERT INTO sessions (token, user_id, created_at, last_used_at, expires_at)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, token, user_id, created_at, last_used_at, expires_at",
        )
        .bind(token)
        .bind(user_id)
        .bind(current_time)
        .bind(current_time)
        .bind(expires_at)
        .fetch_one(self.get_pool())
        .await?;

        Ok(session)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<Session>, sqlx::Error> {
        let session = sqlx::query_as::<_, Session>(
            "SELECT id, token, user_id, created_at, last_used_at, expires_at
             FROM sessions WHERE token = $1",
        )
        .bind(token)
        .fetch_optional(self.get_pool())
        .await?;

        Ok(session)
    }

    async fn update_last_used(&self, token: &str, timestamp: i64) -> Result<(), sqlx::Error> {
        sqlx::query::<SqlxPostgres>("UPDATE sessions SET last_used_at = $1 WHERE token = $2")
            .bind(timestamp)
            .bind(token)
            .execute(self.get_pool())
            .await?;

        Ok(())
    }

    async fn delete(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query::<SqlxPostgres>("DELETE FROM sessions WHERE token = $1")
            .bind(token)
            .execute(self.get_pool())
            .await?;

        info!("Session deleted");
        Ok(())
    }

    async fn delete_expired(&self, current_time: i64) -> Result<(), sqlx::Error> {
        let result = sqlx::query::<SqlxPostgres>(
            "DELETE FROM sessions WHERE expires_at IS NOT NULL AND expires_at < $1",
        )
        .bind(current_time)
        .execute(self.get_pool())
        .await?;

        if result.rows_affected() > 0 {
            info!("Deleted {} expired sessions", result.rows_affected());
        }

        Ok(())
    }
}
