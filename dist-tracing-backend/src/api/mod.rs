mod v0;

use anyhow::{Context, Result};
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use tonic::transport::Server;

#[derive(Debug, Deserialize)]
pub struct Config {
    addr: IpAddr,
    port: u16,
}

impl Config {
    fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.addr, self.port)
    }
}

pub async fn serve(config: Config) -> Result<()> {
    let socket_addr = config.socket_addr();

    let app = Server::builder().add_service(v0::hello());

    app.serve(socket_addr).await.context("serving api")
}
