use std::path::Path;

use super::config::{AppConfig, LoadAppConfigError};

pub trait AppConfigService: Clone + Send + Sync + 'static {
    fn load_from_file(&self, config_path: &Path) -> Result<AppConfig, LoadAppConfigError>;
}
