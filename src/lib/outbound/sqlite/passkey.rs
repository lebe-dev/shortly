use log::info;

use crate::domain::auth::model::User;
use crate::domain::passkey::model::{ChallengeOperation, PasskeyChallenge, PasskeyCredential};
use crate::domain::passkey::ports::{
    PasskeyChallengeRepository, PasskeyCredentialRepository, WebauthnUserRepository,
};

use super::init::Sqlite;

const CREDENTIAL_COLUMNS: &str =
    "id, user_id, credential_id, passkey, name, created_at, last_used_at";

const USER_COLUMNS: &str = "id, gitlab_id, username, email, avatar_url, created_at, updated_at, \
                            max_urls_per_user, max_urls_per_day";

impl PasskeyCredentialRepository for Sqlite {
    async fn create(
        &self,
        user_id: i64,
        credential_id: &str,
        passkey: &str,
        name: &str,
    ) -> Result<PasskeyCredential, sqlx::Error> {
        let created_at = chrono::Utc::now().timestamp();

        let result = sqlx::query(
            "INSERT INTO passkey_credentials (user_id, credential_id, passkey, name, created_at)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(user_id)
        .bind(credential_id)
        .bind(passkey)
        .bind(name)
        .bind(created_at)
        .execute(self.get_pool())
        .await?;

        info!("stored a new passkey for user {}", user_id);

        Ok(PasskeyCredential {
            id: result.last_insert_rowid(),
            user_id,
            credential_id: credential_id.to_string(),
            passkey: passkey.to_string(),
            name: name.to_string(),
            created_at,
            last_used_at: None,
        })
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

impl PasskeyChallengeRepository for Sqlite {
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
        let mut tx = self.get_pool().begin().await?;

        let row = sqlx::query_as::<_, (String, Option<i64>, String, String, i64, i64)>(
            "SELECT id, user_id, operation, state, created_at, expires_at
             FROM passkey_challenges WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        let Some((id, user_id, operation, state, created_at, expires_at)) = row else {
            tx.commit().await?;
            return Ok(None);
        };

        sqlx::query("DELETE FROM passkey_challenges WHERE id = $1")
            .bind(&id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

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

impl WebauthnUserRepository for Sqlite {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::auth::model::GitlabUserInfo;
    use crate::domain::auth::ports::UserRepository;
    use crate::tests::database::get_in_memory_db;

    async fn create_user(db: &Sqlite, gitlab_id: i64, username: &str) -> User {
        db.upsert(&GitlabUserInfo {
            id: gitlab_id,
            username: username.to_string(),
            email: None,
            avatar_url: None,
        })
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_create_and_find_credential() {
        let db = get_in_memory_db().await;
        let user = create_user(&db, 40001, "passkeyuser").await;

        let created =
            PasskeyCredentialRepository::create(&db, user.id, "cred-1", "{\"state\":1}", "MacBook")
                .await
                .unwrap();

        assert!(created.id > 0);
        assert_eq!(created.user_id, user.id);
        assert_eq!(created.name, "MacBook");
        assert_eq!(created.last_used_at, None);

        let found = PasskeyCredentialRepository::find_by_credential_id(&db, "cred-1")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(found.id, created.id);
        assert_eq!(found.passkey, "{\"state\":1}");
    }

    #[tokio::test]
    async fn test_find_by_credential_id_not_found() {
        let db = get_in_memory_db().await;

        let found = PasskeyCredentialRepository::find_by_credential_id(&db, "missing")
            .await
            .unwrap();

        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_find_by_user_id_returns_only_own_credentials() {
        let db = get_in_memory_db().await;
        let first = create_user(&db, 40002, "first").await;
        let second = create_user(&db, 40003, "second").await;

        PasskeyCredentialRepository::create(&db, first.id, "cred-a", "{}", "Key A")
            .await
            .unwrap();
        PasskeyCredentialRepository::create(&db, second.id, "cred-b", "{}", "Key B")
            .await
            .unwrap();

        let credentials = PasskeyCredentialRepository::find_by_user_id(&db, first.id)
            .await
            .unwrap();

        assert_eq!(credentials.len(), 1);
        assert_eq!(credentials[0].credential_id, "cred-a");
    }

    #[tokio::test]
    async fn test_update_passkey_stores_counter_and_last_used() {
        let db = get_in_memory_db().await;
        let user = create_user(&db, 40004, "counteruser").await;

        PasskeyCredentialRepository::create(&db, user.id, "cred-c", "{\"counter\":0}", "Key")
            .await
            .unwrap();

        PasskeyCredentialRepository::update_passkey(&db, "cred-c", "{\"counter\":1}", 1700000000)
            .await
            .unwrap();

        let updated = PasskeyCredentialRepository::find_by_credential_id(&db, "cred-c")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(updated.passkey, "{\"counter\":1}");
        assert_eq!(updated.last_used_at, Some(1700000000));
    }

    #[tokio::test]
    async fn test_delete_is_scoped_to_owner() {
        let db = get_in_memory_db().await;
        let owner = create_user(&db, 40005, "owner").await;
        let stranger = create_user(&db, 40006, "stranger").await;

        let credential = PasskeyCredentialRepository::create(&db, owner.id, "cred-d", "{}", "Key")
            .await
            .unwrap();

        let deleted_by_stranger =
            PasskeyCredentialRepository::delete(&db, stranger.id, credential.id)
                .await
                .unwrap();
        assert_eq!(deleted_by_stranger, 0);
        assert!(
            PasskeyCredentialRepository::find_by_credential_id(&db, "cred-d")
                .await
                .unwrap()
                .is_some()
        );

        let deleted_by_owner = PasskeyCredentialRepository::delete(&db, owner.id, credential.id)
            .await
            .unwrap();
        assert_eq!(deleted_by_owner, 1);
        assert!(
            PasskeyCredentialRepository::find_by_credential_id(&db, "cred-d")
                .await
                .unwrap()
                .is_none()
        );
    }

    #[tokio::test]
    async fn test_delete_by_user_id_and_count() {
        let db = get_in_memory_db().await;
        let user = create_user(&db, 40007, "bulkuser").await;

        PasskeyCredentialRepository::create(&db, user.id, "cred-e", "{}", "Key 1")
            .await
            .unwrap();
        PasskeyCredentialRepository::create(&db, user.id, "cred-f", "{}", "Key 2")
            .await
            .unwrap();

        assert_eq!(
            PasskeyCredentialRepository::count_by_user_id(&db, user.id)
                .await
                .unwrap(),
            2
        );

        let deleted = PasskeyCredentialRepository::delete_by_user_id(&db, user.id)
            .await
            .unwrap();

        assert_eq!(deleted, 2);
        assert_eq!(
            PasskeyCredentialRepository::count_by_user_id(&db, user.id)
                .await
                .unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn test_challenge_can_be_taken_once() {
        let db = get_in_memory_db().await;

        let challenge = PasskeyChallenge {
            id: "challenge-1".to_string(),
            user_id: None,
            operation: ChallengeOperation::Authentication,
            state: "{\"state\":true}".to_string(),
            created_at: 1000,
            expires_at: 1300,
        };

        PasskeyChallengeRepository::create(&db, &challenge)
            .await
            .unwrap();

        let taken = PasskeyChallengeRepository::take(&db, "challenge-1")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(taken.operation, ChallengeOperation::Authentication);
        assert_eq!(taken.state, "{\"state\":true}");
        assert_eq!(taken.user_id, None);

        let taken_again = PasskeyChallengeRepository::take(&db, "challenge-1")
            .await
            .unwrap();
        assert!(taken_again.is_none());
    }

    #[tokio::test]
    async fn test_challenge_keeps_user_and_operation() {
        let db = get_in_memory_db().await;
        let user = create_user(&db, 40008, "challengeuser").await;

        let challenge = PasskeyChallenge {
            id: "challenge-2".to_string(),
            user_id: Some(user.id),
            operation: ChallengeOperation::Registration,
            state: "{}".to_string(),
            created_at: 1000,
            expires_at: 1300,
        };

        PasskeyChallengeRepository::create(&db, &challenge)
            .await
            .unwrap();

        let taken = PasskeyChallengeRepository::take(&db, "challenge-2")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(taken.user_id, Some(user.id));
        assert_eq!(taken.operation, ChallengeOperation::Registration);
    }

    #[tokio::test]
    async fn test_delete_expired_challenges() {
        let db = get_in_memory_db().await;

        for (id, expires_at) in [("expired", 900_i64), ("valid", 5000_i64)] {
            let challenge = PasskeyChallenge {
                id: id.to_string(),
                user_id: None,
                operation: ChallengeOperation::Authentication,
                state: "{}".to_string(),
                created_at: 100,
                expires_at,
            };

            PasskeyChallengeRepository::create(&db, &challenge)
                .await
                .unwrap();
        }

        PasskeyChallengeRepository::delete_expired(&db, 1000)
            .await
            .unwrap();

        assert!(
            PasskeyChallengeRepository::take(&db, "expired")
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            PasskeyChallengeRepository::take(&db, "valid")
                .await
                .unwrap()
                .is_some()
        );
    }

    #[tokio::test]
    async fn test_webauthn_id_roundtrip() {
        let db = get_in_memory_db().await;
        let user = create_user(&db, 40009, "handleuser").await;

        assert!(db.find_webauthn_id(user.id).await.unwrap().is_none());

        db.set_webauthn_id(user.id, "1b1e9d4a-0f7f-4a0e-9d0c-4d7a1b2c3d4e")
            .await
            .unwrap();

        assert_eq!(
            db.find_webauthn_id(user.id).await.unwrap(),
            Some("1b1e9d4a-0f7f-4a0e-9d0c-4d7a1b2c3d4e".to_string())
        );

        let found = db
            .find_by_webauthn_id("1b1e9d4a-0f7f-4a0e-9d0c-4d7a1b2c3d4e")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(found.id, user.id);
        assert_eq!(found.username, "handleuser");
    }

    #[tokio::test]
    async fn test_find_by_webauthn_id_not_found() {
        let db = get_in_memory_db().await;

        let found = db.find_by_webauthn_id("unknown-handle").await.unwrap();

        assert!(found.is_none());
    }
}
