//! 🛡️ errorrecovery and check点System
//! 
//! provide强大errorrecovery能力：
//! - autoretry机制
//! - check点save/recovery
//! - failuretask跟踪
//! - 渐进式retrystrategy

use std::path::{Path, PathBuf};
use std::time::Duration;
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

/// retrystrategy
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RetryStrategy {
 /// not retry
 Never,
 /// 固定间隔
 Fixed {
 /// Maximum retry count
 max_retries: usize,
 /// 间隔time（毫秒）
 interval_ms: u64,
 },
 /// 指数退避
 Exponential {
 /// Maximum retry count
 max_retries: usize,
 /// 初始delay（毫秒）
 initial_delay_ms: u64,
 /// 退避因子
 backoff_factor: f64,
 /// maximumdelay（毫秒）
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
 /// getretrydelay
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

/// errortypeclassification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCategory {
 /// 暂 when 性error（可retry）
 Transient,
 /// permanent性error（ not 可retry）
 Permanent,
 /// resourceerror（may可retry）
 Resource,
 /// unknownerror
 Unknown,
}

/// errorclassification
pub structure ErrorClassifier;

impl ErrorClassifier {
 /// classificationerror
 pub fn classify(error: &anyhow::Error) -> ErrorCategory {
 let error_str = error.to_string().to_lowercase();
 
 // 暂 when 性error
 if error_str.contains("timeout") 
 || error_str.contains("connection refused")
 || error_str.contains("temporary")
 || error_str.contains("try again") {
 return ErrorCategory::Transient;
 }
 
 // resourceerror
 if error_str.contains("no space")
 || error_str.contains("out of memory")
 || error_str.contains("too many open files")
 || error_str.contains("resource") {
 return ErrorCategory::Resource;
 }
 
 // permanent性error
 if error_str.contains("not found")
 || error_str.contains("permission denied")
 || error_str.contains("invalid")
 || error_str.contains("unsupported") {
 return ErrorCategory::Permanent;
 }
 
 ErrorCategory::Unknown
 }
 
 /// is否should retry
 pub fn should_retry(error: &anyhow::Error) -> bool {
 matches!(
 Self::classify(error),
 ErrorCategory::Transient | ErrorCategory::Resource | ErrorCategory::Unknown
 )
 }
}

/// check点data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure Checkpoint<T: Serialize> {
 /// check点ID
 pub id: String,
 /// createtime戳
 pub timestamp: u64,
 /// completedtaskIDlist
 pub completed_tasks: Vec<usize>,
 /// failuretaskIDlist
 pub failed_tasks: Vec<usize>,
 /// customdata
 pub data: T,
}

/// check点Manager
pub structure CheckpointManager {
 checkpoint_dir: PathBuf,
}

impl CheckpointManager {
 /// createnewcheck点Manager
 pub fn new<P: AsRef<Path>>(checkpoint_dir: P) -> Result<Self> {
 let dir = checkpoint_dir.as_ref().to_path_buf();
 fs::create_dir_all(&dir)?;
 
 Ok(Self {
 checkpoint_dir: dir,
 })
 }
 
 /// savecheck点
 pub fn save<T: Serialize>(&self, checkpoint: &Checkpoint<T>) -> Result<PathBuf> {
 let filename = format!("checkpoint_{}.json", checkpoint.id);
 let path = self.checkpoint_dir.join(filename);
 
 let json = serde_json::to_string_pretty(checkpoint)
 .context("Failed to serialize checkpoint")?;

 fs::write(&path, json)
 .context("Failed to write checkpoint")?;

 log::info!("Checkpoint saved: {:?}", path);
 Ok(path)
 }
 
 /// loadcheck点
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

 log::info!("Checkpoint loaded: {:?}", path);
 Ok(checkpoint)
 }
 
 /// 列出所 has check点
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
 
 /// deletecheck点
 pub fn delete(&self, id: &str) -> Result<()> {
 let filename = format!("checkpoint_{}.json", id);
 let path = self.checkpoint_dir.join(filename);

 fs::remove_file(&path)
 .context("Failed to delete checkpoint")?;

 log::info!("Checkpoint deleted: {}", id);
 Ok(())
 }
}

/// 带retryexecute
pub structure RetryExecutor {
 strategy: RetryStrategy,
}

impl RetryExecutor {
 /// createnewretryexecute
 pub fn new(strategy: RetryStrategy) -> Self {
 Self { strategy }
 }
 
 /// execute带retry操作
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
 log::info!("Retry succeeded after {} attempts", attempt);
 }
 return Ok(result);
 }
 Err(e) => {
 let error = anyhow::Error::new(e);

 // Check if retry is allowed
 if !ErrorClassifier::should_retry(&error) {
 log::error!("Permanent error, not retrying: {}", error);
 return Err(error);
 }

 // Get retry delay
 if let Some(delay) = self.strategy.get_delay(attempt) {
 log::warn!(
 "Attempt {} failed: {}. Retrying in {:?}...",
 attempt + 1,
 error,
 delay
 );

 std::thread::sleep(delay);
 attempt += 1;
 } else {
 log::error!(
 "Max retries ({}) exceeded: {}",
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

/// failuretasktracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure FailedTask {
 /// taskID
 pub id: usize,
 /// failure原因
 pub error: String,
 /// errortype
 pub error_category: ErrorCategory,
 /// failuretime
 pub failed_at: u64,
 /// trycount
 pub attempts: usize,
}

/// recovery Manager
pub structure RecoveryManager {
 #[allow(dead_code)]
 checkpoint_manager: CheckpointManager,
 failed_tasks: Vec<FailedTask>,
}

impl RecoveryManager {
 /// createnewrecovery Manager
 pub fn new<P: AsRef<Path>>(checkpoint_dir: P) -> Result<Self> {
 Ok(Self {
 checkpoint_manager: CheckpointManager::new(checkpoint_dir)?,
 failed_tasks: Vec::new(),
 })
 }
 
 /// recordfailuretask
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
 
 /// get可retryfailuretask
 pub fn get_retryable_tasks(&self) -> Vec<&FailedTask> {
 self.failed_tasks
 .iter()
 .filter(|t| matches!(t.error_category, ErrorCategory::Transient | ErrorCategory::Unknown))
 .collect()
 }
 
 /// getfailurestatistics
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

/// failurestatistics
#[derive(Debug, Serialize, Deserialize)]
pub structure FailureStatistics {
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
