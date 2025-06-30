use crate::domain::entity::{model::Entity, ports::EntityRepository};

use super::init::Sqlite;

impl EntityRepository for Sqlite {
    async fn find_all(&self) -> Result<Vec<Entity>, sqlx::Error> {
        todo!()
    }
}
