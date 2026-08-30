use anyhow::anyhow;
use sqlx::{PgPool, SqlitePool};

use crate::domain::auth::model::{GitlabUserInfo, Session, User};
use crate::domain::auth::ports::{SessionRepository, UserRepository};
use crate::domain::passkey::model::{PasskeyChallenge, PasskeyCredential};
use crate::domain::passkey::ports::{
    PasskeyChallengeRepository, PasskeyCredentialRepository, WebauthnUserRepository,
};
use crate::domain::url::audit::{AuditEventWithUser, AuditQueryParams, UrlAuditEvent};
use crate::domain::url::model::{Url, UserQuotas};
use crate::domain::url::ports::UrlRepository;
use crate::outbound::postgres::init::Postgres;
use crate::outbound::sqlite::init::Sqlite;

#[derive(Debug, Clone)]
pub enum Database {
    Sqlite(Sqlite),
    Postgres(Postgres),
}

#[derive(Debug, Clone)]
pub enum DatabasePool {
    Sqlite(SqlitePool),
    Postgres(PgPool),
}

impl Database {
    pub async fn from_connection_string(cnn: &str) -> Result<Self, anyhow::Error> {
        if cnn.starts_with("sqlite://") {
            Ok(Database::Sqlite(Sqlite::new(cnn).await?))
        } else if cnn.starts_with("postgres://") || cnn.starts_with("postgresql://") {
            Ok(Database::Postgres(Postgres::new(cnn).await?))
        } else {
            Err(anyhow!(
                "Unsupported database type. Use sqlite:// or postgres://"
            ))
        }
    }

    pub fn db_type(&self) -> &'static str {
        match self {
            Database::Sqlite(_) => "sqlite",
            Database::Postgres(_) => "postgres",
        }
    }

    pub fn get_pool(&self) -> DatabasePool {
        match self {
            Database::Sqlite(db) => DatabasePool::Sqlite(db.get_pool().clone()),
            Database::Postgres(db) => DatabasePool::Postgres(db.get_pool().clone()),
        }
    }
}

impl UrlRepository for Database {
    async fn save(&self, url: &Url) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.save(url).await,
            Database::Postgres(db) => db.save(url).await,
        }
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Url>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => UrlRepository::find_by_id(db, id).await,
            Database::Postgres(db) => UrlRepository::find_by_id(db, id).await,
        }
    }

    async fn find_by_custom_name(&self, custom_name: &str) -> Result<Option<Url>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.find_by_custom_name(custom_name).await,
            Database::Postgres(db) => db.find_by_custom_name(custom_name).await,
        }
    }

    async fn delete_expired(&self) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => UrlRepository::delete_expired(db).await,
            Database::Postgres(db) => UrlRepository::delete_expired(db).await,
        }
    }

    async fn find_by_user_id(&self, user_id: i64) -> Result<Vec<Url>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => UrlRepository::find_by_user_id(db, user_id).await,
            Database::Postgres(db) => UrlRepository::find_by_user_id(db, user_id).await,
        }
    }

    async fn delete_by_id(&self, id: &str) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.delete_by_id(id).await,
            Database::Postgres(db) => db.delete_by_id(id).await,
        }
    }

    async fn count_by_user_id(&self, user_id: i64) -> Result<i64, sqlx::Error> {
        match self {
            Database::Sqlite(db) => UrlRepository::count_by_user_id(db, user_id).await,
            Database::Postgres(db) => UrlRepository::count_by_user_id(db, user_id).await,
        }
    }

    async fn count_by_user_id_since(
        &self,
        user_id: i64,
        since_timestamp: i64,
    ) -> Result<i64, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.count_by_user_id_since(user_id, since_timestamp).await,
            Database::Postgres(db) => db.count_by_user_id_since(user_id, since_timestamp).await,
        }
    }

    async fn find_user_quotas(&self, user_id: i64) -> Result<Option<UserQuotas>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.find_user_quotas(user_id).await,
            Database::Postgres(db) => db.find_user_quotas(user_id).await,
        }
    }

    async fn record_audit_event(&self, event: &UrlAuditEvent) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.record_audit_event(event).await,
            Database::Postgres(db) => db.record_audit_event(event).await,
        }
    }

    async fn find_all(&self) -> Result<Vec<Url>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => UrlRepository::find_all(db).await,
            Database::Postgres(db) => UrlRepository::find_all(db).await,
        }
    }

    async fn find_audit_events(
        &self,
        params: &AuditQueryParams,
    ) -> Result<(Vec<AuditEventWithUser>, i64), sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.find_audit_events(params).await,
            Database::Postgres(db) => db.find_audit_events(params).await,
        }
    }

    async fn update_last_accessed(&self, url_id: &str, timestamp: i64) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.update_last_accessed(url_id, timestamp).await,
            Database::Postgres(db) => db.update_last_accessed(url_id, timestamp).await,
        }
    }
}

// Implement UserRepository trait for Database enum
impl UserRepository for Database {
    async fn find_by_gitlab_id(&self, gitlab_id: i64) -> Result<Option<User>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.find_by_gitlab_id(gitlab_id).await,
            Database::Postgres(db) => db.find_by_gitlab_id(gitlab_id).await,
        }
    }

    async fn upsert(&self, gitlab_user: &GitlabUserInfo) -> Result<User, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.upsert(gitlab_user).await,
            Database::Postgres(db) => db.upsert(gitlab_user).await,
        }
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => UserRepository::find_by_id(db, id).await,
            Database::Postgres(db) => UserRepository::find_by_id(db, id).await,
        }
    }

    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => UserRepository::find_all(db).await,
            Database::Postgres(db) => UserRepository::find_all(db).await,
        }
    }

    async fn update_quotas(
        &self,
        user_id: i64,
        max_urls_per_user: Option<i32>,
        max_urls_per_day: Option<i32>,
    ) -> Result<User, sqlx::Error> {
        match self {
            Database::Sqlite(db) => {
                db.update_quotas(user_id, max_urls_per_user, max_urls_per_day)
                    .await
            }
            Database::Postgres(db) => {
                db.update_quotas(user_id, max_urls_per_user, max_urls_per_day)
                    .await
            }
        }
    }
}

// Implement SessionRepository trait for Database enum
impl SessionRepository for Database {
    async fn create(
        &self,
        user_id: i64,
        token: &str,
        expires_at: Option<i64>,
    ) -> Result<Session, sqlx::Error> {
        match self {
            Database::Sqlite(db) => SessionRepository::create(db, user_id, token, expires_at).await,
            Database::Postgres(db) => {
                SessionRepository::create(db, user_id, token, expires_at).await
            }
        }
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<Session>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.find_by_token(token).await,
            Database::Postgres(db) => db.find_by_token(token).await,
        }
    }

    async fn update_last_used(&self, token: &str, timestamp: i64) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.update_last_used(token, timestamp).await,
            Database::Postgres(db) => db.update_last_used(token, timestamp).await,
        }
    }

    async fn delete(&self, token: &str) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => SessionRepository::delete(db, token).await,
            Database::Postgres(db) => SessionRepository::delete(db, token).await,
        }
    }

    async fn delete_expired(&self, current_time: i64) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => SessionRepository::delete_expired(db, current_time).await,
            Database::Postgres(db) => SessionRepository::delete_expired(db, current_time).await,
        }
    }
}

impl PasskeyCredentialRepository for Database {
    async fn create(
        &self,
        user_id: i64,
        credential_id: &str,
        passkey: &str,
        name: &str,
    ) -> Result<PasskeyCredential, sqlx::Error> {
        match self {
            Database::Sqlite(db) => {
                PasskeyCredentialRepository::create(db, user_id, credential_id, passkey, name).await
            }
            Database::Postgres(db) => {
                PasskeyCredentialRepository::create(db, user_id, credential_id, passkey, name).await
            }
        }
    }

    async fn find_by_user_id(&self, user_id: i64) -> Result<Vec<PasskeyCredential>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => PasskeyCredentialRepository::find_by_user_id(db, user_id).await,
            Database::Postgres(db) => {
                PasskeyCredentialRepository::find_by_user_id(db, user_id).await
            }
        }
    }

    async fn find_by_credential_id(
        &self,
        credential_id: &str,
    ) -> Result<Option<PasskeyCredential>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.find_by_credential_id(credential_id).await,
            Database::Postgres(db) => db.find_by_credential_id(credential_id).await,
        }
    }

    async fn update_passkey(
        &self,
        credential_id: &str,
        passkey: &str,
        last_used_at: i64,
    ) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => {
                db.update_passkey(credential_id, passkey, last_used_at)
                    .await
            }
            Database::Postgres(db) => {
                db.update_passkey(credential_id, passkey, last_used_at)
                    .await
            }
        }
    }

    async fn delete(&self, user_id: i64, credential_pk: i64) -> Result<u64, sqlx::Error> {
        match self {
            Database::Sqlite(db) => {
                PasskeyCredentialRepository::delete(db, user_id, credential_pk).await
            }
            Database::Postgres(db) => {
                PasskeyCredentialRepository::delete(db, user_id, credential_pk).await
            }
        }
    }

    async fn delete_by_user_id(&self, user_id: i64) -> Result<u64, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.delete_by_user_id(user_id).await,
            Database::Postgres(db) => db.delete_by_user_id(user_id).await,
        }
    }

    async fn count_by_user_id(&self, user_id: i64) -> Result<i64, sqlx::Error> {
        match self {
            Database::Sqlite(db) => {
                PasskeyCredentialRepository::count_by_user_id(db, user_id).await
            }
            Database::Postgres(db) => {
                PasskeyCredentialRepository::count_by_user_id(db, user_id).await
            }
        }
    }
}

impl PasskeyChallengeRepository for Database {
    async fn create(&self, challenge: &PasskeyChallenge) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => PasskeyChallengeRepository::create(db, challenge).await,
            Database::Postgres(db) => PasskeyChallengeRepository::create(db, challenge).await,
        }
    }

    async fn take(&self, id: &str) -> Result<Option<PasskeyChallenge>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.take(id).await,
            Database::Postgres(db) => db.take(id).await,
        }
    }

    async fn delete_expired(&self, current_time: i64) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => {
                PasskeyChallengeRepository::delete_expired(db, current_time).await
            }
            Database::Postgres(db) => {
                PasskeyChallengeRepository::delete_expired(db, current_time).await
            }
        }
    }
}

impl WebauthnUserRepository for Database {
    async fn find_webauthn_id(&self, user_id: i64) -> Result<Option<String>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.find_webauthn_id(user_id).await,
            Database::Postgres(db) => db.find_webauthn_id(user_id).await,
        }
    }

    async fn set_webauthn_id(&self, user_id: i64, webauthn_id: &str) -> Result<(), sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.set_webauthn_id(user_id, webauthn_id).await,
            Database::Postgres(db) => db.set_webauthn_id(user_id, webauthn_id).await,
        }
    }

    async fn find_by_webauthn_id(&self, webauthn_id: &str) -> Result<Option<User>, sqlx::Error> {
        match self {
            Database::Sqlite(db) => db.find_by_webauthn_id(webauthn_id).await,
            Database::Postgres(db) => db.find_by_webauthn_id(webauthn_id).await,
        }
    }
}
