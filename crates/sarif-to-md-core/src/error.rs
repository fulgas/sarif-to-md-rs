use thiserror::Error;

/// Main error type for report processing operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid JSON input that cannot be parsed.
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Error occurred during Markdown template generation.
    #[error("Failed to generate markdown: {0}")]
    GeneratorError(#[from] GeneratorError),

    /// I/O error when reading or writing files.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Errors that can occur during Markdown generation.
#[derive(Debug, Error)]
pub enum GeneratorError {
    /// Template rendering failed due to syntax or data issues.
    #[error("Template error: {0}")]
    TemplateError(#[from] askama::Error),
}

/// Errors that can occur during builder configuration.
#[derive(Debug, Error)]
pub enum BuilderError {
    /// Builder was used without providing required content.
    #[error("Content was not provided")]
    MissingContent,
}
