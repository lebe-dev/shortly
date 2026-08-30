use log::info;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

use super::model::{AuthError, Session, User};
use super::ports::{AuthService, OAuthClient, SessionRepository, UserRepository};

const SESSION_TOKEN_BYTES: usize = 32; // 256 bits

#[derive(Clone)]
pub struct AuthServiceImpl<UR, SR, OC>
where
    UR: UserRepository,
    SR: SessionRepository,
    OC: OAuthClient,
{
    user_repo: UR,
    session_repo: SR,
    oauth_client: OC,
    oauth_base_url: String,
    application_id: String,
    redirect_uri: String,
    session_ttl_days: Option<u32>,
}

impl<UR, SR, OC> AuthServiceImpl<UR, SR, OC>
where
    UR: UserRepository,
    SR: SessionRepository,
    OC: OAuthClient,
{
    pub fn new(
        user_repo: UR,
        session_repo: SR,
        oauth_client: OC,
        oauth_base_url: String,
        application_id: String,
        redirect_uri: String,
        session_ttl_days: Option<u32>,
    ) -> Self {
        Self {
            user_repo,
            session_repo,
            oauth_client,
            oauth_base_url,
            application_id,
            redirect_uri,
            session_ttl_days,
        }
    }

    fn generate_state(&self) -> String {
        let random_number: u128 = rand::thread_rng().r#gen();
        base62::encode(random_number)
    }

    fn generate_session_token(&self) -> String {
        let mut rng = rand::thread_rng();
        let random_bytes: [u8; SESSION_TOKEN_BYTES] = rng.r#gen();

        let mut bytes_16 = [0u8; 16];
        bytes_16.copy_from_slice(&random_bytes[0..16]);
        let random_number = u128::from_be_bytes(bytes_16);

        base62::encode(random_number)
    }

    fn get_current_timestamp(&self) -> Result<i64, AuthError> {
        Ok(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AuthError::Unknown(anyhow::anyhow!("Failed to get timestamp: {}", e)))?
            .as_secs() as i64)
    }
}

impl<UR, SR, OC> AuthService for AuthServiceImpl<UR, SR, OC>
where
    UR: UserRepository,
    SR: SessionRepository,
    OC: OAuthClient,
{
    async fn generate_oauth_url(&self) -> Result<(String, String), AuthError> {
        let state = self.generate_state();

        let auth_url = format!(
            "{}/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&state={}&scope=read_user",
            self.oauth_base_url,
            urlencoding::encode(&self.application_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&state)
        );

        info!("Generated OAuth URL with state: {}", state);

        Ok((auth_url, state))
    }

    async fn complete_oauth(&self, code: &str, _state: &str) -> Result<(User, Session), AuthError> {
        info!("Completing OAuth flow with code");

        let access_token = self.oauth_client.exchange_code(code).await?;

        let gitlab_user = self.oauth_client.fetch_user_info(&access_token).await?;

        let user = self.user_repo.upsert(&gitlab_user).await?;

        let token = self.generate_session_token();
        let current_time = self.get_current_timestamp()?;

        let expires_at = self
            .session_ttl_days
            .map(|days| current_time + (days as i64 * 86400));

        let session = self
            .session_repo
            .create(user.id, &token, expires_at)
            .await?;

        info!("User {} logged in successfully", user.username);

        Ok((user, session))
    }

    async fn create_session(&self, user_id: i64) -> Result<Session, AuthError> {
        let token = self.generate_session_token();
        let current_time = self.get_current_timestamp()?;

        let expires_at = self
            .session_ttl_days
            .map(|days| current_time + (days as i64 * 86400));

        let session = self
            .session_repo
            .create(user_id, &token, expires_at)
            .await?;

        Ok(session)
    }

    async fn validate_session(&self, token: &str) -> Result<User, AuthError> {
        let session = self
            .session_repo
            .find_by_token(token)
            .await?
            .ok_or(AuthError::InvalidSession)?;

        if let Some(expires_at) = session.expires_at {
            let current_time = self.get_current_timestamp()?;
            if current_time > expires_at {
                self.session_repo.delete(token).await?;
                return Err(AuthError::SessionExpired);
            }
        }

        let current_time = self.get_current_timestamp()?;
        self.session_repo
            .update_last_used(token, current_time)
            .await?;

        let user = self
            .user_repo
            .find_by_id(session.user_id)
            .await?
            .ok_or(AuthError::InvalidSession)?;

        Ok(user)
    }

    async fn logout(&self, token: &str) -> Result<(), AuthError> {
        self.session_repo.delete(token).await?;
        info!("User logged out");
        Ok(())
    }

    async fn cleanup_expired_sessions(&self) -> Result<(), AuthError> {
        let current_time = self.get_current_timestamp()?;
        self.session_repo.delete_expired(current_time).await?;
        info!("Expired sessions cleaned up");
        Ok(())
    }
}
