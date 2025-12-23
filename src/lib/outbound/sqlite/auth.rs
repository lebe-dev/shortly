use log::info;

use crate::domain::auth::{
    model::{GitlabUserInfo, Session, User},
    ports::{SessionRepository, UserRepository},
};

use super::init::Sqlite;

impl UserRepository for Sqlite {
    async fn find_by_gitlab_id(&self, gitlab_id: i64) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, gitlab_id, username, email, avatar_url, created_at, updated_at
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

            info!("Updated user: {}", existing_user.username);
            return Ok(existing_user);
        }

        let result = sqlx::query(
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

        let user = User {
            id: result.last_insert_rowid(),
            gitlab_id: gitlab_user.id,
            username: gitlab_user.username.clone(),
            email: gitlab_user.email.clone(),
            avatar_url: gitlab_user.avatar_url.clone(),
            created_at: current_time,
            updated_at: current_time,
        };

        info!("Created new user: {}", user.username);
        Ok(user)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, gitlab_id, username, email, avatar_url, created_at, updated_at
             FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.get_pool())
        .await?;

        Ok(user)
    }
}

impl SessionRepository for Sqlite {
    async fn create(
        &self,
        user_id: i64,
        token: &str,
        expires_at: Option<i64>,
    ) -> Result<Session, sqlx::Error> {
        let current_time = chrono::Utc::now().timestamp();

        let result = sqlx::query(
            "INSERT INTO sessions (token, user_id, created_at, last_used_at, expires_at)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(token)
        .bind(user_id)
        .bind(current_time)
        .bind(current_time)
        .bind(expires_at)
        .execute(self.get_pool())
        .await?;

        Ok(Session {
            id: result.last_insert_rowid(),
            token: token.to_string(),
            user_id,
            created_at: current_time,
            last_used_at: current_time,
            expires_at,
        })
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
        sqlx::query("UPDATE sessions SET last_used_at = $1 WHERE token = $2")
            .bind(timestamp)
            .bind(token)
            .execute(self.get_pool())
            .await?;

        Ok(())
    }

    async fn delete(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM sessions WHERE token = $1")
            .bind(token)
            .execute(self.get_pool())
            .await?;

        info!("Session deleted");
        Ok(())
    }

    async fn delete_expired(&self, current_time: i64) -> Result<(), sqlx::Error> {
        let result =
            sqlx::query("DELETE FROM sessions WHERE expires_at IS NOT NULL AND expires_at < $1")
                .bind(current_time)
                .execute(self.get_pool())
                .await?;

        if result.rows_affected() > 0 {
            info!("Deleted {} expired sessions", result.rows_affected());
        }

        Ok(())
    }
}
