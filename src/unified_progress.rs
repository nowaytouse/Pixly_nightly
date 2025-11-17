/**
 * 🚀 统一进度系统 - 恢复和增强版
 * 
 * Phase 4: 从@archive恢复进度功能，并与统一架构集成
 * 
 * 🎯 价值恢复：
 * - 统一进度回调接口
 * - 多级进度追踪 (Task -> Batch -> File -> Operation)
 * - 可取消操作支持
 * - 与unified_error_system集成
 * 
 * @module unified_progress
 * @enhanced_from @archive/progress.rs
 */
use crate::error::{PixlyError, ErrorBuilder, ErrorSeverity};
use tracing::{info, debug, trace, warn};

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 进度级别 - 恢复并增强
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProgressLevel {
    /// 任务级（整体任务）
    Task,
    /// 批次级（一批文件）
    Batch,
    /// 文件级（单个文件）
    File,
    /// 操作级（文件内的操作）
    Operation,
    /// 🆕 AI预测级（AI预测进度）
    AIPrediction,
}

/// 进度状态 - 恢复并增强
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProgressState {
    /// 准备中
    Preparing,
    /// 进行中
    Running,
    /// 暂停
    Paused,
    /// 完成
    Completed,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
    /// 🆕 等待中（队列中）
    Queued,
    /// 🆕 重试中
    Retrying,
}

/// 统一进度信息 - 增强版
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedProgressInfo {
    /// 进度级别
    pub level: ProgressLevel,
    /// 当前状态
    pub state: ProgressState,
    /// 当前进度（0-100%）
    pub progress: f64,
    /// 已完成数量
    pub completed: u64,
    /// 总数量
    pub total: u64,
    /// 当前操作描述
    pub current_operation: String,
    /// 预计剩余时间（毫秒）
    pub estimated_remaining_ms: Option<u64>,
    /// 开始时间（跳过序列化）
    #[serde(skip, default = "Instant::now")]
    pub start_time: Instant,
    /// 错误信息（如果失败）
    pub error: Option<PixlyError>,
    /// 🆕 吞吐量（items/秒）
    pub throughput: Option<f64>,
    /// 🆕 额外元数据
    pub metadata: HashMap<String, String>,
}

impl UnifiedProgressInfo {
    /// 创建新的进度信息
    pub fn new(level: ProgressLevel, total: u64, operation: String) -> Self {
        Self {
            level,
            state: ProgressState::Preparing,
            progress: 0.0,
            completed: 0,
            total,
            current_operation: operation,
            estimated_remaining_ms: None,
            start_time: Instant::now(),
            error: None,
            throughput: None,
            metadata: HashMap::new(),
        }
    }

    /// 更新进度
    pub fn update(&mut self, completed: u64) {
        self.completed = completed;
        self.progress = if self.total > 0 {
            (completed as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

        // 计算吞吐量和预计剩余时间
        let elapsed = self.start_time.elapsed();
        if elapsed.as_secs() > 0 && completed > 0 {
            self.throughput = Some(completed as f64 / elapsed.as_secs_f64());
            
            if let Some(throughput) = self.throughput {
                let remaining = self.total.saturating_sub(completed);
                if throughput > 0.0 {
                    self.estimated_remaining_ms = Some(
                        ((remaining as f64 / throughput) * 1000.0) as u64
                    );
                }
            }
        }

        if completed >= self.total {
            self.state = ProgressState::Completed;
        } else if self.state == ProgressState::Preparing {
            self.state = ProgressState::Running;
        }
    }

    /// 设置错误状态
    pub fn set_error(&mut self, error: PixlyError) {
        self.state = ProgressState::Failed;
        self.error = Some(error);
    }

    /// 取消操作
    pub fn cancel(&mut self) {
        self.state = ProgressState::Cancelled;
    }

    /// 添加元数据
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// 统一进度回调trait - 增强版
pub trait UnifiedProgressCallback: Send + Sync {
    /// 进度更新回调
    fn on_progress(&self, info: &UnifiedProgressInfo);
    
    /// 状态变更回调
    fn on_state_change(&self, old_state: &ProgressState, new_state: &ProgressState, info: &UnifiedProgressInfo);
    
    /// 错误回调
    fn on_error(&self, error: &PixlyError, info: &UnifiedProgressInfo);
    
    /// 🆕 吞吐量更新回调
    fn on_throughput_update(&self, _throughput: f64, _info: &UnifiedProgressInfo) {
        // 默认实现：空
    }
}

/// 控制台进度回调 - 增强版
pub struct EnhancedConsoleProgressCallback {
    verbose: bool,
    last_update: Arc<Mutex<Instant>>,
    update_interval: Duration,
}

impl EnhancedConsoleProgressCallback {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            last_update: Arc::new(Mutex::new(Instant::now())),
            update_interval: Duration::from_millis(100), // 100ms更新间隔
        }
    }

    fn should_update(&self) -> bool {
        let mut last = self.last_update.lock().unwrap();
        if last.elapsed() >= self.update_interval {
            *last = Instant::now();
            true
        } else {
            false
        }
    }

    fn format_duration(ms: u64) -> String {
        let seconds = ms / 1000;
        let minutes = seconds / 60;
        let hours = minutes / 60;
        
        if hours > 0 {
            format!("{}h{}m", hours, minutes % 60)
        } else if minutes > 0 {
            format!("{}m{}s", minutes, seconds % 60)
        } else {
            format!("{}s", seconds)
        }
    }
}

impl UnifiedProgressCallback for EnhancedConsoleProgressCallback {
    fn on_progress(&self, info: &UnifiedProgressInfo) {
        if !self.should_update() && info.state == ProgressState::Running {
            return;
        }

        let level_emoji = match info.level {
            ProgressLevel::Task => "🎯",
            ProgressLevel::Batch => "📦", 
            ProgressLevel::File => "📄",
            ProgressLevel::Operation => "⚙️",
            ProgressLevel::AIPrediction => "🤖",
        };

        let state_emoji = match info.state {
            ProgressState::Preparing => "🔄",
            ProgressState::Running => "▶️",
            ProgressState::Paused => "⏸️", 
            ProgressState::Completed => "✅",
            ProgressState::Failed => "❌",
            ProgressState::Cancelled => "🚫",
            ProgressState::Queued => "⏳",
            ProgressState::Retrying => "🔁",
        };

        let mut output = format!(
            "{} {} [{:>6.1}%] {}/{} - {}",
            level_emoji, state_emoji, info.progress, info.completed, info.total, info.current_operation
        );

        // 添加吞吐量信息
        if let Some(throughput) = info.throughput {
            output.push_str(&format!(" ({:.1}/s)", throughput));
        }

        // 添加预计剩余时间
        if let Some(remaining_ms) = info.estimated_remaining_ms {
            output.push_str(&format!(" ETA: {}", Self::format_duration(remaining_ms)));
        }

        if self.verbose {
            info!("{}", output);
        } else {
            println!("{}", output);
        }
    }

    fn on_state_change(&self, old_state: &ProgressState, new_state: &ProgressState, info: &UnifiedProgressInfo) {
        debug!("Progress state change: {:?} -> {:?} for {}", old_state, new_state, info.current_operation);
    }

    fn on_error(&self, error: &PixlyError, info: &UnifiedProgressInfo) {
        warn!("Progress error in {}: {} - {}", info.current_operation, error.code, error.message);
    }

    fn on_throughput_update(&self, throughput: f64, info: &UnifiedProgressInfo) {
        if self.verbose {
            trace!("Throughput: {:.2}/s for {}", throughput, info.current_operation);
        }
    }
}

/// 统一进度追踪器 - 增强版
#[derive(Clone)]
pub struct UnifiedProgressTracker {
    info: Arc<Mutex<UnifiedProgressInfo>>,
    callbacks: Arc<Mutex<Vec<Box<dyn UnifiedProgressCallback>>>>,
    is_cancelled: Arc<AtomicBool>,
    update_count: Arc<AtomicU64>,
}

impl UnifiedProgressTracker {
    /// 创建新的统一追踪器
    pub fn new(level: ProgressLevel, total: u64, operation: String) -> Self {
        Self {
            info: Arc::new(Mutex::new(UnifiedProgressInfo::new(level, total, operation))),
            callbacks: Arc::new(Mutex::new(Vec::new())),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            update_count: Arc::new(AtomicU64::new(0)),
        }
    }

    /// 添加回调
    pub fn add_callback(self, callback: Box<dyn UnifiedProgressCallback>) -> Self {
        self.callbacks.lock().unwrap().push(callback);
        self
    }

    /// 更新进度
    pub fn update(&self, completed: u64, operation: Option<String>) -> Result<(), PixlyError> {
        if self.is_cancelled.load(Ordering::Relaxed) {
            return Err(ErrorBuilder::new()
                .code("PROGRESS-001")
                .message("Operation was cancelled")
                .severity(ErrorSeverity::Warning)
                .build());
        }

        let old_state = {
            let mut info = self.info.lock().unwrap();
            let old_state = info.state.clone();
            
            if let Some(op) = operation {
                info.current_operation = op;
            }
            
            info.update(completed);
            old_state
        };

        // 触发回调
        let info = self.info.lock().unwrap().clone();
        let callbacks = self.callbacks.lock().unwrap();
        
        for callback in callbacks.iter() {
            callback.on_progress(&info);
            
            if old_state != info.state {
                callback.on_state_change(&old_state, &info.state, &info);
            }
            
            if let Some(throughput) = info.throughput {
                callback.on_throughput_update(throughput, &info);
            }
        }

        self.update_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// 设置错误
    pub fn set_error(&self, error: PixlyError) {
        let mut info = self.info.lock().unwrap();
        info.set_error(error.clone());
        
        let callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter() {
            callback.on_error(&error, &info);
        }
    }

    /// 取消操作
    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed);
        let mut info = self.info.lock().unwrap();
        info.cancel(); 
    }

    /// 检查是否被取消
    pub fn is_cancelled(&self) -> bool {
        self.is_cancelled.load(Ordering::Relaxed)
    }

    /// 获取当前进度信息
    pub fn get_info(&self) -> UnifiedProgressInfo {
        self.info.lock().unwrap().clone()
    }

    /// 添加元数据
    pub fn add_metadata(&self, key: String, value: String) {
        self.info.lock().unwrap().add_metadata(key, value);
    }

    /// 获取更新次数
    pub fn get_update_count(&self) -> u64 {
        self.update_count.load(Ordering::Relaxed)
    }
}

/// 🆕 多级统一进度管理器
pub struct UnifiedProgressManager {
    trackers: Arc<Mutex<HashMap<String, UnifiedProgressTracker>>>,
}

impl UnifiedProgressManager {
    pub fn new() -> Self {
        Self {
            trackers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 创建新的追踪器
    pub fn create_tracker(&self, id: String, level: ProgressLevel, total: u64, operation: String) -> UnifiedProgressTracker {
        let tracker = UnifiedProgressTracker::new(level, total, operation)
            .add_callback(Box::new(EnhancedConsoleProgressCallback::new(false)));
        
        self.trackers.lock().unwrap().insert(id, tracker.clone());
        tracker
    }

    /// 获取追踪器
    pub fn get_tracker(&self, id: &str) -> Option<UnifiedProgressTracker> {
        self.trackers.lock().unwrap().get(id).cloned()
    }

    /// 移除追踪器
    pub fn remove_tracker(&self, id: &str) {
        self.trackers.lock().unwrap().remove(id);
    }

    /// 取消所有追踪器
    pub fn cancel_all(&self) {
        let trackers = self.trackers.lock().unwrap();
        for tracker in trackers.values() {
            tracker.cancel();
        }
    }

    /// 获取所有活跃追踪器的状态
    pub fn get_all_status(&self) -> Vec<(String, UnifiedProgressInfo)> {
        let trackers = self.trackers.lock().unwrap();
        trackers.iter()
            .map(|(id, tracker)| (id.clone(), tracker.get_info()))
            .collect()
    }
}

impl Default for UnifiedProgressManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_unified_progress_tracker() {
        let tracker = UnifiedProgressTracker::new(
            ProgressLevel::File,
            100,
            "Test operation".to_string()
        ).add_callback(Box::new(EnhancedConsoleProgressCallback::new(true)));

        // 测试更新
        assert!(tracker.update(25, Some("Quarter done".to_string())).is_ok());
        let info = tracker.get_info();
        assert_eq!(info.completed, 25);
        assert_eq!(info.progress, 25.0);
        assert_eq!(info.state, ProgressState::Running);

        // 测试完成
        assert!(tracker.update(100, Some("Completed".to_string())).is_ok());
        let info = tracker.get_info();
        assert_eq!(info.state, ProgressState::Completed);
        
        // 测试取消
        tracker.cancel();
        assert!(tracker.is_cancelled());
    }

    #[test]
    fn test_progress_manager() {
        let manager = UnifiedProgressManager::new();
        
        let tracker = manager.create_tracker(
            "test-task".to_string(),
            ProgressLevel::Task,
            50,
            "Manager test".to_string()
        );

        // 验证追踪器被管理
        assert!(manager.get_tracker("test-task").is_some());
        
        // 测试状态获取
        let status = manager.get_all_status();
        assert_eq!(status.len(), 1);
        assert_eq!(status[0].0, "test-task");
        
        // 测试移除
        manager.remove_tracker("test-task");
        assert!(manager.get_tracker("test-task").is_none());
    }
}
