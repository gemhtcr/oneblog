use crate::entities::category;
use crate::entities::category::ActiveModel;
pub use crate::entities::category::Model;
use crate::entities::post;
pub use crate::entities::prelude::Category;
use crate::entities::prelude::Post;
use futures::stream::StreamExt;
use futures::FutureExt;
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

// count is to return size of categories
pub async fn count(db: &DatabaseConnection) -> Result<u64, DbErr> {
    Category::find()
        .count(db)
        .await
}

pub async fn posts_count(db: &DatabaseConnection) -> Vec<(Model, u64)> {
    let categories = all(db).await.unwrap();
    let ret = categories
        .into_iter()
        .map(|cat| {
            async move {
                let post_count = cat.find_related(Post).count(db).await.unwrap_or(0);
                (cat, post_count)
            }
            .boxed()
        })
        .collect::<futures::stream::FuturesOrdered<_>>()
        .collect::<Vec<_>>()
        .await;

    ret
}

pub async fn find_posts(
    db: &DatabaseConnection,
    id: i32,
) -> Result<(Model, Vec<post::Model>), DbErr> {
    find_posts_with(db, id, None, None).await
}

pub async fn find_posts_with(
    db: &DatabaseConnection,
    id: i32,
    offset: Option<u64>,
    limit: Option<u64>,
) -> Result<(Model, Vec<post::Model>), DbErr> {
    let cat = find(db, id).await?.unwrap();
    let ret = cat
        .find_related(Post)
        .order_by_desc(post::Column::Updated)
        .offset(offset)
        .limit(limit)
        .all(db)
        .await?;
    Ok((cat, ret))
}

pub async fn find_posts_count(db: &DatabaseConnection, id: i32) -> Result<u64, DbErr> {
    let cat = find(db, id).await?.unwrap();
    let ret = cat.find_related(Post).count(db).await?;
    Ok(ret)
}

// offset_and_limit is to get paginated data based on offset and limit
pub async fn offset_and_limit(
    db: &DatabaseConnection,
    offset: Option<u64>,
    limit: Option<u64>,
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

pub async fn update(db: &DatabaseConnection, id: i32, name: &str) -> Result<Model, DbErr> {
    ActiveModel {
        id: ActiveValue::Unchanged(id),
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
