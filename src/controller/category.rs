use crate::Category;
use crate::entities::category;
use crate::entities::category::Model;
pub use crate::entities::category::ActiveModel;
use sea_orm::*;

// insert is to insert a category
pub async fn create(db: &DatabaseConnection, model: ActiveModel) -> Result<Model, DbErr> {
   model.insert(db).await
}

// find is to find by id
pub async fn find(db: &DatabaseConnection, id: i32) -> Result<Option<Model>, DbErr> {
    Category::find_by_id(id).one(db).await
}

// all is to find all categories
pub async fn all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Category::find().all(db).await
}

pub async fn update(db: &DatabaseConnection, active: ActiveModel) -> Result<Model, DbErr> {
    active.update(db).await
}
// delete is 
pub async fn destroy(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
    category::ActiveModel {
        id: ActiveValue::Set(id), // The primary key must be set
        ..Default::default()
    }
    .delete(db)
    .await
}
