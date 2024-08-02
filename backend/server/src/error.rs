use ids_auth as auth;
use ids_database as db;


use auth::error;

use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    AuthError(#[from] error::AuthError),
    #[error("Not Found secrets: {0}")]
    NotFoundSecrets(String),
    #[error("Password hashing failed.")]
    PasswordHashingError,
    #[error("{0}")]
    DbError(#[from] db::error::Error),
    #[error("not found: {0}")]
    NotFound(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Error::AuthError(auth::error::AuthError::EncodeError(_)) => StatusCode::UNAUTHORIZED,
            Error::AuthError(auth::error::AuthError::DecodeError(_)) => StatusCode::UNAUTHORIZED,
            Error::AuthError(auth::error::AuthError::KeySetFailure) => StatusCode::UNAUTHORIZED,
            Error::PasswordHashingError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFoundSecrets(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DbError(db::error::Error::DatabaseError(_)) => StatusCode::CONFLICT,
            Error::DbError(db::error::Error::NotFound(_)) => StatusCode::NOT_FOUND,
            Error::DbError(db::error::Error::AlreadyExsited(_)) => StatusCode::CONFLICT,
            Error::DbError(db::error::Error::MigrationError(_)) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Error::DbError(db::error::Error::Unknown(_)) => StatusCode::NOT_FOUND,
            _ => StatusCode::NOT_FOUND,
        };

        status.into_response()
    }
}
