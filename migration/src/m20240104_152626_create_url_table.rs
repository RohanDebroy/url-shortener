use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Url::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Url::Slug).string().not_null().primary_key())
                    .col(ColumnDef::new(Url::Url).string().not_null())
                    .col(
                        ColumnDef::new(Url::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Url::Views).big_unsigned().default(0))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Url::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Url {
    Table,
    Slug,
    Url,
    CreatedAt,
    Views,
}
