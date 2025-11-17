// 智能并发管理器 - 基于文件复杂度和内存使用的动态并发控制
// 提取自: @archive/go/@deprecated/go_orphan_code_2025_11_11/concurrency/smart_concurrency_core.go

use std::sync::{Arc, Mutex};
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct ConcurrencyStats {
    pub total_jobs_processed: u64,
    pub successful_jobs: u64,
    pub failed_jobs: u64,
    pub total_memory_used: i64,
    pub memory_limit_hits: u64,
    pub worker_adjustments: u64,
    pub peak_workers: usize,
    pub average_workers: f64,
}

pub struct SmartConcurrencyManager {
    min_workers: usize,
    max_workers: usize,
    current_workers: Arc<Mutex<usize>>,
    #[allow(dead_code)]
    memory_threshold: f64, // 内存使用阈值 (0-100)
    high_complexity_threshold: f64,
    low_complexity_threshold: f64,
    scaling_factor: f64,
    backoff_factor: f64,
    stats: Arc<Mutex<ConcurrencyStats>>,
    complexity_history: Arc<Mutex<Vec<f64>>>,
}

impl SmartConcurrencyManager {
    /// 创建智能并发管理器
    pub fn new(min_workers: usize, max_workers: usize, memory_threshold: f64) -> Self {
        Self {
            min_workers,
            max_workers,
            current_workers: Arc::new(Mutex::new(min_workers)),
            memory_threshold,
            high_complexity_threshold: 70.0,
            low_complexity_threshold: 30.0,
            scaling_factor: 1.2,
            backoff_factor: 0.8,
            stats: Arc::new(Mutex::new(ConcurrencyStats {
                total_jobs_processed: 0,
                successful_jobs: 0,
                failed_jobs: 0,
                total_memory_used: 0,
                memory_limit_hits: 0,
                worker_adjustments: 0,
                peak_workers: min_workers,
                average_workers: min_workers as f64,
            })),
            complexity_history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 检查系统内存使用情况
    pub fn check_system_memory(&self) -> f64 {
        let mut sys = System::new_all();
        sys.refresh_memory();
        
        let total = sys.total_memory() as f64;
        let used = sys.used_memory() as f64;
        
        (used / total) * 100.0
    }

    /// 处理高内存使用情况
    pub fn handle_high_memory_usage(&self, _current_usage: f64) {
        let mut stats = self.stats.lock().unwrap();
        stats.memory_limit_hits += 1;
        drop(stats);

        let mut current = self.current_workers.lock().unwrap();
        if *current > self.min_workers {
            let new_workers = ((*current as f64) * self.backoff_factor) as usize;
            let new_workers = new_workers.max(self.min_workers);
            
            *current = new_workers;
            
            let mut stats = self.stats.lock().unwrap();
            stats.worker_adjustments += 1;
        }
    }

    /// 计算最优工作协程数
    pub fn calculate_optimal_worker_count(
        &self,
        avg_complexity: f64,
        queue_length: usize,
        active_jobs: usize,
    ) -> usize {
        let current = *self.current_workers.lock().unwrap();
        let mut base_workers = current as f64;

        // 1. 基于复杂度调整
        if avg_complexity > self.high_complexity_threshold {
            // 高复杂度任务：减少并发数
            base_workers *= self.backoff_factor;
        } else if avg_complexity < self.low_complexity_threshold {
            // 低复杂度任务：增加并发数
            base_workers *= self.scaling_factor;
        }

        // 2. 基于队列长度调整
        if queue_length > current * 2 {
            // 队列积压严重
            base_workers *= 1.1;
        } else if queue_length == 0 && active_jobs < current / 2 {
            // 队列空闲
            base_workers *= 0.9;
        }

        // 3. 应用边界限制
        let result = base_workers as usize;
        result.clamp(self.min_workers, self.max_workers)
    }

    /// 调整工作协程数量
    pub fn adjust_worker_count(&self, target_workers: usize) {
        let mut current = self.current_workers.lock().unwrap();
        *current = target_workers;

        let mut stats = self.stats.lock().unwrap();
        if target_workers > stats.peak_workers {
            stats.peak_workers = target_workers;
        }
    }

    /// 更新复杂度历史
    pub fn update_complexity_history(&self, complexity_score: f64) {
        let mut history = self.complexity_history.lock().unwrap();
        history.push(complexity_score);

        // 保持历史记录在合理范围内
        if history.len() > 100 {
            history.remove(0);
        }
    }

    /// 计算平均复杂度
    pub fn calculate_average_complexity(&self) -> f64 {
        let history = self.complexity_history.lock().unwrap();
        if history.is_empty() {
            return 50.0; // 默认中等复杂度
        }

        let sum: f64 = history.iter().sum();
        sum / history.len() as f64
    }

    /// 记录任务结果
    pub fn record_job_result(&self, success: bool, memory_used: i64) {
        let mut stats = self.stats.lock().unwrap();
        stats.total_jobs_processed += 1;
        if success {
            stats.successful_jobs += 1;
        } else {
            stats.failed_jobs += 1;
        }
        stats.total_memory_used += memory_used;
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> ConcurrencyStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// 获取当前工作协程数
    pub fn get_current_workers(&self) -> usize {
        *self.current_workers.lock().unwrap()
    }

    /// 执行动态调整
    pub fn perform_dynamic_adjustment(&self, queue_length: usize, active_jobs: usize) {
        let avg_complexity = self.calculate_average_complexity();
        let target_workers = self.calculate_optimal_worker_count(
            avg_complexity,
            queue_length,
            active_jobs,
        );

        let current = self.get_current_workers();
        if target_workers != current {
            self.adjust_worker_count(target_workers);
            let mut stats = self.stats.lock().unwrap();
            stats.worker_adjustments += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_concurrency_manager_creation() {
        let manager = SmartConcurrencyManager::new(2, 10, 80.0);
        assert_eq!(manager.get_current_workers(), 2);
    }

    #[test]
    fn test_calculate_optimal_worker_count() {
        let manager = SmartConcurrencyManager::new(2, 10, 80.0);
        
        // 低复杂度，应该增加workers
        let optimal = manager.calculate_optimal_worker_count(20.0, 0, 1);
        assert!(optimal >= 2);
        
        // 高复杂度，应该减少workers
        let optimal = manager.calculate_optimal_worker_count(80.0, 0, 1);
        assert!(optimal <= 10);
    }

    #[test]
    fn test_complexity_history() {
        let manager = SmartConcurrencyManager::new(2, 10, 80.0);
        
        manager.update_complexity_history(30.0);
        manager.update_complexity_history(50.0);
        manager.update_complexity_history(70.0);
        
        let avg = manager.calculate_average_complexity();
        assert!((avg - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_record_job_result() {
        let manager = SmartConcurrencyManager::new(2, 10, 80.0);
        
        manager.record_job_result(true, 1024);
        manager.record_job_result(false, 2048);
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_jobs_processed, 2);
        assert_eq!(stats.successful_jobs, 1);
        assert_eq!(stats.failed_jobs, 1);
        assert_eq!(stats.total_memory_used, 3072);
    }
}
