use sea_orm_migration::{async_trait, prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        _manager
            .alter_table(
                Table::alter()
                .table(Users::Table)
                .add_column(
                    ColumnDef::new(Users::Password)
                        .string()
                        .not_null()
                        .default(""),
                )
                .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        _manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::Password)
                    .to_owned()
            )
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Password
}
