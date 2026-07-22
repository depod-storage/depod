use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{User, UserWithPass};

pub async fn insert_user(
    pool: &PgPool,
    username: &str,
    password_hash: &str
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            insert into users (username, password_hash) 
            values ($1, $2)
            returning id, username, user_perms, project_perms, brand_perms, model_perms, item_perms
        "#,
        username, password_hash
    ).fetch_one(pool).await
}

pub async fn find_by_username_wp(
    pool: &PgPool,
    username: &str
) -> Result<Option<UserWithPass>, sqlx::Error> {
    sqlx::query_as!(
        UserWithPass,
        "select id, username, user_perms, project_perms, brand_perms, model_perms, item_perms, password_hash from users where username = $1",
        username
    ).fetch_optional(pool).await
}

pub async fn find_by_username(
    pool: &PgPool,
    username: &str
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "select id, username, user_perms, project_perms, brand_perms, model_perms, item_perms from users where username = $1",
        username
    ).fetch_optional(pool).await
}

pub async fn find_by_id_wp(
    pool: &PgPool,
    id: Uuid
) -> Result<Option<UserWithPass>, sqlx::Error> {
    sqlx::query_as!(
        UserWithPass,
        "select id, username, user_perms, project_perms, brand_perms, model_perms, item_perms, password_hash from users where id = $1",
        id
    ).fetch_optional(pool).await
}

pub async fn find_by_id(
    pool: &PgPool,
    id: Uuid
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "select id, username, user_perms, project_perms, brand_perms, model_perms, item_perms from users where id = $1",
        id
    ).fetch_optional(pool).await
}

pub async fn list_users(
    pool: &PgPool
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            select id, username, user_perms, project_perms, brand_perms, model_perms, item_perms
            from users
            order by username
        "#
    ).fetch_all(pool).await
}

pub async fn add_u_perm_to_user(
    pool: &PgPool,
    id: Uuid,
    perm: i32,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        update users
        set user_perms = user_perms | $2
        where id = $1
        returning id, username, user_perms, project_perms, brand_perms, model_perms, item_perms
        "#,
        id,
        perm
    )
    .fetch_one(pool)
    .await
}

pub async fn remove_u_perm_from_user(
    pool: &PgPool,
    id: Uuid,
    perm: i32,
) -> Result<User, sqlx::Error> {
    let mask = !perm;
    sqlx::query_as!(
        User,
        r#"
        update users
        set user_perms = user_perms & $2
        where id = $1
        returning id, username, user_perms, project_perms, brand_perms, model_perms, item_perms
        "#,
        id,
        mask
    )
    .fetch_one(pool)
    .await
}
