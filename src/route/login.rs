use crate::authentication::AuthError;
use crate::authentication::{validate_credentials, Credentials};
use crate::session_state::TypedSession;
use crate::utils::error_chain_fmt;
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;
use actix_web_flash_messages::IncomingFlashMessages;

use secrecy::Secret;
use std::fmt::Write;

pub async fn login_form(
    flash_messages: IncomingFlashMessages,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> HttpResponse {
    let mut flash_error = String::new();
    //use crate::authentication::password::compute_password_hash;
    //use secrecy::ExposeSecret;
    //let r = compute_password_hash(Secret::new("admin".to_string())).unwrap();
    //tracing::info!("password hash = {}", r.expose_secret());
    //let r = crate::authentication::password::verify_password_hash(r, Secret::new("admin".to_string()));
    //tracing::info!("password = {:?}", r);

    for m in flash_messages.iter() {
        //writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
        writeln!(flash_error, "{}", m.content()).unwrap();
    }
    let html = hbs
        .render(
            "login_form",
            &serde_json::json!({"error_html": flash_error}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip(form, db, session),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
// We are now injecting `PgPool` to retrieve stored credentials from the database
pub async fn login(
    form: web::Form<FormData>,
    db: web::Data<sea_orm::DatabaseConnection>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };
    use secrecy::ExposeSecret;
    tracing::info!(
        "====== {}:{}",
        credentials.username,
        credentials.password.expose_secret()
    );
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    match validate_credentials(credentials, &db).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/admin/dashboard"))
                .finish())
        }
        Err(e) => {
            tracing::error!("{:?}", e);
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(login_redirect(e))
        }
    }
}

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish();
    InternalError::from_response(e, response)
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
