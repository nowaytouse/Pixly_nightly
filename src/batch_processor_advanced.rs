//! 高性能批量处理系统
//! 
//! 提供真实的并发处理能力：
//! - Rayon并行处理
//! - 实时进度报告
//! - 错误处理和恢复
//! - 性能统计

use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// 批量处理配置
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// 最大并发线程数 (0=自动)
    pub max_threads: usize,
    /// 是否启用详细日志
    pub verbose: bool,
    /// 失败后是否继续
    pub continue_on_error: bool,
    /// 进度更新间隔（毫秒）
    pub progress_interval_ms: u64,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_threads: 0,
            verbose: true,
            continue_on_error: true,
            progress_interval_ms: 500,
        }
    }
}

/// 批量处理统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchStats {
    pub total: usize,
    pub completed: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub skipped: usize,
    pub elapsed_ms: u64,
    pub bytes_saved: u64,
    pub avg_speed: f64,
    pub eta_seconds: u64,
}

impl BatchStats {
    fn new(total: usize) -> Self {
        Self {
            total,
            completed: 0,
            succeeded: 0,
            failed: 0,
            skipped: 0,
            elapsed_ms: 0,
            bytes_saved: 0,
            avg_speed: 0.0,
            eta_seconds: 0,
        }
    }
    
    pub fn percent_complete(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.completed as f64 / self.total as f64) * 100.0
        }
    }
    
    fn update(&mut self, completed: usize, succeeded: usize, failed: usize, skipped: usize, 
              bytes_saved: u64, elapsed: Duration) {
        self.completed = completed;
        self.succeeded = succeeded;
        self.failed = failed;
        self.skipped = skipped;
        self.bytes_saved = bytes_saved;
        self.elapsed_ms = elapsed.as_millis() as u64;
        
        let elapsed_secs = elapsed.as_secs_f64();
        if elapsed_secs > 0.0 {
            self.avg_speed = completed as f64 / elapsed_secs;
        }
        
        let remaining = self.total.saturating_sub(completed);
        if self.avg_speed > 0.0 {
            self.eta_seconds = (remaining as f64 / self.avg_speed) as u64;
        }
    }
}

/// 任务失败记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFailure {
    pub path: PathBuf,
    pub error: String,
    pub timestamp: u64,
}

/// 批量处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub stats: BatchStats,
    pub failures: Vec<TaskFailure>,
    pub is_success: bool,
}

impl BatchResult {
    pub fn has_failures(&self) -> bool {
        !self.failures.is_empty()
    }
    
    pub fn summary(&self) -> String {
        format!(
            "Total: {}, Succeeded: {}, Failed: {}, Skipped: {}, Time: {:.2}s, Speed: {:.1} files/s",
            self.stats.total,
            self.stats.succeeded,
            self.stats.failed,
            self.stats.skipped,
            self.stats.elapsed_ms as f64 / 1000.0,
            self.stats.avg_speed
        )
    }
}

/// 批量处理器
pub struct BatchProcessor {
    config: BatchConfig,
}

impl BatchProcessor {
    pub fn new(config: BatchConfig) -> Self {
        Self { config }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(BatchConfig::default())
    }
    
    /// 处理文件列表
    pub fn process<F>(&self, files: Vec<PathBuf>, processor: F) -> Result<BatchResult>
    where
        F: Fn(&Path) -> Result<u64> + Send + Sync,
    {
        let total = files.len();
        
        if total == 0 {
            return Ok(BatchResult {
                stats: BatchStats::new(0),
                failures: vec![],
                is_success: true,
            });
        }
        
        let num_threads = if self.config.max_threads == 0 {
            num_cpus::get()
        } else {
            self.config.max_threads
        };
        
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🚀 Batch processing started");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("   Total files:    {}", total);
        println!("   Thread pool:    {} threads", num_threads);
        println!("   Verbose mode:   {}", self.config.verbose);
        println!("   Continue on error: {}", self.config.continue_on_error);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        let completed = Arc::new(AtomicUsize::new(0));
        let succeeded = Arc::new(AtomicUsize::new(0));
        let failed = Arc::new(AtomicUsize::new(0));
        let skipped = Arc::new(AtomicUsize::new(0));
        let bytes_saved = Arc::new(AtomicU64::new(0));
        let failures = Arc::new(Mutex::new(Vec::new()));
        let first_fatal_error = Arc::new(Mutex::new(None::<String>));
        
        let start_time = Instant::now();
        let last_report = Arc::new(Mutex::new(Instant::now()));
        
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .thread_name(|i| format!("batch-worker-{}", i))
            .build()
            .context("Failed to create thread pool")?;
        
        pool.install(|| {
            files.par_iter().for_each(|file| {
                let file_name = file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                
                if self.config.verbose {
                    println!("🔄 Processing: {}", file_name);
                }
                
                match processor(file) {
                    Ok(saved) => {
                        succeeded.fetch_add(1, Ordering::Relaxed);
                        bytes_saved.fetch_add(saved, Ordering::Relaxed);
                        
                        if self.config.verbose {
                            println!("✅ Success: {} (saved {} bytes)", file_name, saved);
                        }
                    }
                    Err(e) => {
                        failed.fetch_add(1, Ordering::Relaxed);
                        eprintln!("❌ Failed: {}", file_name);
                        eprintln!("   Error: {}", e);
                        
                        if let Ok(mut f) = failures.lock() {
                            f.push(TaskFailure {
                                path: file.clone(),
                                error: e.to_string(),
                                timestamp: start_time.elapsed().as_secs(),
                            });
                        }
                        
                        if !self.config.continue_on_error
                            && let Ok(mut first_error) = first_fatal_error.lock()
                            && first_error.is_none()
                        {
                            *first_error = Some(format!(
                                "文件: {:?}\n错误: {}", 
                                file, e
                            ));
                        }
                    }
                }
                
                let current_completed = completed.fetch_add(1, Ordering::Relaxed) + 1;
                
                if let Ok(mut last) = last_report.try_lock() {
                    let now = Instant::now();
                    if now.duration_since(*last).as_millis() as u64 >= self.config.progress_interval_ms {
                        let elapsed = start_time.elapsed();
                        let current_succeeded = succeeded.load(Ordering::Relaxed);
                        let current_failed = failed.load(Ordering::Relaxed);
                        let current_saved = bytes_saved.load(Ordering::Relaxed);
                        
                        let percent = (current_completed as f64 / total as f64) * 100.0;
                        let speed = current_completed as f64 / elapsed.as_secs_f64();
                        let remaining = total - current_completed;
                        let eta = if speed > 0.0 { remaining as f64 / speed } else { 0.0 };
                        
                        println!(
                            "📊 进度: {}/{} ({:.1}%) | 成功: {} | 失败: {} | 速度: {:.1} 文件/秒 | 预计: {:.0}秒",
                            current_completed, total, percent, current_succeeded, current_failed, speed, eta
                        );
                        
                        if current_saved > 0 {
                            let saved_mb = current_saved as f64 / (1024.0 * 1024.0);
                            println!("   💾 Saved: {:.2} MB", saved_mb);
                        }
                        
                        *last = now;
                    }
                }
            });
        });
        
        if let Ok(first_error) = first_fatal_error.lock()
            && let Some(error_msg) = first_error.as_ref()
        {
            eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            eprintln!("❌ Batch processing aborted due to error");
            eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            eprintln!("{}", error_msg);
            eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            eprintln!("💡 Tip: Use --continue-on-error to skip failed files");
            eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            
            return Err(anyhow::anyhow!(
                "批量处理中止\n\n{}\n\n使用 --continue-on-error 继续处理", 
                error_msg
            ));
        }
        
        let elapsed = start_time.elapsed();
        let final_completed = completed.load(Ordering::Relaxed);
        let final_succeeded = succeeded.load(Ordering::Relaxed);
        let final_failed = failed.load(Ordering::Relaxed);
        let final_skipped = skipped.load(Ordering::Relaxed);
        let final_saved = bytes_saved.load(Ordering::Relaxed);
        
        let mut stats = BatchStats::new(total);
        stats.update(
            final_completed,
            final_succeeded,
            final_failed,
            final_skipped,
            final_saved,
            elapsed
        );
        
        let failures_list = failures.lock().unwrap().clone();
        
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🏁 Batch processing completed");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("   Total:          {}", stats.total);
        println!("   Completed:      {}", stats.completed);
        println!("   Succeeded:      ✅ {}", stats.succeeded);
        println!("   Failed:         ❌ {}", stats.failed);
        println!("   Skipped:        ⏭️  {}", stats.skipped);
        println!("   Time elapsed:   {:.2}s", elapsed.as_secs_f64());
        println!("   Average speed:  {:.1} files/s", stats.avg_speed);
        
        if final_saved > 0 {
            let saved_mb = final_saved as f64 / (1024.0 * 1024.0);
            println!("   Space saved:    {:.2} MB", saved_mb);
        }
        
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        let is_success = final_failed == 0;
        
        Ok(BatchResult {
            stats,
            failures: failures_list,
            is_success,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_batch_processor_creation() {
        let processor = BatchProcessor::with_defaults();
        assert!(processor.config.verbose);
        assert!(processor.config.continue_on_error);
    }
    
    #[test]
    fn test_batch_stats() {
        let mut stats = BatchStats::new(100);
        assert_eq!(stats.total, 100);
        assert_eq!(stats.percent_complete(), 0.0);
        
        stats.update(50, 45, 5, 0, 1000000, Duration::from_secs(10));
        assert_eq!(stats.completed, 50);
        assert_eq!(stats.succeeded, 45);
        assert_eq!(stats.failed, 5);
        assert_eq!(stats.percent_complete(), 50.0);
        assert!(stats.avg_speed > 0.0);
    }
    
    #[test]
    fn test_empty_batch() {
        let processor = BatchProcessor::with_defaults();
        let result = processor.process(vec![], |_| Ok(0));
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_success);
        assert_eq!(result.stats.total, 0);
    }
}
