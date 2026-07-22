use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::{backend::hash_passwd, db::user, models::user::User};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("User does not exists.")]
    NoUser,
    #[error("Username already exists")]
    DuplicateUsername,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Password hashing failed")]
    PasswordHash(#[from] argon2::password_hash::Error),
}

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<User, UserServiceError> {
    let hash = hash_passwd(password)?;
    let u = match user::insert_user(pool, username, &hash).await {
        Ok(u) => u,

        Err(sqlx::Error::Database(e)) => {
            if e.code().as_deref() == Some("23505") {
                return Err(UserServiceError::DuplicateUsername);
            }

            return Err(sqlx::Error::Database(e).into());
        }

        Err(e) => return Err(e.into()),
    };
    Ok(u)
}

pub async fn add_u_perm_to_user(
    pool: &PgPool,
    id: Uuid,
    perm: i32,
) -> Result<User, UserServiceError> {
    let u = user::add_u_perm_to_user(pool, id, perm).await?;
    Ok(u)
}

pub async fn remove_u_perm_from_user(
    pool: &PgPool,
    id: Uuid,
    perm: i32,
) -> Result<User, UserServiceError> {
    let u = user::remove_u_perm_from_user(pool, id, perm).await?;
    Ok(u)
}

pub async fn get_user(
    pool: &PgPool,
    id: Uuid
) -> Result<User, UserServiceError> {
    let u = user::find_by_id(pool, id).await?;
    match u {
        Some(u) => Ok(u),
        None => Err(UserServiceError::NoUser)
    }
}
