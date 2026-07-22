use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::models::model::{Model, ModelType};

#[derive(Debug, Error)]
pub enum ModelServiceError {
    #[error("Duplication error")]
    ModelDuplicate,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Password hashing failed")]
    PasswordHash(#[from] argon2::password_hash::Error),
}

pub async fn create_model(
    pool: &PgPool,
    brand_id: &Uuid,
    name: &str,
    ty: &ModelType
) -> Result<Model, ModelServiceError> {
    let u = match crate::db::model::inser_model(pool, brand_id, name, ty).await {
        Ok(u) => u,
        Err(sqlx::Error::Database(e)) => {
            if e.code().as_deref() == Some("23505") {
                return Err(ModelServiceError::ModelDuplicate);
            }
            return Err(sqlx::Error::Database(e).into());
        }
        Err(e) => return Err(e.into()),
    };
    Ok(u)
}

pub async fn get_model(
    pool: &PgPool,
    id: Uuid
) -> Result<Option<Model>, ModelServiceError> {
    let m = match crate::db::model::find_model_by_id(pool, id).await {
        Ok(m) => m,
        Err(e) => return Err(e.into())
    };
    Ok(m)
}

pub async fn get_model_list(
    pool: &PgPool,
) -> Result<Vec<Model>, ModelServiceError> {
    let m = match crate::db::model::list_models(pool).await {
        Ok(m) => m,
        Err(e) => return Err(e.into())
    };
    Ok(m)
}

pub async fn get_model_list_of_brand(
    pool: &PgPool,
    brand_id: Uuid
) -> Result<Vec<Model>, ModelServiceError> {
    let m = match crate::db::model::list_models_of_brand(pool, brand_id).await {
        Ok(m) => m,
        Err(e) => return Err(e.into())
    };
    Ok(m)
}
