use crate::backend::AppState;
use anyhow::Context;
use directories::ProjectDirs;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::fs;

pub struct Config {
    dirs: ProjectDirs,
    cors: Cors,
    app_state: AppState,
}

#[derive(Debug, Deserialize, Default)]
struct RawConfig {
    #[serde(default)]
    cors: Cors,
}

#[derive(Debug, Deserialize, Default)]
struct Cors {
    #[serde(default)]
    allowed_hosts: Vec<String>,
}

impl Config {
    pub async fn load_config() -> anyhow::Result<Self> {
        let dirs = ProjectDirs::from("com", "meowball", "depod")
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

        let config_path = dirs.config_dir().join("config.toml");
        let raw: RawConfig = match fs::read_to_string(&config_path) {
            Ok(contents) => {
                toml::from_str(&contents).context("failed to parse config.toml")?
            }
            Err(_) => RawConfig::default(),
        };

        let db_url = env::var("DATABASE_URL").context("Database Url missing.")?;
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .context("Could not connect to server.")?;

        let jwt_secret = env::var("JWT_SECRET").context("JWT Secret missing")?;
        let jwt_expire: i64 = env::var("JWT_EXPIRE")
            .context("JWT Expire missing")?
            .parse()
            .context("JWT Expire must be int")?;
        let domain = env::var("DOMAIN").context("DOMAIN missing")?;

        Ok(Self {
            dirs,
            cors: raw.cors,
            app_state: AppState::new(pool, jwt_secret, jwt_expire, domain),
        })
    }

    pub fn allowed_hosts(&self) -> &[String] {
        &self.cors.allowed_hosts
    }

    pub fn dirs(&self) -> &ProjectDirs {
        &self.dirs
    }

    pub fn app_state(&self) -> &AppState {
        &self.app_state
    }

    pub fn app_state_owned(&self) -> AppState {
        self.app_state.clone()
    }

    pub fn move_app_state(self) -> AppState {
        self.app_state
    }
}
