use sqlx::PgPool;

pub mod admin;
pub mod server;


pub(in crate::entry) async fn check_bootstrap(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let bootstrap: bool = sqlx::query_scalar("select bootstrap from app_config where id = 1")
        .fetch_one(pool)
        .await?;
    Ok(bootstrap)
}
