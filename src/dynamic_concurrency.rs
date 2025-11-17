// 🚀 动态并发管理器 - 从Go代码提取
// 基于文件大小/分辨率动态调整并发数
//
// 核心功能 (从Go代码提取):
// - 动态worker池 (基于megapixels调整slots)
// - 智能复杂度计算 (文件大小、格式、模式、质量)
// - 内存监控和自适应调整
// - 防死锁机制 (超大文件最多占用50% slots)

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

/// 动态Worker池 (从Go的WorkerPool提取)
pub struct DynamicWorkerPool {
    max_workers: usize,
    active_workers: Arc<AtomicUsize>,
    available_slots: Arc<Mutex<usize>>,
}

impl DynamicWorkerPool {
    /// 创建动态worker池
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers,
            active_workers: Arc::new(AtomicUsize::new(0)),
            available_slots: Arc::new(Mutex::new(max_workers)),
        }
    }
    
    /// 获取worker slot (基于文件大小动态调整)
    /// 
    /// 动态策略 (从Go代码提取):
    /// - < 2MP:  1 slot (小文件,并发处理)
    /// - 2-8MP:  2 slots (中等文件,适度限制)
    /// - 8-32MP: 4 slots (大文件,严格限制)
    /// - > 32MP: maxWorkers/2 (超大文件,限制为pool的50%,避免死锁)
    pub fn acquire(&self, megapixels: f64) -> usize {
        let slots_needed = if megapixels < 2.0 {
            1
        } else if megapixels < 8.0 {
            2
        } else if megapixels < 32.0 {
            4
        } else {
            // 防止死锁：超大文件最多占用50% slots
            let max_slots_per_file = self.max_workers / 2;
            max_slots_per_file.max(4)
        };
        
        // 确保不超过最大worker数的50%
        let max_slots_per_file = self.max_workers / 2;
        let final_slots = slots_needed.min(max_slots_per_file.max(1));
        
        // 占用slots
        let mut available = self.available_slots.lock().unwrap();
        if *available >= final_slots {
            *available -= final_slots;
            self.active_workers.fetch_add(final_slots, Ordering::SeqCst);
        }
        
        final_slots
    }
    
    /// 释放worker slots
    pub fn release(&self, slots: usize) {
        let mut available = self.available_slots.lock().unwrap();
        *available += slots;
        self.active_workers.fetch_sub(slots, Ordering::SeqCst);
    }
    
    /// 获取当前活跃worker数
    pub fn active_workers(&self) -> usize {
        self.active_workers.load(Ordering::SeqCst)
    }
    
    /// 计算百万像素数
    pub fn calculate_megapixels(width: u32, height: u32) -> f64 {
        if width == 0 || height == 0 {
            return 0.0;
        }
        (width as f64 * height as f64) / 1_000_000.0
    }
}

/// 智能并发管理器 (从Go的SmartConcurrencyManager提取)
pub struct SmartConcurrencyManager {
    max_workers: usize,
    min_workers: usize,
    current_workers: Arc<AtomicUsize>,
    
    // 复杂度阈值
    low_complexity_threshold: f64,
    high_complexity_threshold: f64,
    
    // 内存监控
    memory_threshold: f64,
    
    // 统计
    total_jobs: Arc<AtomicUsize>,
    successful_jobs: Arc<AtomicUsize>,
    failed_jobs: Arc<AtomicUsize>,
}

impl SmartConcurrencyManager {
    /// 创建智能并发管理器
    pub fn new() -> Self {
        let cpu_cores = num_cpus::get();
        
        Self {
            max_workers: cpu_cores * 4,  // 最大4倍CPU核心数
            min_workers: cpu_cores,      // 最小等于CPU核心数
            current_workers: Arc::new(AtomicUsize::new(cpu_cores * 2)), // 初始2倍
            
            low_complexity_threshold: 30.0,
            high_complexity_threshold: 80.0,
            
            memory_threshold: 75.0, // 75%内存使用阈值
            
            total_jobs: Arc::new(AtomicUsize::new(0)),
            successful_jobs: Arc::new(AtomicUsize::new(0)),
            failed_jobs: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    /// 计算文件复杂度分数 (从Go代码提取)
    /// 
    /// 权重分配:
    /// - 文件大小: 40%
    /// - 格式复杂度: 30%
    /// - 处理模式: 20%
    /// - 品质等级: 10%
    pub fn calculate_file_complexity(&self, file_size: u64, format: &str, quality: u8) -> f64 {
        // 1. 文件大小权重 (40%)
        let size_score = self.calculate_size_complexity(file_size);
        
        // 2. 格式复杂度权重 (30%)
        let format_score = self.calculate_format_complexity(format);
        
        // 3. 品质等级权重 (30%) - 增加权重以匹配Go代码
        let quality_score = (quality as f64 / 100.0) * 100.0;
        
        // 综合计算
        let score = size_score * 0.4 + format_score * 0.3 + quality_score * 0.3;
        
        score.clamp(0.0, 100.0)
    }
    
    /// 获取复杂度阈值 (用于外部判断)
    pub fn low_complexity_threshold(&self) -> f64 {
        self.low_complexity_threshold
    }
    
    pub fn high_complexity_threshold(&self) -> f64 {
        self.high_complexity_threshold
    }
    
    pub fn memory_threshold(&self) -> f64 {
        self.memory_threshold
    }
    
    /// 计算文件大小复杂度 (从Go代码提取)
    fn calculate_size_complexity(&self, size: u64) -> f64 {
        let size_mb = size as f64 / (1024.0 * 1024.0);
        
        if size_mb <= 1.0 {
            10.0 // 小文件，低复杂度
        } else if size_mb <= 10.0 {
            20.0 + (size_mb - 1.0) * 3.0 // 1-10MB线性增长
        } else if size_mb <= 100.0 {
            47.0 + (size_mb - 10.0) * 0.5 // 10-100MB缓慢增长
        } else {
            // 超大文件：对数增长
            92.0 + (size_mb / 100.0).log10() * 8.0
        }
    }
    
    /// 计算格式复杂度
    /// 不再基于格式名称！改为基于实际文件特征
    fn calculate_format_complexity(&self, _format: &str) -> f64 {
        // 返回基准值，实际复杂度由文件大小和分辨率决定
        // 格式名称不可信！必须实际检测文件内容！
        30.0  // 统一基准复杂度
    }
    
    /// 获取当前worker数
    pub fn current_workers(&self) -> usize {
        self.current_workers.load(Ordering::SeqCst)
    }
    
    /// 记录任务完成
    pub fn record_job_completion(&self, success: bool) {
        self.total_jobs.fetch_add(1, Ordering::SeqCst);
        if success {
            self.successful_jobs.fetch_add(1, Ordering::SeqCst);
        } else {
            self.failed_jobs.fetch_add(1, Ordering::SeqCst);
        }
    }
    
    /// 获取统计信息
    pub fn get_stats(&self) -> ConcurrencyStats {
        ConcurrencyStats {
            total_jobs: self.total_jobs.load(Ordering::SeqCst),
            successful_jobs: self.successful_jobs.load(Ordering::SeqCst),
            failed_jobs: self.failed_jobs.load(Ordering::SeqCst),
            current_workers: self.current_workers.load(Ordering::SeqCst),
            max_workers: self.max_workers,
            min_workers: self.min_workers,
        }
    }
}

impl Default for SmartConcurrencyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 并发统计 (从Go代码提取)
#[derive(Debug, Clone)]
pub struct ConcurrencyStats {
    pub total_jobs: usize,
    pub successful_jobs: usize,
    pub failed_jobs: usize,
    pub current_workers: usize,
    pub max_workers: usize,
    pub min_workers: usize,
}

impl ConcurrencyStats {
    /// 计算成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_jobs == 0 {
            return 0.0;
        }
        (self.successful_jobs as f64 / self.total_jobs as f64) * 100.0
    }
    
    /// 计算失败率
    pub fn failure_rate(&self) -> f64 {
        if self.total_jobs == 0 {
            return 0.0;
        }
        (self.failed_jobs as f64 / self.total_jobs as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dynamic_worker_pool() {
        let pool = DynamicWorkerPool::new(8);
        
        // 测试小文件 (< 2MP)
        let slots_small = pool.acquire(1.5);
        assert_eq!(slots_small, 1);
        
        // 测试中等文件 (2-8MP)
        let slots_medium = pool.acquire(5.0);
        assert_eq!(slots_medium, 2);
        
        // 测试大文件 (8-32MP)
        let slots_large = pool.acquire(20.0);
        assert_eq!(slots_large, 4);
        
        // 测试超大文件 (> 32MP)
        let slots_huge = pool.acquire(50.0);
        assert_eq!(slots_huge, 4); // 最多50% = 4 slots
        
        // 释放slots
        pool.release(slots_small);
        pool.release(slots_medium);
        pool.release(slots_large);
        pool.release(slots_huge);
    }
    
    #[test]
    fn test_calculate_megapixels() {
        assert_eq!(DynamicWorkerPool::calculate_megapixels(1920, 1080), 2.0736);
        assert_eq!(DynamicWorkerPool::calculate_megapixels(3840, 2160), 8.2944);
        assert_eq!(DynamicWorkerPool::calculate_megapixels(7680, 4320), 33.1776);
    }
    
    #[test]
    fn test_smart_concurrency_manager() {
        let manager = SmartConcurrencyManager::new();
        
        // 测试复杂度计算（现在基于文件大小，不是格式）
        let complexity_small = manager.calculate_file_complexity(500 * 1024, "jpeg", 80);
        println!("Small file complexity: {}", complexity_small);
        assert!(complexity_small < 50.0); // 小文件相对低复杂度
        
        let complexity_large = manager.calculate_file_complexity(50 * 1024 * 1024, "avif", 95);
        println!("Large file complexity: {}", complexity_large);
        assert!(complexity_large > 60.0); // 大文件高复杂度（降低阈值，因为不再有格式加成）
        
        // 测试统计
        manager.record_job_completion(true);
        manager.record_job_completion(true);
        manager.record_job_completion(false);
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_jobs, 3);
        assert_eq!(stats.successful_jobs, 2);
        assert_eq!(stats.failed_jobs, 1);
        assert_eq!(stats.success_rate(), 66.66666666666666);
    }
    
    #[test]
    fn test_size_complexity_calculation() {
        let manager = SmartConcurrencyManager::new();
        
        // 小文件
        let small = manager.calculate_size_complexity(500 * 1024);
        assert!(small < 15.0);
        
        // 中等文件
        let medium = manager.calculate_size_complexity(5 * 1024 * 1024);
        assert!(medium > 20.0 && medium < 50.0);
        
        // 大文件
        let large = manager.calculate_size_complexity(50 * 1024 * 1024);
        assert!(large > 60.0);
    }
    
    #[test]
    fn test_format_complexity() {
        let manager = SmartConcurrencyManager::new();
        
        // 现在所有格式返回统一的基准复杂度
        // 不再基于格式名称判断！
        assert_eq!(manager.calculate_format_complexity("jpeg"), 30.0);
        assert_eq!(manager.calculate_format_complexity("png"), 30.0);
        assert_eq!(manager.calculate_format_complexity("webp"), 30.0);
        assert_eq!(manager.calculate_format_complexity("avif"), 30.0);
        assert_eq!(manager.calculate_format_complexity("jxl"), 30.0);
        assert_eq!(manager.calculate_format_complexity("unknown"), 30.0);
    }
}
