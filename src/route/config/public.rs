use crate::route;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(route::index::index))
        .route("/posts/{post}", web::get().to(route::index::post_id))
        .route("/posts/page/{page}", web::get().to(route::index::posts))
        .route(
            "/posts/category/{category}/page/{page}",
            web::get().to(route::index::posts_with_category),
        )
        .route("/login", web::get().to(route::login::login_form))
        .route("/login", web::post().to(route::login::login))
        // Search
        .service(
            web::scope("/search")
                .route("", web::post().to(route::index::search))
                .route(
                    "/page/{page}",
                    web::post().to(route::index::search_with_page),
                ),
        )
        // static files listing
        .service(actix_files::Files::new("/assets", "assets/").show_files_listing());
}
