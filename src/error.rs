use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OneBlogError {
    #[error("{self:?}")]
    DbError(#[from] sea_orm::DbErr),
    #[error("{self:?}")]
    Render(#[from] handlebars::RenderError),
    #[error("{self:?}")]
    InvalidParameter(String),
    #[error(transparent)]
    ActixWeb(#[from] actix_web::Error),
    #[error(transparent)]
    AuthError(#[from] crate::authentication::AuthError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl OneBlogError {
    pub fn ok<T>(t: T) -> Result<T, OneBlogError> {
        Ok(t)
    }
    pub fn err<T>(err: Self) -> Result<T, Self> {
        err.into()
    }
}

impl actix_web::ResponseError for OneBlogError {
    fn status_code(&self) -> StatusCode {
        match self {
            OneBlogError::InvalidParameter(err) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}

impl<T> From<OneBlogError> for Result<T, OneBlogError> {
    fn from(err: OneBlogError) -> Self {
        Err(err)
    }
}
