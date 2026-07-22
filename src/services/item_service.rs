use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::models::item::{Item, ItemStatus};

#[derive(Debug, Error)]
pub enum ItemServiceError {
    #[error("Duplication error")]
    ItemDuplicate,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Password hashing failed")]
    PasswordHash(#[from] argon2::password_hash::Error),
}

pub async fn create_item(
    pool: &PgPool,
    model_id: &Uuid,
    project_id: &Uuid,
    serial: &str,
    status: &ItemStatus
) -> Result<Item, ItemServiceError> {
    let u = match crate::db::item::inser_item(pool, model_id, project_id, serial, status).await {
        Ok(u) => u,
        Err(sqlx::Error::Database(e)) => {
            if e.code().as_deref() == Some("23505") {
                return Err(ItemServiceError::ItemDuplicate);
            }
            return Err(sqlx::Error::Database(e).into());
        }
        Err(e) => return Err(e.into()),
    };
    Ok(u)
}

pub async fn get_item_list(
    pool: &PgPool,
) -> Result<Vec<Item>, ItemServiceError> {
    let m = match crate::db::item::list_items(pool).await {
        Ok(m) => m,
        Err(e) => return Err(e.into())
    };
    Ok(m)
}
