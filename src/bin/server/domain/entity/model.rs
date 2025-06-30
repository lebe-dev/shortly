use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub path: String,
    pub file_name: String,
    pub size: u64,
    pub optimized: bool,
    pub optimization_date: i64,
}

#[derive(Debug, Error)]
pub enum EntityError {
    #[error("file not found: '{file_path}'")]
    FileNotFound { file_path: String },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
