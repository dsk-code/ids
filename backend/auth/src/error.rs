use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to set key")]
    KeySetFailure,
    #[error("JWT encoding error: {0}")]
    EncodeError(String),
    #[error("JWT decoding error: {0}")]
    DecodeError(String),
}
