use sqlx::PgPool;
use thiserror::Error;

use crate::models::projects::Project;

#[derive(Debug, Error)]
pub enum ProjectServiceError {
    #[error("Duplication error")]
    CityProjectDuplicate,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Password hashing failed")]
    PasswordHash(#[from] argon2::password_hash::Error),
}

pub async fn create_project(
    pool: &PgPool,
    name: &str,
    city: &str
) -> Result<Project, ProjectServiceError> {
    let u = match crate::db::project::insert_project(pool, name, city).await {
        Ok(u) => u,
        Err(sqlx::Error::Database(e)) => {
            if e.code().as_deref() == Some("23505") {
                return Err(ProjectServiceError::CityProjectDuplicate);
            }
            return Err(sqlx::Error::Database(e).into());
        }
        Err(e) => return Err(e.into()),
    };
    Ok(u)
}

pub async fn get_project_list(
    pool: &PgPool,
) -> Result<Vec<Project>, ProjectServiceError> {
    let m = match crate::db::project::list_projects(pool).await {
        Ok(m) => m,
        Err(e) => return Err(e.into())
    };
    Ok(m)
}
