mod api;
mod backend;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use anyhow::{Context, Result};
use configured::Configured;
use serde::Deserialize;
use tracing::error;

#[actix_web::main]
async fn main() -> Result<()> {
    if let Err(err) = init_logger() {
        error!(
            error = format!("{err:#}"),
                backtrace=%err.backtrace(),
                "dist-tracing-gateway exited with error"
        );
    }
    let config = Config::load().context("load configuration")?;
    let backend = backend::Backend::new(config.backend);
    let _ = api::run(config.api, backend)?.await;
    Ok(())
}

fn init_logger() -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json())
        .try_init()
        .context("initialize tracing subscriber")
}

#[derive(Debug, Deserialize)]
struct Config {
    api: api::Config,
    backend: backend::Config,
}
