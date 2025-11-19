use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub id: String,
    pub original_url: String,
    /// TTL in seconds
    pub ttl: u32,
    pub created: i64,
}

#[derive(Debug, Error)]
pub enum ShortUrlGenerationError {
    #[error("Invalid original URL")]
    InvalidOriginalUrl,
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
