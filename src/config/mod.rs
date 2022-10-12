pub mod crypto;

use std::sync::Arc;

use color_eyre::Result;
use dotenv::dotenv;
use eyre::Context;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::{instrument, log::info};

use self::crypto::CryptoService;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i64,
    pub database_url: String,
    pub secret_key: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();

        info!("Loading config from environment variables");

        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        let host = config.get_string("host").context("Unwrapping HOST")?;
        let port = config.get_string("port").context("Unwrapping PORT")?;
        let port = port.parse::<i64>()?;
        let database_url = config
            .get_string("database_url")
            .context("Unwrapping DATABASE_URL")?;
        let secret_key = config
            .get_string("secret_key")
            .context("Unwrapping SECRET_KEY")?;
        Ok(Config {
            host,
            port,
            database_url,
            secret_key,
        })
    }

    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database pool");
        PgPoolOptions::new()
            .idle_timeout(Duration::from_secs(30))
            .connect(&self.database_url)
            .await
            .context("Creating database pool")
    }

    pub fn crypto_service(&self) -> CryptoService {
        CryptoService {
            key: Arc::new(self.secret_key.clone()),
        }
    }
}
