use std::{collections::BTreeMap, path::Path};

use server_lib::{
    domain::{
        auth::{model::GitlabUserInfo, model::User, ports::UserRepository},
        url::model::{Url, UrlAccess},
    },
    outbound::postgres::{init::Postgres, migration::PostgresMigrationManager},
};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor, wait::LogWaitStrategy},
    runners::AsyncRunner,
};

pub fn migrations() -> BTreeMap<String, String> {
    std::fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations/postgres"))
        .expect("read PostgreSQL migrations directory")
        .map(|entry| entry.expect("read migration entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "sql"))
        .map(|path| {
            (
                path.file_name().unwrap().to_str().unwrap().to_owned(),
                std::fs::read_to_string(&path).expect("read migration SQL"),
            )
        })
        .collect()
}

pub struct TestPostgres {
    pub db: Postgres,
    // Keep the container alive until the test and its database operations finish.
    _container: ContainerAsync<GenericImage>,
}

impl TestPostgres {
    pub async fn empty() -> Self {
        let container = GenericImage::new("postgres", "17.6-alpine")
            .with_exposed_port(5432.tcp())
            // PostgreSQL first starts a temporary server while initializing the database.
            .with_wait_for(WaitFor::log(
                LogWaitStrategy::stdout_or_stderr("database system is ready to accept connections")
                    .with_times(2),
            ))
            .with_env_var("POSTGRES_USER", "shortly")
            .with_env_var("POSTGRES_PASSWORD", "shortly")
            .with_env_var("POSTGRES_DB", "shortly_test")
            .start()
            .await
            .expect("start PostgreSQL testcontainer; a running Docker daemon is required");
        let host = container.get_host().await.expect("get container host");
        let port = container
            .get_host_port_ipv4(5432.tcp())
            .await
            .expect("get PostgreSQL port");
        let db = Postgres::new(&format!(
            "postgres://shortly:shortly@{host}:{port}/shortly_test"
        ))
        .await
        .expect("connect to test PostgreSQL");
        Self {
            db,
            _container: container,
        }
    }

    pub async fn new() -> Self {
        let test = Self::empty().await;
        let manager = PostgresMigrationManager::new(test.db.get_pool().clone(), migrations());
        manager.initialize().await.unwrap();
        manager.run_migrations().await.unwrap();
        test
    }
}

pub async fn user(db: &Postgres, gitlab_id: i64, username: &str) -> User {
    db.upsert(&GitlabUserInfo {
        id: gitlab_id,
        username: username.into(),
        email: Some(format!("{username}@example.com")),
        avatar_url: None,
    })
    .await
    .unwrap()
}

pub fn url(id: &str, user_id: Option<i64>, created: i64) -> Url {
    Url {
        id: id.into(),
        original_url: format!("https://example.com/{id}"),
        ttl: 3600,
        created,
        user_id,
        custom_name: None,
        last_accessed: None,
        access: UrlAccess::Public,
    }
}
