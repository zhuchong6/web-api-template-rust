use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::response::ApiResponse;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("服务器找不到该资源")]
    NotFound,

    #[error("请求方法不支持")]
    MethodNotAllowed,

    #[error("{0}")]
    Biz(String),

    #[error("错误: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("数据库异常：{0}")]
    Database(#[from] sea_orm::DbErr),
}

impl ApiError {
    pub fn state_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Biz(_) => StatusCode::OK,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Database(_) | ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.state_code();
        let body = axum::Json(ApiResponse::<()>::err(self.to_string()));

        (status_code, body).into_response()
    }
}
