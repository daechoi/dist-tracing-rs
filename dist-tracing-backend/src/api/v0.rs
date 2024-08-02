use proto::hello_server::HelloServer;
use tonic::{Request, Response, Status};

mod proto {
    tonic::include_proto!("dist_tracing");
}

pub struct HelloService;

pub fn hello() -> HelloServer<HelloService> {
    HelloServer::new(HelloService)
}

#[tonic::async_trait]
impl proto::hello_server::Hello for HelloService {
    async fn say_hello(
        &self,
        _request: Request<proto::HelloRequest>,
    ) -> Result<Response<proto::HelloResponse>, Status> {
        let reply = proto::HelloResponse {
            msg: "Hello, I am Rust/Tonic Service!".to_string(),
        };

        Ok(Response::new(reply))
    }
}
