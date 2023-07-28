use crate::entities::category;
use crate::entities::category::ActiveModel;
use crate::entities::category::Model;
use crate::entities::prelude::Category;
use sea_orm::*;

// Category controller

// create is to create a category
pub async fn create(db: &DatabaseConnection, name: &str) -> Result<Model, DbErr> {
    ActiveModel {
        name: ActiveValue::Set(name.to_string()),
        updated: ActiveValue::Set(chrono::offset::Utc::now()),
        created: ActiveValue::Set(chrono::offset::Utc::now()),
        ..Default::default()
    }
    .insert(db)
    .await
}

// find is to find by id
pub async fn find(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, DbErr> {
    Category::find_by_id(id).one(db).await
}

// all is to find all category and order by updated timestamp
pub async fn all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Category::find()
        .order_by_desc(category::Column::Updated)
        .all(db)
        .await
}

// offset_and_limit is to get paginated data based on offset and limit
pub async fn offset_and_limit(
    db: &DatabaseConnection,
    limit: u64,
    offset: u64,
) -> Result<Vec<Model>, DbErr> {
    Category::find()
        .order_by_desc(category::Column::Updated)
        .offset(offset)
        .limit(limit)
        .all(db)
        .await
}

// cursor is to get paginated data from cursor
pub async fn cursor(db: &DatabaseConnection, cursor: u64, limit: u64) -> Result<Vec<Model>, DbErr> {
    Category::find()
        .order_by_desc(category::Column::Updated)
        .cursor_by(category::Column::Id)
        .after(cursor)
        .first(limit)
        .all(db)
        .await
}

pub async fn update(db: &DatabaseConnection, id: u64, name: &str) -> Result<Model, DbErr> {
    ActiveModel {
        name: ActiveValue::Set(name.to_string()),
        updated: ActiveValue::Set(chrono::offset::Utc::now()),
        ..Default::default()
    }
    .update(db)
    .await
}
// delete is
pub async fn destroy(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
    ActiveModel {
        id: ActiveValue::Set(id), // The primary key must be set
        ..Default::default()
    }
    .delete(db)
    .await
}
