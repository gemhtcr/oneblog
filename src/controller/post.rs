use crate::entities::post;
pub use crate::entities::post::ActiveModel;
pub use crate::entities::post::Model;
use crate::entities::prelude::Post;
use sea_orm::*;

// insert is to insert a post
pub async fn create(
    db: &DatabaseConnection,
    title: &str,
    description: &str,
    category_name: Option<String>,
) -> Result<Model, DbErr> {
    ActiveModel {
        title: ActiveValue::Set(title.to_string()),
        description: ActiveValue::Set(description.to_string()),
        category_name: ActiveValue::Set(category_name),
        updated: ActiveValue::Set(chrono::offset::Utc::now()),
        created: ActiveValue::Set(chrono::offset::Utc::now()),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn count(db: &DatabaseConnection) -> Result<u64, DbErr> {
    Post::find().count(db).await
}

pub async fn search(
    db: &DatabaseConnection,
    pattern: String,
    page: u64,
    per_page: u64,
) -> Result<(Vec<post::Model>, sea_orm::ItemsAndPagesNumber), DbErr> {
    let pattern = format!("%{}%", pattern);
    let paginator = Post::find()
        .filter(
            sea_orm::Condition::any()
                .add(sea_orm::Condition::all().add(post::Column::Title.like(&pattern)))
                .add(sea_orm::Condition::all().add(post::Column::Description.like(&pattern))),
        )
        .order_by_desc(post::Column::Updated)
        .paginate(db, per_page);
    let info = paginator.num_items_and_pages().await?;

    paginator.fetch_page(page - 1).await.map(|p| (p, info))
}

pub fn paginator(
    db: &DatabaseConnection,
    per_page: u64,
) -> sea_orm::Paginator<'_, DatabaseConnection, SelectModel<Model>> {
    Post::find()
        .order_by_desc(post::Column::Updated)
        .paginate(db, per_page)
}

// find is to find by id
pub async fn find(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, DbErr> {
    Post::find_by_id(id).one(db).await
}

// all is to find all posts and order by updated timestamp
pub async fn all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Post::find()
        .order_by_desc(post::Column::Updated)
        .all(db)
        .await
}

pub async fn offset_and_limit(
    db: &DatabaseConnection,
    offset: u64,
    limit: u64,
) -> Result<Vec<Model>, DbErr> {
    Post::find()
        .order_by_desc(post::Column::Updated)
        .offset(offset)
        .limit(limit)
        .all(db)
        .await
}

pub async fn cursor(db: &DatabaseConnection, cursor: u64, limit: u64) -> Result<Vec<Model>, DbErr> {
    Post::find()
        .order_by_desc(post::Column::Updated)
        .cursor_by(post::Column::Id)
        .after(cursor)
        .first(limit)
        .all(db)
        .await
}

pub async fn update(
    db: &DatabaseConnection,
    post_id: i32,
    title: &str,
    description: &str,
    category_name: Option<String>,
) -> Result<Model, DbErr> {
    ActiveModel {
        id: ActiveValue::Unchanged(post_id),
        title: ActiveValue::Set(title.to_string()),
        description: ActiveValue::Set(description.to_string()),
        category_name: ActiveValue::Set(category_name),
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
