/**
 * Batch Processor - 高性能批量处理系统
 * 
 * 🔥 Phase 40.24.3: 批量处理性能优化
 * 
 * 🎯 架构原则（@PROJECT_QUALITY_MANIFESTO.md）：
 * - 真实的并发处理，不是模拟
 * - 响亮的错误报告，不掩盖问题
 * - 详细的日志记录，完整的执行轨迹
 * - 真实的资源管理，不是作弊
 * - 失败就失败，不要fallback
 * 
 * @module batch_processor
 */
// 🔧 统一日志系统
use tracing::{info, warn, error, debug};


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
    
    /// 内存使用限制（MB，0=无限制）
    pub memory_limit_mb: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_threads: 0, // 自动检测
            verbose: true,
            continue_on_error: true,
            progress_interval_ms: 500,
            memory_limit_mb: 0,
        }
    }
}

/// 批量处理统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchStats {
    /// 总任务数
    pub total: usize,
    
    /// 已完成数
    pub completed: usize,
    
    /// 成功数
    pub succeeded: usize,
    
    /// 失败数
    pub failed: usize,
    
    /// 跳过数
    pub skipped: usize,
    
    /// 总耗时（毫秒）
    pub elapsed_ms: u64,
    
    /// 总节省字节数
    pub bytes_saved: u64,
    
    /// 平均处理速度（文件/秒）
    pub avg_speed: f64,
    
    /// 预估剩余时间（秒）
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
    
    /// 计算完成百分比
    pub fn percent_complete(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.completed as f64 / self.total as f64) * 100.0
        }
    }
    
    /// 更新统计信息
    fn update(&mut self, completed: usize, succeeded: usize, failed: usize, skipped: usize, 
              bytes_saved: u64, elapsed: Duration) {
        self.completed = completed;
        self.succeeded = succeeded;
        self.failed = failed;
        self.skipped = skipped;
        self.bytes_saved = bytes_saved;
        self.elapsed_ms = elapsed.as_millis() as u64;
        
        // 计算处理速度
        let elapsed_secs = elapsed.as_secs_f64();
        if elapsed_secs > 0.0 {
            self.avg_speed = completed as f64 / elapsed_secs;
        }
        
        // 计算ETA
        let remaining = self.total.saturating_sub(completed);
        if self.avg_speed > 0.0 {
            self.eta_seconds = (remaining as f64 / self.avg_speed) as u64;
        }
    }
}

/// 任务失败记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFailure {
    /// 文件路径
    pub path: PathBuf,
    
    /// 错误信息
    pub error: String,
    
    /// 失败时间戳
    pub timestamp: u64,
}

/// 批量处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    /// 统计信息
    pub stats: BatchStats,
    
    /// 失败任务列表
    pub failures: Vec<TaskFailure>,
    
    /// 是否完全成功
    pub is_success: bool,
}

impl BatchResult {
    /// 是否有任何失败
    pub fn has_failures(&self) -> bool {
        !self.failures.is_empty()
    }
    
    /// 获取可读的总结
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
    /// 创建新的批量处理器
    pub fn new(config: BatchConfig) -> Self {
        Self { config }
    }
    
    /// 使用默认配置创建
    pub fn with_defaults() -> Self {
        Self::new(BatchConfig::default())
    }
    
    /// 处理文件列表
    /// 
    /// 🔥 核心批量处理方法
    /// 
    /// # 真实性原则
    /// - 真实的并发处理（不是串行模拟）
    /// - 真实的错误报告（不掩盖）
    /// - 真实的进度更新（不是估算）
    /// 
    /// # 参数
    /// - `files`: 要处理的文件列表
    /// - `processor`: 单个文件的处理函数
    /// 
    pub fn process<F>(&self, files: Vec<PathBuf>, processor: F) -> Result<BatchResult>
    where
        F: Fn(&Path) -> Result<u64> + Send + Sync,
    {
        let total = files.len();
        
        if total == 0 {
            warn!("⚠️  No files to process");
            return Ok(BatchResult {
                stats: BatchStats::new(0),
                failures: vec![],
                is_success: true,
            });
        }
        
        // 配置线程数
        let num_threads = if self.config.max_threads == 0 {
            num_cpus::get()
        } else {
            self.config.max_threads
        };
        
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("🚀 Batch Processing Started");
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("   Total files:    {}", total);
        info!("   Thread pool:    {} threads", num_threads);
        info!("   Verbose mode:   {}", self.config.verbose);
        info!("   Continue on error: {}", self.config.continue_on_error);
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // 线程安全的计数器
        let completed = Arc::new(AtomicUsize::new(0));
        let succeeded = Arc::new(AtomicUsize::new(0));
        let failed = Arc::new(AtomicUsize::new(0));
        let skipped = Arc::new(AtomicUsize::new(0));
        let bytes_saved = Arc::new(AtomicU64::new(0));
        
        // 失败记录
        let failures = Arc::new(Mutex::new(Vec::new()));
        
        // 🔥 第一个致命错误（用于continue_on_error=false时）
        let first_fatal_error = Arc::new(Mutex::new(None::<String>));
        
        // 开始计时
        let start_time = Instant::now();
        let last_report = Arc::new(Mutex::new(Instant::now()));
        
        // 配置 Rayon 线程池
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .thread_name(|i| format!("batch-worker-{}", i))
            .build()
            .context("❌ Failed to create thread pool")?;
        
        // 并发处理
        pool.install(|| {
            files.par_iter().for_each(|file| {
                let file_name = file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                
                if self.config.verbose {
                    debug!("🔄 Processing: {}", file_name);
                }
                
                // 处理单个文件
                match processor(file) {
                    Ok(saved) => {
                        succeeded.fetch_add(1, Ordering::Relaxed);
                        bytes_saved.fetch_add(saved, Ordering::Relaxed);
                        
                        if self.config.verbose {
                            debug!("✅ Success: {} (saved {} bytes)", file_name, saved);
                        }
                    }
                    Err(e) => {
                        failed.fetch_add(1, Ordering::Relaxed);
                        
                        // 🔥 响亮的错误报告（不掩盖）
                        error!("❌ FAILED: {}", file_name);
                        error!("   Error: {}", e);
                        
                        // 记录失败
                        if let Ok(mut f) = failures.lock() {
                            f.push(TaskFailure {
                                path: file.clone(),
                                error: e.to_string(),
                                timestamp: start_time.elapsed().as_secs(),
                            });
                        }
                        
                        // 🔥 记录第一个致命错误（不panic，在循环后返回）
                        if !self.config.continue_on_error {
                            if let Ok(mut first_error) = first_fatal_error.lock() {
                                if first_error.is_none() {
                                    *first_error = Some(format!(
                                        "File: {:?}\nError: {}", 
                                        file, e
                                    ));
                                }
                            }
                        }
                    }
                }
                
                // 更新完成计数
                let current_completed = completed.fetch_add(1, Ordering::Relaxed) + 1;
                
                // 定期报告进度
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
                        
                        info!(
                            "📊 Progress: {}/{} ({:.1}%) | Success: {} | Failed: {} | Speed: {:.1} files/s | ETA: {:.0}s",
                            current_completed, total, percent, current_succeeded, current_failed, speed, eta
                        );
                        
                        if current_saved > 0 {
                            let saved_mb = current_saved as f64 / (1024.0 * 1024.0);
                            info!("   💾 Saved: {:.2} MB", saved_mb);
                        }
                        
                        *last = now;
                    }
                }
            });
        });
        
        // 🔥 检查是否有致命错误（continue_on_error=false时）
        if let Ok(first_error) = first_fatal_error.lock() {
            if let Some(error_msg) = first_error.as_ref() {
                error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                error!("❌ Batch processing aborted due to error");
                error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                error!("{}", error_msg);
                error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                error!("💡 Tip: Use --continue-on-error to skip failed files");
                error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                
                return Err(anyhow::anyhow!(
                    "Batch processing aborted\n\n{}\n\nUse --continue-on-error to continue despite errors", 
                    error_msg
                ));
            }
        }
        
        // 最终统计
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
        
        // 🔥 响亮的最终报告
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("🏁 Batch Processing Complete");
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("   Total:          {}", stats.total);
        info!("   Completed:      {}", stats.completed);
        info!("   Succeeded:      ✅ {}", stats.succeeded);
        info!("   Failed:         ❌ {}", stats.failed);
        info!("   Skipped:        ⏭️  {}", stats.skipped);
        info!("   Time:           {:.2}s", elapsed.as_secs_f64());
        info!("   Avg Speed:      {:.1} files/s", stats.avg_speed);
        
        if final_saved > 0 {
            let saved_mb = final_saved as f64 / (1024.0 * 1024.0);
            info!("   Bytes Saved:    💾 {:.2} MB", saved_mb);
        }
        
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // 如果有失败，响亮地报告
        if !failures_list.is_empty() {
            error!("⚠️  {} FAILURES DETECTED", failures_list.len());
            error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            
            for (i, failure) in failures_list.iter().enumerate().take(10) {
                error!("   {}. {:?}", i + 1, failure.path.file_name().unwrap_or_default());
                error!("      Error: {}", failure.error);
            }
            
            if failures_list.len() > 10 {
                error!("   ... and {} more failures", failures_list.len() - 10);
            }
            
            error!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }
        
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
    fn test_batch_stats() {
        let stats = BatchStats::new(100);
        assert_eq!(stats.total, 100);
        assert_eq!(stats.percent_complete(), 0.0);
    }
    
    #[test]
    fn test_batch_config_default() {
        let config = BatchConfig::default();
        assert_eq!(config.max_threads, 0); // Auto
        assert!(config.verbose);
        assert!(config.continue_on_error);
    }
}
