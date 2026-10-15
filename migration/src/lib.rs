pub use sea_orm_migration::prelude::*;

mod m20261015_061558_add_password_to_users;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20261015_061558_add_password_to_users::Migration)]
    }
}

