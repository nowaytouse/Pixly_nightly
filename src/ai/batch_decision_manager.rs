//! �� Intelligent Batch Decision Manager
//! 
//! Processing batch conversion intelligent decisions：
//! - Corrupted file skip strategy
//! - Failure retry mechanism
//! - Priority scheduling
//! - Resource allocation optimization

use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

/// Batch task priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
 Low = 0,
 Normal = 1,
 High = 2,
 Critical = 3,
}

/// taskstatus
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
 Pending,
 Running,
 Completed,
 Failed { reason: String, retry_count: u32 },
 Skipped { reason: String },
 Retrying { retry_count: u32 }, // new增：重试状态，saveretry_count
}

/// batchtask
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure BatchTask {
 pub id: usize,
 pub input_path: PathBuf,
 pub output_path: PathBuf,
 pub priority: TaskPriority,
 pub status: TaskStatus,
 pub file_size: u64,
 pub estimated_time_ms: u64,
}

/// Batch decision configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub structure BatchDecisionConfig {
 /// Maximum retry count
 pub max_retries: u32,
 /// Skip corrupted files
 pub skip_corrupted: bool,
 /// Skip oversized files（MB）
 pub skip_size_threshold_mb: Option<u64>,
 /// Enable priority scheduling
 pub enable_priority_scheduling: bool,
 /// Continue after failure
 pub continue_on_error: bool,
}

impl Default for BatchDecisionConfig {
 fn default() -> Self {
 Self {
 max_retries: 3,
 skip_corrupted: true,
 skip_size_threshold_mb: Some(500), // 跳过>500MBfile
 enable_priority_scheduling: true,
 continue_on_error: true,
 }
 }
}

/// Intelligent Batch Decision Manager
pub structure BatchDecisionManager {
 config: BatchDecisionConfig,
 tasks: Vec<BatchTask>,
 failed_tasks: HashMap<usize, String>,
 skipped_tasks: HashMap<usize, String>,
}

impl BatchDecisionManager {
 /// Create new manager
 pub fn new(config: BatchDecisionConfig) -> Self {
 Self {
 config,
 tasks: Vec::new(),
 failed_tasks: HashMap::new(),
 skipped_tasks: HashMap::new(),
 }
 }

 /// addtask
 pub fn add_task(&mut self, task: BatchTask) {
 self.tasks.push(task);
 }

 /// Add multiple tasks
 pub fn add_tasks(&mut self, tasks: Vec<BatchTask>) {
 self.tasks.extend(tasks);
 }

 /// Get next pending task（considering priority）
 pub fn next_task(&mut self) -> Option<&mut BatchTask> {
 if self.config.enable_priority_scheduling {
 // Priority scheduling：Find highest priority pending task
 self.tasks
 .iter_mut()
 .filter(|t| matches!(t.status, TaskStatus::Pending))
 .max_by_key(|t| t.priority)
 } else {
 // FIFO scheduling
 self.tasks
 .iter_mut()
 .find(|t| matches!(t.status, TaskStatus::Pending))
 }
 }

 /// Check if file should be skipped
 pub fn should_skip(&self, task: &BatchTask) -> Option<String> {
 // Check file size
 if let Some(threshold) = self.config.skip_size_threshold_mb {
 let size_mb = task.file_size / (1024 * 1024);
 if size_mb > threshold {
 return Some(format!("File too large: {}MB > {}MB", size_mb, threshold));
 }
 }

 // Check if file exists
 if !task.input_path.exists() {
 return Some("File not found".to_string());
 }

 None
 }

 /// processingtaskfailure
 pub fn handle_failure(&mut self, task_id: usize, error: String) -> Result<bool> {
 let task = self.tasks.iter_mut()
 .find(|t| t.id == task_id)
 .context("Task not found")?;

 // Get current retry count
 let current_retry_count = match &task.status {
 TaskStatus::Failed { retry_count, .. } => *retry_count,
 TaskStatus::Retrying { retry_count } => *retry_count,
 _ => 0, // 首次失败，重试次数for0
 };

 // Check if retry is needed
 if current_retry_count < self.config.max_retries {
 let new_retry_count = current_retry_count + 1;
 task.status = TaskStatus::Retrying { retry_count: new_retry_count };
 log::info!("Task {} will retry ({}/{})", task_id, new_retry_count, self.config.max_retries);
 Ok(true)
 } else {
 // Max retries reached, mark as failed
 task.status = TaskStatus::Failed {
 reason: error.clone(),
 retry_count: current_retry_count
 };
 self.failed_tasks.insert(task_id, error);
 log::error!("Task {} failed after {} retries", task_id, current_retry_count);
 Ok(self.config.continue_on_error)
 }
 }

 /// Mark task as skipped
 pub fn skip_task(&mut self, task_id: usize, reason: String) {
 if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
 task.status = TaskStatus::Skipped { reason: reason.clone() };
 self.skipped_tasks.insert(task_id, reason);
 }
 }

 /// getstatisticsinformation
 pub fn get_statistics(&self) -> BatchStatistics {
 let mut stats = BatchStatistics::default();
 
 for task in &self.tasks {
 match &task.status {
 TaskStatus::Pending => stats.pending += 1,
 TaskStatus::Running => stats.running += 1,
 TaskStatus::Completed => stats.completed += 1,
 TaskStatus::Failed { .. } => stats.failed += 1,
 TaskStatus::Skipped { .. } => stats.skipped += 1,
 TaskStatus::Retrying { .. } => stats.pending += 1, // 重试状态算作pending
 }
 }
 
 stats.total = self.tasks.len();
 stats
 }

 /// getfailuretasklist
 pub fn get_failed_tasks(&self) -> &HashMap<usize, String> {
 &self.failed_tasks
 }

 /// getskiptasklist
 pub fn get_skipped_tasks(&self) -> &HashMap<usize, String> {
 &self.skipped_tasks
 }
}

/// batchstatisticsinformation
#[derive(Debug, Default, Serialize, Deserialize)]
pub structure BatchStatistics {
 pub total: usize,
 pub pending: usize,
 pub running: usize,
 pub completed: usize,
 pub failed: usize,
 pub skipped: usize,
}

impl BatchStatistics {
 /// Calculate success rate
 pub fn success_rate(&self) -> f64 {
 if self.total == 0 {
 return 0.0;
 }
 self.completed as f64 / self.total as f64
 }

 /// Calculate completion rate（including failed and skipped）
 pub fn completion_rate(&self) -> f64 {
 if self.total == 0 {
 return 0.0;
 }
 (self.completed + self.failed + self.skipped) as f64 / self.total as f64
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_priority_scheduling() {
 let mut manager = BatchDecisionManager::new(BatchDecisionConfig::default());
 
 // Add tasks with different priorities
 manager.add_task(BatchTask {
 id: 1,
 input_path: PathBuf::from("test1.jpg"),
 output_path: PathBuf::from("test1.webp"),
 priority: TaskPriority::Low,
 status: TaskStatus::Pending,
 file_size: 1000,
 estimated_time_ms: 100,
 });
 
 manager.add_task(BatchTask {
 id: 2,
 input_path: PathBuf::from("test2.jpg"),
 output_path: PathBuf::from("test2.webp"),
 priority: TaskPriority::High,
 status: TaskStatus::Pending,
 file_size: 1000,
 estimated_time_ms: 100,
 });
 
 // Should get high priority task first
 let next = manager.next_task().unwrap();
 assert_eq!(next.id, 2);
 assert_eq!(next.priority, TaskPriority::High);
 }

 #[test]
 fn test_retry_mechanism() {
 let mut manager = BatchDecisionManager::new(BatchDecisionConfig {
 max_retries: 2,
 ..Default::default()
 });
 
 manager.add_task(BatchTask {
 id: 1,
 input_path: PathBuf::from("test.jpg"),
 output_path: PathBuf::from("test.webp"),
 priority: TaskPriority::Normal,
 status: TaskStatus::Pending,
 file_size: 1000,
 estimated_time_ms: 100,
 });
 
 // First failure (retry_count=0) - should retry
 let should_continue = manager.handle_failure(1, "Test error".to_string()).unwrap();
 assert!(should_continue);
 assert_eq!(manager.failed_tasks.len(), 0); // 还未达tomaximum重试次数
 
 // Second failure (retry_count=1) - should retry
 let should_continue = manager.handle_failure(1, "Test error".to_string()).unwrap();
 assert!(should_continue);
 assert_eq!(manager.failed_tasks.len(), 0); // 还未达tomaximum重试次数
 
 // Third failure (retry_count=2) - Reached maximum retry count
 let should_continue = manager.handle_failure(1, "Test error".to_string()).unwrap();
 assert!(should_continue); // continue_on_error = true
 assert_eq!(manager.failed_tasks.len(), 1); // 现应该加入failed_tasks
 }
}
