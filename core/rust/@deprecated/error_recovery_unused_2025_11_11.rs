/**
 * Error Recovery System - 错误恢复机制
 * 
 * 🔥 Phase 40.24.4: 错误恢复机制增强
 * 
 * 🎯 架构原则（@PROJECT_QUALITY_MANIFESTO.md）：
 * - 真实的错误持久化，不是内存临时存储
 * - 响亮的错误分类，不掩盖问题
 * - 可靠的断点续传，不是简单重试
 * 
 * @module error_recovery
 */
// 🔧 统一日志系统
use tracing::{info, warn, error, debug};


use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// 错误分类
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// 文件系统错误（权限、不存在等）
    FileSystem,
    /// 格式不支持
    UnsupportedFormat,
    /// 编码器错误
    EncoderError,
    /// 解码器错误
    DecoderError,
    /// 内存不足
    OutOfMemory,
    /// AI服务错误
    AIServiceError,
    /// 网络错误
    NetworkError,
    /// 超时
    Timeout,
    /// 用户取消
    UserCancelled,
    /// 未知错误
    Unknown,
}

impl ErrorCategory {
    /// 是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ErrorCategory::NetworkError
                | ErrorCategory::Timeout
                | ErrorCategory::AIServiceError
        )
    }
    
    /// 错误严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            ErrorCategory::UserCancelled => ErrorSeverity::Info,
            ErrorCategory::UnsupportedFormat => ErrorSeverity::Warning,
            ErrorCategory::OutOfMemory => ErrorSeverity::Critical,
            ErrorCategory::FileSystem => ErrorSeverity::Error,
            _ => ErrorSeverity::Error,
        }
    }
}

/// 错误严重程度
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// 任务错误记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskError {
    /// 任务ID
    pub task_id: String,
    
    /// 文件路径
    pub file_path: PathBuf,
    
    /// 错误分类
    pub category: ErrorCategory,
    
    /// 错误消息
    pub message: String,
    
    /// 错误详情（堆栈等）
    pub details: Option<String>,
    
    /// 发生时间
    pub timestamp: u64,
    
    /// 重试次数
    pub retry_count: u32,
    
    /// 是否已恢复
    pub recovered: bool,
}

impl TaskError {
    /// 创建新的错误记录
    pub fn new(
        task_id: String,
        file_path: PathBuf,
        category: ErrorCategory,
        message: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            task_id,
            file_path,
            category,
            message,
            details: None,
            timestamp,
            retry_count: 0,
            recovered: false,
        }
    }
    
    /// 添加详情
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
    
    /// 增加重试计数
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }
    
    /// 标记为已恢复
    pub fn mark_recovered(&mut self) {
        self.recovered = true;
    }
}

/// 重试策略
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// 最大重试次数
    pub max_retries: u32,
    
    /// 重试延迟（毫秒）
    pub retry_delay_ms: u64,
    
    /// 是否使用指数退避
    pub exponential_backoff: bool,
    
    /// 最大延迟（毫秒）
    pub max_delay_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay_ms: 1000,
            exponential_backoff: true,
            max_delay_ms: 30000,
        }
    }
}

impl RetryPolicy {
    /// 计算延迟时间
    pub fn calculate_delay(&self, retry_count: u32) -> u64 {
        if self.exponential_backoff {
            let delay = self.retry_delay_ms * 2u64.pow(retry_count);
            delay.min(self.max_delay_ms)
        } else {
            self.retry_delay_ms
        }
    }
}

/// 错误恢复管理器
pub struct ErrorRecoveryManager {
    /// 错误日志文件路径
    log_path: PathBuf,
    
    /// 重试策略
    retry_policy: RetryPolicy,
    
    /// 错误记录缓存
    errors: Vec<TaskError>,
}

impl ErrorRecoveryManager {
    /// 创建新的管理器
    pub fn new(log_path: PathBuf) -> Result<Self> {
        // 确保日志目录存在
        if let Some(parent) = log_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create error log directory")?;
        }
        
        Ok(Self {
            log_path,
            retry_policy: RetryPolicy::default(),
            errors: Vec::new(),
        })
    }
    
    /// 使用自定义重试策略
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }
    
    /// 记录错误
    /// 
    /// 🔥 响亮地记录错误，不掩盖
    pub fn record_error(&mut self, error: TaskError) -> Result<()> {
        let severity = error.category.severity();
        
        // 🔥 响亮地输出到日志
        match severity {
            ErrorSeverity::Critical => {
                error!("🔴 CRITICAL ERROR: {}", error.message);
                error!("   File: {:?}", error.file_path);
                error!("   Category: {:?}", error.category);
            }
            ErrorSeverity::Error => {
                error!("❌ ERROR: {}", error.message);
                error!("   File: {:?}", error.file_path);
            }
            ErrorSeverity::Warning => {
                warn!("⚠️  WARNING: {}", error.message);
            }
            ErrorSeverity::Info => {
                info!("ℹ️  INFO: {}", error.message);
            }
        }
        
        if let Some(details) = &error.details {
            debug!("   Details: {}", details);
        }
        
        // 添加到缓存
        self.errors.push(error.clone());
        
        // 持久化到文件
        self.persist_errors()?;
        
        Ok(())
    }
    
    /// 持久化错误到文件
    fn persist_errors(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.errors)
            .context("Failed to serialize errors")?;
        
        fs::write(&self.log_path, json)
            .context("Failed to write error log")?;
        
        Ok(())
    }
    
    /// 加载错误日志
    pub fn load_errors(&mut self) -> Result<()> {
        if !self.log_path.exists() {
            info!("No existing error log found");
            return Ok(());
        }
        
        let json = fs::read_to_string(&self.log_path)
            .context("Failed to read error log")?;
        
        self.errors = serde_json::from_str(&json)
            .context("Failed to parse error log")?;
        
        info!("Loaded {} error records", self.errors.len());
        
        Ok(())
    }
    
    /// 获取可重试的错误
    pub fn get_retryable_errors(&self) -> Vec<&TaskError> {
        self.errors
            .iter()
            .filter(|e| {
                !e.recovered
                    && e.category.is_retryable()
                    && e.retry_count < self.retry_policy.max_retries
            })
            .collect()
    }
    
    /// 重试任务
    /// 
    /// 🔥 真实的重试机制，不是模拟
    pub fn retry_task<F>(&mut self, task_id: &str, mut retry_fn: F) -> Result<bool>
    where
        F: FnMut(&Path) -> Result<()>,
    {
        // 查找错误记录
        let error_idx = self
            .errors
            .iter()
            .position(|e| e.task_id == task_id && !e.recovered)
            .context("Task error not found")?;
        
        let mut error = self.errors[error_idx].clone();
        
        // 检查是否可重试
        if !error.category.is_retryable() {
            warn!("⚠️  Task {} is not retryable", task_id);
            return Ok(false);
        }
        
        if error.retry_count >= self.retry_policy.max_retries {
            error!("❌ Task {} exceeded max retries", task_id);
            return Ok(false);
        }
        
        // 计算延迟
        let delay = self.retry_policy.calculate_delay(error.retry_count);
        info!("🔄 Retrying task {} (attempt {})", task_id, error.retry_count + 1);
        info!("   Delay: {}ms", delay);
        
        std::thread::sleep(std::time::Duration::from_millis(delay));
        
        // 尝试重试
        error.increment_retry();
        
        match retry_fn(&error.file_path) {
            Ok(()) => {
                info!("✅ Task {} recovered successfully", task_id);
                error.mark_recovered();
                self.errors[error_idx] = error;
                self.persist_errors()?;
                Ok(true)
            }
            Err(e) => {
                error!("❌ Task {} retry failed: {}", task_id, e);
                self.errors[error_idx] = error;
                self.persist_errors()?;
                Ok(false)
            }
        }
    }
    
    /// 获取错误统计
    pub fn get_statistics(&self) -> ErrorStatistics {
        let total = self.errors.len();
        let recovered = self.errors.iter().filter(|e| e.recovered).count();
        let retryable = self.get_retryable_errors().len();
        
        let mut by_category = std::collections::HashMap::new();
        for error in &self.errors {
            *by_category.entry(format!("{:?}", error.category)).or_insert(0) += 1;
        }
        
        ErrorStatistics {
            total_errors: total,
            recovered_count: recovered,
            retryable_count: retryable,
            errors_by_category: by_category,
        }
    }
    
    /// 清除已恢复的错误
    pub fn clear_recovered(&mut self) -> Result<()> {
        let before = self.errors.len();
        self.errors.retain(|e| !e.recovered);
        let removed = before - self.errors.len();
        
        if removed > 0 {
            info!("🧹 Cleared {} recovered errors", removed);
            self.persist_errors()?;
        }
        
        Ok(())
    }
}

/// 错误统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStatistics {
    pub total_errors: usize,
    pub recovered_count: usize,
    pub retryable_count: usize,
    pub errors_by_category: std::collections::HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_category() {
        assert!(ErrorCategory::NetworkError.is_retryable());
        assert!(!ErrorCategory::UnsupportedFormat.is_retryable());
    }
    
    #[test]
    fn test_retry_policy() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.calculate_delay(0), 1000);
        assert_eq!(policy.calculate_delay(1), 2000);
        assert_eq!(policy.calculate_delay(2), 4000);
    }
    
    #[test]
    fn test_task_error() {
        let mut error = TaskError::new(
            "task1".to_string(),
            PathBuf::from("test.jpg"),
            ErrorCategory::NetworkError,
            "Connection failed".to_string(),
        );
        
        assert_eq!(error.retry_count, 0);
        error.increment_retry();
        assert_eq!(error.retry_count, 1);
    }
}
