use std::future::Future;

use super::model::{AuthError, GitlabUserInfo, Session, User};

pub trait AuthService: Send + Sync + Clone + 'static {
    /// Generate OAuth authorization URL and state
    fn generate_oauth_url(
        &self,
    ) -> impl Future<Output = Result<(String, String), AuthError>> + Send;

    /// Complete OAuth flow: exchange code for token, fetch user info, create session
    fn complete_oauth(
        &self,
        code: &str,
        state: &str,
    ) -> impl Future<Output = Result<(User, Session), AuthError>> + Send;

    /// Validate session token and return user
    fn validate_session(&self, token: &str)
    -> impl Future<Output = Result<User, AuthError>> + Send;

    /// Logout (invalidate session)
    fn logout(&self, token: &str) -> impl Future<Output = Result<(), AuthError>> + Send;

    /// Cleanup expired sessions
    fn cleanup_expired_sessions(&self) -> impl Future<Output = Result<(), AuthError>> + Send;
}

pub trait UserRepository: Send + Sync + Clone + 'static {
    /// Find user by GitLab ID
    fn find_by_gitlab_id(
        &self,
        gitlab_id: i64,
    ) -> impl Future<Output = Result<Option<User>, sqlx::Error>> + Send;

    /// Create or update user
    fn upsert(
        &self,
        gitlab_user: &GitlabUserInfo,
    ) -> impl Future<Output = Result<User, sqlx::Error>> + Send;

    /// Find user by internal ID
    fn find_by_id(&self, id: i64)
    -> impl Future<Output = Result<Option<User>, sqlx::Error>> + Send;
}

pub trait SessionRepository: Send + Sync + Clone + 'static {
    /// Create new session
    fn create(
        &self,
        user_id: i64,
        token: &str,
        expires_at: Option<i64>,
    ) -> impl Future<Output = Result<Session, sqlx::Error>> + Send;

    /// Find session by token
    fn find_by_token(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<Option<Session>, sqlx::Error>> + Send;

    /// Update last_used_at timestamp
    fn update_last_used(
        &self,
        token: &str,
        timestamp: i64,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    /// Delete session (logout)
    fn delete(&self, token: &str) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    /// Delete expired sessions
    fn delete_expired(
        &self,
        current_time: i64,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
}

pub trait OAuthClient: Send + Sync + Clone + 'static {
    /// Exchange authorization code for access token
    fn exchange_code(&self, code: &str) -> impl Future<Output = Result<String, AuthError>> + Send;

    /// Fetch GitLab user info using access token
    fn fetch_user_info(
        &self,
        access_token: &str,
    ) -> impl Future<Output = Result<GitlabUserInfo, AuthError>> + Send;
}
