use crate::controller;
use crate::controller::post;
use crate::utils;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;

// GET /admin/posts/{post_id}/edit
pub async fn edit_form(
    post_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let mut post = controller::post::find(&db, *post_id).await.unwrap();
    match post.as_mut() {
        Some(
            inner @ controller::post::Model {
                category_name: None,
                ..
            },
        ) => {
            inner.category_name = Some("None".to_string());
        }
        _ => {}
    };
    let categories = controller::category::posts_count(&db).await;

    let html = hbs
        .render(
            "admin/edit_form",
            &serde_json::json!({
                "header": "admin/_header",
                "sidebar": "_sidebar",
                "categories": categories,
                "post": post,
            }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
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
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
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
    .await
    .unwrap();

    FlashMessage::success(format!(r#"Edit "{}" with success"#, edit_form_data.title)).send();
    Ok(utils::see_other("/admin"))
}

// GET /admin/posts/{post_id}
pub async fn delete(
    post_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let _ret = controller::post::destroy(&db, *post_id).await.unwrap();
    FlashMessage::success("Delete a post with success").send();
    Ok(utils::see_other("/admin"))
}

// GET /admin/posts/new/
pub async fn new_form(
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let categories = controller::category::posts_count(&db).await;
    let html = hbs
        .render(
            "admin/new_form",
            &serde_json::json!({
                "header": "admin/_header",
                "sidebar": "_sidebar",
                "categories": categories,
            }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

// POST /admin/posts/
#[derive(serde::Deserialize, Debug)]
pub struct NewFormData {
    title: String,
    description: String,
    category_name: Option<String>,
}
pub async fn new(
    mut form: web::Form<NewFormData>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
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
    .await
    .unwrap();
    FlashMessage::success(format!(r#"Create "{}" with success"#, form.title)).send();
    Ok(utils::see_other("/admin"))
}

pub async fn posts(
    page: web::Path<i32>,
    mut per_page: Option<web::Query<usize>>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let per_page = per_page.map(|inner| inner.into_inner()).unwrap_or(3);
    let page = page.into_inner() as usize;
    let counts = controller::post::count(&db).await.unwrap();
    let pages = utils::paginate(
        counts as usize,
        per_page,
        page,
        Some("<".to_string()),
        Some(">".to_string()),
    );

    let posts =
        controller::post::offset_and_limit(&db, ((page - 1) * per_page) as u64, per_page as u64)
            .await
            .unwrap();
    let categories = controller::category::posts_count(&db).await;
    let html = hbs
        .render(
            "admin/index",
            &serde_json::json!({
                "header": "admin/_header",
                "sidebar":"admin/_sidebar",
                "posts": posts,
                "pages": pages,
                "categories": categories
            }),
        )
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
