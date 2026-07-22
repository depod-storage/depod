pub mod api;
pub mod backend;
pub mod db;
pub mod entry;
pub mod extractors;
pub mod models;
pub mod services;

use actix_web::{HttpResponse, Responder, get};
use clap::{Parser, Subcommand};
use serde_json::json;

use crate::backend::{config::Config};

#[derive(Parser)]
#[command(version, about)]
struct CLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short = 't', long, alias = "tls")]
        https: bool,
        #[arg(short = 'c', long, default_value_t = false)]
        cookie: bool
    },
    Admin,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "envoriment": "dev",
        "version": "0.0.1-alpha"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let cli = CLI::parse();
    let config = Config::load_config().await.expect("Failed to load configs!");

    match cli.command {
        Command::Run { port, host, https, cookie} => {
            let cookie = https || cookie;

            crate::entry::server::server(config, host, port, https, cookie).await
        }
        Command::Admin => crate::entry::admin::add_admin(config.move_app_state().pool()).await,
    }
}
