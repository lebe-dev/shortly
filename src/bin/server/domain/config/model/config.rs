use config::ConfigError;
use serde::Deserialize;
use thiserror::Error;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub bind: String,

    pub log_level: String,
    pub log_target: String,

    pub data_path: String,

    pub db_cnn: String,
}

#[derive(Debug, Error)]
pub enum LoadAppConfigError {
    #[error(transparent)]
    ConfigLoadError(#[from] ConfigError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
