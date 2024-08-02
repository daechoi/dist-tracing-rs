mod api;
mod backend;

use anyhow::{Context, Result};
use configured::Configured;
use serde::Deserialize;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::load().context("load configuration")?;
    let backend = backend::Backend::new(config.backend);
    let _ = api::run(config.api, backend)?.await;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Config {
    api: api::Config,
    backend: backend::Config,
}
