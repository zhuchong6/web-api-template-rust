use axum::Router;

use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
};

mod system;
mod user;

pub fn create_router() -> Router<AppState> {
    let user_router = Router::new()
        .nest("/user", user::create_router())
        .fallback(async || -> ApiResult<()> {
            tracing::warn!("Not Found");
            Err(ApiError::NotFound)
        })
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        });
    Router::new().nest("/api", user_router)
}
