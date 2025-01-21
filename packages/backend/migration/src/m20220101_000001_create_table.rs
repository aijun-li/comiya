use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(History::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(History::ComicId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string(History::ChapterId))
                    .col(string(History::ComicName))
                    .col(string(History::ChapterName))
                    .col(unsigned(History::Page))
                    .col(boolean(History::Visible))
                    .col(timestamp(History::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(History::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(History::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum History {
    Table,
    ComicId,
    ChapterId,
    ComicName,
    ChapterName,
    Page,
    Visible,
    CreatedAt,
    UpdatedAt,
}
