use log::info;

use crate::domain::auth::model::User;
use crate::domain::passkey::model::{ChallengeOperation, PasskeyChallenge, PasskeyCredential};
use crate::domain::passkey::ports::{
    PasskeyChallengeRepository, PasskeyCredentialRepository, WebauthnUserRepository,
};

use super::init::Postgres;

const CREDENTIAL_COLUMNS: &str =
    "id, user_id, credential_id, passkey, name, created_at, last_used_at";

const USER_COLUMNS: &str = "id, gitlab_id, username, email, avatar_url, created_at, updated_at, \
                            max_urls_per_user, max_urls_per_day";

impl PasskeyCredentialRepository for Postgres {
    async fn create(
        &self,
        user_id: i64,
        credential_id: &str,
        passkey: &str,
        name: &str,
    ) -> Result<PasskeyCredential, sqlx::Error> {
        let created_at = chrono::Utc::now().timestamp();

        let credential = sqlx::query_as::<_, PasskeyCredential>(&format!(
            "INSERT INTO passkey_credentials (user_id, credential_id, passkey, name, created_at)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING {CREDENTIAL_COLUMNS}"
        ))
        .bind(user_id)
        .bind(credential_id)
        .bind(passkey)
        .bind(name)
        .bind(created_at)
        .fetch_one(self.get_pool())
        .await?;

        info!("stored a new passkey for user {}", user_id);

        Ok(credential)
    }

    async fn find_by_user_id(&self, user_id: i64) -> Result<Vec<PasskeyCredential>, sqlx::Error> {
        let credentials = sqlx::query_as::<_, PasskeyCredential>(&format!(
            "SELECT {CREDENTIAL_COLUMNS} FROM passkey_credentials
             WHERE user_id = $1
             ORDER BY created_at DESC"
        ))
        .bind(user_id)
        .fetch_all(self.get_pool())
        .await?;

        Ok(credentials)
    }

    async fn find_by_credential_id(
        &self,
        credential_id: &str,
    ) -> Result<Option<PasskeyCredential>, sqlx::Error> {
        let credential = sqlx::query_as::<_, PasskeyCredential>(&format!(
            "SELECT {CREDENTIAL_COLUMNS} FROM passkey_credentials WHERE credential_id = $1"
        ))
        .bind(credential_id)
        .fetch_optional(self.get_pool())
        .await?;

        Ok(credential)
    }

    async fn update_passkey(
        &self,
        credential_id: &str,
        passkey: &str,
        last_used_at: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE passkey_credentials SET passkey = $1, last_used_at = $2
             WHERE credential_id = $3",
        )
        .bind(passkey)
        .bind(last_used_at)
        .bind(credential_id)
        .execute(self.get_pool())
        .await?;

        Ok(())
    }

    async fn delete(&self, user_id: i64, credential_pk: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM passkey_credentials WHERE id = $1 AND user_id = $2")
            .bind(credential_pk)
            .bind(user_id)
            .execute(self.get_pool())
            .await?;

        Ok(result.rows_affected())
    }

    async fn delete_by_user_id(&self, user_id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM passkey_credentials WHERE user_id = $1")
            .bind(user_id)
            .execute(self.get_pool())
            .await?;

        Ok(result.rows_affected())
    }

    async fn count_by_user_id(&self, user_id: i64) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM passkey_credentials WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(self.get_pool())
        .await?;

        Ok(count)
    }
}

impl PasskeyChallengeRepository for Postgres {
    async fn create(&self, challenge: &PasskeyChallenge) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO passkey_challenges (id, user_id, operation, state, created_at, expires_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(&challenge.id)
        .bind(challenge.user_id)
        .bind(challenge.operation.to_string())
        .bind(&challenge.state)
        .bind(challenge.created_at)
        .bind(challenge.expires_at)
        .execute(self.get_pool())
        .await?;

        Ok(())
    }

    async fn take(&self, id: &str) -> Result<Option<PasskeyChallenge>, sqlx::Error> {
        // A single statement keeps the challenge single use even with several instances running.
        let row = sqlx::query_as::<_, (String, Option<i64>, String, String, i64, i64)>(
            "DELETE FROM passkey_challenges WHERE id = $1
             RETURNING id, user_id, operation, state, created_at, expires_at",
        )
        .bind(id)
        .fetch_optional(self.get_pool())
        .await?;

        let Some((id, user_id, operation, state, created_at, expires_at)) = row else {
            return Ok(None);
        };

        let Some(operation) = ChallengeOperation::parse(&operation) else {
            return Ok(None);
        };

        Ok(Some(PasskeyChallenge {
            id,
            user_id,
            operation,
            state,
            created_at,
            expires_at,
        }))
    }

    async fn delete_expired(&self, current_time: i64) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM passkey_challenges WHERE expires_at < $1")
            .bind(current_time)
            .execute(self.get_pool())
            .await?;

        if result.rows_affected() > 0 {
            info!(
                "deleted {} expired passkey challenges",
                result.rows_affected()
            );
        }

        Ok(())
    }
}

impl WebauthnUserRepository for Postgres {
    async fn find_webauthn_id(&self, user_id: i64) -> Result<Option<String>, sqlx::Error> {
        let webauthn_id =
            sqlx::query_scalar::<_, Option<String>>("SELECT webauthn_id FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_optional(self.get_pool())
                .await?
                .flatten();

        Ok(webauthn_id)
    }

    async fn set_webauthn_id(&self, user_id: i64, webauthn_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET webauthn_id = $1 WHERE id = $2")
            .bind(webauthn_id)
            .bind(user_id)
            .execute(self.get_pool())
            .await?;

        Ok(())
    }

    async fn find_by_webauthn_id(&self, webauthn_id: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(&format!(
            "SELECT {USER_COLUMNS} FROM users WHERE webauthn_id = $1"
        ))
        .bind(webauthn_id)
        .fetch_optional(self.get_pool())
        .await?;

        Ok(user)
    }
}
