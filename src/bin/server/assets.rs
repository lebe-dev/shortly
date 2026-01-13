use anyhow::{Result, anyhow};
use rust_embed::Embed;
use std::collections::BTreeMap;

#[derive(Embed)]
#[folder = "migrations/sqlite/"]
pub struct SqliteMigrationAssets;

#[derive(Embed)]
#[folder = "migrations/postgres/"]
pub struct PostgresMigrationAssets;

/// Load migrations for a specific database type from embedded assets
pub fn load_migrations_for_database(db_type: &str) -> Result<BTreeMap<String, String>> {
    match db_type {
        "sqlite" => load_from_embed::<SqliteMigrationAssets>(),
        "postgres" => load_from_embed::<PostgresMigrationAssets>(),
        _ => Err(anyhow!("Unknown database type: {}", db_type)),
    }
}

fn load_from_embed<T: Embed>() -> Result<BTreeMap<String, String>> {
    let mut migrations = BTreeMap::new();

    for file_name in T::iter() {
        if file_name.ends_with(".sql")
            && let Some(content) = T::get(&file_name)
        {
            let sql = std::str::from_utf8(content.data.as_ref())
                .map_err(|e| anyhow!("Invalid UTF-8 in migration {}: {}", file_name, e))?;
            migrations.insert(file_name.to_string(), sql.to_string());
        }
    }

    Ok(migrations)
}

/// Load migrations from filesystem (for tests)
pub fn load_migrations_from_fs(db_type: &str) -> Result<BTreeMap<String, String>> {
    use std::fs;
    use std::path::Path;

    let migrations_dir = Path::new("migrations").join(db_type);
    if !migrations_dir.exists() {
        return Err(anyhow!(
            "Migrations directory does not exist: {}",
            migrations_dir.display()
        ));
    }

    let mut migrations = BTreeMap::new();

    for entry in fs::read_dir(&migrations_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            let file_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow!("Invalid file name"))?;

            let content = fs::read_to_string(&path)?;
            migrations.insert(file_name.to_string(), content);
        }
    }

    Ok(migrations)
}
