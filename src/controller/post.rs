use crate::Post;
use crate::entities::post;
use crate::entities::post::Model;
pub use crate::entities::post::ActiveModel;
use sea_orm::*;

// insert is to insert a post
pub async fn create(db: &DatabaseConnection, model: ActiveModel) -> Result<Model, DbErr> {
    model.insert(db).await
}

// find is to find by id
pub async fn find(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, DbErr> {
    Post::find_by_id(id).one(db).await
}

// all is to find all posts
pub async fn all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Post::find().all(db).await
}

pub async fn update(db: &DatabaseConnection, active: ActiveModel) -> Result<Model, DbErr> {
    active.update(db).await
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
