use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use log::{info, warn};
use uuid::Uuid;
use webauthn_rs::prelude::{
    DiscoverableAuthentication, DiscoverableKey, Passkey, PasskeyRegistration, PublicKeyCredential,
    RegisterPublicKeyCredential,
};
use webauthn_rs::{Webauthn, WebauthnBuilder};

use crate::domain::auth::model::{Session, User};
use crate::domain::auth::ports::{AuthService, UserRepository};

use super::model::{
    ChallengeOperation, PasskeyAuthenticationStart, PasskeyChallenge, PasskeyCredential,
    PasskeyError, PasskeyRegistrationStart,
};
use super::ports::{
    PasskeyChallengeRepository, PasskeyCredentialRepository, PasskeyService, WebauthnUserRepository,
};

/// Default name given to a credential when the browser does not suggest one.
const DEFAULT_CREDENTIAL_NAME: &str = "Passkey";

/// Maximum length of a user supplied credential name.
const MAX_CREDENTIAL_NAME_LENGTH: usize = 64;

/// Relying party settings required to run WebAuthn ceremonies.
#[derive(Debug, Clone)]
pub struct PasskeyServiceConfig {
    /// Relying party ID, the registrable domain of the service (e.g. `shortly.company.com`)
    pub rp_id: String,
    /// Origin the browser must report (e.g. `https://shortly.company.com`)
    pub rp_origin: String,
    /// Name shown by the authenticator
    pub rp_name: String,
    /// Lifetime of an unfinished ceremony, in seconds
    pub challenge_ttl: i64,
}

#[derive(Clone)]
pub struct PasskeyServiceImpl<CR, HR, UR, A>
where
    CR: PasskeyCredentialRepository,
    HR: PasskeyChallengeRepository,
    UR: UserRepository + WebauthnUserRepository,
    A: AuthService,
{
    webauthn: Arc<Webauthn>,
    credential_repo: CR,
    challenge_repo: HR,
    user_repo: UR,
    auth_service: A,
    challenge_ttl: i64,
}

impl<CR, HR, UR, A> PasskeyServiceImpl<CR, HR, UR, A>
where
    CR: PasskeyCredentialRepository,
    HR: PasskeyChallengeRepository,
    UR: UserRepository + WebauthnUserRepository,
    A: AuthService,
{
    pub fn new(
        config: PasskeyServiceConfig,
        credential_repo: CR,
        challenge_repo: HR,
        user_repo: UR,
        auth_service: A,
    ) -> Result<Self, PasskeyError> {
        let origin = url::Url::parse(&config.rp_origin).map_err(|e| {
            PasskeyError::Unknown(anyhow::anyhow!(
                "invalid passkey relying party origin '{}': {}",
                config.rp_origin,
                e
            ))
        })?;

        let webauthn = WebauthnBuilder::new(&config.rp_id, &origin)
            .map_err(|e| PasskeyError::Webauthn(e.to_string()))?
            .rp_name(&config.rp_name)
            .build()
            .map_err(|e| PasskeyError::Webauthn(e.to_string()))?;

        Ok(Self {
            webauthn: Arc::new(webauthn),
            credential_repo,
            challenge_repo,
            user_repo,
            auth_service,
            challenge_ttl: config.challenge_ttl,
        })
    }

    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs() as i64)
            .unwrap_or_default()
    }

    /// Read the WebAuthn handle of a user, creating one on the first registration.
    async fn resolve_webauthn_id(&self, user_id: i64) -> Result<Uuid, PasskeyError> {
        if let Some(existing) = self.user_repo.find_webauthn_id(user_id).await?
            && let Ok(uuid) = Uuid::parse_str(&existing)
        {
            return Ok(uuid);
        }

        let webauthn_id = Uuid::new_v4();
        self.user_repo
            .set_webauthn_id(user_id, &webauthn_id.to_string())
            .await?;

        info!("assigned WebAuthn handle to user {}", user_id);

        Ok(webauthn_id)
    }

    async fn store_challenge<S: serde::Serialize>(
        &self,
        user_id: Option<i64>,
        operation: ChallengeOperation,
        state: &S,
    ) -> Result<String, PasskeyError> {
        let serialized = serde_json::to_string(state)
            .map_err(|e| PasskeyError::Unknown(anyhow::anyhow!("failed to store state: {}", e)))?;

        let created_at = Self::current_timestamp();

        let challenge = PasskeyChallenge {
            id: Uuid::new_v4().to_string(),
            user_id,
            operation,
            state: serialized,
            created_at,
            expires_at: created_at + self.challenge_ttl,
        };

        self.challenge_repo.create(&challenge).await?;

        Ok(challenge.id)
    }

    async fn take_challenge(
        &self,
        challenge_id: &str,
        operation: ChallengeOperation,
    ) -> Result<PasskeyChallenge, PasskeyError> {
        let challenge = self
            .challenge_repo
            .take(challenge_id)
            .await?
            .ok_or(PasskeyError::ChallengeNotFound)?;

        if challenge.operation != operation {
            return Err(PasskeyError::ChallengeMismatch);
        }

        if challenge.expires_at < Self::current_timestamp() {
            return Err(PasskeyError::ChallengeNotFound);
        }

        Ok(challenge)
    }

    fn decode_passkey(credential: &PasskeyCredential) -> Result<Passkey, PasskeyError> {
        serde_json::from_str(&credential.passkey).map_err(|e| {
            PasskeyError::Unknown(anyhow::anyhow!(
                "stored passkey {} is corrupted: {}",
                credential.credential_id,
                e
            ))
        })
    }

    fn encode_credential_id(raw: &[u8]) -> String {
        URL_SAFE_NO_PAD.encode(raw)
    }

    fn sanitize_name(name: Option<String>) -> String {
        let name = name.unwrap_or_default();
        let name = name.trim();

        if name.is_empty() {
            return DEFAULT_CREDENTIAL_NAME.to_string();
        }

        name.chars().take(MAX_CREDENTIAL_NAME_LENGTH).collect()
    }
}

impl<CR, HR, UR, A> PasskeyService for PasskeyServiceImpl<CR, HR, UR, A>
where
    CR: PasskeyCredentialRepository,
    HR: PasskeyChallengeRepository,
    UR: UserRepository + WebauthnUserRepository,
    A: AuthService,
{
    async fn start_registration(
        &self,
        user: &User,
    ) -> Result<PasskeyRegistrationStart, PasskeyError> {
        let webauthn_id = self.resolve_webauthn_id(user.id).await?;

        let existing = self.credential_repo.find_by_user_id(user.id).await?;

        let exclude_credentials = existing
            .iter()
            .map(Self::decode_passkey)
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|passkey| passkey.cred_id().clone())
            .collect::<Vec<_>>();

        let exclude_credentials = if exclude_credentials.is_empty() {
            None
        } else {
            Some(exclude_credentials)
        };

        let (options, state) = self
            .webauthn
            .start_passkey_registration(
                webauthn_id,
                &user.username,
                &user.username,
                exclude_credentials,
            )
            .map_err(|e| PasskeyError::Webauthn(e.to_string()))?;

        let challenge_id = self
            .store_challenge(Some(user.id), ChallengeOperation::Registration, &state)
            .await?;

        info!("started passkey registration for user {}", user.username);

        Ok(PasskeyRegistrationStart {
            challenge_id,
            options,
        })
    }

    async fn finish_registration(
        &self,
        user: &User,
        challenge_id: &str,
        credential: &RegisterPublicKeyCredential,
        name: Option<String>,
    ) -> Result<PasskeyCredential, PasskeyError> {
        let challenge = self
            .take_challenge(challenge_id, ChallengeOperation::Registration)
            .await?;

        if challenge.user_id != Some(user.id) {
            warn!(
                "passkey registration challenge does not belong to user {}",
                user.username
            );
            return Err(PasskeyError::ChallengeMismatch);
        }

        let state: PasskeyRegistration = serde_json::from_str(&challenge.state).map_err(|e| {
            PasskeyError::Unknown(anyhow::anyhow!("failed to read stored state: {}", e))
        })?;

        let passkey = self
            .webauthn
            .finish_passkey_registration(credential, &state)
            .map_err(|e| PasskeyError::Webauthn(e.to_string()))?;

        let credential_id = Self::encode_credential_id(passkey.cred_id());

        if self
            .credential_repo
            .find_by_credential_id(&credential_id)
            .await?
            .is_some()
        {
            return Err(PasskeyError::CredentialAlreadyRegistered);
        }

        let serialized = serde_json::to_string(&passkey).map_err(|e| {
            PasskeyError::Unknown(anyhow::anyhow!("failed to store passkey: {}", e))
        })?;

        let stored = self
            .credential_repo
            .create(
                user.id,
                &credential_id,
                &serialized,
                &Self::sanitize_name(name),
            )
            .await?;

        info!("registered a new passkey for user {}", user.username);

        Ok(stored)
    }

    async fn start_authentication(&self) -> Result<PasskeyAuthenticationStart, PasskeyError> {
        let (mut options, state) = self
            .webauthn
            .start_discoverable_authentication()
            .map_err(|e| PasskeyError::Webauthn(e.to_string()))?;

        // The login is started by a button, not by an autofill hint, so the
        // conditional mediation requested by the library must be dropped.
        options.mediation = None;

        let challenge_id = self
            .store_challenge(None, ChallengeOperation::Authentication, &state)
            .await?;

        Ok(PasskeyAuthenticationStart {
            challenge_id,
            options,
        })
    }

    async fn finish_authentication(
        &self,
        challenge_id: &str,
        credential: &PublicKeyCredential,
    ) -> Result<(User, Session), PasskeyError> {
        let challenge = self
            .take_challenge(challenge_id, ChallengeOperation::Authentication)
            .await?;

        let state: DiscoverableAuthentication =
            serde_json::from_str(&challenge.state).map_err(|e| {
                PasskeyError::Unknown(anyhow::anyhow!("failed to read stored state: {}", e))
            })?;

        let (webauthn_id, _) = self
            .webauthn
            .identify_discoverable_authentication(credential)
            .map_err(|e| PasskeyError::Webauthn(e.to_string()))?;

        // Passkeys may only be used to enter an account that already exists.
        let user = self
            .user_repo
            .find_by_webauthn_id(&webauthn_id.to_string())
            .await?
            .ok_or(PasskeyError::UnknownUser)?;

        let stored_credentials = self.credential_repo.find_by_user_id(user.id).await?;

        if stored_credentials.is_empty() {
            return Err(PasskeyError::UnknownUser);
        }

        let passkeys = stored_credentials
            .iter()
            .map(Self::decode_passkey)
            .collect::<Result<Vec<_>, _>>()?;

        let discoverable_keys = passkeys
            .iter()
            .map(DiscoverableKey::from)
            .collect::<Vec<_>>();

        let result = self
            .webauthn
            .finish_discoverable_authentication(credential, state, &discoverable_keys)
            .map_err(|e| PasskeyError::Webauthn(e.to_string()))?;

        let used_credential_id = Self::encode_credential_id(result.cred_id());

        let mut used_passkey = passkeys
            .into_iter()
            .find(|passkey| Self::encode_credential_id(passkey.cred_id()) == used_credential_id)
            .ok_or(PasskeyError::CredentialNotFound)?;

        used_passkey.update_credential(&result);

        let serialized = serde_json::to_string(&used_passkey).map_err(|e| {
            PasskeyError::Unknown(anyhow::anyhow!("failed to store passkey: {}", e))
        })?;

        self.credential_repo
            .update_passkey(&used_credential_id, &serialized, Self::current_timestamp())
            .await?;

        let session = self
            .auth_service
            .create_session(user.id)
            .await
            .map_err(|e| PasskeyError::Unknown(anyhow::anyhow!("{}", e)))?;

        info!("user {} logged in with a passkey", user.username);

        Ok((user, session))
    }

    async fn list_credentials(&self, user_id: i64) -> Result<Vec<PasskeyCredential>, PasskeyError> {
        Ok(self.credential_repo.find_by_user_id(user_id).await?)
    }

    async fn delete_credential(
        &self,
        user_id: i64,
        credential_pk: i64,
    ) -> Result<(), PasskeyError> {
        let deleted = self.credential_repo.delete(user_id, credential_pk).await?;

        if deleted == 0 {
            return Err(PasskeyError::CredentialNotFound);
        }

        info!("deleted passkey {} of user {}", credential_pk, user_id);

        Ok(())
    }

    async fn delete_user_credentials(&self, user_id: i64) -> Result<u64, PasskeyError> {
        let deleted = self.credential_repo.delete_by_user_id(user_id).await?;

        info!("deleted {} passkeys of user {}", deleted, user_id);

        Ok(deleted)
    }

    async fn count_credentials(&self, user_id: i64) -> Result<i64, PasskeyError> {
        Ok(self.credential_repo.count_by_user_id(user_id).await?)
    }

    async fn cleanup_expired_challenges(&self) -> Result<(), PasskeyError> {
        self.challenge_repo
            .delete_expired(Self::current_timestamp())
            .await?;

        Ok(())
    }
}
