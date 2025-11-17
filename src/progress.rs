// 🚀 统一进度追踪系统
// 从 @archive/rust_broken/src/converter/unified_progress.rs 提取并增强
//
// 核心功能:
// - 多级进度追踪 (Task -> Batch -> File -> Operation)
// - 可取消操作支持
// - 吞吐量计算
// - 预计剩余时间
// - 进度回调机制

use anyhow::{Result, bail};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 进度级别
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProgressLevel {
    Task,
    Batch,
    File,
    Operation,
    AIPrediction,
}

/// 进度状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProgressState {
    Preparing,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
    Queued,
    Retrying,
}

/// 进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressInfo {
    pub level: ProgressLevel,
    pub state: ProgressState,
    pub progress: f64,
    pub completed: u64,
    pub total: u64,
    pub current_operation: String,
    pub estimated_remaining_ms: Option<u64>,
    #[serde(skip, default = "Instant::now")]
    pub start_time: Instant,
    pub throughput: Option<f64>,
    pub metadata: HashMap<String, String>,
}

impl ProgressInfo {
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
            throughput: None,
            metadata: HashMap::new(),
        }
    }

    pub fn update(&mut self, completed: u64) {
        self.completed = completed;
        self.progress = if self.total > 0 {
            (completed as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };

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

    pub fn cancel(&mut self) {
        self.state = ProgressState::Cancelled;
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// 进度回调trait
pub trait ProgressCallback: Send + Sync {
    fn on_progress(&self, info: &ProgressInfo);
    fn on_state_change(&self, old_state: &ProgressState, new_state: &ProgressState, info: &ProgressInfo);
}

/// 控制台进度回调
pub struct ConsoleProgressCallback {
    verbose: bool,
    last_update: Arc<Mutex<Instant>>,
    update_interval: Duration,
}

impl ConsoleProgressCallback {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            last_update: Arc::new(Mutex::new(Instant::now())),
            update_interval: Duration::from_millis(100),
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

impl ProgressCallback for ConsoleProgressCallback {
    fn on_progress(&self, info: &ProgressInfo) {
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

        if self.verbose {
            // 详细模式：显示吞吐量和ETA
            if let Some(throughput) = info.throughput {
                output.push_str(&format!(" ({:.1}/s)", throughput));
            }

            if let Some(remaining_ms) = info.estimated_remaining_ms {
                output.push_str(&format!(" ETA: {}", Self::format_duration(remaining_ms)));
            }
        }

        println!("{}", output);
    }

    fn on_state_change(&self, _old_state: &ProgressState, _new_state: &ProgressState, _info: &ProgressInfo) {
        // 可选实现
    }
}

/// 进度追踪器
#[derive(Clone)]
pub struct ProgressTracker {
    info: Arc<Mutex<ProgressInfo>>,
    callbacks: Arc<Mutex<Vec<Box<dyn ProgressCallback>>>>,
    is_cancelled: Arc<AtomicBool>,
    update_count: Arc<AtomicU64>,
}

impl ProgressTracker {
    pub fn new(level: ProgressLevel, total: u64, operation: String) -> Self {
        Self {
            info: Arc::new(Mutex::new(ProgressInfo::new(level, total, operation))),
            callbacks: Arc::new(Mutex::new(Vec::new())),
            is_cancelled: Arc::new(AtomicBool::new(false)),
            update_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn add_callback(self, callback: Box<dyn ProgressCallback>) -> Self {
        self.callbacks.lock().unwrap().push(callback);
        self
    }

    pub fn update(&self, completed: u64, operation: Option<String>) -> Result<()> {
        if self.is_cancelled.load(Ordering::Relaxed) {
            bail!("Operation was cancelled");
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

        let info = self.info.lock().unwrap().clone();
        let callbacks = self.callbacks.lock().unwrap();
        
        for callback in callbacks.iter() {
            callback.on_progress(&info);
            
            if old_state != info.state {
                callback.on_state_change(&old_state, &info.state, &info);
            }
        }

        self.update_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed);
        let mut info = self.info.lock().unwrap();
        info.cancel(); 
    }

    pub fn is_cancelled(&self) -> bool {
        self.is_cancelled.load(Ordering::Relaxed)
    }

    pub fn get_info(&self) -> ProgressInfo {
        self.info.lock().unwrap().clone()
    }

    pub fn add_metadata(&self, key: String, value: String) {
        self.info.lock().unwrap().add_metadata(key, value);
    }
}

/// 进度管理器
pub struct ProgressManager {
    trackers: Arc<Mutex<HashMap<String, ProgressTracker>>>,
}

impl ProgressManager {
    pub fn new() -> Self {
        Self {
            trackers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_tracker(&self, id: String, level: ProgressLevel, total: u64, operation: String) -> ProgressTracker {
        let tracker = ProgressTracker::new(level, total, operation)
            .add_callback(Box::new(ConsoleProgressCallback::new(false)));
        
        self.trackers.lock().unwrap().insert(id, tracker.clone());
        tracker
    }

    pub fn get_tracker(&self, id: &str) -> Option<ProgressTracker> {
        self.trackers.lock().unwrap().get(id).cloned()
    }

    pub fn remove_tracker(&self, id: &str) {
        self.trackers.lock().unwrap().remove(id);
    }

    pub fn cancel_all(&self) {
        let trackers = self.trackers.lock().unwrap();
        for tracker in trackers.values() {
            tracker.cancel();
        }
    }

    pub fn get_all_status(&self) -> Vec<(String, ProgressInfo)> {
        let trackers = self.trackers.lock().unwrap();
        trackers.iter()
            .map(|(id, tracker)| (id.clone(), tracker.get_info()))
            .collect()
    }
}

impl Default for ProgressManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_tracker() {
        let tracker = ProgressTracker::new(
            ProgressLevel::File,
            100,
            "Test operation".to_string()
        ).add_callback(Box::new(ConsoleProgressCallback::new(false)));

        assert!(tracker.update(25, Some("Quarter done".to_string())).is_ok());
        let info = tracker.get_info();
        assert_eq!(info.completed, 25);
        assert_eq!(info.progress, 25.0);
        assert_eq!(info.state, ProgressState::Running);

        assert!(tracker.update(100, Some("Completed".to_string())).is_ok());
        let info = tracker.get_info();
        assert_eq!(info.state, ProgressState::Completed);
        
        tracker.cancel();
        assert!(tracker.is_cancelled());
    }
}
