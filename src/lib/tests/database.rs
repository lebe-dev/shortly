use crate::outbound::sqlite::init::Sqlite;

pub async fn get_in_memory_db() -> Sqlite {
    Sqlite::new("sqlite::memory:").await.unwrap()
}
