use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;
use webauthn_rs::prelude::{CreationChallengeResponse, RequestChallengeResponse};

/// A passkey registered by a user. `passkey` holds the serialized
/// `webauthn_rs::prelude::Passkey` value, `credential_id` its base64url identifier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, FromRow)]
pub struct PasskeyCredential {
    pub id: i64,
    pub user_id: i64,
    pub credential_id: String,
    pub passkey: String,
    pub name: String,
    pub created_at: i64,
    pub last_used_at: Option<i64>,
}

/// WebAuthn ceremony state kept between the begin and the finish request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasskeyChallenge {
    pub id: String,
    pub user_id: Option<i64>,
    pub operation: ChallengeOperation,
    pub state: String,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChallengeOperation {
    Registration,
    Authentication,
}

impl ChallengeOperation {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "registration" => Some(ChallengeOperation::Registration),
            "authentication" => Some(ChallengeOperation::Authentication),
            _ => None,
        }
    }
}

impl std::fmt::Display for ChallengeOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChallengeOperation::Registration => write!(f, "registration"),
            ChallengeOperation::Authentication => write!(f, "authentication"),
        }
    }
}

/// Options handed to the browser to start a registration ceremony.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasskeyRegistrationStart {
    pub challenge_id: String,
    pub options: CreationChallengeResponse,
}

/// Options handed to the browser to start an authentication ceremony.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasskeyAuthenticationStart {
    pub challenge_id: String,
    pub options: RequestChallengeResponse,
}

#[derive(Debug, Error)]
pub enum PasskeyError {
    #[error("Passkey authentication is not enabled")]
    Disabled,

    #[error("Challenge not found or expired")]
    ChallengeNotFound,

    #[error("Challenge does not match the requested operation")]
    ChallengeMismatch,

    #[error("No account is linked to this passkey")]
    UnknownUser,

    #[error("Passkey not found")]
    CredentialNotFound,

    #[error("This passkey is already registered")]
    CredentialAlreadyRegistered,

    #[error("WebAuthn ceremony failed: {0}")]
    Webauthn(String),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
