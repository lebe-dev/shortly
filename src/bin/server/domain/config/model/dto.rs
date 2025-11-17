use serde::Serialize;

use super::config::AppConfig;

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    pub bind: String,

    pub log_level: String,
    pub log_target: String,
}

impl From<AppConfig> for AppConfigDto {
    fn from(config: AppConfig) -> Self {
        AppConfigDto {
            bind: config.bind,
            log_level: config.log_level,
            log_target: config.log_target,
        }
    }
}
