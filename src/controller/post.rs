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
/*
    use sea_orm::query::QueryTrait;
    use sea_orm::EntityTrait;
    use sea_orm::ActiveValue;
    use sea_orm::sea_query::QueryStatementBuilder;

    let stmt = crate::post::ActiveModel {
        id: ActiveValue::Unchanged(1),
        title: ActiveValue::Set("abc".to_string()),
        description: ActiveValue::Set("des".to_string()),
        catgegory_name: ActiveValue::Set(None),
        updated: ActiveValue::Set(chrono::offset::Utc::now()),
        ..Default::default()
    };

    //let r: <Post as sea_orm::entity::EntityTrait>::Model = Post::update(stmt);
    let mut r= Post::update(stmt);
    let stmt = r.query();
    let stmt = stmt.to_string(sea_orm::sea_query::MysqlQueryBuilder);
    info!(?stmt);



*/
