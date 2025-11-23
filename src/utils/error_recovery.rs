//! 🛡️ 错误恢复和检查点系统
//! 
//! 提供强大的错误恢复能力：
//! - 自动重试机制
//! - 检查点保存/恢复
//! - 失败任务跟踪
//! - 渐进式重试策略

use std::path::{Path, PathBuf};
use std::time::Duration;
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

/// 重试策略
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RetryStrategy {
    /// 不重试
    Never,
    /// 固定间隔
    Fixed {
        /// 最大重试次数
        max_retries: usize,
        /// 间隔时间（毫秒）
        interval_ms: u64,
    },
    /// 指数退避
    Exponential {
        /// 最大重试次数
        max_retries: usize,
        /// 初始延迟（毫秒）
        initial_delay_ms: u64,
        /// 退避因子
        backoff_factor: f64,
        /// 最大延迟（毫秒）
        max_delay_ms: u64,
    },
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self::Exponential {
            max_retries: 3,
            initial_delay_ms: 100,
            backoff_factor: 2.0,
            max_delay_ms: 5000,
        }
    }
}

impl RetryStrategy {
    /// 获取重试延迟
    pub fn get_delay(&self, attempt: usize) -> Option<Duration> {
        match self {
            Self::Never => None,
            Self::Fixed { max_retries, interval_ms } => {
                if attempt < *max_retries {
                    Some(Duration::from_millis(*interval_ms))
                } else {
                    None
                }
            }
            Self::Exponential { max_retries, initial_delay_ms, backoff_factor, max_delay_ms } => {
                if attempt < *max_retries {
                    let delay = (*initial_delay_ms as f64) * backoff_factor.powi(attempt as i32);
                    let clamped = delay.min(*max_delay_ms as f64) as u64;
                    Some(Duration::from_millis(clamped))
                } else {
                    None
                }
            }
        }
    }
}

/// 错误类型分类
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// 暂时性错误（可重试）
    Transient,
    /// 永久性错误（不可重试）
    Permanent,
    /// 资源错误（可能可重试）
    Resource,
    /// 未知错误
    Unknown,
}

/// 错误分类器
pub struct ErrorClassifier;

impl ErrorClassifier {
    /// 分类错误
    pub fn classify(error: &anyhow::Error) -> ErrorCategory {
        let error_str = error.to_string().to_lowercase();
        
        // 暂时性错误
        if error_str.contains("timeout") 
            || error_str.contains("connection refused")
            || error_str.contains("temporary")
            || error_str.contains("try again") {
            return ErrorCategory::Transient;
        }
        
        // 资源错误
        if error_str.contains("no space")
            || error_str.contains("out of memory")
            || error_str.contains("too many open files")
            || error_str.contains("resource")  {
            return ErrorCategory::Resource;
        }
        
        // 永久性错误
        if error_str.contains("not found")
            || error_str.contains("permission denied")
            || error_str.contains("invalid")
            || error_str.contains("unsupported") {
            return ErrorCategory::Permanent;
        }
        
        ErrorCategory::Unknown
    }
    
    /// 是否应该重试
    pub fn should_retry(error: &anyhow::Error) -> bool {
        matches!(
            Self::classify(error),
            ErrorCategory::Transient | ErrorCategory::Resource | ErrorCategory::Unknown
        )
    }
}

/// 检查点数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint<T: Serialize> {
    /// 检查点ID
    pub id: String,
    /// 创建时间戳
    pub timestamp: u64,
    /// 完成的任务ID列表
    pub completed_tasks: Vec<usize>,
    /// 失败的任务ID列表
    pub failed_tasks: Vec<usize>,
    /// 自定义数据
    pub data: T,
}

/// 检查点管理器
pub struct CheckpointManager {
    checkpoint_dir: PathBuf,
}

impl CheckpointManager {
    /// 创建新的检查点管理器
    pub fn new<P: AsRef<Path>>(checkpoint_dir: P) -> Result<Self> {
        let dir = checkpoint_dir.as_ref().to_path_buf();
        fs::create_dir_all(&dir)?;
        
        Ok(Self {
            checkpoint_dir: dir,
        })
    }
    
    /// 保存检查点
    pub fn save<T: Serialize>(&self, checkpoint: &Checkpoint<T>) -> Result<PathBuf> {
        let filename = format!("checkpoint_{}.json", checkpoint.id);
        let path = self.checkpoint_dir.join(filename);
        
        let json = serde_json::to_string_pretty(checkpoint)
            .context("Failed to serialize checkpoint")?;
        
        fs::write(&path, json)
            .context("Failed to write checkpoint")?;
        
        log::info!("💾 Checkpoint saved: {:?}", path);
        Ok(path)
    }
    
    /// 加载检查点
    pub fn load<T>(&self, id: &str) -> Result<Checkpoint<T>>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let filename = format!("checkpoint_{}.json", id);
        let path = self.checkpoint_dir.join(filename);
        
        let json = fs::read_to_string(&path)
            .context("Failed to read checkpoint")?;
        
        let checkpoint = serde_json::from_str(&json)
            .context("Failed to deserialize checkpoint")?;
        
        log::info!("📂 Checkpoint loaded: {:?}", path);
        Ok(checkpoint)
    }
    
    /// 列出所有检查点
    pub fn list_checkpoints(&self) -> Result<Vec<String>> {
        let mut checkpoints = Vec::new();
        
        for entry in fs::read_dir(&self.checkpoint_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(name) = path.file_stem()
                && let Some(name_str) = name.to_str()
                && name_str.starts_with("checkpoint_") {
                    let id = name_str.strip_prefix("checkpoint_").unwrap();
                    checkpoints.push(id.to_string());
            }
        }
        
        Ok(checkpoints)
    }
    
    /// 删除检查点
    pub fn delete(&self, id: &str) -> Result<()> {
        let filename = format!("checkpoint_{}.json", id);
        let path = self.checkpoint_dir.join(filename);
        
        fs::remove_file(&path)
            .context("Failed to delete checkpoint")?;
        
        log::info!("🗑️ Checkpoint deleted: {}", id);
        Ok(())
    }
}

/// 带重试的执行器
pub struct RetryExecutor {
    strategy: RetryStrategy,
}

impl RetryExecutor {
    /// 创建新的重试执行器
    pub fn new(strategy: RetryStrategy) -> Self {
        Self { strategy }
    }
    
    /// 执行带重试的操作
    pub fn execute<F, T, E>(&self, mut operation: F) -> Result<T>
    where
        F: FnMut() -> Result<T, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let mut attempt = 0;
        
        loop {
            match operation() {
                Ok(result) => {
                    if attempt > 0 {
                        log::info!("✅ Retry succeeded after {} attempts", attempt);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    let error = anyhow::Error::new(e);
                    
                    // 检查是否应该重试
                    if !ErrorClassifier::should_retry(&error) {
                        log::error!("❌ Permanent error, not retrying: {}", error);
                        return Err(error);
                    }
                    
                    // 获取重试延迟
                    if let Some(delay) = self.strategy.get_delay(attempt) {
                        log::warn!(
                            "⚠️ Attempt {} failed: {}. Retrying in {:?}...",
                            attempt + 1,
                            error,
                            delay
                        );
                        
                        std::thread::sleep(delay);
                        attempt += 1;
                    } else {
                        log::error!(
                            "❌ Max retries ({}) exceeded: {}",
                            attempt,
                            error
                        );
                        return Err(error);
                    }
                }
            }
        }
    }
}

/// 失败任务追踪器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedTask {
    /// 任务ID
    pub id: usize,
    /// 失败原因
    pub error: String,
    /// 错误类型
    pub error_category: ErrorCategory,
    /// 失败时间
    pub failed_at: u64,
    /// 尝试次数
    pub attempts: usize,
}

/// 恢复管理器
pub struct RecoveryManager {
    #[allow(dead_code)]
    checkpoint_manager: CheckpointManager,
    failed_tasks: Vec<FailedTask>,
}

impl RecoveryManager {
    /// 创建新的恢复管理器
    pub fn new<P: AsRef<Path>>(checkpoint_dir: P) -> Result<Self> {
        Ok(Self {
            checkpoint_manager: CheckpointManager::new(checkpoint_dir)?,
            failed_tasks: Vec::new(),
        })
    }
    
    /// 记录失败任务
    pub fn record_failure(&mut self, task_id: usize, error: &anyhow::Error, timestamp: u64) {
        let failed_task = FailedTask {
            id: task_id,
            error: error.to_string(),
            error_category: ErrorClassifier::classify(error),
            failed_at: timestamp,
            attempts: 1,
        };
        
        self.failed_tasks.push(failed_task);
    }
    
    /// 获取可重试的失败任务
    pub fn get_retryable_tasks(&self) -> Vec<&FailedTask> {
        self.failed_tasks
            .iter()
            .filter(|t| matches!(t.error_category, ErrorCategory::Transient | ErrorCategory::Unknown))
            .collect()
    }
    
    /// 获取失败统计
    pub fn get_failure_stats(&self) -> FailureStatistics {
        let total = self.failed_tasks.len();
        let permanent = self.failed_tasks.iter()
            .filter(|t| t.error_category == ErrorCategory::Permanent)
            .count();
        let transient = self.failed_tasks.iter()
            .filter(|t| t.error_category == ErrorCategory::Transient)
            .count();
        let resource = self.failed_tasks.iter()
            .filter(|t| t.error_category == ErrorCategory::Resource)
            .count();
        
        FailureStatistics {
            total_failures: total,
            permanent_failures: permanent,
            transient_failures: transient,
            resource_failures: resource,
            retryable_count: transient + resource,
        }
    }
}

/// 失败统计
#[derive(Debug, Serialize, Deserialize)]
pub struct FailureStatistics {
    pub total_failures: usize,
    pub permanent_failures: usize,
    pub transient_failures: usize,
    pub resource_failures: usize,
    pub retryable_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_retry_strategy_fixed() {
        let strategy = RetryStrategy::Fixed {
            max_retries: 3,
            interval_ms: 100,
        };
        
        assert_eq!(strategy.get_delay(0), Some(Duration::from_millis(100)));
        assert_eq!(strategy.get_delay(2), Some(Duration::from_millis(100)));
        assert_eq!(strategy.get_delay(3), None);
    }
    
    #[test]
    fn test_retry_strategy_exponential() {
        let strategy = RetryStrategy::Exponential {
            max_retries: 3,
            initial_delay_ms: 100,
            backoff_factor: 2.0,
            max_delay_ms: 1000,
        };
        
        assert_eq!(strategy.get_delay(0), Some(Duration::from_millis(100)));
        assert_eq!(strategy.get_delay(1), Some(Duration::from_millis(200)));
        assert_eq!(strategy.get_delay(2), Some(Duration::from_millis(400)));
        assert_eq!(strategy.get_delay(3), None);
    }
    
    #[test]
    fn test_error_classification() {
        let timeout_error = anyhow::anyhow!("Connection timeout");
        assert_eq!(ErrorClassifier::classify(&timeout_error), ErrorCategory::Transient);
        
        let not_found_error = anyhow::anyhow!("File not found");
        assert_eq!(ErrorClassifier::classify(&not_found_error), ErrorCategory::Permanent);
        
        let resource_error = anyhow::anyhow!("Out of memory");
        assert_eq!(ErrorClassifier::classify(&resource_error), ErrorCategory::Resource);
    }
    
    #[test]
    fn test_should_retry() {
        assert!(ErrorClassifier::should_retry(&anyhow::anyhow!("Timeout")));
        assert!(!ErrorClassifier::should_retry(&anyhow::anyhow!("Invalid format")));
    }
    
    #[test]
    fn test_checkpoint_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CheckpointManager::new(temp_dir.path()).unwrap();
        
        let checkpoint = Checkpoint {
            id: "test123".to_string(),
            timestamp: 1000,
            completed_tasks: vec![1, 2, 3],
            failed_tasks: vec![4],
            data: "custom data".to_string(),
        };
        
        manager.save(&checkpoint).unwrap();
        let loaded: Checkpoint<String> = manager.load("test123").unwrap();
        
        assert_eq!(loaded.id, "test123");
        assert_eq!(loaded.completed_tasks, vec![1, 2, 3]);
        assert_eq!(loaded.data, "custom data");
    }
    
    #[test]
    fn test_retry_executor() {
        let executor = RetryExecutor::new(RetryStrategy::Fixed {
            max_retries: 3,
            interval_ms: 1,
        });
        
        let mut attempts = 0;
        let result = executor.execute(|| {
            attempts += 1;
            if attempts < 3 {
                Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Timeout"))
            } else {
                Ok(42)
            }
        });
        
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts, 3);
    }
}
