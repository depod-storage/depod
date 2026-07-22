use sqlx::PgPool;
use uuid::Uuid;

use crate::models::model::{Model, ModelType};

pub async fn inser_model(
    pool: &PgPool,
    brand_id: &Uuid,
    name: &str,
    ty: &ModelType,
) -> Result<Model, sqlx::Error> {
    sqlx::query_as!(
        Model,
        r#"
            insert into models (brand_id, name, ty)
            values ($1, $2, $3::model_type)
            returning id, brand_id, name, ty as "ty: ModelType"
        "#,
        brand_id,
        name,
        ty as &ModelType
    )
    .fetch_one(pool)
    .await
}

pub async fn find_model_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Model>, sqlx::Error> {
    sqlx::query_as!(
        Model,
        r#"
            select id, brand_id, name, ty as "ty: ModelType"
            from models
            where id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn list_models(pool: &PgPool) -> Result<Vec<Model>, sqlx::Error> {
    sqlx::query_as!(
        Model,
        r#"
            select id, brand_id, name, ty as "ty: ModelType"
            from models
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn list_models_of_brand(
    pool: &PgPool,
    brand_id: Uuid,
) -> Result<Vec<Model>, sqlx::Error> {
    sqlx::query_as!(
        Model,
        r#"
            select id, brand_id, name, ty as "ty: ModelType"
            from models
            where brand_id = $1
        "#,
        brand_id
    )
    .fetch_all(pool)
    .await
}
