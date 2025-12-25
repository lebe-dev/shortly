use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub id: String,
    pub original_url: String,
    /// TTL in seconds (0 = no expiration for named URLs)
    pub ttl: u32,
    pub created: i64,
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
}

#[derive(Debug, Error)]
pub enum ShortUrlGenerationError {
    #[error("Invalid original URL")]
    InvalidOriginalUrl,
    #[error("Invalid custom name: {0}")]
    InvalidCustomName(String),
    #[error("Custom name already exists")]
    CustomNameExists,
    #[error("Custom name is reserved")]
    CustomNameReserved,
    #[error("Rate limit exceeded: too many URLs created today")]
    RateLimitExceeded,
    #[error("User URL limit exceeded")]
    UserLimitExceeded,
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum CheckCustomNameError {
    #[error("Custom name already exists")]
    AlreadyExists,
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub type CleanupExpiredUrlsError = FindUrlError;

#[derive(Debug, Error)]
pub enum FindUrlError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum DeleteUrlError {
    #[error("URL not found")]
    NotFound,
    #[error("Unauthorized: you can only delete your own URLs")]
    Unauthorized,
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
