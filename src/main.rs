mod app;
mod config;
mod database;
mod entity;
mod logger;
mod server;

use anyhow::Ok;
use axum::{
    Router, debug_handler,
    extract::State,
    response::IntoResponse,
    routing::{self},
};
use entity::prelude::*;
use sea_orm::{Condition, prelude::*};
use tokio::net::TcpListener;

use crate::entity::sys_user;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    logger::init();
    let db = database::init().await?;

    let router = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(get_all_users))
        .with_state(db);

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

#[debug_handler]
async fn get_all_users(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    let users = SysUser::find()
        .filter(
            Condition::all()
                .add(sys_user::Column::Phone.eq("1"))
                .add(sys_user::Column::IsDeleted.eq(false)),
        )
        .all(&db)
        .await
        .unwrap();
    axum::Json(users)
}
