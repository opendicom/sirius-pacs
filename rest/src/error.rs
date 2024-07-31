use axum::{
    http::StatusCode, 
    response::IntoResponse,
    Json
};
use diesel_async::pooled_connection::deadpool::PoolError;
use serde_json::json;
use thiserror::Error;
use tracing::error;

/// Custom application errors
#[derive(Debug,Error)]
pub enum AppError {
    #[error("Internal Server Error")]
    _InternalServerError,

    #[error("Database pool error: {0}")]
    DatabasePoolError(#[from]PoolError),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("I/O Error: {0}")]
    IoError(#[from]std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("{self}");
        let (status, err_msg) = match self {
            AppError::_InternalServerError |
            AppError::DatabasePoolError(_) |
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
            ),
            AppError::IoError(err) => (
                StatusCode::NOT_FOUND,
                err.to_string(),
            ),
        };
        
        // Create a JSON response containing the error message
        (status, Json(json!({ "message": err_msg }))).into_response()
    }
}


// region: -- Utilities ------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------

/// Utility function for custom error on unhandled routes
pub async fn not_found_error() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}


/// Utility function for mapping an error into a [`AppError::DatabaseError(err)`]
/// response.
pub fn database_error<E>(err: E) -> AppError
where
    E: std::error::Error,
{
    AppError::DatabaseError(err.to_string())
}

// endregion: -- Utilities ------------------------------------------------------------------------------
