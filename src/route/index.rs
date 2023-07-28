use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;
use actix_web_flash_messages::IncomingFlashMessages;
use crate::controller;

// GET /admin
pub async fn index(
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error>{

    let posts = controller::post::all(&db).await.unwrap();
    let html = hbs
        .render("index", &serde_json::json!({"posts": posts}))
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();
    
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn post_id(
    post_id: web::Path<i32>,
    db: web::Data<sea_orm::DatabaseConnection>,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> Result<actix_web::HttpResponse, actix_web::Error>{
    let post = controller::post::find(&db, *post_id).await.unwrap();

    let html = hbs
        .render("post_id", &serde_json::json!({"post": post}))
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();
    
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))

}
