use crate::controller::category;
use actix_web::web;

// GET admin/categories
pub async fn index(
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    todo!()
}

pub struct Pagination {
    page_number: i32,
    per_page: i32,
}

// GET admin/categories/page/{page_number}
pub async fn page(
    pagination: web::Path<Pagination>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    todo!()
}

// POST admin/categories
pub async fn new(
    name: web::Form<String>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    todo!()
}

// GET admin/categories/{category_id}/edit
pub async fn edit(
    category_id: web::Form<String>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    todo!()
}

// DELETE admin/categories/{category_id}
pub async fn delete(
    category_id: web::Form<String>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    todo!()
}
