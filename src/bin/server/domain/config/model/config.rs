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

    pub short_url: ShortUrl,

    pub scheduler: Scheduler,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ShortUrl {
    pub ttl: u32,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Scheduler {
    pub cleanup_expired_urls: String,
}

#[derive(Debug, Error)]
pub enum LoadAppConfigError {
    #[error(transparent)]
    ConfigLoadError(#[from] ConfigError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
