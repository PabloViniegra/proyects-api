use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;
use validator::ValidationErrors;

/// Error response schema for OpenAPI documentation
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

/// Application-specific error types
#[derive(Error, Debug)]
pub enum AppError {
    /// Project not found error
    #[error("Project not found with id: {0}")]
    ProjectNotFound(String),

    /// Technology not found error
    #[error("Technology not found with id: {0}")]
    TechnologyNotFound(String),

    /// User not found error
    #[error("User not found with id: {0}")]
    UserNotFound(String),

    /// Duplicate resource error
    #[error("Duplicate resource: {0}")]
    DuplicateResource(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Internal server error
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        AppError::ValidationError(errors.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", error);
        AppError::DatabaseError(error.to_string())
    }
}

/// Converts AppError into an HTTP response
///
/// This implementation allows AppError to be used directly as a handler return type
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ProjectNotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::TechnologyNotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::UserNotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::DuplicateResource(msg) => (StatusCode::CONFLICT, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", msg)),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ErrorResponse {
            error: error_message,
        });

        (status, body).into_response()
    }
}

/// Type alias for Results using AppError
pub type Result<T> = std::result::Result<T, AppError>;
