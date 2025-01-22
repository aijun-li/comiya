use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LibComic::Table)
                    .if_not_exists()
                    .col(string(LibComic::Id).primary_key())
                    .col(string(LibComic::Name))
                    .col(string(LibComic::Cover))
                    .col(timestamp(LibComic::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(LibComic::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LibComic::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LibComic {
    Table,
    Id,
    Name,
    Cover,
    CreatedAt,
    UpdatedAt,
}
