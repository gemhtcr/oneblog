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

const DATABASE_URL: &str = "mysql://root@localhost:3306/oneblog";

// admin/everythinghastostartsomewhere

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

    let stmt = crate::post::ActiveModel {
        id: ActiveValue::Unchanged(1),
        title: ActiveValue::Set("abc".to_string()),
        description: ActiveValue::Set("des".to_string()),
        category_id: ActiveValue::Set(None),
        updated: ActiveValue::Set(chrono::offset::Utc::now()),
        ..Default::default()
    };

    //let r: <Post as sea_orm::entity::EntityTrait>::Model = Post::update(stmt);
    //let mut r = Post::update(stmt);
    //let stmt = r.query();
    //let stmt = stmt.to_string(sea_orm::sea_query::MysqlQueryBuilder);
    //info!(?stmt);

    HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .service(
                web::scope("/admin")
                    //.wrap(from_fn(authentication::middleware::reject_anonymous_users))
                    .route("/", web::get().to(route::admin::index::index))
                    // edit
                    .route(
                        "/posts/{post_id}/edit",
                        web::get().to(route::admin::post::edit_form),
                    )
                    .route("/posts/{post_id}", web::post().to(route::admin::post::edit))
                    // new
                    .route(
                        "/posts/{post_id}/new",
                        web::get().to(route::admin::post::new_form),
                    )
                    .route("/posts", web::post().to(route::admin::post::new))
                    // delete
                    .route(
                        "/posts/{post_id}",
                        web::delete().to(route::admin::post::delete),
                    ),
            )
            //.wrap(TracingLogger::default())
            .route("/login", web::get().to(route::login::login_form))
            .route("/login", web::post().to(route::login::login))
            .service(Files::new("/assets", "assets/").show_files_listing())
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(web::Data::new(db.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
