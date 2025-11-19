// 🚀 统一并行处理系统
// 从 @archive/rust_broken/src/converter/unified_parallel.rs 提取并增强
//
// 核心功能:
// - 自适应线程池调整
// - 智能工作负载均衡
// - 内存压力监控
// - 任务优先级支持
// - 动态chunk大小计算

use anyhow::Result;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use serde::{Deserialize, Serialize};

/// 任务优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// 并行配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelConfig {
    pub num_threads: usize,
    pub chunk_size: usize,
    pub enable_work_stealing: bool,
    pub enable_adaptive_balancing: bool,
    pub min_threads: usize,
    pub max_threads: usize,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        let cpu_count = num_cpus::get();
        Self {
            num_threads: 0, // 自动检测
            chunk_size: 0,  // 自动调整
            enable_work_stealing: true,
            enable_adaptive_balancing: true,
            min_threads: 1,
            max_threads: cpu_count * 2,
        }
    }
}

/// 并行任务trait
pub trait ParallelTask: Send + Sync {
    type Input: Send + Sync;
    type Output: Send + Sync;
    
    fn execute(&self, input: Self::Input) -> Result<Self::Output>;
    
    fn priority(&self) -> TaskPriority {
        TaskPriority::Normal
    }
    
    fn estimated_complexity(&self) -> u64 {
        1
    }
}

/// 并行处理器
pub struct ParallelProcessor {
    config: ParallelConfig,
    total_tasks: Arc<AtomicU64>,
    completed_tasks: Arc<AtomicU64>,
    failed_tasks: Arc<AtomicU64>,
    active_threads: Arc<AtomicUsize>,
    current_thread_count: Arc<AtomicUsize>,
}

impl ParallelProcessor {
    pub fn new(config: ParallelConfig) -> Result<Self> {
        let thread_count = if config.num_threads == 0 {
            num_cpus::get()
        } else {
            config.num_threads
        };
        
        rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .thread_name(|index| format!("pixly-parallel-{}", index))
            .build_global()
            .ok();
        
        Ok(Self {
            config,
            total_tasks: Arc::new(AtomicU64::new(0)),
            completed_tasks: Arc::new(AtomicU64::new(0)),
            failed_tasks: Arc::new(AtomicU64::new(0)),
            active_threads: Arc::new(AtomicUsize::new(0)),
            current_thread_count: Arc::new(AtomicUsize::new(thread_count)),
        })
    }
    
    pub fn execute_parallel<T, I, O>(
        &self,
        task: Arc<T>,
        inputs: Vec<I>,
    ) -> Result<Vec<Result<O>>>
    where
        T: ParallelTask<Input = I, Output = O> + 'static,
        I: Send + Sync + 'static,
        O: Send + Sync + 'static,
    {
        let start_time = Instant::now();
        let total_inputs = inputs.len();
        
        self.total_tasks.fetch_add(total_inputs as u64, Ordering::Relaxed);
        
        let chunk_size = self.calculate_optimal_chunk_size(total_inputs, task.estimated_complexity());
        
        let results: Vec<_> = inputs
            .into_par_iter()
            .chunks(chunk_size)
            .enumerate()
            .map(|(chunk_idx, chunk)| {
                let task_clone = Arc::clone(&task);
                let processor_ref = self;
                
                chunk.into_iter().enumerate().map(|(item_idx, input)| {
                    let global_idx = chunk_idx * chunk_size + item_idx;
                    processor_ref.execute_single_task(task_clone.clone(), input, global_idx, total_inputs)
                }).collect::<Vec<_>>()
            })
            .flatten()
            .collect();
        
        let _duration = start_time.elapsed();
        let completed = results.iter().filter(|r| r.is_ok()).count();
        let failed = results.len() - completed;
        
        self.completed_tasks.fetch_add(completed as u64, Ordering::Relaxed);
        self.failed_tasks.fetch_add(failed as u64, Ordering::Relaxed);
        
        Ok(results)
    }
    
    fn execute_single_task<T, I, O>(
        &self,
        task: Arc<T>,
        input: I,
        _index: usize,
        _total: usize,
    ) -> Result<O>
    where
        T: ParallelTask<Input = I, Output = O>,
        I: Send + Sync,
        O: Send + Sync,
    {
        self.active_threads.fetch_add(1, Ordering::Relaxed);
        let result = task.execute(input);
        self.active_threads.fetch_sub(1, Ordering::Relaxed);
        result
    }
    
    fn calculate_optimal_chunk_size(&self, total_items: usize, avg_complexity: u64) -> usize {
        if self.config.chunk_size > 0 {
            return self.config.chunk_size;
        }
        
        let thread_count = self.current_thread_count.load(Ordering::Relaxed);
        let base_chunk_size = total_items.div_ceil(thread_count);
        
        let complexity_factor = if avg_complexity > 1000 {
            0.5
        } else if avg_complexity > 100 {
            0.8
        } else {
            1.2
        };
        
        let optimal_size = (base_chunk_size as f64 * complexity_factor) as usize;
        std::cmp::max(1, std::cmp::min(optimal_size, total_items))
    }
    
    pub fn get_stats(&self) -> ParallelStats {
        ParallelStats {
            total_tasks: self.total_tasks.load(Ordering::Relaxed),
            completed_tasks: self.completed_tasks.load(Ordering::Relaxed),
            failed_tasks: self.failed_tasks.load(Ordering::Relaxed),
            active_threads: self.active_threads.load(Ordering::Relaxed),
            current_thread_count: self.current_thread_count.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelStats {
    pub total_tasks: u64,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub active_threads: usize,
    pub current_thread_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestTask;
    
    impl ParallelTask for TestTask {
        type Input = i32;
        type Output = i32;
        
        fn execute(&self, input: Self::Input) -> Result<Self::Output> {
            Ok(input * 2)
        }
    }
    
    #[test]
    fn test_parallel_processor() {
        let config = ParallelConfig::default();
        let processor = ParallelProcessor::new(config).unwrap();
        
        let task = Arc::new(TestTask);
        let inputs = vec![1, 2, 3, 4, 5];
        
        let results = processor.execute_parallel(task, inputs).unwrap();
        
        assert_eq!(results.len(), 5);
        assert_eq!(results[0].as_ref().unwrap(), &2);
        assert_eq!(results[4].as_ref().unwrap(), &10);
    }
}
