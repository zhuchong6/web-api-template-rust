use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::response::ApiResponse;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,

    #[error("Method Not Allowed")]
    MethodNotAllowed,

    #[error("{0}")]
    Biz(String),

    #[error("Error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ApiError {
    pub fn state_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Biz(_) => StatusCode::OK,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
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

