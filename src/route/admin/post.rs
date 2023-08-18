use crate::controller;
use crate::error::OneBlogError;
use crate::utils;
use actix_web::web;
use actix_web_flash_messages::FlashMessage;

// GET /admin/posts/{post_id}/edit
pub async fn edit_form(
    post_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
    //) -> Result<actix_web::HttpResponse, actix_web::Error> {
) -> impl actix_web::Responder {
    let mut post = controller::post::find(&db, *post_id).await?;
    if let 
        Some(
            inner @ controller::post::Model {
                category_name: None,
                ..
            },
        ) = post.as_mut() {
            inner.category_name = Some("None".to_string());
        }
    let categories = controller::category::posts_count(&db).await?;

    let html = hbs.render(
        "admin/edit_form",
        &serde_json::json!({
            "header": "admin/_header",
            "sidebar": "admin/_sidebar",
            "categories": categories,
            "post": post,
        }),
    )?;

    OneBlogError::ok(utils::html(html))
}

// POST /admin/posts/{post_id}
#[derive(serde::Deserialize, Debug)]
pub struct EditFormData {
    post_id: i32,
    title: String,
    description: String,
    category_name: Option<String>,
}
pub async fn edit(
    edit_form_data: web::Form<EditFormData>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> impl actix_web::Responder {
    tracing::info!(?edit_form_data);
    // convert "none" into None
    let mut edit_form_data = edit_form_data.into_inner();
    edit_form_data.category_name = edit_form_data
        .category_name
        .filter(|inner| inner.to_lowercase() != "none");
    let _model = controller::post::update(
        &db,
        edit_form_data.post_id,
        &edit_form_data.title,
        &edit_form_data.description,
        edit_form_data.category_name.to_owned(),
    )
    .await?;

    FlashMessage::success(format!(r#"Edited "{}" with success"#, edit_form_data.title)).send();
    OneBlogError::ok(utils::see_other("/admin"))
}

// GET /admin/posts/{post_id}
pub async fn delete(
    post_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> impl actix_web::Responder {
    let deleted = controller::post::destroy(&db, *post_id).await?;
    tracing::debug!(?deleted);
    match deleted.rows_affected {
        1.. => FlashMessage::success("Deleted a post with success").send(),
        _ => FlashMessage::warning("Failed to delete a post cause it didn't exist").send(),
    }
    OneBlogError::ok(utils::see_other("/admin"))
}

// GET /admin/posts/new/
pub async fn new_form(
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let categories = controller::category::posts_count(&db).await?;
    let html = hbs.render(
        "admin/new_form",
        &serde_json::json!({
            "header": "admin/_header",
            "sidebar": "admin/_sidebar",
            "categories": categories,
        }),
    )?;

    OneBlogError::ok(utils::html(html))
}

// POST /admin/posts/
#[derive(serde::Deserialize, Debug)]
pub struct NewFormData {
    title: String,
    description: String,
    category_name: Option<String>,
}
pub async fn new(
    form: web::Form<NewFormData>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> impl actix_web::Responder {
    let mut form = form.into_inner();
    // convert "none" into None
    form.category_name = form
        .category_name
        .filter(|inner| inner.to_lowercase() != "none");
    let _model = controller::post::create(
        &db,
        &form.title,
        &form.description,
        form.category_name.to_owned(),
    )
    .await?;
    FlashMessage::success(format!(r#"Created "{}" with success"#, form.title)).send();
    OneBlogError::ok(utils::see_other("/admin"))
}

pub async fn posts(
    page: web::Path<i32>,
    per_page: Option<web::Query<usize>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let per_page = per_page.map(|inner| inner.into_inner()).unwrap_or(3);
    let page = page.into_inner() as usize;
    let counts = controller::post::count(&db).await?;
    let pages = utils::paginate(
        counts as usize,
        per_page,
        page,
        Some("<".to_string()),
        Some(">".to_string()),
    );

    let posts =
        controller::post::offset_and_limit(&db, ((page - 1) * per_page) as u64, per_page as u64)
            .await?;
    let categories = controller::category::posts_count(&db).await?;
    let html = hbs.render(
        "admin/index",
        &serde_json::json!({
            "header": "admin/_header",
            "sidebar":"admin/_sidebar",
            "posts": posts,
            "pages": pages,
            "categories": categories
        }),
    )?;

    OneBlogError::ok(utils::html(html))
}
