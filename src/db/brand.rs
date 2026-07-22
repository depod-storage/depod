use sqlx::PgPool;
use uuid::Uuid;

use crate::models::brand::Brand;

pub async fn insert_brand(
    pool: &PgPool,
    name: &str,
) -> Result<Brand, sqlx::Error> {
    sqlx::query_as!(
        Brand,
        r#"
            insert into brands (name)
            values ($1)
            returning id, name
        "#,
        name
    ).fetch_one(pool).await
}

pub async fn find_brand_by_id(
    pool: &PgPool,
    id: Uuid
) -> Result<Option<Brand>, sqlx::Error> {
    sqlx::query_as!(
        Brand,
        r#"
            select id, name
            from brands
            where id = $1
        "#,
        id
    ).fetch_optional(pool).await
}

pub async fn list_brands(
    pool: &PgPool
) -> Result<Vec<Brand>, sqlx::Error> {
    sqlx::query_as!(
        Brand,
        r#"
            select id, name
            from brands
            order by name desc
        "#
    ).fetch_all(pool).await
}
