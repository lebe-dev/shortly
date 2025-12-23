use crate::outbound::sqlite::{init::Sqlite, migration::MigrationManager};

pub async fn get_in_memory_db() -> Sqlite {
    let db = Sqlite::new("sqlite::memory:").await.unwrap();

    // Load migrations from filesystem for tests
    let migrations = crate::outbound::sqlite::migration::load_migrations_from_fs()
        .expect("Failed to load migrations from filesystem");

    // Run migrations to setup tables
    let migration_manager = MigrationManager::new(db.get_pool().clone(), migrations);
    migration_manager.initialize().await.unwrap();
    migration_manager.run_migrations().await.unwrap();

    db
}
