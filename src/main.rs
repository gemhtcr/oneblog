#![allow(unused)]
use entities::{prelude::*, *};
use sea_orm::*;
use handlebars::Handlebars;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_session::SessionMiddleware;
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use secrecy::Secret;
use secrecy::ExposeSecret;
use actix_session::storage::RedisSessionStore;


use oneblog::*;

use tracing::info;
use tracing_actix_web::TracingLogger;

const DATABASE_URL: &str = "mysql://root@localhost:3306/oneblog";

// admin/everythinghastostartsomewhere

#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{
    tracing_subscriber::fmt::init();
    //let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    //telemetry::init_subscriber(subscriber);
    let mut conn: ConnectOptions = DATABASE_URL.into();
    conn.sqlx_logging(false);
    let db = Database::connect(conn).await.unwrap();
	let key = "super-long-and-secret-random-key-needed-to-verify-message-integrity".to_string();
	let hmac_secret = Secret::new(key);
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
	let redis_uri = Secret::new("redis://127.0.0.1:6379".to_string());
	let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await.unwrap();

    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".hbs", "./templates/").unwrap();
    let db = Data::new(db);
    HttpServer::new(move || {
        App::new()
            //.wrap(message_framework.clone())
            //.wrap(SessionMiddleware::new(
            //    redis_store.clone(),
            //    secret_key.clone(),
            //))
            .wrap(TracingLogger::default())
            .route("/login", web::get().to(route::login::login_form))
            .route("/login", web::post().to(route::login::login))
            .app_data(handlebars.clone())
            .app_data(db.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
/*
    let cat = model::category::ActiveModel {
        name: ActiveValue::Set("cat 1".to_string()),
        created: ActiveValue::Set(chrono::offset::Utc::now()),
        updated: ActiveValue::Set(chrono::offset::Utc::now()),
        ..Default::default()
    };
    let cat = model::category::create(&db, cat).await?;

    let cat = model::category::ActiveModel {
        name: ActiveValue::Set("cat 2".to_string()),
        created: ActiveValue::Set(chrono::offset::Utc::now()),
        updated: ActiveValue::Set(chrono::offset::Utc::now()),
        ..Default::default()
    };
    let cat = model::category::create(&db, cat).await?;

    let post = model::post::ActiveModel {
        title: ActiveValue::Set("title 1".to_string()),
        description: ActiveValue::Set("description 1".to_string()),
        category_id: ActiveValue::Set(Some(1)),
        ..Default::default()
    };
    model::post::create(&db, post).await?;

    let post = model::post::ActiveModel {
        title: ActiveValue::Set("title 2".to_string()),
        description: ActiveValue::Set("description 2".to_string()),
        category_id: ActiveValue::Set(Some(2)),
        ..Default::default()
    };
    model::post::create(&db, post).await?;

    model::category::destroy(&db, 1).await?;
    */
}
