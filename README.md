# dist-tracing-rs

Simple dockerized Rust/actix/tonic based demo.

## Run the backend

From the workspace root directory:

```
RUSTLOG=dist_tracing_backend=debug,info \
    CONFIG_DIR=dist-tracing-backend/config \
    APP__API__PORT=8090 \
    cargo run -p dist-tracing-backend \
    | jq
```

## Run the gateway

From the workspace root directory:

```
RUSTLOG=dist_tracing_gateway=debug,info \
    CONFIG_DIR=dist-tracing-gateway/config \
    APP__API__PORT=8080 \
    APP__BACKEND__ENDPOINT=http://localhost:8090 \
    cargo run -p dist-tracing-gateway \
    | jq
```

## License

This code is open source software licensed under [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0.html).
