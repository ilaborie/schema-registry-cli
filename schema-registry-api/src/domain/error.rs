/// An API error
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error)]
#[error("[{error_code}] {message}")]
pub struct ApiError {
    /// The error code
    pub error_code: u32,

    /// The message
    pub message: String,
}

/// An Schema id error
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error)]
#[error("Invalid schema id, expected a number, got {0}")]
pub struct SchemaIdError(pub(crate) String);

/// An Schema version error
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error)]
#[error(
    "Valid values for versionId are between [1,2^31-1] or the string latest (or -1). But got {0}"
)]
pub struct SchemaVersionError(pub(crate) String);

/// An subject error
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, thiserror::Error)]
pub enum SubjectNameError {
    /// Empty name
    #[error("A subject could not be empty")]
    EmptyName,

    /// Invalid char
    #[error("A subject could contains control char, got {0}")]
    InvalidChar(String),
}
