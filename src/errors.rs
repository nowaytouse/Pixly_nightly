// src/errors.rs
//! Unified Error Handling Module
//!
//! Uses thiserror to provide structured error types with
//! context information and error chain tracing.
//!
//! ## Design Principles
//! 1. All errors contain sufficient context information for debugging
//! 2. Use `#[from]` for automatic conversion of common error types
//! 3. Support error chain tracing (`#[source]`)
//! 4. Provide clear error messages

use thiserror::Error;
use std::path::PathBuf;

/// Online learning related errors
#[derive(Error, Debug)]
pub enum OnlineLearningError {
    #[error("Failed to acquire lock for {operation}: {source}")]
    LockAcquisition {
        operation: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error("Experience buffer persistence failed at {path}: {source}")]
    PersistenceError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("Experience deserialization failed: {source}")]
    DeserializationError {
        #[source]
        source: serde_json::Error,
    },
    
    #[error("Model update failed after {attempts} attempts: {last_error}")]
    ModelUpdateError {
        attempts: u32,
        last_error: String,
    },
    
    #[error("Feature extraction failed for {file_path}: {reason}")]
    FeatureExtractionError {
        file_path: PathBuf,
        reason: String,
    },
    
    #[error("Python ML bridge communication failed: {details}")]
    PythonBridgeError {
        details: String,
    },
}

/// Conversion related errors
#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Input file not found: {path}")]
    InputNotFound { path: PathBuf },
    
    #[error("Output directory creation failed: {path}")]
    OutputDirCreation { path: PathBuf },
    
    #[error("Image processing failed: {operation}")]
    ImageProcessing { operation: String },
    
    #[error("Format not supported: {format} for file {path}")]
    UnsupportedFormat { format: String, path: PathBuf },
    
    #[error("Quality validation failed: expected {expected}, got {actual}")]
    QualityValidation { expected: f64, actual: f64 },
    
    #[error("Path conversion failed: {path}")]
    PathConversion { path: PathBuf },
    
    #[error("Command execution failed: {command}")]
    CommandExecution { command: String },
    
    #[error("Serialization failed: {context}")]
    Serialization { context: String },
    
    #[error("Array conversion failed: expected {expected} elements, got {actual}")]
    ArrayConversion { expected: usize, actual: usize },
}

/// IO related errors
#[derive(Error, Debug)]
pub enum IOError {
    #[error("File read failed: {path}")]
    FileRead { 
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("File write failed: {path}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("Directory creation failed: {path}")]
    DirectoryCreation {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

/// Unified Result type
pub type PixlyResult<T> = Result<T, PixlyError>;

/// Top-level error type
#[derive(Error, Debug)]
pub enum PixlyError {
    #[error(transparent)]
    OnlineLearning(#[from] OnlineLearningError),
    
    #[error(transparent)]
    Conversion(#[from] ConversionError),
    
    #[error(transparent)]
    IO(#[from] IOError),
    
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    
    #[error("{0}")]
    Custom(String),
}

// Convenience error constructors
impl PixlyError {
    pub fn custom(msg: impl Into<String>) -> Self {
        PixlyError::Custom(msg.into())
    }
}

/// Safe path conversion helper function
///
/// Converts Path to &str, returning a clear error message if it fails
/// This function can be used throughout the project
pub fn path_to_str(path: &std::path::Path) -> anyhow::Result<&str> {
    path.to_str()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Path contains invalid UTF-8 characters: {}",
                path.display()
            )
        })
}
