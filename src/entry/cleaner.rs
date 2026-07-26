use sqlx::PgPool;

pub async fn cleaner_thread(pool: PgPool) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60 * 60));

    loop {
        interval.tick().await;

        match sqlx::query!(
            r#"
                delete from revoked_tokens
                where expires_at <= now()
            "#
        )
        .execute(&pool)
        .await
        {
            Ok(result) => {
                tracing::debug!(
                    deleted = result.rows_affected(),
                    "cleaned expired revoked tokens"
                );
            }

            Err(error) => {
                tracing::error!(
                    %error,
                    "failed to clean expired revoked tokens"
                );
            }
        }
    }
}
