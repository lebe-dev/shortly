use config::ConfigError;
use serde::Deserialize;
use thiserror::Error;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub bind: String,

    pub log_level: String,
    pub log_target: String,

    pub db_cnn: String,

    pub base_url: String,

    pub short_url: ShortUrlConfig,

    pub scheduler: SchedulerConfig,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ShortUrlConfig {
    pub ttl: u32,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SchedulerConfig {
    pub cleanup_expired_urls: String,
}

#[derive(Debug, Error)]
pub enum LoadAppConfigError {
    #[error(transparent)]
    ConfigLoadError(#[from] ConfigError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
