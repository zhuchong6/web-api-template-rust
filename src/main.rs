mod config;
mod database;
mod logger;

use anyhow::Ok;
use axum::{
    Router, debug_handler,
    routing::{self},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let _db = database::init().await?;

    let router = Router::new().route("/", routing::get(index));

    let port = config::get().server().port();

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    tracing::info!("Listening on http://localhost:{port}");

    axum::serve(listener, router).await?;
    Ok(())
}

// debug_handler 这个宏axum会校验你的handler是不是符合要求，输出具体的错误信息
#[debug_handler]
async fn index() -> &'static str {
    "Hello, Rust!"
}
