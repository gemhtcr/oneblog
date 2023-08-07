use crate::controller;
use crate::controller::post;
use crate::utils;
use actix_web::web;

// GET /admin/posts/{post_id}/edit
pub async fn edit_form(
    post_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    todo!()
}

// POST /admin/posts/{post_id}
#[derive(serde::Deserialize)]
pub struct EditFormData {
    post_id: i32,
    title: String,
    description: String,
    category_name: Option<String>,
}
pub async fn edit(
    edit_form_data: web::Path<EditFormData>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let _model = controller::post::update(
        &db,
        edit_form_data.post_id,
        &edit_form_data.title,
        &edit_form_data.description,
        edit_form_data.category_name.to_owned(),
    )
    .await
    .unwrap();

    Ok(utils::see_other("/admin"))
}

// DELETE /admin/posts/{post_id}
pub async fn delete(
    post_id: web::Form<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let _ret = controller::post::destroy(&db, *post_id).await.unwrap();
    Ok(utils::see_other("/admin"))
}

// GET /admin/posts/new/
pub async fn new_form(
    form: web::Form<NewFormData>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    //let _model = controller::post::create(&db, &form.title, &form.description, form.category_name).await.unwrap();
    //Ok(utils::see_other("/admin"))
    todo!()
}

// POST /admin/posts/
#[derive(serde::Deserialize)]
pub struct NewFormData {
    title: String,
    description: String,
    category_name: Option<String>,
}
pub async fn new(
    form: web::Form<NewFormData>,
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let _model = controller::post::create(
        &db,
        &form.title,
        &form.description,
        form.category_name.to_owned(),
    )
    .await
    .unwrap();
    Ok(utils::see_other("/admin"))
}
