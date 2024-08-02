use crate::backend::Backend;
use actix_web::{dev::Server, get, web, App, HttpResponse, HttpServer, Responder};

use anyhow::{Context, Result};
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/greet")]
async fn greet(grpc: web::Data<Backend>) -> impl Responder {
    let msg = grpc
        .as_ref()
        .hello()
        .await
        .context("calling backend hello?")
        .unwrap();
    HttpResponse::Ok().body(format!("Hello received: {}", msg))
}

pub fn run(config: Config, backend: Backend) -> Result<Server> {
    let grpc_backend = web::Data::new(backend);
    let server = HttpServer::new(move || {
        App::new()
            .service(index)
            .service(greet)
            .app_data(grpc_backend.clone())
    })
    .bind(config.socket_addr())
    .context("Failed to bind server to address")?;

    Ok(server.run())
}

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
