use config::ConfigError;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub bind: String,

    #[serde(alias = "log_level")]
    pub log_level: String,
    #[serde(alias = "log_target")]
    pub log_target: String,

    #[serde(alias = "db_cnn")]
    pub db_cnn: String,

    #[serde(alias = "base_url")]
    pub base_url: String,

    #[serde(alias = "short_url")]
    pub short_url: ShortUrlConfig,

    #[serde(alias = "scheduler")]
    pub scheduler: SchedulerConfig,

    pub features: FeaturesConfig,

    pub auth: AuthConfig,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ShortUrlConfig {
    pub ttl: u32,
    pub max_length: usize,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SchedulerConfig {
    #[serde(alias = "cleanup_expired_urls")]
    pub cleanup_expired_urls: String,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct FeaturesConfig {
    #[serde(alias = "create_url")]
    pub create_url: FeatureToggle,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct FeatureToggle {
    pub enabled: bool,
    #[serde(alias = "auth_only")]
    pub auth_only: bool,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AuthConfig {
    pub enabled: bool,
    #[serde(rename = "type")]
    pub auth_type: AuthType,
    pub providers: AuthProviders,
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    Gitlab,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AuthProviders {
    pub gitlab: GitlabAuthProvider,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GitlabAuthProvider {
    #[serde(alias = "base_url")]
    pub base_url: String,
    #[serde(alias = "application_id")]
    pub application_id: String,
    pub secret: String,
}

#[derive(Debug, Error)]
pub enum LoadAppConfigError {
    #[error(transparent)]
    ConfigLoadError(#[from] ConfigError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AppConfig {{\n  bind: {},\n  log_level: {},\n  log_target: {},\n  db_cnn: <redacted>,\n  base_url: {},\n  short_url: {},\n  scheduler: {},\n  features: {},\n  auth: {}\n}}",
            self.bind,
            self.log_level,
            self.log_target,
            self.base_url,
            self.short_url,
            self.scheduler,
            self.features,
            self.auth
        )
    }
}

impl fmt::Display for ShortUrlConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ShortUrlConfig {{ ttl: {}, max_length: {} }}",
            self.ttl, self.max_length
        )
    }
}

impl fmt::Display for SchedulerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SchedulerConfig {{ cleanup_expired_urls: {} }}",
            self.cleanup_expired_urls
        )
    }
}

impl fmt::Display for FeaturesConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FeaturesConfig {{ create_url: {} }}", self.create_url)
    }
}

impl fmt::Display for FeatureToggle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FeatureToggle {{ enabled: {}, auth_only: {} }}",
            self.enabled, self.auth_only
        )
    }
}

impl fmt::Display for AuthConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AuthConfig {{ enabled: {}, type: {}, providers: {} }}",
            self.enabled, self.auth_type, self.providers
        )
    }
}

impl fmt::Display for AuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthType::Gitlab => write!(f, "gitlab"),
        }
    }
}

impl fmt::Display for AuthProviders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AuthProviders {{ gitlab: {} }}", self.gitlab)
    }
}

impl fmt::Display for GitlabAuthProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GitlabAuthProvider {{ base_url: {}, application_id: {}, secret: <redacted> }}",
            self.base_url, self.application_id
        )
    }
}
