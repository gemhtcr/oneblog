use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use oneblog::database;
use oneblog::error;
use oneblog::route;
use oneblog::telemetry;
use oneblog::template_engine;
use secrecy::ExposeSecret;
use secrecy::Secret;

#[tokio::main]
async fn main() -> Result<(), crate::error::OneBlogError> {
    // telementry
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    // database
    let db = database::init().await?;
    // Flash message
    let key = "super-long-and-secret-random-key-needed-to-verify-message-integrity".to_string();
    let hmac_secret = Secret::new(key);
    let secret_key = actix_web::cookie::Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    // Session
    let redis_uri = Secret::new("redis://127.0.0.1:6379".to_string());
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;
    // Template engine
    let handlebars = template_engine::init()?;
    HttpServer::new(move || {
        App::new()
            // Public
            .configure(route::config::public)
            // Admin
            .configure(route::config::admin)
            //.wrap(TracingLogger::default())
            .app_data(web::Data::new(handlebars.clone()))
            .app_data(web::Data::new(db.clone()))
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
