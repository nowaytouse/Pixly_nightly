//! �� 智能批量决策管理器
//! 
//! 处理批量转换中的智能决策：
//! - 损坏文件跳过策略
//! - 失败重试机制
//! - 优先级调度
//! - 资源分配优化

use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

/// 批量任务优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// 任务状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed { reason: String, retry_count: u32 },
    Skipped { reason: String },
    Retrying { retry_count: u32 }, // 新增：重试状态，保存retry_count
}

/// 批量任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTask {
    pub id: usize,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub file_size: u64,
    pub estimated_time_ms: u64,
}

/// 批量决策配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDecisionConfig {
    /// 最大重试次数
    pub max_retries: u32,
    /// 跳过损坏文件
    pub skip_corrupted: bool,
    /// 跳过过大文件（MB）
    pub skip_size_threshold_mb: Option<u64>,
    /// 启用优先级调度
    pub enable_priority_scheduling: bool,
    /// 失败后继续
    pub continue_on_error: bool,
}

impl Default for BatchDecisionConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            skip_corrupted: true,
            skip_size_threshold_mb: Some(500), // 跳过>500MB文件
            enable_priority_scheduling: true,
            continue_on_error: true,
        }
    }
}

/// 智能批量决策管理器
pub struct BatchDecisionManager {
    config: BatchDecisionConfig,
    tasks: Vec<BatchTask>,
    failed_tasks: HashMap<usize, String>,
    skipped_tasks: HashMap<usize, String>,
}

impl BatchDecisionManager {
    /// 创建新的管理器
    pub fn new(config: BatchDecisionConfig) -> Self {
        Self {
            config,
            tasks: Vec::new(),
            failed_tasks: HashMap::new(),
            skipped_tasks: HashMap::new(),
        }
    }

    /// 添加任务
    pub fn add_task(&mut self, task: BatchTask) {
        self.tasks.push(task);
    }

    /// 添加多个任务
    pub fn add_tasks(&mut self, tasks: Vec<BatchTask>) {
        self.tasks.extend(tasks);
    }

    /// 获取下一个待处理任务（考虑优先级）
    pub fn next_task(&mut self) -> Option<&mut BatchTask> {
        if self.config.enable_priority_scheduling {
            // 优先级调度：找到最高优先级的待处理任务
            self.tasks
                .iter_mut()
                .filter(|t| matches!(t.status, TaskStatus::Pending))
                .max_by_key(|t| t.priority)
        } else {
            // FIFO调度
            self.tasks
                .iter_mut()
                .find(|t| matches!(t.status, TaskStatus::Pending))
        }
    }

    /// 检查文件是否应该跳过
    pub fn should_skip(&self, task: &BatchTask) -> Option<String> {
        // 检查文件大小
        if let Some(threshold) = self.config.skip_size_threshold_mb {
            let size_mb = task.file_size / (1024 * 1024);
            if size_mb > threshold {
                return Some(format!("File too large: {}MB > {}MB", size_mb, threshold));
            }
        }

        // 检查文件是否存在
        if !task.input_path.exists() {
            return Some("File not found".to_string());
        }

        None
    }

    /// 处理任务失败
    pub fn handle_failure(&mut self, task_id: usize, error: String) -> Result<bool> {
        let task = self.tasks.iter_mut()
            .find(|t| t.id == task_id)
            .context("Task not found")?;

        // 获取当前重试次数
        let current_retry_count = match &task.status {
            TaskStatus::Failed { retry_count, .. } => *retry_count,
            TaskStatus::Retrying { retry_count } => *retry_count,
            _ => 0, // 首次失败，重试次数为0
        };

        // 检查是否应该重试
        if current_retry_count < self.config.max_retries {
            let new_retry_count = current_retry_count + 1;
            task.status = TaskStatus::Retrying { retry_count: new_retry_count };
            log::info!("🔄 Task {} will retry ({}/{})", task_id, new_retry_count, self.config.max_retries);
            Ok(true)
        } else {
            // 达到最大重试次数，标记为失败
            task.status = TaskStatus::Failed { 
                reason: error.clone(), 
                retry_count: current_retry_count 
            };
            self.failed_tasks.insert(task_id, error);
            log::error!("❌ Task {} failed after {} retries", task_id, current_retry_count);
            Ok(self.config.continue_on_error)
        }
    }

    /// 标记任务为跳过
    pub fn skip_task(&mut self, task_id: usize, reason: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = TaskStatus::Skipped { reason: reason.clone() };
            self.skipped_tasks.insert(task_id, reason);
        }
    }

    /// 获取统计信息
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

    /// 获取失败任务列表
    pub fn get_failed_tasks(&self) -> &HashMap<usize, String> {
        &self.failed_tasks
    }

    /// 获取跳过任务列表
    pub fn get_skipped_tasks(&self) -> &HashMap<usize, String> {
        &self.skipped_tasks
    }
}

/// 批量统计信息
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BatchStatistics {
    pub total: usize,
    pub pending: usize,
    pub running: usize,
    pub completed: usize,
    pub failed: usize,
    pub skipped: usize,
}

impl BatchStatistics {
    /// 计算成功率
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        self.completed as f64 / self.total as f64
    }

    /// 计算完成率（包括失败和跳过）
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
        
        // 添加不同优先级的任务
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
        
        // 应该先获取高优先级任务
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
        
        // 第一次失败 (retry_count=0) - 应该重试
        let should_continue = manager.handle_failure(1, "Test error".to_string()).unwrap();
        assert!(should_continue);
        assert_eq!(manager.failed_tasks.len(), 0); // 还未达到最大重试次数
        
        // 第二次失败 (retry_count=1) - 应该重试
        let should_continue = manager.handle_failure(1, "Test error".to_string()).unwrap();
        assert!(should_continue);
        assert_eq!(manager.failed_tasks.len(), 0); // 还未达到最大重试次数
        
        // 第三次失败 (retry_count=2) - 达到最大重试次数
        let should_continue = manager.handle_failure(1, "Test error".to_string()).unwrap();
        assert!(should_continue); // continue_on_error = true
        assert_eq!(manager.failed_tasks.len(), 1); // 现在应该加入failed_tasks
    }
}
