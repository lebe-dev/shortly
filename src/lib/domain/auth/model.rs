use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: i64,
    pub gitlab_id: i64,
    pub username: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub max_urls_per_user: i32,
    pub max_urls_per_day: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct Session {
    pub id: i64,
    pub token: String,
    pub user_id: i64,
    pub created_at: i64,
    pub last_used_at: i64,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitlabUserInfo {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid OAuth state parameter")]
    InvalidState,

    #[error("OAuth authorization failed: {0}")]
    OAuthFailed(String),

    #[error("Failed to fetch user info from GitLab")]
    UserInfoFailed,

    #[error("Invalid session token")]
    InvalidSession,

    #[error("Session expired")]
    SessionExpired,

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    HttpError(#[from] reqwest::Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
