use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    id: Uuid,
    exp: i64
}

pub fn create_jwt(id: Uuid, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        id,
        exp: Utc::now().timestamp() + 98000,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn get_user_uuid(token: &str, secret: &str) -> Option<Uuid> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .ok()?;

    Some(data.claims.id)
}

pub async fn get_user_from_token(
    token: &str,
    pool: &PgPool,
    secret: &str,
) -> Result<Option<User>, sqlx::Error> {
    let id = match get_user_uuid(token, secret) {
        Some(id) => id,
        None => return Ok(None),
    };

    crate::db::user::find_by_id(pool, id).await
}
