use crate::controller;
use crate::error::OneBlogError;
use crate::utils;
use actix_web::web;
use actix_web_flash_messages::FlashMessage;
use actix_web_flash_messages::IncomingFlashMessages;

#[derive(serde::Serialize, serde::Deserialize)]
struct MyFlashMessage {
    content: String,
    level: String,
}

// GET admin/categories
pub async fn index(
    per_page: Option<web::Query<usize>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
    flash_messages: IncomingFlashMessages,
) -> impl actix_web::Responder {
    let per_page = per_page.map(|inner| inner.into_inner()).unwrap_or(3);
    let paginator = controller::category::paginator(&db, per_page as u64);
    let categories = paginator.fetch_page(0).await?;
    let sea_orm::ItemsAndPagesNumber {
        number_of_items,
        number_of_pages: _,
    } = paginator.num_items_and_pages().await?;
    tracing::info!(?number_of_items);
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
    let html = hbs.render(
        "admin/categories",
        &serde_json::json!({
            "pages": pages,
            "categories": categories,
            "alerts": alerts,
        }),
    )?;

    OneBlogError::ok(utils::html(html))
}

// GET admin/categories/page/{page_number}
pub async fn page(
    page: web::Path<i32>,
    per_page: Option<web::Query<usize>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let per_page = per_page.map(|inner| inner.into_inner()).unwrap_or(3);
    let page = page.into_inner();
    let paginator = controller::category::paginator(&db, per_page as u64);
    let categories = paginator.fetch_page(1).await?;
    let sea_orm::ItemsAndPagesNumber {
        number_of_items,
        number_of_pages: _,
    } = paginator.num_items_and_pages().await?;
    let pages = utils::paginate(
        number_of_items as usize,
        per_page,
        page as usize,
        Some("<".to_string()),
        Some(">".to_string()),
    );
    let html = hbs.render(
        "admin/categories",
        &serde_json::json!({
            "pages": pages,
            "categories": categories,
        }),
    )?;

    OneBlogError::ok(utils::html(html))
}

// GET /admin/categories/new/
pub async fn new_form(
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let categories = controller::category::posts_count(&db)
        .await
        .map_err(Into::<crate::error::OneBlogError>::into)?;
    let html = hbs.render(
        "admin/categories_new_form",
        &serde_json::json!({
            "categories": categories,
        }),
    )?;

    OneBlogError::ok(utils::html(html))
}

// POST admin/categories
#[derive(serde::Deserialize, Debug)]
pub struct NewFormData {
    name: String,
}
pub async fn new(
    form: web::Form<NewFormData>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> impl actix_web::Responder {
    let form = form.into_inner();
    let _model = controller::category::create(&db, &form.name).await?;
    FlashMessage::success(format!(r#"Created "{}" with success"#, form.name)).send();

    OneBlogError::ok(utils::see_other("/admin/categories"))
}

// GET /admin/categories/{category_id}/edit
pub async fn edit_form(
    category_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let category = controller::category::find(&db, *category_id).await?;
    let html = hbs.render(
        "admin/categories_edit_form",
        &serde_json::json!({
            "category": category,
        }),
    )?;

    OneBlogError::ok(utils::html(html))
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
) -> impl actix_web::Responder {
    tracing::info!(?edit_form_data);
    // convert "none" into None
    let edit_form_data = edit_form_data.into_inner();
    let _model =
        controller::category::update(&db, edit_form_data.category_id, &edit_form_data.name).await?;
    FlashMessage::success(format!(r#"Edited "{}" with success"#, edit_form_data.name)).send();

    OneBlogError::ok(utils::see_other("/admin/categories"))
}

// GET admin/categories/{category_id}/delete
pub async fn delete(
    category_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
    //) -> Result<actix_web::HttpResponse, actix_web::Error> {
) -> impl actix_web::Responder {
    let _ret = controller::category::destroy(&db, *category_id).await?;
    FlashMessage::success("Deleted a category with success").send();

    OneBlogError::ok(utils::see_other("/admin/categories"))
}
