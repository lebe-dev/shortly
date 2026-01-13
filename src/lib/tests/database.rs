use crate::outbound::sqlite::{init::Sqlite, migration::MigrationManager};

pub async fn get_in_memory_db() -> Sqlite {
    let db = Sqlite::new("sqlite::memory:").await.unwrap();

    let migrations_dir = std::path::Path::new("migrations/sqlite");
    let mut migrations = std::collections::BTreeMap::new();

    for entry in std::fs::read_dir(migrations_dir).expect("Failed to read migrations directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            let file_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .expect("Invalid file name");
            let content = std::fs::read_to_string(&path).expect("Failed to read migration file");
            migrations.insert(file_name.to_string(), content);
        }
    }

    let migration_manager = MigrationManager::new(db.get_pool().clone(), migrations);
    migration_manager.initialize().await.unwrap();
    migration_manager.run_migrations().await.unwrap();

    db
}
