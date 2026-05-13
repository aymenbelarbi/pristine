//! Error types for Pristine

use thiserror::Error;

/// Main error type for Pristine
#[derive(Error, Debug)]
pub enum PristineError {
    /// Source not found
    #[error("Source not found: {0}")]
    SourceNotFound(String),

    /// Invalid source format
    #[error("Invalid source format: {0}")]
    InvalidSource(String),

    /// Git operation failed
    #[error("Git error: {0}")]
    GitError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Policy violation
    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    /// Secret detected
    #[error("Secret detected in file {file}: {pattern}")]
    SecretDetected { file: String, pattern: String },

    /// Budget exceeded
    #[error("Budget exceeded: {0}")]
    BudgetExceeded(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Cache error
    #[error("Cache error: {0}")]
    CacheError(String),

    /// Compression error
    #[error("Compression error: {0}")]
    CompressionError(String),

    /// Selection error
    #[error("Selection error: {0}")]
    SelectionError(String),

    /// Render error
    #[error("Render error: {0}")]
    RenderError(String),

    /// General error
    #[error("Error: {0}")]
    General(String),
}

/// Result type alias for Pristine
pub type Result<T> = std::result::Result<T, PristineError>;

impl PristineError {
    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::SourceNotFound(_) => 3,
            Self::InvalidSource(_) => 2,
            Self::PolicyViolation(_) => 4,
            Self::SecretDetected { .. } => 5,
            Self::BudgetExceeded(_) => 6,
            Self::NetworkError(_) => 7,
            Self::GitError(_) => 8,
            _ => 1,
        }
    }
}
