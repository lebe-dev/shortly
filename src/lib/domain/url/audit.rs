use serde::{Deserialize, Serialize};

/// Parameters for querying audit events
#[derive(Debug, Clone, Default)]
pub struct AuditQueryParams {
    pub event_type: Option<AuditEventType>,
    pub actor_user_id: Option<i64>,
    pub target_user_id: Option<i64>,
    pub url_name: Option<String>,
    pub username: Option<String>,
    pub date_from: Option<i64>,
    pub date_to: Option<i64>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UrlAuditEvent {
    pub id: Option<i64>,
    pub event_type: AuditEventType,
    pub actor_user_id: i64,
    pub target_user_id: i64,
    pub url_name: Option<String>,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    CreateUrl,
    DeleteUrl,
    UserLogin,
    UserLogout,
    UserQuotaUpdate,
}

impl std::fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditEventType::CreateUrl => write!(f, "create_url"),
            AuditEventType::DeleteUrl => write!(f, "delete_url"),
            AuditEventType::UserLogin => write!(f, "user_login"),
            AuditEventType::UserLogout => write!(f, "user_logout"),
            AuditEventType::UserQuotaUpdate => write!(f, "user_quota_update"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuditEventWithUser {
    pub id: i64,
    pub event_type: AuditEventType,
    pub actor_user_id: i64,
    pub actor_username: String,
    pub target_user_id: i64,
    pub target_username: String,
    pub url_name: Option<String>,
    pub created_at: i64,
}
