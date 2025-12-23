use anyhow::{Result, anyhow};
use rust_embed::Embed;
use std::collections::BTreeMap;

#[derive(Embed)]
#[folder = "migrations/"]
pub struct MigrationAssets;

/// Load migrations from embedded assets (for production use)
pub fn load_migrations_from_assets() -> Result<BTreeMap<String, String>> {
    let mut migration_files: BTreeMap<String, String> = BTreeMap::new();

    for file in MigrationAssets::iter() {
        let file_name = file.as_ref();
        if file_name.ends_with(".sql") {
            if let Some(content) = MigrationAssets::get(file_name) {
                let content_str = std::str::from_utf8(&content.data)
                    .map_err(|e| anyhow!("Failed to read migration file {}: {}", file_name, e))?;
                migration_files.insert(file_name.to_string(), content_str.to_string());
            }
        }
    }

    Ok(migration_files)
}
