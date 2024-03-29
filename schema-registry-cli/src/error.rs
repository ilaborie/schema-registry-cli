use std::path::PathBuf;

use schema_registry_api::{SchemaRegistryError, SubjectNameError};

/// A schema registry CLI error
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    /// Fail during tracing initialization
    #[error("Fail to initialize tracing because {0}")]
    InitTracingError(String),

    /// Client error
    #[error(transparent)]
    ApiError(#[from] SchemaRegistryError),

    /// Subject name error
    #[error(transparent)]
    SubjectNameError(#[from] SubjectNameError),

    /// I/O error
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    /// Invalid schema extension
    #[error("Cannot determine schema extension for {0}")]
    InvalidSchemaExtension(PathBuf),
}
