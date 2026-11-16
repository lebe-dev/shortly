use std::collections::BTreeMap;

use server_lib::outbound::postgres::migration::PostgresMigrationManager;

use crate::support::{TestPostgres, migrations};

#[tokio::test]
async fn migrations_apply_once_and_preserve_data() {
    let test = TestPostgres::new().await;
    let user = crate::support::user(&test.db, 42, "alice").await;
    let manager = PostgresMigrationManager::new(test.db.get_pool().clone(), migrations());
    manager.initialize().await.unwrap();
    manager.run_migrations().await.unwrap();

    let applied: Vec<String> = sqlx::query_scalar("SELECT name FROM _migrations ORDER BY name")
        .fetch_all(test.db.get_pool())
        .await
        .unwrap();
    assert!(!applied.is_empty());
    assert_eq!(applied, migrations().into_keys().collect::<Vec<_>>());
    use server_lib::domain::auth::ports::UserRepository;
    assert_eq!(test.db.find_by_gitlab_id(42).await.unwrap(), Some(user));
}

#[tokio::test]
async fn failed_migration_rolls_back_schema_data_and_tracking() {
    let test = TestPostgres::empty().await;
    let manager = PostgresMigrationManager::new(
        test.db.get_pool().clone(),
        BTreeMap::from([
            (
                "001_valid.sql".into(),
                "CREATE TABLE kept (id INTEGER);".into(),
            ),
            (
                "002_invalid.sql".into(),
                "INSERT INTO kept VALUES (1); CREATE TABLE rolled_back (id INTEGER); \
                 INSERT INTO missing_table VALUES (1);"
                    .into(),
            ),
        ]),
    );
    manager.initialize().await.unwrap();
    assert!(manager.run_migrations().await.is_err());

    let applied: Vec<String> = sqlx::query_scalar("SELECT name FROM _migrations ORDER BY name")
        .fetch_all(test.db.get_pool())
        .await
        .unwrap();
    assert_eq!(applied, ["001_valid.sql"]);
    let rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM kept")
        .fetch_one(test.db.get_pool())
        .await
        .unwrap();
    assert_eq!(rows, 0);
    let table: Option<String> = sqlx::query_scalar("SELECT to_regclass('rolled_back')::text")
        .fetch_one(test.db.get_pool())
        .await
        .unwrap();
    assert_eq!(table, None);
}
