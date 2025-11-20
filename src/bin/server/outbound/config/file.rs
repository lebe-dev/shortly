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
        let mut builder = Config::builder().add_source(config::File::from(config_path));

        if let Ok(val) = std::env::var("BIND") {
            builder = builder.set_override("bind", val)?;
        }
        if let Ok(val) = std::env::var("LOG_LEVEL") {
            builder = builder.set_override("log-level", val)?;
        }
        if let Ok(val) = std::env::var("LOG_TARGET") {
            builder = builder.set_override("log-target", val)?;
        }
        if let Ok(val) = std::env::var("DB_CNN") {
            builder = builder.set_override("db-cnn", val)?;
        }
        if let Ok(val) = std::env::var("BASE_URL") {
            builder = builder.set_override("base-url", val)?;
        }
        if let Ok(val) = std::env::var("SHORT_URL_TTL") {
            builder = builder.set_override("short-url.ttl", val)?;
        }
        if let Ok(val) = std::env::var("SCHEDULER_CLEANUP_EXPIRED_URLS") {
            builder = builder.set_override("scheduler.cleanup-expired-urls", val)?;
        }

        let config = builder.build()?;
        let app_config: AppConfig = config.try_deserialize()?;
        Ok(app_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::path::PathBuf;

    #[test]
    #[serial]
    fn test_load_config_success() {
        unsafe {
            std::env::remove_var("BIND");
            std::env::remove_var("LOG_LEVEL");
            std::env::remove_var("BASE_URL");
            std::env::remove_var("SHORT_URL_TTL");
            std::env::remove_var("SCHEDULER_CLEANUP_EXPIRED_URLS");
        }

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

    #[test]
    #[serial]
    fn test_env_override_simple_fields() {
        unsafe {
            std::env::set_var("BIND", "127.0.0.1:9090");
            std::env::set_var("LOG_LEVEL", "info");
            std::env::set_var("BASE_URL", "https://prod.example.com");
        }

        let service = AppConfigServiceImpl;
        let config_path = PathBuf::from("test-data/config.yml");

        let result = service.load_from_file(&config_path);
        assert!(result.is_ok(), "Config loading should succeed");

        let config = result.unwrap();

        assert_eq!(config.bind, "127.0.0.1:9090");
        assert_eq!(config.log_level, "info");
        assert_eq!(config.base_url, "https://prod.example.com");

        assert_eq!(config.log_target, "console");
        assert_eq!(config.db_cnn, "sqlite://./data/app.db?mode=rwc");

        unsafe {
            std::env::remove_var("BIND");
            std::env::remove_var("LOG_LEVEL");
            std::env::remove_var("BASE_URL");
        }
    }

    #[test]
    #[serial]
    fn test_env_override_nested_fields() {
        unsafe {
            std::env::set_var("SHORT_URL_TTL", "720");
            std::env::set_var("SCHEDULER_CLEANUP_EXPIRED_URLS", "0 0 12 * * *");
        }

        let service = AppConfigServiceImpl;
        let config_path = PathBuf::from("test-data/config.yml");

        let result = service.load_from_file(&config_path);
        assert!(result.is_ok(), "Config loading should succeed");

        let config = result.unwrap();

        assert_eq!(config.short_url.ttl, 720);
        assert_eq!(config.scheduler.cleanup_expired_urls, "0 0 12 * * *");

        unsafe {
            std::env::remove_var("SHORT_URL_TTL");
            std::env::remove_var("SCHEDULER_CLEANUP_EXPIRED_URLS");
        }
    }
}
