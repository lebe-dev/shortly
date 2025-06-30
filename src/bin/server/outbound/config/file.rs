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
