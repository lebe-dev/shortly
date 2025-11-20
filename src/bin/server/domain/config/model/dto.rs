use serde::Serialize;

use super::config::AppConfig;

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    /// Short URL TTL in hours
    pub short_url_ttl: u32,
    /// Maximum URL length in characters
    pub max_url_length: usize,
}

impl From<AppConfig> for AppConfigDto {
    fn from(config: AppConfig) -> Self {
        AppConfigDto {
            short_url_ttl: config.short_url.ttl,
            max_url_length: config.short_url.max_length,
        }
    }
}
