use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db::{self, user::get_jwt_ver}, models::user::User};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    exp: i64,
    v: i32,
}

pub fn create_jwt(
    sub: Uuid,
    secret: &str,
    version: i32,
    expire: i64
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub,
        exp: Utc::now().timestamp() + expire,
        v: version,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn get_user_uuid(token: &str, secret: &str) -> Option<(Uuid, i32)> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .ok()?;
    if data.claims.exp <= chrono::Utc::now().timestamp() { return None; }

    Some((data.claims.sub, data.claims.v))
}

pub async fn get_user_from_token(
    token: &str,
    pool: &PgPool,
    secret: &str,
) -> Result<Option<User>, sqlx::Error> {
    let (user_id, token_version) = match get_user_uuid(token, secret) {
        Some(data) => data,
        None => return Ok(None),
    };

    let u = crate::db::user::find_by_id(pool, user_id).await;
    let ver = match get_jwt_ver(pool, user_id).await? {
        Some(ver) => ver,
        None => return Ok(None),
    };
    if token_version != ver {
        return Ok(None);
    }
    u
}

pub async fn revoke_token(
    token: &str,
    secret: &str,
    pool: &PgPool
) -> Result<(), ()>{
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| ())?;
    db::auth::revoke_token(pool, token, DateTime::from_timestamp(data.claims.exp, 0).expect("Thats imposible")).await.map_err(|_| ())?;
    Ok(())
}
