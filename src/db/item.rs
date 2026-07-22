use sqlx::PgPool;
use uuid::Uuid;

use crate::models::item::{Item, ItemStatus};

pub async fn inser_item(
    pool: &PgPool,
    model_id: &Uuid,
    project_id: &Uuid,
    serial: &str,
    status: &ItemStatus,
) -> Result<Item, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"
            insert into items (model_id, project_id, serial, status)
            values ($1, $2, $3, $4::item_status)
            returning id, model_id, project_id, serial, status as "status: ItemStatus"
        "#,
        model_id, project_id, serial, status as &ItemStatus,
    )
    .fetch_one(pool)
    .await
}

pub async fn find_item_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"
            select id, model_id, project_id, serial, status as "status: ItemStatus"
            from items
            where id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn list_items(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"
            select id, model_id, project_id, serial, status as "status: ItemStatus"
            from items
        "#
    )
    .fetch_all(pool)
    .await
}
