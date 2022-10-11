use color_eyre::Result;
use dotenv::dotenv;
use eyre::Context;
use serde::Deserialize;
use tracing::{instrument, log::info};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i64,
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
        Ok(Config { host, port })
    }
}
