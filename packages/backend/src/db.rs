use anyhow::Result;
use entity::history;
use sea_orm::{sea_query::OnConflict, DatabaseConnection, EntityTrait, QueryOrder};

pub async fn upsert_history(db: &DatabaseConnection, item: history::ActiveModel) -> Result<()> {
    history::Entity::insert(item)
        .on_conflict(
            OnConflict::column(history::Column::ComicId)
                .update_columns([
                    history::Column::ChapterId,
                    history::Column::ChapterName,
                    history::Column::ComicName,
                    history::Column::Page,
                    history::Column::Visible,
                    history::Column::UpdatedAt,
                ])
                .to_owned(),
        )
        .exec(db)
        .await?;

    Ok(())
}

pub async fn get_history_list(db: &DatabaseConnection) -> Result<Vec<history::Model>> {
    let list = history::Entity::find()
        .order_by_desc(history::Column::UpdatedAt)
        .all(db)
        .await?;

    Ok(list)
}
