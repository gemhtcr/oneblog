use crate::entities::prelude::Users;
use crate::entities::users;
pub use crate::entities::users::ActiveModel;
use crate::entities::users::Model;
use crate::telemetry::spawn_blocking_with_tracing;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use sea_orm::*;
use secrecy::{ExposeSecret, Secret};
use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Get stored credentials", skip(username, db))]
async fn get_stored_credentials(
    username: &str,
    db: &sea_orm::DatabaseConnection,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
    Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await?
        .map(
            |Model {
                 user_id,
                 password_hash,
                 ..
             }| {
                uuid::Uuid::from_str(&user_id)
                    .map_err(anyhow::Error::new)
                    .map(|uuid| (uuid, Secret::new(password_hash)))
            },
        )
        .transpose()
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, db))]
pub async fn validate_credentials(
    credentials: Credentials,
    db: &sea_orm::DatabaseConnection,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.username, db).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task.")??;

    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown username."))
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(
    name = "Validate password hash",
    skip(expected_password_hash, password_candidate)
)]
pub fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}

#[tracing::instrument(name = "Change password", skip(password))]
pub async fn change_password(
    user_id: uuid::Uuid,
    password: Secret<String>,
) -> Result<(), anyhow::Error> {
    let _password_hash = spawn_blocking_with_tracing(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;
    /*
    sqlx::query!(
        r#"
        UPDATE users
        SET password_hash = $1
        WHERE user_id = $2
        "#,
        password_hash.expose_secret(),
        user_id
    )
    .execute(pool)
    .await
    .context("Failed to change user's password in the database.")?;
    */
    //controller::users::update().await.unwrap();
    Ok(())
}

pub fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None)?,
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();
    Ok(Secret::new(password_hash))
}
