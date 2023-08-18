use crate::authentication::AuthError;
use crate::authentication::{validate_credentials, Credentials};
use crate::error::OneBlogError;
use crate::session_state::TypedSession;
use crate::utils;
use crate::utils::error_chain_fmt;
use actix_web::error::InternalError;
use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;
use actix_web_flash_messages::IncomingFlashMessages;
use actix_web_flash_messages::Level;
use secrecy::Secret;

#[derive(serde::Serialize)]
struct MyFlashMessage {
    content: String,
    level: String,
}

pub async fn login_form(
    flash_messages: IncomingFlashMessages,
    hbs: web::Data<handlebars::Handlebars<'_>>,
) -> impl actix_web::Responder {
    let alerts = flash_messages
        .iter()
        .map(|msg| MyFlashMessage {
            content: msg.content().to_string(),
            level: match msg.level() {
                Level::Error => "danger".to_string(),
                other => other.to_string(),
            },
        })
        .collect::<Vec<_>>();
    let html = hbs.render("login_form", &serde_json::json!({"alerts": alerts}))?;

    OneBlogError::ok(utils::html(html))
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
