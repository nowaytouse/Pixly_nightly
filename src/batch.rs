use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use anyhow::{anyhow, Result};

/// 批处理进度信息（从归档 batch.rs 精炼而来，但与具体文件/格式解耦）
#[derive(Debug, Clone)]
pub struct BatchProgress {
    /// 当前处理的任务索引（从 0 开始）
    pub current: usize,
    /// 总任务数
    pub total: usize,
    /// 已完成的任务数
    pub completed: usize,
    /// 失败的任务数
    pub failed: usize,
    /// 当前整体进度百分比（0-100）
    pub percentage: f32,
    /// 已用时间（毫秒）
    pub elapsed_ms: u64,
    /// 预估剩余时间（毫秒）
    pub eta_ms: u64,
    /// 处理速度（任务数/秒）
    pub speed: f64,
}

pub type BatchProgressCallback = Arc<dyn Fn(BatchProgress) + Send + Sync>;

/// 单个任务失败信息
#[derive(Debug, Clone)]
pub struct BatchFailure {
    pub index: usize,
    pub error: String,
}

/// 批处理结果
#[derive(Debug, Clone)]
pub struct BatchResult {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub failures: Vec<BatchFailure>,
    pub total_time_ms: u64,
}

/// 批处理配置
#[derive(Clone)]
pub struct BatchOptions {
    pub parallel: bool,
    pub max_threads: usize,
    pub progress_callback: Option<BatchProgressCallback>,
}

impl BatchOptions {
    pub fn new() -> Self {
        let max_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .max(1);

        Self {
            parallel: true,
            max_threads,
            progress_callback: None,
        }
    }

    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    pub fn with_max_threads(mut self, max_threads: usize) -> Self {
        self.max_threads = max_threads.max(1);
        self
    }

    pub fn with_progress_callback(mut self, callback: BatchProgressCallback) -> Self {
        self.progress_callback = Some(callback);
        self
    }
}

impl Default for BatchOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// 通用批处理器：从归档 BatchConverter 中提炼出的“任务循环 + 并发 + 进度回调”模式
pub struct BatchProcessor {
    options: BatchOptions,
}

impl BatchProcessor {
    pub fn new(options: BatchOptions) -> Self {
        Self { options }
    }

    pub fn options(&self) -> &BatchOptions {
        &self.options
    }

    /// 处理一批任务
    ///
    /// `worker` 为纯函数，不做文件 IO 约束，由调用方决定具体行为。
    pub fn process<T, F, E>(&self, tasks: Vec<T>, worker: F) -> Result<BatchResult>
    where
        T: Send + Sync + 'static,
        F: Fn(&T) -> Result<(), E> + Send + Sync + 'static,
        E: std::error::Error + Send + Sync + 'static,
    {
        let start = Instant::now();
        let total = tasks.len();

        if total == 0 {
            return Ok(BatchResult {
                total: 0,
                succeeded: 0,
                failed: 0,
                failures: Vec::new(),
                total_time_ms: 0,
            });
        }

        let inner_result = if self.options.parallel && self.options.max_threads > 1 {
            self.process_parallel(tasks, worker, start)?
        } else {
            self.process_sequential(tasks, worker, start)?
        };

        let elapsed = start.elapsed().as_millis() as u64;

        Ok(BatchResult {
            total_time_ms: elapsed,
            ..inner_result
        })
    }

    /// 从总任务数和配置推导实际使用的线程数
    ///
    /// 借鉴归档 parallel_encoder.rs 中的启发：
    /// - 小批次任务使用较少线程，避免线程调度开销
    /// - 中等/大型批次使用配置中的最大线程数
    pub(crate) fn effective_thread_count(&self, total_tasks: usize) -> usize {
        let max_threads = self.options.max_threads.max(1);

        if total_tasks == 0 {
            return 1;
        }

        let base = if total_tasks < 10 {
            // 小任务：使用最多一半线程
            (max_threads / 2).max(1)
        } else if total_tasks < 100 {
            // 中等任务：使用全部线程
            max_threads
        } else {
            // 大任务：同样使用全部线程（后续可按需要扩展）
            max_threads
        };

        base.min(total_tasks).max(1)
    }

    fn process_sequential<T, F, E>(
        &self,
        tasks: Vec<T>,
        worker: F,
        start: Instant,
    ) -> Result<BatchResult>
    where
        T: Send + Sync + 'static,
        F: Fn(&T) -> Result<(), E> + Send + Sync + 'static,
        E: std::error::Error + Send + Sync + 'static,
    {
        let total = tasks.len();
        let mut succeeded = 0usize;
        let mut failed = 0usize;
        let mut failures = Vec::new();

        for (index, task) in tasks.iter().enumerate() {
            if let Some(ref callback) = self.options.progress_callback {
                let completed = succeeded + failed;

                let elapsed = start.elapsed();
                let elapsed_ms = elapsed.as_millis() as u64;
                let elapsed_secs = elapsed.as_secs_f64();

                let processed = completed;
                let speed = if elapsed_secs > 0.0 {
                    processed as f64 / elapsed_secs
                } else {
                    0.0
                };

                let remaining = total.saturating_sub(processed);
                let eta_ms = if speed > 0.0 {
                    ((remaining as f64 / speed) * 1000.0) as u64
                } else {
                    0
                };

                let percentage = if total > 0 {
                    (completed as f32 / total as f32) * 100.0
                } else {
                    0.0
                };

                callback(BatchProgress {
                    current: index,
                    total,
                    completed,
                    failed,
                    percentage,
                    elapsed_ms,
                    eta_ms,
                    speed,
                });
            }

            match worker(task) {
                Ok(()) => {
                    succeeded += 1;
                }
                Err(err) => {
                    failed += 1;
                    failures.push(BatchFailure {
                        index,
                        error: err.to_string(),
                    });
                }
            }
        }

        Ok(BatchResult {
            total,
            succeeded,
            failed,
            failures,
            total_time_ms: 0,
        })
    }

    fn process_parallel<T, F, E>(
        &self,
        tasks: Vec<T>,
        worker: F,
        start: Instant,
    ) -> Result<BatchResult>
    where
        T: Send + Sync + 'static,
        F: Fn(&T) -> Result<(), E> + Send + Sync + 'static,
        E: std::error::Error + Send + Sync + 'static,
    {
        let total = tasks.len();
        let tasks = Arc::new(tasks);
        let worker = Arc::new(worker);

        let succeeded = Arc::new(AtomicUsize::new(0));
        let failed = Arc::new(AtomicUsize::new(0));
        let failures = Arc::new(Mutex::new(Vec::new()));
        let current = Arc::new(AtomicUsize::new(0));
        let progress_callback = self.options.progress_callback.clone();

        let thread_count = self.effective_thread_count(total);

        let mut handles = Vec::with_capacity(thread_count);

        for _ in 0..thread_count {
            let tasks = Arc::clone(&tasks);
            let worker = Arc::clone(&worker);
            let succeeded = Arc::clone(&succeeded);
            let failed = Arc::clone(&failed);
            let failures = Arc::clone(&failures);
            let current = Arc::clone(&current);
            let progress_callback = progress_callback.clone();


            let handle = std::thread::spawn(move || loop {
                let index = current.fetch_add(1, Ordering::SeqCst);
                if index >= total {
                    break;
                }

                if let Some(ref callback) = progress_callback {
                    let completed = succeeded.load(Ordering::Relaxed);
                    let failed_count = failed.load(Ordering::Relaxed);
                    let processed = completed + failed_count;

                    let elapsed = start.elapsed();
                    let elapsed_ms = elapsed.as_millis() as u64;
                    let elapsed_secs = elapsed.as_secs_f64();

                    let speed = if elapsed_secs > 0.0 {
                        processed as f64 / elapsed_secs
                    } else {
                        0.0
                    };

                    let remaining = total.saturating_sub(processed);
                    let eta_ms = if speed > 0.0 {
                        ((remaining as f64 / speed) * 1000.0) as u64
                    } else {
                        0
                    };

                    let percentage = if total > 0 {
                        (processed as f32 / total as f32) * 100.0
                    } else {
                        0.0
                    };

                    callback(BatchProgress {
                        current: index,
                        total,
                        completed,
                        failed: failed_count,
                        percentage,
                        elapsed_ms,
                        eta_ms,
                        speed,
                    });
                }

                let result = worker(&tasks[index]);

                match result {
                    Ok(()) => {
                        succeeded.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(err) => {
                        failed.fetch_add(1, Ordering::Relaxed);
                        if let Ok(mut guard) = failures.lock() {
                            guard.push(BatchFailure {
                                index,
                                error: err.to_string(),
                            });
                        }
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle
                .join()
                .map_err(|_| anyhow!("batch worker thread panicked"))?;
        }

        let succeeded_count = succeeded.load(Ordering::Relaxed);
        let failed_count = failed.load(Ordering::Relaxed);
        let failures_vec = failures
            .lock()
            .map_err(|_| anyhow!("batch failures mutex poisoned"))?
            .clone();

        Ok(BatchResult {
            total,
            succeeded: succeeded_count,
            failed: failed_count,
            failures: failures_vec,
            total_time_ms: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_processor_sequential_basic() {
        let options = BatchOptions::new().with_parallel(false);
        let processor = BatchProcessor::new(options);

        let tasks: Vec<u32> = (0..10).collect();
        let sum = Arc::new(AtomicUsize::new(0));
        let sum_clone = Arc::clone(&sum);

        let result = processor
            .process(tasks, move |value| {
                sum_clone.fetch_add(*value as usize, Ordering::Relaxed);
                Ok(()) as std::result::Result<(), std::io::Error>
            })
            .unwrap();

        assert_eq!(result.total, 10);
        assert_eq!(result.succeeded, 10);
        assert_eq!(result.failed, 0);
        assert_eq!(result.failures.len(), 0);
        assert_eq!(sum.load(Ordering::Relaxed), 45);
    }

    #[test]
    fn effective_thread_count_scales_with_task_size() {
        let options = BatchOptions::new().with_max_threads(8);
        let processor = BatchProcessor::new(options);

        // total_tasks 为 0 或 1 时至少使用 1 个线程
        assert_eq!(processor.effective_thread_count(0), 1);
        assert_eq!(processor.effective_thread_count(1), 1);

        // 小批次任务（<10）：最多使用一半线程，但不超过 total_tasks
        assert_eq!(processor.effective_thread_count(5), 4);

        // 中等批次任务（10-99）：使用全部线程
        assert_eq!(processor.effective_thread_count(10), 8);
        assert_eq!(processor.effective_thread_count(50), 8);

        // 大批次任务（>=100）：同样使用全部线程
        assert_eq!(processor.effective_thread_count(200), 8);
    }

    #[test]
    fn batch_processor_parallel_with_progress_and_failures() {
        let progress_events = Arc::new(Mutex::new(Vec::new()));
        let progress_events_clone = Arc::clone(&progress_events);

        let options = BatchOptions::new().with_progress_callback(Arc::new(move |p| {
            let mut guard = progress_events_clone.lock().unwrap();
            guard.push((p.current, p.completed, p.failed));
        }));

        let processor = BatchProcessor::new(options);

        let tasks: Vec<u32> = (0..20).collect();

        let result = processor
            .process(tasks, move |value| {
                if *value % 5 == 0 {
                    Err(std::io::Error::other(
                        "simulated failure",
                    ))
                } else {
                    Ok(())
                }
            })
            .unwrap();

        assert_eq!(result.total, 20);
        assert_eq!(result.succeeded + result.failed, 20);
        assert_eq!(result.failed, 4);
        assert_eq!(result.failures.len(), 4);

        let progress = progress_events.lock().unwrap();
        assert!(!progress.is_empty());
    }
}
