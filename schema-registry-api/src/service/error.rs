use crate::ApiError;

/// A schema registry error
#[derive(Debug, thiserror::Error)]
pub enum SchemaRegistryError {
    /// An error with reqwest
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// An error with reqwest middleware
    #[error(transparent)]
    ReqwestMiddlewareError(#[from] reqwest_middleware::Error),

    /// An error when building an URL
    #[error(transparent)]
    UrlError(#[from] url::ParseError),

    /// An API error
    #[error(transparent)]
    ApiError(#[from] ApiError),

    /// A schema registry error
    #[error("Schema registry error {0}")]
    SchemaRegistryError(String),
}
