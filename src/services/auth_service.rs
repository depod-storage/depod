use argon2::{PasswordHash, PasswordVerifier};
use sqlx::PgPool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to login")]
    LoginFail,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Password hashing failed")]
    PasswordHash(#[from] argon2::password_hash::Error),
    #[error("JeasonWebToken failed.")]
    JWT(#[from] jsonwebtoken::errors::Error)
}

pub async fn login_user(
    pool: &PgPool,
    username: &str,
    password: &str,
    secret: &str
) -> Result<String, AuthError> {
    let user = crate::db::user::find_by_username_wp(pool, username).await?;
    let user = match user {
        Some(u) => u,
        None => return Err(AuthError::LoginFail)
    };

    let is_log = argon2::Argon2::default().verify_password(password.as_bytes(), &PasswordHash::new(&user.password_hash)?).is_ok(); //is ok to not reveal details.
    if !is_log { return Err(AuthError::LoginFail) }
    let tok = crate::backend::jwt::create_jwt(user.id, secret)?;
    Ok(tok)
}
