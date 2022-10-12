#[macro_use]
extern crate validator_derive;

mod config;
mod handlers;
mod models;

use crate::{config::Config, handlers::app_config};
use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Failed to load config");

    let pool = config.db_pool().await.expect("Database configuration");

    let crypto_service = config.crypto_service();

    info!("Starting server at {}:{}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(app_config)
            .app_data(pool.clone())
            .app_data(crypto_service.clone())
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
