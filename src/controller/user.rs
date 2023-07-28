pub use crate::entities::users::ActiveModel;
use crate::authentication;
use crate::entities::prelude::Users;
use crate::entities::users;
use crate::entities::users::Model;
use sea_orm::*;
use secrecy::ExposeSecret;
use secrecy::Secret;

// insert is to insert a users
pub async fn create(
    db: &DatabaseConnection,
    username: &str,
    raw_password: Secret<String>,
    category_id: Option<i32>,
) -> Result<Model, DbErr> {
    let password_hash = authentication::password::compute_password_hash(raw_password).unwrap();
    let uuid = uuid::Uuid::new_v4();
    ActiveModel {
        user_id: ActiveValue::Set(uuid.to_string()),
        username: ActiveValue::Set(username.to_string()),
        password_hash: ActiveValue::Set(password_hash.expose_secret().clone()),
        ..Default::default()
    }
    .insert(db)
    .await
}

// find is to find by id
pub async fn find(db: &DatabaseConnection, user_id: uuid::Uuid) -> Result<Option<Model>, DbErr> {
    Users::find_by_id(user_id.to_string()).one(db).await
}

// all is to find all users and order by updated timestamp
pub async fn all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Users::find().all(db).await
}

pub async fn update(db: &DatabaseConnection, active: ActiveModel) -> Result<Model, DbErr> {
    active.update(db).await
}
// delete is
pub async fn destroy(db: &DatabaseConnection, uuid: uuid::Uuid) -> Result<DeleteResult, DbErr> {
    ActiveModel {
        user_id: ActiveValue::Set(uuid.to_string()), // The primary key must be set
        ..Default::default()
    }
    .delete(db)
    .await
}

