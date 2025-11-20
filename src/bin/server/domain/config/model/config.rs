use config::ConfigError;
use serde::Deserialize;
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

#[derive(Debug, Error)]
pub enum LoadAppConfigError {
    #[error(transparent)]
    ConfigLoadError(#[from] ConfigError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
