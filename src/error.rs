use anyhow::Error as AnyhowError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("resource not found")]
    NotFound,
    #[error("validation error: {0}")]
    Validation(String),
    #[error("database error: {0}")]
    Database(#[from] DbErr),
    #[error("internal server error")]
    Unexpected(#[from] AnyhowError),
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
    message: &'a str,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::NotFound => "resource not found",
            Self::Validation(msg) => msg,
            Self::Database(_) => "database error",
            Self::Unexpected(_) => "internal server error",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = ErrorBody {
            error: match &self {
                AppError::NotFound => "not_found",
                AppError::Validation(_) => "validation_error",
                AppError::Database(_) => "database_error",
                AppError::Unexpected(_) => "unexpected_error",
            },
            message: self.message(),
        };
        (status, Json(body)).into_response()
    }
}
