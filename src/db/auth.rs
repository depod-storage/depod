use chrono::{DateTime, Utc};
use sha2::Digest;
use sqlx::PgPool;

pub async fn is_revoked(pool: &PgPool, token: &str) -> Result<bool, sqlx::Error> {
    let token_h = hex::encode(sha2::Sha256::digest(token.as_bytes()));
    sqlx::query_scalar!(
        r#"
            select exists (
            select 1
            from revoked_tokens
            where token = $1
        ) as "exists!: bool"
        "#,
        token_h,
    )
    .fetch_one(pool)
    .await
}

pub async fn revoke_token(
    pool: &PgPool,
    token: &str,
    expires_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    let token_h = hex::encode(sha2::Sha256::digest(token.as_bytes()));
    sqlx::query!(
        "
        insert into revoked_tokens (token, expires_at)
        values ($1, $2)
        on conflict (token) do nothing
        ",
        token_h,
        expires_at
    )
    .execute(pool)
    .await?;
    Ok(())
}
