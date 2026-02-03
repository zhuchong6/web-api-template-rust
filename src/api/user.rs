use anyhow::Context;

use axum::extract::Query;
use axum::{Router, debug_handler, extract::State, routing};
use sea_orm::{Condition, QueryOrder, QueryTrait, prelude::*};
use serde::Deserialize;

use crate::app::AppState;
use crate::common::{Page, PaginationParam};
use crate::entity::{prelude::*, sys_user};
use crate::error::ApiResult;
use crate::response::ApiResponse;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(get_all_users))
        .route("/page", routing::get(find_user_page))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    keyword: Option<String>,
    #[serde(flatten)] // 展开，平级结构
    pagination: PaginationParam,
}

#[debug_handler]
async fn find_user_page(
    State(AppState { db }): State<AppState>,
    Query(UserQueryParams {
        keyword,
        pagination,
    }): Query<UserQueryParams>,
) -> ApiResult<ApiResponse<Page<sys_user::Model>>> {
    tracing::info!("get all users");
    let paginator = SysUser::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .add(sys_user::Column::RealName.contains(keyword))
                    .add(sys_user::Column::Phone.contains(keyword)),
            )
        })
        .order_by_desc(sys_user::Column::CreateTime)
        .paginate(&db, pagination.size);

    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page - 1).await?;

    let page = Page::from_pagination(pagination, total, items);

    Ok(ApiResponse::ok("ok", Some(page)))
}

#[debug_handler]
#[tracing::instrument(name = "get_all_users", skip_all, fields(query_use = "all"))]
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
