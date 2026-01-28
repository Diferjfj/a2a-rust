//! Client-specific error types

use crate::a2a::error::A2AError;
use crate::InternalError;

/// Client-specific errors
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Transport error: {0}")]
    Transport(String),
    #[error("Authentication error: {0}")]
    Authentication(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
}

impl From<ClientError> for A2AError {
    fn from(err: ClientError) -> Self {
        A2AError::Internal(InternalError {
            code: -32603,
            message: err.to_string(),
            data: None,
        })
    }
}
