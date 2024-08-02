mod api;

use anyhow::{Context, Result};
use configured::Configured;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    if let Err(err) = init_tracing() {
        error!(%err, "Failed to initialize tracing");
    }
    if let Err(ref err) = run().await {
        error!(
            error = format!("{err:#}"), 
            backtrace = %err.backtrace(), 
            "shutting down");
    }
}

async fn run() -> Result<()> {
    let config = Config::load().context("load configuration")?;
    info!(?config, "starting");

    api::serve(config.api).await
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    api: api::Config,
}

fn init_tracing() -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .try_init()
        .context("initialize tracing subscriber")
}
