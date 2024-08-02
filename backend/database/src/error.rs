use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to migrate: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[error("failed in database: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("already existed: {0}")]
    AlreadyExsited(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unknown: {0}")]
    Unknown(String),
}
