use serde::Serialize;
use std::fmt;

use super::config::{AppConfig, AuthType};
use server_lib::domain::url::service::BASE_RESERVED_NAMES;

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SessionDto {
    pub authenticated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfoDto>,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoDto {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    /// Short URL TTL in hours
    pub short_url_ttl: u32,
    /// Maximum URL length in characters
    pub max_url_length: usize,
    /// Base URL for the service
    pub base_url: String,
    /// Feature flags configuration
    pub features: FeaturesConfigDto,
    /// Authentication configuration
    pub auth: AuthConfigDto,
    /// Scheduler configuration
    pub scheduler: SchedulerConfigDto,
    /// Metrics configuration
    pub metrics: MetricsConfigDto,
    /// Session information for the current user
    pub session: SessionDto,
    /// Admin data (only present for admin users)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<AdminDataDto>,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeaturesConfigDto {
    /// URL creation configuration
    pub create_url: CreateUrlConfigDto,
    /// Named URLs configuration
    pub named_urls: NamedUrlsConfigDto,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NamedUrlsConfigDto {
    pub enabled: bool,
    pub min_length: usize,
    pub max_length: usize,
    pub reserved_names: Vec<String>,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateUrlConfigDto {
    pub enabled: bool,
    pub auth_only: bool,
    pub max_per_user: u32,
    pub max_per_day: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_urls: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_urls_today: Option<u32>,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthConfigDto {
    /// Whether authentication is enabled
    pub enabled: bool,
    /// Authentication provider type (e.g., "gitlab")
    pub auth_type: AuthType,
    /// Optional note to display on login page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// GitLab provider configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitlab: Option<GitlabProviderDto>,
    /// Passkey provider configuration
    pub passkey: PasskeyProviderDto,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PasskeyProviderDto {
    /// Whether login with a passkey is available
    pub enabled: bool,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GitlabProviderDto {
    /// GitLab instance base URL
    pub base_url: String,
    /// GitLab application ID
    pub application_id: String,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchedulerConfigDto {
    /// Cron expression for cleanup job
    pub cleanup_expired_urls: String,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetricsConfigDto {
    /// Whether metrics endpoint is enabled
    pub enabled: bool,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdminDataDto {
    /// All URLs in the system (for admin users only)
    pub all_urls: Vec<AdminUrlDto>,
    /// All users in the system (for admin users only)
    pub users: Vec<AdminUserDto>,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdminUrlDto {
    /// Short URL ID
    pub id: String,
    /// Original long URL
    pub original_url: String,
    /// Creation timestamp (Unix epoch)
    pub created: i64,
    /// TTL in seconds (0 = no expiration)
    pub ttl: u32,
    /// User ID who created this URL
    pub user_id: Option<i64>,
    /// Username who created this URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Custom name for the URL (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// Last accessed timestamp (Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed: Option<i64>,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdminUserDto {
    /// User ID
    pub id: i64,
    /// Username
    pub username: String,
    /// Email (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Avatar URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Creation timestamp (Unix epoch)
    pub created_at: i64,
    /// Number of URLs created by this user
    pub url_count: u32,
    /// Maximum URLs allowed per user
    pub max_urls_per_user: i32,
    /// Maximum URLs allowed per day
    pub max_urls_per_day: i32,
    /// Whether this user is an admin
    pub is_admin: bool,
    /// Number of passkeys registered by this user
    pub passkey_count: i64,
}

impl From<AppConfig> for AppConfigDto {
    fn from(config: AppConfig) -> Self {
        let mut merged_reserved_names: Vec<String> =
            BASE_RESERVED_NAMES.iter().map(|s| s.to_string()).collect();

        for name in &config.features.named_urls.reserved_names {
            let name_lower = name.to_lowercase();
            if !merged_reserved_names
                .iter()
                .any(|r| r.to_lowercase() == name_lower)
            {
                merged_reserved_names.push(name.clone());
            }
        }

        let gitlab_dto = if config.auth.auth_type == AuthType::Gitlab {
            Some(GitlabProviderDto {
                base_url: config.auth.providers.gitlab.base_url.clone(),
                application_id: config.auth.providers.gitlab.application_id.clone(),
            })
        } else {
            None
        };

        AppConfigDto {
            short_url_ttl: config.short_url.ttl,
            max_url_length: config.short_url.max_length,
            base_url: config.base_url.clone(),
            features: FeaturesConfigDto {
                create_url: CreateUrlConfigDto {
                    enabled: config.features.create_url.enabled,
                    auth_only: config.features.create_url.auth_only,
                    max_per_user: config.features.create_url.max_per_user,
                    max_per_day: config.features.create_url.max_per_day,
                    current_urls: None,
                    current_urls_today: None,
                },
                named_urls: NamedUrlsConfigDto {
                    enabled: config.features.named_urls.enabled,
                    min_length: config.features.named_urls.min_length,
                    max_length: config.features.named_urls.max_length,
                    reserved_names: merged_reserved_names,
                },
            },
            auth: AuthConfigDto {
                enabled: config.auth.enabled,
                auth_type: config.auth.auth_type,
                note: config.auth.note,
                gitlab: gitlab_dto,
                passkey: PasskeyProviderDto { enabled: false },
            },
            scheduler: SchedulerConfigDto {
                cleanup_expired_urls: config.scheduler.cleanup_expired_urls.clone(),
            },
            metrics: MetricsConfigDto {
                enabled: config.metrics.enabled,
            },
            session: SessionDto {
                authenticated: false,
                user: None,
            },
            admin: None,
        }
    }
}

impl fmt::Display for AppConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AppConfigDto {{\n  short_url_ttl: {},\n  max_url_length: {},\n  base_url: {},\n  features: {},\n  auth: {},\n  scheduler: {},\n  metrics: {}\n}}",
            self.short_url_ttl,
            self.max_url_length,
            self.base_url,
            self.features,
            self.auth,
            self.scheduler,
            self.metrics
        )
    }
}

impl fmt::Display for FeaturesConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FeaturesConfigDto {{ create_url: {}, named_urls: {} }}",
            self.create_url, self.named_urls
        )
    }
}

impl fmt::Display for CreateUrlConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CreateUrlConfigDto {{ enabled: {}, auth_only: {}, max_per_user: {}, max_per_day: {} }}",
            self.enabled, self.auth_only, self.max_per_user, self.max_per_day
        )
    }
}

impl fmt::Display for NamedUrlsConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NamedUrlsConfigDto {{ enabled: {}, min_length: {}, max_length: {} }}",
            self.enabled, self.min_length, self.max_length
        )
    }
}

impl fmt::Display for AuthConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AuthConfigDto {{ enabled: {}, auth_type: {}, note: {:?}, gitlab: {:?}, passkey: {} }}",
            self.enabled, self.auth_type, self.note, self.gitlab, self.passkey.enabled
        )
    }
}

impl fmt::Display for SchedulerConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SchedulerConfigDto {{ cleanup_expired_urls: {} }}",
            self.cleanup_expired_urls
        )
    }
}

impl fmt::Display for MetricsConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MetricsConfigDto {{ enabled: {} }}", self.enabled)
    }
}
