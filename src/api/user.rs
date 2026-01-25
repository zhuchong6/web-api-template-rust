use anyhow::Context;

use axum::{Router, debug_handler, extract::State, routing};
use sea_orm::{Condition, prelude::*};

use crate::app::AppState;
use crate::entity::{prelude::*, sys_user};
use crate::error::ApiResult;
use crate::response::ApiResponse;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", routing::get(get_all_users))
}

#[debug_handler]
#[tracing::instrument(name = "get_all_users", skip_all, fields(query_use= "all"))]
async fn get_all_users(
    State(AppState { db }): State<AppState>,
) -> ApiResult<ApiResponse<Vec<sys_user::Model>>> {
    tracing::info!("get all users");
    let users = SysUser::find()
        .filter(
            Condition::all()
                .add(sys_user::Column::Phone.eq("1"))
                .add(sys_user::Column::IsDeleted.eq(false)),
        )
        .all(&db)
        .await
        .context("Failed to query users")?;

    Ok(ApiResponse::ok("ok", Some(users)))
}
