mod config;
mod logger;

use axum::{
    Router, debug_handler,
    routing::{self},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    logger::init();

    let router = Router::new().route("/", routing::get(index));

    let port = config::get().server.port();

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    tracing::info!("Listening on http://localhost:{port}");

    axum::serve(listener, router).await.unwrap();
}

// debug_handler 这个宏axum会校验你的handler是不是符合要求，输出具体的错误信息
#[debug_handler]
async fn index() -> &'static str {
    "Hello, Rust!"
}
