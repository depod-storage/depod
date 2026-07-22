use sqlx::PgPool;
use uuid::Uuid;

use crate::models::projects::Project;

pub async fn insert_project(
    pool: &PgPool,
    name: &str,
    city: &str
) -> Result<Project, sqlx::Error> {
    sqlx::query_as!(
        Project,
        r#"
            insert into projects (name, city)
            values ($1, $2)
            returning id, name, city, created_at
        "#,
        name, city
    ).fetch_one(pool).await
}

pub async fn find_project_by_id(
    pool: &PgPool,
    id: Uuid
) -> Result<Option<Project>, sqlx::Error> {
    sqlx::query_as!(
        Project,
        r#"
            select id, name, city, created_at
            from projects
            where id = $1
        "#,
        id
    ).fetch_optional(pool).await
}

pub async fn list_projects(
    pool: &PgPool
) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as!(
        Project,
        r#"
            select id, name, city, created_at
            from projects
            order by created_at desc
        "#
    ).fetch_all(pool).await
}
