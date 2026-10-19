pub use sea_orm_migration::prelude::*;

mod m20261015_061558_add_password_to_users;
mod m20261019_123703_add_gin_index_on_users_metadata;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261015_061558_add_password_to_users::Migration),
            Box::new(m20261019_123703_add_gin_index_on_users_metadata::Migration),
        ]
    }
}

