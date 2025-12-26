use log::info;

use crate::domain::auth::{
    model::{GitlabUserInfo, Session, User},
    ports::{SessionRepository, UserRepository},
};

use super::init::Sqlite;

impl UserRepository for Sqlite {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::auth::ports::{SessionRepository, UserRepository};
    use crate::tests::database::get_in_memory_db;
    use chrono::Utc;

    // UserRepository tests

    #[tokio::test]
    async fn test_find_by_gitlab_id_not_found() {
        let db = get_in_memory_db().await;

        let result = db.find_by_gitlab_id(999999).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_upsert_creates_new_user() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 12345,
            username: "testuser".to_string(),
            email: Some("test@example.com".to_string()),
            avatar_url: Some("https://example.com/avatar.png".to_string()),
        };

        let user = db.upsert(&gitlab_user).await.unwrap();

        assert_eq!(user.gitlab_id, 12345);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, Some("test@example.com".to_string()));
        assert_eq!(
            user.avatar_url,
            Some("https://example.com/avatar.png".to_string())
        );
        assert!(user.created_at > 0);
        assert!(user.updated_at > 0);
        assert_eq!(user.created_at, user.updated_at);
    }

    #[tokio::test]
    async fn test_upsert_updates_existing_user() {
        let db = get_in_memory_db().await;

        // Create initial user
        let initial_user = GitlabUserInfo {
            id: 12345,
            username: "oldname".to_string(),
            email: Some("old@example.com".to_string()),
            avatar_url: Some("https://example.com/old.png".to_string()),
        };

        let created_user = db.upsert(&initial_user).await.unwrap();
        let created_at = created_user.created_at;

        // Wait a moment to ensure updated_at will be different
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Update the same user
        let updated_info = GitlabUserInfo {
            id: 12345,
            username: "newname".to_string(),
            email: Some("new@example.com".to_string()),
            avatar_url: Some("https://example.com/new.png".to_string()),
        };

        let updated_user = db.upsert(&updated_info).await.unwrap();

        assert_eq!(updated_user.id, created_user.id);
        assert_eq!(updated_user.gitlab_id, 12345);
        assert_eq!(updated_user.username, "newname");
        assert_eq!(updated_user.email, Some("new@example.com".to_string()));
        assert_eq!(
            updated_user.avatar_url,
            Some("https://example.com/new.png".to_string())
        );
        assert_eq!(updated_user.created_at, created_at);
        assert!(updated_user.updated_at >= created_at);
    }

    #[tokio::test]
    async fn test_upsert_with_null_email_and_avatar() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 12346,
            username: "noavatar".to_string(),
            email: None,
            avatar_url: None,
        };

        let user = db.upsert(&gitlab_user).await.unwrap();

        assert_eq!(user.gitlab_id, 12346);
        assert_eq!(user.username, "noavatar");
        assert_eq!(user.email, None);
        assert_eq!(user.avatar_url, None);
    }

    #[tokio::test]
    async fn test_find_by_id_success() {
        let db = get_in_memory_db().await;

        // Create a user first
        let gitlab_user = GitlabUserInfo {
            id: 12347,
            username: "findme".to_string(),
            email: Some("findme@example.com".to_string()),
            avatar_url: None,
        };

        let created_user = db.upsert(&gitlab_user).await.unwrap();

        // Find by ID
        let found_user = db.find_by_id(created_user.id).await.unwrap();

        assert!(found_user.is_some());
        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.gitlab_id, 12347);
        assert_eq!(found_user.username, "findme");
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let db = get_in_memory_db().await;

        let result = db.find_by_id(999999).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_find_by_gitlab_id_success() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 12348,
            username: "gitlabfind".to_string(),
            email: Some("gitlab@example.com".to_string()),
            avatar_url: None,
        };

        db.upsert(&gitlab_user).await.unwrap();

        let found_user = db.find_by_gitlab_id(12348).await.unwrap();

        assert!(found_user.is_some());
        let found_user = found_user.unwrap();
        assert_eq!(found_user.gitlab_id, 12348);
        assert_eq!(found_user.username, "gitlabfind");
    }

    // SessionRepository tests

    #[tokio::test]
    async fn test_create_session_without_expiration() {
        let db = get_in_memory_db().await;

        // Create a user first
        let gitlab_user = GitlabUserInfo {
            id: 20001,
            username: "sessionuser".to_string(),
            email: Some("session@example.com".to_string()),
            avatar_url: None,
        };
        let user = db.upsert(&gitlab_user).await.unwrap();

        // Create session
        let session = db.create(user.id, "test_token_123", None).await.unwrap();

        assert!(session.id > 0);
        assert_eq!(session.token, "test_token_123");
        assert_eq!(session.user_id, user.id);
        assert!(session.created_at > 0);
        assert_eq!(session.created_at, session.last_used_at);
        assert_eq!(session.expires_at, None);
    }

    #[tokio::test]
    async fn test_create_session_with_expiration() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 20002,
            username: "expiringuser".to_string(),
            email: None,
            avatar_url: None,
        };
        let user = db.upsert(&gitlab_user).await.unwrap();

        let expires_at = Utc::now().timestamp() + 3600;

        let session = db
            .create(user.id, "expiring_token", Some(expires_at))
            .await
            .unwrap();

        assert_eq!(session.token, "expiring_token");
        assert_eq!(session.expires_at, Some(expires_at));
    }

    #[tokio::test]
    async fn test_find_by_token_success() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 20003,
            username: "tokenuser".to_string(),
            email: None,
            avatar_url: None,
        };
        let user = db.upsert(&gitlab_user).await.unwrap();

        let created_session = db.create(user.id, "find_token", None).await.unwrap();

        let found_session = db.find_by_token("find_token").await.unwrap();

        assert!(found_session.is_some());
        let found_session = found_session.unwrap();
        assert_eq!(found_session.id, created_session.id);
        assert_eq!(found_session.token, "find_token");
        assert_eq!(found_session.user_id, user.id);
    }

    #[tokio::test]
    async fn test_find_by_token_not_found() {
        let db = get_in_memory_db().await;

        let result = db.find_by_token("nonexistent_token").await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_update_last_used() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 20004,
            username: "updateuser".to_string(),
            email: None,
            avatar_url: None,
        };
        let user = db.upsert(&gitlab_user).await.unwrap();

        let session = db.create(user.id, "update_token", None).await.unwrap();
        let original_last_used = session.last_used_at;

        // Wait a moment
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let new_timestamp = Utc::now().timestamp();
        db.update_last_used("update_token", new_timestamp)
            .await
            .unwrap();

        let updated_session = db.find_by_token("update_token").await.unwrap().unwrap();

        assert_eq!(updated_session.last_used_at, new_timestamp);
        assert!(updated_session.last_used_at >= original_last_used);
    }

    #[tokio::test]
    async fn test_delete_session() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 20005,
            username: "deleteuser".to_string(),
            email: None,
            avatar_url: None,
        };
        let user = db.upsert(&gitlab_user).await.unwrap();

        db.create(user.id, "delete_token", None).await.unwrap();

        // Verify session exists
        let session_before = db.find_by_token("delete_token").await.unwrap();
        assert!(session_before.is_some());

        // Delete session
        db.delete("delete_token").await.unwrap();

        // Verify session is deleted
        let session_after = db.find_by_token("delete_token").await.unwrap();
        assert!(session_after.is_none());
    }

    #[tokio::test]
    async fn test_delete_expired_sessions() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 20006,
            username: "expireduser".to_string(),
            email: None,
            avatar_url: None,
        };
        let user = db.upsert(&gitlab_user).await.unwrap();

        let current_time = Utc::now().timestamp();

        // Create expired session
        db.create(user.id, "expired_token", Some(current_time - 3600))
            .await
            .unwrap();

        // Create valid session with future expiration
        db.create(user.id, "valid_token", Some(current_time + 3600))
            .await
            .unwrap();

        // Create session without expiration
        db.create(user.id, "no_expiry_token", None).await.unwrap();

        // Delete expired sessions
        db.delete_expired(current_time).await.unwrap();

        // Verify expired session is deleted
        let expired = db.find_by_token("expired_token").await.unwrap();
        assert!(expired.is_none());

        // Verify valid sessions still exist
        let valid = db.find_by_token("valid_token").await.unwrap();
        assert!(valid.is_some());

        let no_expiry = db.find_by_token("no_expiry_token").await.unwrap();
        assert!(no_expiry.is_some());
    }

    #[tokio::test]
    async fn test_delete_expired_with_no_expired_sessions() {
        let db = get_in_memory_db().await;

        let current_time = Utc::now().timestamp();

        let result = db.delete_expired(current_time).await;

        assert!(result.is_ok());
    }

    // UserRepository::update_quotas tests

    #[tokio::test]
    async fn test_update_quotas_both_fields() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 30001,
            username: "quotauser".to_string(),
            email: None,
            avatar_url: None,
        };
        let user = db.upsert(&gitlab_user).await.unwrap();

        let updated_user = db.update_quotas(user.id, Some(50), Some(5)).await.unwrap();

        assert_eq!(updated_user.max_urls_per_user, 50);
        assert_eq!(updated_user.max_urls_per_day, 5);
        assert!(updated_user.updated_at >= user.updated_at);
    }

    #[tokio::test]
    async fn test_update_quotas_partial_per_user() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 30002,
            username: "partialquota".to_string(),
            email: None,
            avatar_url: None,
        };
        let user = db.upsert(&gitlab_user).await.unwrap();
        let original_per_day = user.max_urls_per_day;

        let updated_user = db.update_quotas(user.id, Some(75), None).await.unwrap();

        assert_eq!(updated_user.max_urls_per_user, 75);
        assert_eq!(updated_user.max_urls_per_day, original_per_day);
    }

    #[tokio::test]
    async fn test_update_quotas_partial_per_day() {
        let db = get_in_memory_db().await;

        let gitlab_user = GitlabUserInfo {
            id: 30003,
            username: "partialdaily".to_string(),
            email: None,
            avatar_url: None,
        };
        let user = db.upsert(&gitlab_user).await.unwrap();
        let original_per_user = user.max_urls_per_user;

        let updated_user = db.update_quotas(user.id, None, Some(20)).await.unwrap();

        assert_eq!(updated_user.max_urls_per_user, original_per_user);
        assert_eq!(updated_user.max_urls_per_day, 20);
    }

    #[tokio::test]
    async fn test_update_quotas_not_found() {
        let db = get_in_memory_db().await;

        let result = db.update_quotas(999999, Some(50), Some(5)).await;

        assert!(result.is_err());
        match result {
            Err(sqlx::Error::RowNotFound) => (),
            _ => panic!("Expected RowNotFound error"),
        }
    }
}
