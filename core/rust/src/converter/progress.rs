/**
 * Progress Callback System - 统一进度回调系统
 * 
 * 🔥 Phase 40.24.5: 进度回调系统统一
 * 
 * 🎯 架构原则（@PROJECT_QUALITY_MANIFESTO.md）：
 * - 真实的进度报告，不是估算
 * - 统一的回调接口，不是各自实现
 * - 可取消的操作，不是强制执行
 * 
 * @module progress
 */
// 🔧 统一日志系统
use tracing::{info, debug, trace};


use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// 进度级别
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
}

/// 进度状态
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
}

/// 进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressInfo {
    /// 进度级别
    pub level: ProgressLevel,
    
    /// 进度状态
    pub state: ProgressState,
    
    /// 当前值
    pub current: u64,
    
    /// 总数
    pub total: u64,
    
    /// 消息
    pub message: String,
    
    /// 已用时间（毫秒）
    pub elapsed_ms: u64,
    
    /// 预估剩余时间（毫秒）
    pub eta_ms: u64,
    
    /// 处理速度（单位/秒）
    pub speed: f64,
    
    /// 额外数据
    pub extra: Option<serde_json::Value>,
}

impl ProgressInfo {
    /// 创建新的进度信息
    pub fn new(level: ProgressLevel, total: u64) -> Self {
        Self {
            level,
            state: ProgressState::Preparing,
            current: 0,
            total,
            message: String::new(),
            elapsed_ms: 0,
            eta_ms: 0,
            speed: 0.0,
            extra: None,
        }
    }
    
    /// 计算完成百分比
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        }
    }
    
    /// 是否完成
    pub fn is_done(&self) -> bool {
        matches!(
            self.state,
            ProgressState::Completed | ProgressState::Failed | ProgressState::Cancelled
        )
    }
    
    /// 更新进度
    pub fn update(&mut self, current: u64, message: String, elapsed: Duration) {
        self.current = current;
        self.message = message;
        self.elapsed_ms = elapsed.as_millis() as u64;
        
        // 计算速度和ETA
        let elapsed_secs = elapsed.as_secs_f64();
        if elapsed_secs > 0.0 {
            self.speed = current as f64 / elapsed_secs;
            
            let remaining = self.total.saturating_sub(current);
            if self.speed > 0.0 {
                self.eta_ms = ((remaining as f64 / self.speed) * 1000.0) as u64;
            }
        }
    }
}

/// 进度回调 Trait
pub trait ProgressCallback: Send + Sync {
    /// 报告进度
    fn on_progress(&self, info: &ProgressInfo);
    
    /// 检查是否应该取消
    fn should_cancel(&self) -> bool {
        false
    }
}

/// 默认控制台进度回调
pub struct ConsoleProgressCallback {
    verbose: bool,
}

impl ConsoleProgressCallback {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

impl ProgressCallback for ConsoleProgressCallback {
    fn on_progress(&self, info: &ProgressInfo) {
        match info.level {
            ProgressLevel::Task | ProgressLevel::Batch => {
                info!(
                    "📊 {} Progress: {}/{} ({:.1}%) | {} | Speed: {:.1}/s | ETA: {}s",
                    match info.level {
                        ProgressLevel::Task => "Task",
                        ProgressLevel::Batch => "Batch",
                        _ => "",
                    },
                    info.current,
                    info.total,
                    info.percent(),
                    info.message,
                    info.speed,
                    info.eta_ms / 1000
                );
            }
            ProgressLevel::File => {
                if self.verbose {
                    debug!("   File: {} ({:.1}%)", info.message, info.percent());
                }
            }
            ProgressLevel::Operation => {
                if self.verbose {
                    trace!("      Op: {}", info.message);
                }
            }
        }
    }
}

/// 进度追踪器
pub struct ProgressTracker {
    /// 进度信息
    info: Arc<Mutex<ProgressInfo>>,
    
    /// 回调函数列表
    callbacks: Vec<Box<dyn ProgressCallback>>,
    
    /// 取消标志
    cancelled: Arc<AtomicBool>,
    
    /// 开始时间
    start_time: Instant,
    
    /// 最后更新时间
    last_update: Arc<Mutex<Instant>>,
    
    /// 更新间隔（毫秒）
    update_interval_ms: u64,
}

impl ProgressTracker {
    /// 创建新的追踪器
    pub fn new(level: ProgressLevel, total: u64) -> Self {
        Self {
            info: Arc::new(Mutex::new(ProgressInfo::new(level, total))),
            callbacks: Vec::new(),
            cancelled: Arc::new(AtomicBool::new(false)),
            start_time: Instant::now(),
            last_update: Arc::new(Mutex::new(Instant::now())),
            update_interval_ms: 500,
        }
    }
    
    /// 添加回调
    pub fn add_callback<C: ProgressCallback + 'static>(mut self, callback: C) -> Self {
        self.callbacks.push(Box::new(callback));
        self
    }
    
    /// 设置更新间隔
    pub fn with_update_interval(mut self, interval_ms: u64) -> Self {
        self.update_interval_ms = interval_ms;
        self
    }
    
    /// 开始任务
    pub fn start(&self, message: String) {
        let mut info = self.info.lock().unwrap();
        info.state = ProgressState::Running;
        info.message = message;
        drop(info);
        
        self.notify();
    }
    
    /// 更新进度
    pub fn update(&self, current: u64, message: String) {
        let elapsed = self.start_time.elapsed();
        
        // 检查更新间隔
        {
            let mut last = self.last_update.lock().unwrap();
            if last.elapsed().as_millis() < self.update_interval_ms as u128 {
                return;
            }
            *last = Instant::now();
        }
        
        // 更新进度信息
        {
            let mut info = self.info.lock().unwrap();
            info.update(current, message, elapsed);
        }
        
        self.notify();
    }
    
    /// 增量更新
    pub fn increment(&self, message: String) {
        let current = {
            let mut info = self.info.lock().unwrap();
            info.current += 1;
            info.current
        };
        
        self.update(current, message);
    }
    
    /// 完成任务
    pub fn complete(&self, message: String) {
        let elapsed = self.start_time.elapsed();
        
        {
            let mut info = self.info.lock().unwrap();
            info.state = ProgressState::Completed;
            info.current = info.total;
            info.message = message;
            info.elapsed_ms = elapsed.as_millis() as u64;
        }
        
        self.notify();
    }
    
    /// 失败任务
    pub fn fail(&self, message: String) {
        let elapsed = self.start_time.elapsed();
        
        {
            let mut info = self.info.lock().unwrap();
            info.state = ProgressState::Failed;
            info.message = message;
            info.elapsed_ms = elapsed.as_millis() as u64;
        }
        
        self.notify();
    }
    
    /// 取消任务
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
        
        {
            let mut info = self.info.lock().unwrap();
            info.state = ProgressState::Cancelled;
            info.message = "Cancelled by user".to_string();
        }
        
        self.notify();
    }
    
    /// 检查是否应该取消
    pub fn should_cancel(&self) -> bool {
        // 检查内部取消标志
        if self.cancelled.load(Ordering::Relaxed) {
            return true;
        }
        
        // 检查回调是否要求取消
        for callback in &self.callbacks {
            if callback.should_cancel() {
                self.cancelled.store(true, Ordering::Relaxed);
                return true;
            }
        }
        
        false
    }
    
    /// 通知所有回调
    fn notify(&self) {
        let info = self.info.lock().unwrap().clone();
        for callback in &self.callbacks {
            callback.on_progress(&info);
        }
    }
    
    /// 获取当前进度信息
    pub fn get_info(&self) -> ProgressInfo {
        self.info.lock().unwrap().clone()
    }
}

/// 多级进度追踪器
pub struct MultiLevelProgressTracker {
    /// 任务级追踪器
    task_tracker: Arc<ProgressTracker>,
    
    /// 当前批次追踪器
    batch_tracker: Option<Arc<ProgressTracker>>,
    
    /// 当前文件追踪器
    file_tracker: Option<Arc<ProgressTracker>>,
}

impl MultiLevelProgressTracker {
    /// 创建新的多级追踪器
    pub fn new(total_tasks: u64) -> Self {
        let task_tracker = Arc::new(
            ProgressTracker::new(ProgressLevel::Task, total_tasks)
                .add_callback(ConsoleProgressCallback::new(false))
        );
        
        Self {
            task_tracker,
            batch_tracker: None,
            file_tracker: None,
        }
    }
    
    /// 开始新批次
    pub fn start_batch(&mut self, total_files: u64) {
        self.batch_tracker = Some(Arc::new(
            ProgressTracker::new(ProgressLevel::Batch, total_files)
                .add_callback(ConsoleProgressCallback::new(false))
        ));
    }
    
    /// 开始新文件
    pub fn start_file(&mut self, file_name: String, total_ops: u64) {
        self.file_tracker = Some(Arc::new(
            ProgressTracker::new(ProgressLevel::File, total_ops)
                .add_callback(ConsoleProgressCallback::new(true))
        ));
        
        if let Some(batch) = &self.batch_tracker {
            batch.increment(format!("Processing: {}", file_name));
        }
    }
    
    /// 完成当前文件
    pub fn complete_file(&mut self) {
        if let Some(file) = &self.file_tracker {
            file.complete("File processed".to_string());
        }
        self.file_tracker = None;
        
        self.task_tracker.increment("File completed".to_string());
    }
    
    /// 检查是否应该取消
    pub fn should_cancel(&self) -> bool {
        self.task_tracker.should_cancel()
    }
    
    /// 获取任务追踪器
    pub fn task_tracker(&self) -> &Arc<ProgressTracker> {
        &self.task_tracker
    }
    
    /// 获取批次追踪器
    pub fn batch_tracker(&self) -> Option<&Arc<ProgressTracker>> {
        self.batch_tracker.as_ref()
    }
    
    /// 获取文件追踪器
    pub fn file_tracker(&self) -> Option<&Arc<ProgressTracker>> {
        self.file_tracker.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_progress_info() {
        let mut info = ProgressInfo::new(ProgressLevel::Task, 100);
        assert_eq!(info.percent(), 0.0);
        
        info.update(50, "Half done".to_string(), Duration::from_secs(10));
        assert_eq!(info.percent(), 50.0);
        assert_eq!(info.speed, 5.0);
    }
    
    #[test]
    fn test_progress_tracker() {
        let tracker = ProgressTracker::new(ProgressLevel::Task, 10)
            .with_update_interval(0); // 设置为0以禁用间隔检查
        
        tracker.start("Starting".to_string());
        assert_eq!(tracker.get_info().state, ProgressState::Running);
        
        tracker.update(5, "Half done".to_string());
        assert_eq!(tracker.get_info().current, 5);
        
        tracker.complete("Done".to_string());
        assert_eq!(tracker.get_info().state, ProgressState::Completed);
        assert!(tracker.get_info().is_done());
    }
    
    #[test]
    fn test_cancellation() {
        let tracker = ProgressTracker::new(ProgressLevel::Task, 10);
        assert!(!tracker.should_cancel());
        
        tracker.cancel();
        assert!(tracker.should_cancel());
        assert_eq!(tracker.get_info().state, ProgressState::Cancelled);
    }
}
