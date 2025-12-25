use serde::Serialize;
use std::fmt;

use super::config::{AppConfig, AuthType};
use server_lib::domain::url::service::BASE_RESERVED_NAMES;

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
}

impl From<AppConfig> for AppConfigDto {
    fn from(config: AppConfig) -> Self {
        // Merge base reserved names with config reserved names
        let mut merged_reserved_names: Vec<String> =
            BASE_RESERVED_NAMES.iter().map(|s| s.to_string()).collect();

        // Add config reserved names, avoiding duplicates
        for name in &config.features.named_urls.reserved_names {
            let name_lower = name.to_lowercase();
            if !merged_reserved_names
                .iter()
                .any(|r| r.to_lowercase() == name_lower)
            {
                merged_reserved_names.push(name.clone());
            }
        }

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
            },
        }
    }
}

impl fmt::Display for AppConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AppConfigDto {{\n  short_url_ttl: {},\n  max_url_length: {},\n  base_url: {},\n  features: {},\n  auth: {}\n}}",
            self.short_url_ttl, self.max_url_length, self.base_url, self.features, self.auth
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
            "AuthConfigDto {{ enabled: {}, auth_type: {}, note: {:?} }}",
            self.enabled, self.auth_type, self.note
        )
    }
}
