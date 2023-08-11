use crate::controller;
use crate::controller::category;
use crate::utils;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;
use actix_web_flash_messages::IncomingFlashMessages;

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
    let categories = controller::category::offset_and_limit(&db, Some(0), Some(per_page as u64))
        .await
        .unwrap();
    let counts = controller::category::count(&db).await.unwrap();
    tracing::info!(?counts);
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
    page: web::Path<i32>,
    mut per_page: Option<web::Query<usize>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let per_page = per_page.map(|inner| inner.into_inner()).unwrap_or(3);
    let page = page.into_inner();
    let categories = controller::category::offset_and_limit(
        &db,
        Some((page as u64 - 1) * per_page as u64),
        Some(per_page as u64),
    )
    .await
    .unwrap();
    let counts = controller::category::count(&db).await.unwrap();
    tracing::info!(?counts);
    let pages = utils::paginate(
        counts as usize,
        per_page,
        page as usize,
        Some("<".to_string()),
        Some(">".to_string()),
    );

    let html = hbs
        .render(
            "admin/categories",
            &serde_json::json!({
                "header": "admin/_header",
                "sidebar": "admin/_sidebar",
                "pages": pages,
                "categories": categories,
            }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

// GET /admin/categories/new/
pub async fn new_form(
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let categories = controller::category::posts_count(&db).await;
    let html = hbs
        .render(
            "admin/categories_new_form",
            &serde_json::json!({
                "header": "admin/_header",
                "sidebar": "admin/_sidebar",
                "categories": categories,
            }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

// POST admin/categories
#[derive(serde::Deserialize, Debug)]
pub struct NewFormData {
    name: String,
}
pub async fn new(
    mut form: web::Form<NewFormData>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let mut form = form.into_inner();
    let _model = controller::category::create(&db, &form.name).await.unwrap();
    FlashMessage::success(format!(r#"Created "{}" with success"#, form.name)).send();
    Ok(utils::see_other("/admin/categories"))
}

// GET /admin/categories/{category_id}/edit
pub async fn edit_form(
    category_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let mut category = controller::category::find(&db, *category_id).await.unwrap();
    let html = hbs
        .render(
            "admin/categories_edit_form",
            &serde_json::json!({
                "header": "admin/_header",
                "sidebar": "admin/_sidebar",
                "category": category,
            }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

// POST /admin/categories/{category_id}
#[derive(serde::Deserialize, Debug)]
pub struct EditFormData {
    category_id: i32,
    name: String,
}
pub async fn edit(
    edit_form_data: web::Form<EditFormData>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    tracing::info!(?edit_form_data);
    // convert "none" into None
    let mut edit_form_data = edit_form_data.into_inner();
    let _model =
        controller::category::update(&db, edit_form_data.category_id, &edit_form_data.name)
            .await
            .unwrap();

    FlashMessage::success(format!(r#"Edited "{}" with success"#, edit_form_data.name)).send();
    Ok(utils::see_other("/admin/categories"))
}

// GET admin/categories/{category_id}/delete
pub async fn delete(
    category_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let _ret = controller::category::destroy(&db, *category_id)
        .await
        .unwrap();
    FlashMessage::success("Deleted a category with success").send();
    Ok(utils::see_other("/admin/categories"))
}
