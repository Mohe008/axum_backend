use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261019_123703_add_gin_index_on_users_metadata"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        let db = _manager.get_connection();
        db.execute_unprepared(
            "CREATE INDEX idx_users_metadata ON users USING GIN (metadata);"
        )
        .await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        let db = _manager.get_connection();
        db.execute_unprepared("DROP INDEX idx_users_metadata;").await?;
        Ok(())
    }
}
