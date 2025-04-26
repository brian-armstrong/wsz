//! Error types for the wsz library

use thiserror::Error;

/// Errors that can occur during WSZ file operations
#[derive(Error, Debug)]
pub enum WszError {
    /// IO errors that occur when reading files
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// ZIP format errors
    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Not found error
    #[error("Not found: {0}")]
    NotFound(String),

    /// Image processing errors
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),

    /// Argument errors
    #[error("Argument error: {0}")]
    ArgumentError(String),

    /// Invalid format in config file
    #[error("Invalid format on line {line}: {error}")]
    InvalidFormat { line: usize, error: String },

    /// Missing section in the file
    #[error("Missing section: {0}")]
    MissingSection(String),
}

/// Result type for WSZ operations
pub type Result<T> = std::result::Result<T, WszError>;
