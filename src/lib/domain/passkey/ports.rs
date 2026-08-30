use std::future::Future;

use webauthn_rs::prelude::{PublicKeyCredential, RegisterPublicKeyCredential};

use crate::domain::auth::model::{Session, User};

use super::model::{
    PasskeyAuthenticationStart, PasskeyChallenge, PasskeyCredential, PasskeyError,
    PasskeyRegistrationStart,
};

pub trait PasskeyService: Send + Sync + Clone + 'static {
    /// Begin registration of a new passkey for an already authenticated user
    fn start_registration(
        &self,
        user: &User,
    ) -> impl Future<Output = Result<PasskeyRegistrationStart, PasskeyError>> + Send;

    /// Complete registration and store the credential
    fn finish_registration(
        &self,
        user: &User,
        challenge_id: &str,
        credential: &RegisterPublicKeyCredential,
        name: Option<String>,
    ) -> impl Future<Output = Result<PasskeyCredential, PasskeyError>> + Send;

    /// Begin a passwordless login ceremony (no username required)
    fn start_authentication(
        &self,
    ) -> impl Future<Output = Result<PasskeyAuthenticationStart, PasskeyError>> + Send;

    /// Complete the login ceremony and open a session for the existing account
    fn finish_authentication(
        &self,
        challenge_id: &str,
        credential: &PublicKeyCredential,
    ) -> impl Future<Output = Result<(User, Session), PasskeyError>> + Send;

    /// List passkeys owned by a user
    fn list_credentials(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Vec<PasskeyCredential>, PasskeyError>> + Send;

    /// Delete a single passkey owned by a user
    fn delete_credential(
        &self,
        user_id: i64,
        credential_pk: i64,
    ) -> impl Future<Output = Result<(), PasskeyError>> + Send;

    /// Delete every passkey of a user (used by administrators)
    fn delete_user_credentials(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<u64, PasskeyError>> + Send;

    /// Count passkeys owned by a user
    fn count_credentials(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<i64, PasskeyError>> + Send;

    /// Remove ceremony state that was never completed
    fn cleanup_expired_challenges(&self) -> impl Future<Output = Result<(), PasskeyError>> + Send;
}

pub trait PasskeyCredentialRepository: Send + Sync + Clone + 'static {
    /// Store a new credential
    fn create(
        &self,
        user_id: i64,
        credential_id: &str,
        passkey: &str,
        name: &str,
    ) -> impl Future<Output = Result<PasskeyCredential, sqlx::Error>> + Send;

    /// All credentials of a user, newest first
    fn find_by_user_id(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Vec<PasskeyCredential>, sqlx::Error>> + Send;

    /// Find a credential by its base64url identifier
    fn find_by_credential_id(
        &self,
        credential_id: &str,
    ) -> impl Future<Output = Result<Option<PasskeyCredential>, sqlx::Error>> + Send;

    /// Persist the updated credential state (signature counter, backup flags)
    fn update_passkey(
        &self,
        credential_id: &str,
        passkey: &str,
        last_used_at: i64,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    /// Delete a credential, scoped to its owner. Returns the number of deleted rows
    fn delete(
        &self,
        user_id: i64,
        credential_pk: i64,
    ) -> impl Future<Output = Result<u64, sqlx::Error>> + Send;

    /// Delete every credential of a user. Returns the number of deleted rows
    fn delete_by_user_id(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<u64, sqlx::Error>> + Send;

    /// Number of credentials owned by a user
    fn count_by_user_id(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<i64, sqlx::Error>> + Send;
}

pub trait PasskeyChallengeRepository: Send + Sync + Clone + 'static {
    /// Store ceremony state
    fn create(
        &self,
        challenge: &PasskeyChallenge,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    /// Fetch and delete ceremony state. Each challenge may be used only once
    fn take(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<PasskeyChallenge>, sqlx::Error>> + Send;

    /// Delete ceremony state that has expired
    fn delete_expired(
        &self,
        current_time: i64,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
}

/// Access to the stable WebAuthn user handle stored alongside the account.
pub trait WebauthnUserRepository: Send + Sync + Clone + 'static {
    /// Read the WebAuthn handle of a user, if one was already assigned
    fn find_webauthn_id(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Option<String>, sqlx::Error>> + Send;

    /// Assign a WebAuthn handle to a user
    fn set_webauthn_id(
        &self,
        user_id: i64,
        webauthn_id: &str,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    /// Find the account a WebAuthn handle belongs to
    fn find_by_webauthn_id(
        &self,
        webauthn_id: &str,
    ) -> impl Future<Output = Result<Option<User>, sqlx::Error>> + Send;
}
