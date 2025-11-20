// src/errors.rs
//! 🚨 统一错误处理模块
//! 
//! 使用thiserror提供结构化的错误类型，
//! 包含上下文信息和错误链追踪。
//!
//! ## 设计原则
//! 1. 所有错误都包含足够的上下文信息用于调试
//! 2. 使用 `#[from]` 自动转换常见错误类型
//! 3. 支持错误链追踪 (`#[source]`)
//! 4. 提供清晰的错误消息

use thiserror::Error;
use std::path::PathBuf;

/// 在线学习相关错误
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

/// 转换相关错误
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

/// IO相关错误
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

/// 统一的Result类型
pub type PixlyResult<T> = Result<T, PixlyError>;

/// 顶层错误类型
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

// 便捷的错误构造函数
impl PixlyError {
    pub fn custom(msg: impl Into<String>) -> Self {
        PixlyError::Custom(msg.into())
    }
}

/// 🔥 安全的路径转换辅助函数
/// 
/// 将Path转换为&str，如果失败则返回清晰的错误信息
/// 这个函数可以在整个项目中使用
pub fn path_to_str(path: &std::path::Path) -> anyhow::Result<&str> {
    path.to_str()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Path contains invalid UTF-8 characters: {}",
                path.display()
            )
        })
}
