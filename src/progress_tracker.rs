//! 统一进度跟踪系统
//! 
//! 提供多级进度追踪和实时更新

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// 进度级别
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProgressLevel {
    Task,
    Batch,
    File,
    Operation,
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
            throughput: None,
            metadata: HashMap::new(),
        }
    }
    
    pub fn update(&mut self, completed: u64, start_time: Instant) {
        self.completed = completed;
        self.progress = if self.total > 0 {
            (completed as f64 / self.total as f64) * 100.0
        } else {
            0.0
        };
        
        let elapsed = start_time.elapsed();
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
}

/// 进度跟踪器
pub struct ProgressTracker {
    completed: Arc<AtomicU64>,
    total: u64,
    start_time: Instant,
    info: Arc<Mutex<ProgressInfo>>,
}

impl ProgressTracker {
    pub fn new(level: ProgressLevel, total: u64, operation: String) -> Self {
        Self {
            completed: Arc::new(AtomicU64::new(0)),
            total,
            start_time: Instant::now(),
            info: Arc::new(Mutex::new(ProgressInfo::new(level, total, operation))),
        }
    }
    
    pub fn increment(&self) {
        let completed = self.completed.fetch_add(1, Ordering::Relaxed) + 1;
        let mut info = self.info.lock().unwrap();
        info.update(completed, self.start_time);
    }
    
    pub fn set_completed(&self, completed: u64) {
        self.completed.store(completed, Ordering::Relaxed);
        let mut info = self.info.lock().unwrap();
        info.update(completed, self.start_time);
    }
    
    pub fn get_info(&self) -> ProgressInfo {
        self.info.lock().unwrap().clone()
    }
    
    pub fn get_progress(&self) -> f64 {
        let completed = self.completed.load(Ordering::Relaxed);
        if self.total > 0 {
            (completed as f64 / self.total as f64) * 100.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_progress_tracker() {
        let tracker = ProgressTracker::new(ProgressLevel::Task, 100, "测试任务".to_string());
        
        tracker.increment();
        assert_eq!(tracker.get_progress(), 1.0);
        
        tracker.set_completed(50);
        assert_eq!(tracker.get_progress(), 50.0);
    }
}
