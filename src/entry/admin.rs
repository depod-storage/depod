use std::io::Write;
use sqlx::PgPool;

use crate::{
    backend::hash_passwd,
    entry::check_bootstrap,
    models::auth::{B_PERM_ALL, I_PERM_ALL, M_PERM_ALL, P_PERM_ALL, U_PERM_ALL},
};

pub async fn add_admin(pool: &PgPool) -> std::io::Result<()> {
    let bootstrap = match check_bootstrap(&pool).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read app_config: {e}");
            return Ok(());
        }
    };

    if bootstrap {
        print!(
            "There is allready an admin? You sure you want to do this? Pls use app for after bootstrap usage. [y/N]: "
        );
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" => {}
            _ => {
                println!("Cancelled");
                return Ok(());
            }
        }
    }

    let mut username = String::new();
    let mut password = String::new();
    print!("Username: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut username).unwrap();
    print!("Password: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut password).unwrap();
    let username = username.trim().to_owned();
    let password = password.trim().to_owned();

    let hash = match hash_passwd(&password) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Failed to hash password... {e}");
            return Ok(());
        }
    };

    match sqlx::query!(
        "insert into users (username, password_hash, user_perms, project_perms, brand_perms, model_perms, item_perms) values ($1, $2, $3, $4, $5, $6, $7)",
        &username,
        hash,
        U_PERM_ALL,
        P_PERM_ALL,
        B_PERM_ALL,
        M_PERM_ALL,
        I_PERM_ALL
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            sqlx::query!("update app_config set bootstrap = true where id = 1")
                .execute(pool)
                .await
                .ok();
            println!("Added admin...");
        }
        Err(e) => println!("Database error: {e}"),
    };
    Ok(())
}
