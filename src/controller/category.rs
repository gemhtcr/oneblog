use crate::entities::category;
use crate::entities::category::ActiveModel;
pub use crate::entities::category::Model;
use crate::entities::post;
pub use crate::entities::prelude::Category;
use crate::entities::prelude::Post;
use futures::stream::StreamExt;
use futures::FutureExt;
use sea_orm::*;
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
    Category::find().count(db).await
}

pub async fn posts_count(db: &DatabaseConnection) -> Result<Vec<(Model, u64)>, DbErr> {
    let categories = all(db).await?;
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

    Ok(ret)
}

pub async fn find_posts(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<(Model, Vec<post::Model>)>, DbErr> {
    // One category can have many posts (one-to-many)
    Category::find_by_id(id)
        .find_with_related(Post)
        .all(db)
        .await
        .map(IntoIterator::into_iter)
        .map(|mut inner| inner.next())
}

// The `page` is 1-dinex
pub async fn find_posts_with(
    db: &DatabaseConnection,
    id: i32,
    page: u64,
    per_page: u64,
) -> Result<Option<(Model, Vec<post::Model>)>, DbErr> {
    let Some(cate) = find(db, id).await? else {
        return Ok(None);
    };

    cate.find_related(Post)
        .order_by_desc(post::Column::Updated)
        .paginate(db, per_page)
        .fetch_page(page - 1)
        .await
        .map(|inner| Some((cate, inner)))
}

pub async fn find_posts_count(db: &DatabaseConnection, id: i32) -> Result<u64, DbErr> {
    let Some(cat) = find(db, id).await?
    else { // Note that we return 0 to simplify edge cases if model doesn't exist
        return Ok(0);
    };

    // A category can have many posts
    cat.find_related(Post).count(db).await
}

pub fn paginator(
    db: &DatabaseConnection,
    per_page: u64,
) -> sea_orm::Paginator<DatabaseConnection, SelectModel<Model>> {
    Category::find()
        .order_by_desc(category::Column::Updated)
        .paginate(db, per_page)
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
