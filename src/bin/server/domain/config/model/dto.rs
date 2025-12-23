use serde::Serialize;
use std::fmt;

use super::config::{AppConfig, AuthType};

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    /// Short URL TTL in hours
    pub short_url_ttl: u32,
    /// Maximum URL length in characters
    pub max_url_length: usize,
    /// Feature flags configuration
    pub features: FeaturesConfigDto,
    /// Authentication configuration
    pub auth: AuthConfigDto,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeaturesConfigDto {
    /// Whether URL creation is enabled
    pub create_url_enabled: bool,
    /// Whether URL creation requires authentication
    pub create_url_auth_only: bool,
}

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthConfigDto {
    /// Whether authentication is enabled
    pub enabled: bool,
    /// Authentication provider type (e.g., "gitlab")
    pub auth_type: AuthType,
}

impl From<AppConfig> for AppConfigDto {
    fn from(config: AppConfig) -> Self {
        AppConfigDto {
            short_url_ttl: config.short_url.ttl,
            max_url_length: config.short_url.max_length,
            features: FeaturesConfigDto {
                create_url_enabled: config.features.create_url.enabled,
                create_url_auth_only: config.features.create_url.auth_only,
            },
            auth: AuthConfigDto {
                enabled: config.auth.enabled,
                auth_type: config.auth.auth_type,
            },
        }
    }
}

impl fmt::Display for AppConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AppConfigDto {{\n  short_url_ttl: {},\n  max_url_length: {},\n  features: {},\n  auth: {}\n}}",
            self.short_url_ttl, self.max_url_length, self.features, self.auth
        )
    }
}

impl fmt::Display for FeaturesConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FeaturesConfigDto {{ create_url_enabled: {}, create_url_auth_only: {} }}",
            self.create_url_enabled, self.create_url_auth_only
        )
    }
}

impl fmt::Display for AuthConfigDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AuthConfigDto {{ enabled: {}, auth_type: {} }}",
            self.enabled, self.auth_type
        )
    }
}
