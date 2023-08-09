#![allow(unused)]
use actix_files::Files;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_lab::middleware::from_fn;
use entities::{prelude::*, *};
use handlebars::Handlebars;
use oneblog::*;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use secrecy::ExposeSecret;
use secrecy::Secret;
use tracing::info;
use tracing_actix_web::TracingLogger;

const DATABASE_URL: &str = "mysql://root@127.0.0.1:3306/oneblog";

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    //tracing_subscriber::fmt::init();
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let mut conn: ConnectOptions = DATABASE_URL.into();
    conn.sqlx_logging(false);
    let db = Database::connect(conn).await.unwrap();
    let key = "super-long-and-secret-random-key-needed-to-verify-message-integrity".to_string();
    let hmac_secret = Secret::new(key);
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_uri = Secret::new("redis://127.0.0.1:6379".to_string());
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret())
        .await
        .unwrap();
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./src/view/")
        .unwrap();
    use sea_orm::query::QueryTrait;
    use sea_orm::sea_query::QueryStatementBuilder;
    use sea_orm::ActiveValue;
    use sea_orm::EntityTrait;

    //let handlebars = crate::Handlebars::new();
    use handlebars::handlebars_helper;
    use serde_json::Value as Json;

    HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .route("/", web::get().to(route::index::index))
            .route("/index.html", web::get().to(route::index::index))
            .route("/posts/{post_id}", web::get().to(route::index::post_id))
            .route(
                "/posts/page/{page_number}",
                web::get().to(route::index::posts),
            )
            .route(
                "/posts/category/{category_id}/page/{page_number}",
                web::get().to(route::index::posts_with_category),
            )
            .service(
                web::scope("/admin")
                    .wrap(from_fn(authentication::middleware::reject_anonymous_users))
                    .route("", web::get().to(route::admin::index::index))
                    .route("/", web::get().to(route::admin::index::index))
                    .route("/dashboard", web::get().to(route::admin::index::index))
                    // page
                    .route(
                        "/posts/page/{page_number}",
                        web::get().to(route::admin::post::posts),
                    )
                    // edit
                    .route(
                        "/posts/{post_id}/edit",
                        web::get().to(route::admin::post::edit_form),
                    )
                    .route("/posts/{post_id}", web::post().to(route::admin::post::edit))
                    // new
                    .route("/posts/new", web::get().to(route::admin::post::new_form))
                    .route("/posts", web::post().to(route::admin::post::new))
                    // delete
                    .route(
                        "/posts/{post_id}/delete",
                        web::get().to(route::admin::post::delete),
                    )
                    // categories
                    .route("/categories", web::get().to(route::admin::category::index))
                    // categories, page
                    .route("/categories/page/{page_number}", web::get().to(route::admin::category::page))
                    // categories, new_form
                    .route("/categories/new", web::get().to(route::admin::category::new_form))
                    // categories, new
                    .route("/categories", web::post().to(route::admin::category::new))
                    // categories, edit_form
                    .route("/categories/{category_id}/edit", web::get().to(route::admin::category::edit_form))
                    // categories, edit
                    .route("/categories/{category_id}", web::post().to(route::admin::category::edit))
                    // categories, delete
                    .route("/categories/{category_id}/delete", web::get().to(route::admin::category::delete))
                    // logout
                    .route("/logout", web::get().to(route::admin::logout::logout)),
            )
            //.wrap(TracingLogger::default())
            .route("/login", web::get().to(route::login::login_form))
            .route("/login", web::post().to(route::login::login))
            .service(actix_files::Files::new("/assets", "assets/").show_files_listing())
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(web::Data::new(db.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
