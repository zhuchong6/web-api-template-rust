mod app;
mod config;
mod database;
mod entity;
mod logger;
mod server;

use axum::{
    Router, debug_handler,
    extract::State,
    response::IntoResponse,
    routing::{self},
};
use entity::prelude::*;
use sea_orm::{Condition, prelude::*};

use crate::{app::AppState, entity::sys_user};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(get_all_users));

    app::run(router).await
}

// debug_handler 这个宏axum会校验你的handler是不是符合要求，输出具体的错误信息
#[debug_handler]
async fn index() -> &'static str {
    "Hello, Rust!"
}

#[debug_handler]
async fn get_all_users(State(AppState { db }): State<AppState>) -> impl IntoResponse {
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
