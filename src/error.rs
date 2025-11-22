use std::path::PathBuf;

/// Main error type for AutoTest operations.
///
/// This enum provides detailed error reporting for different types of failures
/// that can occur during test generation, from file I/O errors to analysis failures.
#[derive(thiserror::Error, Debug)]
pub enum AutoTestError {
    #[error("Failed to read file '{path}': {source}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write file '{path}': {source}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse Rust code in '{path}': {source}")]
    ParseFailed {
        path: PathBuf,
        #[source]
        source: syn::Error,
    },

    #[error("Syntax error in '{path}' at line {line}: {message}")]
    SyntaxError {
        path: PathBuf,
        line: usize,
        message: String,
    },

    #[error("Unsupported type '{type_name}' - consider adding custom mapping")]
    UnsupportedType { type_name: String },

    #[error("Configuration file error: {source}")]
    Config {
        #[from]
        source: config::ConfigError,
    },

    #[error("I/O operation failed: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("YAML serialization error: {source}")]
    Yaml {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("Timeout exceeded: operation took too long")]
    Timeout,

    #[error("Project root not found: {path}")]
    ProjectRootNotFound { path: PathBuf },

    #[error("Invalid configuration: {message}")]
    InvalidConfig { message: String },
}

/// Result type that uses AutoTestError as the error variant.
pub type Result<T> = std::result::Result<T, AutoTestError>;
