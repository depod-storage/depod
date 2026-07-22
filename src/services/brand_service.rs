use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::models::{brand::Brand};

#[derive(Debug, Error)]
pub enum BrandServiceError {
    #[error("Duplication error")]
    BrandDuplicate,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Password hashing failed")]
    PasswordHash(#[from] argon2::password_hash::Error),
}

pub async fn create_brand(
    pool: &PgPool,
    name: &str,
) -> Result<Brand, BrandServiceError> {
    let u = match crate::db::brand::insert_brand(pool, name).await {
        Ok(u) => u,
        Err(sqlx::Error::Database(e)) => {
            if e.code().as_deref() == Some("23505") {
                return Err(BrandServiceError::BrandDuplicate);
            }
            return Err(sqlx::Error::Database(e).into());
        }
        Err(e) => return Err(e.into()),
    };
    Ok(u)
}

pub async fn get_brand_list(
    pool: &PgPool,
) -> Result<Vec<Brand>, BrandServiceError> {
    let m = match crate::db::brand::list_brands(pool).await {
        Ok(m) => m,
        Err(e) => return Err(e.into())
    };
    Ok(m)
}

pub async fn get_brand(
    pool: &PgPool,
    id: Uuid
) -> Result<Option<Brand>, BrandServiceError> {
    let m = match crate::db::brand::find_brand_by_id(pool, id).await {
        Ok(m) => m,
        Err(e) => return Err(e.into())
    };
    Ok(m)
}
