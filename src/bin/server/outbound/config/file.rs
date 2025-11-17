use config::Config;
use std::path::Path;

use crate::domain::config::model::{
    config::{AppConfig, LoadAppConfigError},
    ports::AppConfigService,
};

#[derive(Debug, Clone)]
pub struct AppConfigServiceImpl;

impl AppConfigService for AppConfigServiceImpl {
    fn load_from_file(&self, config_path: &Path) -> Result<AppConfig, LoadAppConfigError> {
        let config = Config::builder()
            .add_source(config::File::from(config_path))
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;
        Ok(app_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_config_success() {
        let service = AppConfigServiceImpl;
        let config_path = PathBuf::from("test-data/config.yml");

        let result = service.load_from_file(&config_path);

        assert!(result.is_ok(), "Config loading should succeed");

        let config = result.unwrap();
        assert_eq!(config.bind, "0.0.0.0:8080");
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.log_target, "console");
        assert_eq!(config.base_url, "http://localhost:8080");
        assert_eq!(config.db_cnn, "sqlite://./data/app.db?mode=rwc");
        assert_eq!(config.short_url.ttl, 168);
        assert_eq!(config.scheduler.cleanup_expired_urls, "0 0 * * *");
    }
}
