use thiserror::Error;

#[derive(Debug, Error)]
pub enum PermifyError {
    #[error("Request failed: {0}")]
    RequestError(String),
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
}
