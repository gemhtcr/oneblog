use crate::controller;
use crate::error::OneBlogError;
use crate::utils;
use actix_web::web;
use actix_web_flash_messages::IncomingFlashMessages;

#[derive(serde::Serialize, serde::Deserialize)]
struct MyFlashMessage {
    content: String,
    level: String,
}

// GET /admin
pub async fn index(
    per_page: Option<web::Query<usize>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
    flash_messages: IncomingFlashMessages,
) -> impl actix_web::Responder {
    let per_page = per_page.map(|inner| inner.into_inner()).unwrap_or(3);
    let paginator = controller::post::paginator(&db, per_page as u64);
    let posts = paginator.fetch_page(0).await?;
    let sea_orm::ItemsAndPagesNumber {
        number_of_items,
        number_of_pages: _,
    } = paginator.num_items_and_pages().await?;
    let pages = utils::paginate(
        number_of_items as usize,
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
    let categories = controller::category::posts_count(&db).await?;
    let html = hbs.render(
        "admin/index",
        &serde_json::json!({
            "posts": posts,
            "pages": pages,
            "categories": categories,
            "alerts": alerts,
        }),
    )?;

    OneBlogError::ok(utils::html(html))
}
