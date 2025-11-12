/**
 * Task Queue System - 任务队列系统
 * 
 * 🔥 Phase 46.14+: F-004 批量处理队列
 * 
 * 功能：
 * - 任务持久化（断点续传）
 * - 优先级管理
 * - 任务暂停/恢复/取消
 * - 状态监控
 * - 并发控制
 * 
 * @module task_queue
 */
use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::collections::{VecDeque, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
// 🔧 统一日志系统
use tracing::{info, error, debug};

/// 任务状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// 等待中
    Pending,
    /// 运行中
    Running,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
    /// 已暂停
    Paused,
}

/// 任务优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    /// 低优先级
    Low = 0,
    /// 正常优先级
    Normal = 1,
    /// 高优先级
    High = 2,
    /// 紧急
    Urgent = 3,
}

/// 转换任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionTask {
    /// 任务ID
    pub id: String,
    
    /// 输入文件路径
    pub input_path: PathBuf,
    
    /// 输出文件路径
    pub output_path: PathBuf,
    
    /// 目标格式
    pub target_format: String,
    
    /// 优化模式
    pub optimize_mode: String,
    
    /// 目标质量
    pub target_quality: Option<u8>,
    
    /// 任务状态
    pub status: TaskStatus,
    
    /// 优先级
    pub priority: TaskPriority,
    
    /// 创建时间（Unix时间戳）
    pub created_at: u64,
    
    /// 开始时间
    pub started_at: Option<u64>,
    
    /// 完成时间
    pub completed_at: Option<u64>,
    
    /// 错误信息
    pub error: Option<String>,
    
    /// 进度百分比 (0-100)
    pub progress: u8,
}

impl ConversionTask {
    /// 创建新任务
    pub fn new(
        input_path: PathBuf,
        output_path: PathBuf,
        target_format: String,
        optimize_mode: String,
    ) -> Self {
        let id = Self::generate_id(&input_path);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            id,
            input_path,
            output_path,
            target_format,
            optimize_mode,
            target_quality: None,
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            created_at: now,
            started_at: None,
            completed_at: None,
            error: None,
            progress: 0,
        }
    }
    
    /// 生成任务ID
    fn generate_id(input_path: &Path) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();
        
        let mut hasher = DefaultHasher::new();
        input_path.hash(&mut hasher);
        now.hash(&mut hasher);
        
        format!("task_{:x}", hasher.finish())
    }
    
    /// 标记任务为运行中
    pub fn mark_running(&mut self) {
        self.status = TaskStatus::Running;
        self.started_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }
    
    /// 标记任务完成
    pub fn mark_completed(&mut self) {
        self.status = TaskStatus::Completed;
        self.progress = 100;
        self.completed_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }
    
    /// 标记任务失败
    pub fn mark_failed(&mut self, error: String) {
        self.status = TaskStatus::Failed;
        self.error = Some(error);
        self.completed_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }
}

/// 任务队列配置
#[derive(Debug, Clone)]
pub struct QueueConfig {
    /// 最大并发任务数
    pub max_concurrent: usize,
    
    /// 持久化文件路径
    pub persist_path: PathBuf,
    
    /// 自动保存间隔（秒）
    pub auto_save_interval: u64,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            max_concurrent: num_cpus::get(),
            persist_path: PathBuf::from("./task_queue.json"),
            auto_save_interval: 30,
        }
    }
}

/// 任务队列
pub struct TaskQueue {
    /// 配置
    config: QueueConfig,
    
    /// 待处理任务（优先级队列）
    pending_tasks: Arc<Mutex<VecDeque<ConversionTask>>>,
    
    /// 运行中任务
    running_tasks: Arc<Mutex<HashMap<String, ConversionTask>>>,
    
    /// 已完成任务（最近100个）
    completed_tasks: Arc<Mutex<VecDeque<ConversionTask>>>,
    
    /// 是否正在运行
    is_running: Arc<Mutex<bool>>,
}

impl TaskQueue {
    /// 创建新队列
    pub fn new(config: QueueConfig) -> Self {
        Self {
            config,
            pending_tasks: Arc::new(Mutex::new(VecDeque::new())),
            running_tasks: Arc::new(Mutex::new(HashMap::new())),
            completed_tasks: Arc::new(Mutex::new(VecDeque::new())),
            is_running: Arc::new(Mutex::new(false)),
        }
    }
    
    /// 使用默认配置创建
    pub fn with_defaults() -> Self {
        Self::new(QueueConfig::default())
    }
    
    /// 添加任务
    pub fn add_task(&self, mut task: ConversionTask) -> Result<String> {
        task.status = TaskStatus::Pending;
        let task_id = task.id.clone();
        
        let mut pending = self.pending_tasks.lock().unwrap();
        
        // 按优先级插入
        let insert_pos = pending
            .iter()
            .position(|t| t.priority < task.priority)
            .unwrap_or(pending.len());
        
        pending.insert(insert_pos, task);
        
        info!("Task added to queue: {}", task_id);
        drop(pending);
        
        // 保存队列状态
        self.save_state()?;
        
        Ok(task_id)
    }
    
    /// 获取下一个待处理任务
    pub fn pop_next_task(&self) -> Option<ConversionTask> {
        let mut pending = self.pending_tasks.lock().unwrap();
        let task = pending.pop_front();
        
        if let Some(ref t) = task {
            debug!("Popped task: {}", t.id);
        }
        
        task
    }
    
    /// 标记任务为运行中
    pub fn mark_task_running(&self, mut task: ConversionTask) -> Result<()> {
        task.mark_running();
        let task_id = task.id.clone();
        
        let mut running = self.running_tasks.lock().unwrap();
        running.insert(task_id.clone(), task);
        
        info!("Task started: {}", task_id);
        Ok(())
    }
    
    /// 标记任务完成
    pub fn mark_task_completed(&self, task_id: &str) -> Result<()> {
        let mut running = self.running_tasks.lock().unwrap();
        
        if let Some(mut task) = running.remove(task_id) {
            task.mark_completed();
            
            let mut completed = self.completed_tasks.lock().unwrap();
            completed.push_back(task);
            
            // 只保留最近100个
            while completed.len() > 100 {
                completed.pop_front();
            }
            
            info!("Task completed: {}", task_id);
            drop(running);
            drop(completed);
            
            self.save_state()?;
            Ok(())
        } else {
            bail!("Task not found in running tasks: {}", task_id)
        }
    }
    
    /// 标记任务失败
    pub fn mark_task_failed(&self, task_id: &str, error: String) -> Result<()> {
        let mut running = self.running_tasks.lock().unwrap();
        
        if let Some(mut task) = running.remove(task_id) {
            task.mark_failed(error.clone());
            
            let mut completed = self.completed_tasks.lock().unwrap();
            completed.push_back(task);
            
            while completed.len() > 100 {
                completed.pop_front();
            }
            
            error!("Task failed: {} - {}", task_id, error);
            drop(running);
            drop(completed);
            
            self.save_state()?;
            Ok(())
        } else {
            bail!("Task not found in running tasks: {}", task_id)
        }
    }
    
    /// 取消任务
    pub fn cancel_task(&self, task_id: &str) -> Result<()> {
        // 先尝试从pending中移除
        let mut pending = self.pending_tasks.lock().unwrap();
        if let Some(pos) = pending.iter().position(|t| t.id == task_id) {
            pending.remove(pos);
            info!("Task cancelled from pending: {}", task_id);
            drop(pending);
            self.save_state()?;
            return Ok(());
        }
        drop(pending);
        
        // 尝试从running中取消（实际运行中的任务可能无法立即停止）
        let mut running = self.running_tasks.lock().unwrap();
        if let Some(mut task) = running.remove(task_id) {
            task.status = TaskStatus::Cancelled;
            info!("Task marked as cancelled: {}", task_id);
            drop(running);
            self.save_state()?;
            return Ok(());
        }
        
        bail!("Task not found: {}", task_id)
    }
    
    /// 获取队列统计
    pub fn get_stats(&self) -> QueueStats {
        let pending = self.pending_tasks.lock().unwrap();
        let running = self.running_tasks.lock().unwrap();
        let completed = self.completed_tasks.lock().unwrap();
        
        let failed_count = completed.iter()
            .filter(|t| t.status == TaskStatus::Failed)
            .count();
        
        let succeeded_count = completed.iter()
            .filter(|t| t.status == TaskStatus::Completed)
            .count();
        
        QueueStats {
            pending_count: pending.len(),
            running_count: running.len(),
            completed_count: succeeded_count,
            failed_count,
            total_processed: succeeded_count + failed_count,
        }
    }
    
    /// 保存队列状态到磁盘
    pub fn save_state(&self) -> Result<()> {
        let pending = self.pending_tasks.lock().unwrap();
        let running = self.running_tasks.lock().unwrap();
        let completed = self.completed_tasks.lock().unwrap();
        
        let state = QueueState {
            pending_tasks: pending.iter().cloned().collect(),
            running_tasks: running.values().cloned().collect(),
            completed_tasks: completed.iter().cloned().collect(),
        };
        
        let json = serde_json::to_string_pretty(&state)
            .context("Failed to serialize queue state")?;
        
        fs::write(&self.config.persist_path, json)
            .context("Failed to write queue state")?;
        
        debug!("Queue state saved to {:?}", self.config.persist_path);
        Ok(())
    }
    
    /// 从磁盘加载队列状态
    pub fn load_state(&self) -> Result<()> {
        if !self.config.persist_path.exists() {
            info!("No saved queue state found");
            return Ok(());
        }
        
        let json = fs::read_to_string(&self.config.persist_path)
            .context("Failed to read queue state")?;
        
        let state: QueueState = serde_json::from_str(&json)
            .context("Failed to parse queue state")?;
        
        let mut pending = self.pending_tasks.lock().unwrap();
        let _running = self.running_tasks.lock().unwrap();
        let mut completed = self.completed_tasks.lock().unwrap();
        
        // 恢复pending任务
        for task in state.pending_tasks {
            pending.push_back(task);
        }
        
        // running任务重新加入pending（因为可能程序中断）
        for task in state.running_tasks {
            let mut task = task;
            task.status = TaskStatus::Pending;
            task.started_at = None;
            pending.push_back(task);
        }
        
        // 恢复completed任务
        for task in state.completed_tasks {
            completed.push_back(task);
        }
        
        info!("Queue state loaded: {} pending, {} completed", 
              pending.len(), completed.len());
        
        Ok(())
    }
    
    /// 清空队列
    pub fn clear(&self) -> Result<()> {
        let mut pending = self.pending_tasks.lock().unwrap();
        let mut running = self.running_tasks.lock().unwrap();
        let mut completed = self.completed_tasks.lock().unwrap();
        
        pending.clear();
        running.clear();
        completed.clear();
        
        info!("Queue cleared");
        drop(pending);
        drop(running);
        drop(completed);
        
        self.save_state()?;
        Ok(())
    }
}

/// 队列状态（用于持久化）
#[derive(Debug, Serialize, Deserialize)]
struct QueueState {
    pending_tasks: Vec<ConversionTask>,
    running_tasks: Vec<ConversionTask>,
    completed_tasks: Vec<ConversionTask>,
}

/// 队列统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStats {
    /// 待处理任务数
    pub pending_count: usize,
    
    /// 运行中任务数
    pub running_count: usize,
    
    /// 已完成任务数
    pub completed_count: usize,
    
    /// 失败任务数
    pub failed_count: usize,
    
    /// 总处理数
    pub total_processed: usize,
}
