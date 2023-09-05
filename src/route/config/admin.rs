use crate::authentication;
use crate::route;
use actix_web::web;
use actix_web_lab::middleware::from_fn;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .wrap(from_fn(authentication::middleware::reject_anonymous_users))
            .route("", web::get().to(route::admin::index::index))
            .route("/dashboard", web::get().to(route::admin::index::index))
            // page
            .route(
                "/posts/page/{page}",
                web::get().to(route::admin::post::posts),
            )
            // edit
            .route(
                "/posts/{post}/edit",
                web::get().to(route::admin::post::edit_form),
            )
            .route("/posts/{post}", web::post().to(route::admin::post::edit))
            // new
            .route("/posts/new", web::get().to(route::admin::post::new_form))
            .route("/posts", web::post().to(route::admin::post::new))
            // delete
            .route(
                "/posts/{post}/delete",
                web::get().to(route::admin::post::delete),
            )
            // categories
            .route("/categories", web::get().to(route::admin::category::index))
            // categories, page
            .route(
                "/categories/page/{page}",
                web::get().to(route::admin::category::page),
            )
            // categories, new_form
            .route(
                "/categories/new",
                web::get().to(route::admin::category::new_form),
            )
            // categories, new
            .route("/categories", web::post().to(route::admin::category::new))
            // categories, edit_form
            .route(
                "/categories/{category}/edit",
                web::get().to(route::admin::category::edit_form),
            )
            // categories, edit
            .route(
                "/categories/{category}",
                web::post().to(route::admin::category::edit),
            )
            // categories, delete
            .route(
                "/categories/{category}/delete",
                web::get().to(route::admin::category::delete),
            )
            // logout
            .route("/logout", web::get().to(route::admin::logout::logout)),
    );
}
