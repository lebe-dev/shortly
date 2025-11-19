use serde::Serialize;

use super::config::AppConfig;

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    /// Short URL TTL in hours
    pub short_url_ttl: u32,
}

impl From<AppConfig> for AppConfigDto {
    fn from(config: AppConfig) -> Self {
        AppConfigDto {
            short_url_ttl: config.short_url.ttl,
        }
    }
}
