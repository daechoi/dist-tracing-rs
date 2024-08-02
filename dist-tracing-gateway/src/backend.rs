use anyhow::{Context, Result};
use serde::Deserialize;
use tracing::debug;

mod proto {
    tonic::include_proto!("dist_tracing");
}

#[derive(Debug, Deserialize)]
pub struct Config {
    endpoint: String,
}

pub struct Backend {
    config: Config,
}

impl Backend {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn hello(&self) -> Result<String> {
        let mut client = proto::hello_client::HelloClient::connect(self.config.endpoint.to_owned())
            .await
            .with_context(|| format!("connect to endpoint {}", self.config.endpoint))?;

        let msg = client
            .say_hello(proto::HelloRequest {})
            .await
            .with_context(|| format!("call rpc hello on endpoint{}", self.config.endpoint))?
            .into_inner()
            .msg;

        debug!(
            msg,
            self.config.endpoint, "received response from rpc say_hello"
        );
        Ok(msg)
    }
}
