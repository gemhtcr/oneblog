use crate::controller::category;
use actix_web::web;
use crate::utils;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;
use actix_web_flash_messages::IncomingFlashMessages;
use crate::controller;

#[derive(serde::Serialize, serde::Deserialize)]
struct MyFlashMessage {
    content: String,
    level: String,
}
// GET admin/categories
pub async fn index(
    mut per_page: Option<web::Query<usize>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
    flash_messages: IncomingFlashMessages,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let per_page = per_page.map(|inner| inner.into_inner()).unwrap_or(3);
    let categories = controller::category::posts_count(&db).await;
    let counts = categories.len();

    let pages = utils::paginate(
        counts as usize,
        per_page,
        1,
        Some("<".to_string()),
        Some(">".to_string()),
    );

    let alerts = flash_messages
        .iter()
        .map(|msg| MyFlashMessage {
            content: msg.content().to_string(),
            level: msg.level().to_string(),
        })
        .collect::<Vec<_>>();

    let html = hbs
        .render(
            "admin/categories",
            &serde_json::json!({
                "header": "admin/_header",
                "sidebar": "admin/_sidebar",
                "pages": pages,
                "categories": categories,
                "alerts": alerts,
            }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
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
