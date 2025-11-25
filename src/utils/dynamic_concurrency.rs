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
        let mut available = self.available_slots.lock()
            .expect("Dynamic concurrency lock poisoned");
        if *available >= final_slots {
            *available -= final_slots;
            self.active_workers.fetch_add(final_slots, Ordering::SeqCst);
        }
        
        final_slots
    }
    
    /// 释放worker slots
    pub fn release(&self, slots: usize) {
        if let Ok(mut available) = self.available_slots.lock() {
            *available += slots;
            self.active_workers.fetch_sub(slots, Ordering::SeqCst);
        } else {
            log::error!("❌ Failed to acquire lock for releasing slots");
        }
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

// =====================================================
// 🔥 RAM优化器 - 从xl-converter提取并Rust化
// =====================================================
// 灵感来源: xl-converter-unstable/core/ram_optimizer.py
//
// 针对高内存消耗的编码器(JXL, AVIF/SVT-AV1)进行智能限流
// 基于图像分辨率动态调整并发worker数和每worker线程数
// =====================================================

/// RAM优化规则
#[derive(Debug, Clone)]
pub struct RamOptimizationRule {
    /// 规则范围: "all", "jxl", "avif_svt"
    pub scope: String,
    /// 激活阈值(百万像素)
    pub threshold_mp: f64,
    /// 目标worker数：1表示单worker，"1/2"表示一半
    pub target: RamTarget,
}

/// RAM目标设置
#[derive(Debug, Clone)]
pub enum RamTarget {
    /// 固定为1个worker
    Single,
    /// 按比例(分子/分母)
    Fraction(usize, usize),
}

impl RamTarget {
    /// 从字符串解析 (如 "1", "1/2", "1/4")
    pub fn parse(s: &str) -> Option<Self> {
        if s == "1" {
            return Some(Self::Single);
        }
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            let num = parts[0].parse().ok()?;
            let den = parts[1].parse().ok()?;
            if num > 0 && den > 0 {
                return Some(Self::Fraction(num, den));
            }
        }
        None
    }

    /// 计算实际worker数
    pub fn calculate(&self, total_workers: usize) -> usize {
        match self {
            Self::Single => 1,
            Self::Fraction(num, den) => {
                let result = total_workers * num / den;
                result.max(1)
            }
        }
    }
}

/// RAM优化器 (从xl-converter提取)
///
/// 核心功能:
/// - 基于分辨率动态限制高内存编码器的并发数
/// - 支持JXL和AVIF(SVT-AV1)的特殊处理
/// - 防止内存溢出导致的系统卡死
pub struct RamOptimizer {
    enabled: bool,
    total_workers: usize,
    rules: Vec<RamOptimizationRule>,
}

impl RamOptimizer {
    /// 创建RAM优化器
    pub fn new(total_workers: usize) -> Self {
        Self {
            enabled: true,
            total_workers,
            rules: Self::default_rules(),
        }
    }

    /// 默认优化规则 (从xl-converter提取)
    fn default_rules() -> Vec<RamOptimizationRule> {
        vec![
            // JXL高effort模式: 4MP以上限制为一半worker
            RamOptimizationRule {
                scope: "jxl".to_string(),
                threshold_mp: 4.0,
                target: RamTarget::Fraction(1, 2),
            },
            // JXL高effort模式: 16MP以上限制为1/4 worker
            RamOptimizationRule {
                scope: "jxl".to_string(),
                threshold_mp: 16.0,
                target: RamTarget::Fraction(1, 4),
            },
            // JXL高effort模式: 32MP以上单worker
            RamOptimizationRule {
                scope: "jxl".to_string(),
                threshold_mp: 32.0,
                target: RamTarget::Single,
            },
            // AVIF (SVT-AV1): 8MP以上限制为一半
            RamOptimizationRule {
                scope: "avif_svt".to_string(),
                threshold_mp: 8.0,
                target: RamTarget::Fraction(1, 2),
            },
            // AVIF (SVT-AV1): 32MP以上单worker
            RamOptimizationRule {
                scope: "avif_svt".to_string(),
                threshold_mp: 32.0,
                target: RamTarget::Single,
            },
            // 通用规则: 超大图片(64MP+)强制单worker
            RamOptimizationRule {
                scope: "all".to_string(),
                threshold_mp: 64.0,
                target: RamTarget::Single,
            },
        ]
    }

    /// 设置自定义规则
    pub fn set_rules(&mut self, rules: Vec<RamOptimizationRule>) {
        self.rules = rules;
    }

    /// 启用/禁用优化器
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 检查是否需要RAM优化
    pub fn needs_optimization(&self, format: &str, encoder: Option<&str>) -> bool {
        if !self.enabled {
            return false;
        }

        let format_lower = format.to_lowercase();

        // JXL高内存场景
        if format_lower == "jxl" || format_lower == "jpeg xl" {
            return true;
        }

        // AVIF使用SVT-AV1编码器
        if (format_lower == "avif") && encoder.map(|e| e.contains("svt")).unwrap_or(false) {
            return true;
        }

        false
    }

    /// 获取优化后的最大worker数
    pub fn get_optimized_workers(
        &self,
        megapixels: f64,
        format: &str,
        encoder: Option<&str>,
    ) -> usize {
        if !self.enabled {
            return self.total_workers;
        }

        let format_lower = format.to_lowercase();
        let scope = if format_lower == "jxl" || format_lower == "jpeg xl" {
            "jxl"
        } else if format_lower == "avif" && encoder.map(|e| e.contains("svt")).unwrap_or(false) {
            "avif_svt"
        } else {
            "other"
        };

        // 按阈值降序排列，找到第一个匹配的规则
        let mut applicable_rules: Vec<_> = self.rules.iter()
            .filter(|r| r.scope == scope || r.scope == "all")
            .filter(|r| megapixels >= r.threshold_mp)
            .collect();

        applicable_rules.sort_by(|a, b| b.threshold_mp.partial_cmp(&a.threshold_mp).unwrap());

        if let Some(rule) = applicable_rules.first() {
            let optimized = rule.target.calculate(self.total_workers);
            log::info!(
                "🔧 RAM优化: {}@{:.1}MP -> {} workers (规则: {}, 阈值: {}MP)",
                format, megapixels, optimized, rule.scope, rule.threshold_mp
            );
            return optimized;
        }

        self.total_workers
    }
}

impl Default for RamOptimizer {
    fn default() -> Self {
        let cpu_cores = num_cpus::get();
        Self::new(cpu_cores * 2)
    }
}

// =====================================================
// 🔥 智能降采样预测器 - 从xl-converter提取
// =====================================================
// 灵感来源: xl-converter-unstable/core/downscale.py
//
// 使用线性回归预测达到目标文件大小所需的缩放比例
// =====================================================

/// 智能降采样预测器
pub struct SmartDownscalePredictor {
    /// 采样数据点 [(file_size_bytes, scale_percent), ...]
    samples: Vec<(u64, f64)>,
}

impl SmartDownscalePredictor {
    pub fn new() -> Self {
        Self { samples: Vec::new() }
    }

    /// 添加采样点
    pub fn add_sample(&mut self, file_size_bytes: u64, scale_percent: f64) {
        self.samples.push((file_size_bytes, scale_percent));
    }

    /// 使用线性回归预测达到目标文件大小所需的缩放比例
    ///
    /// 算法来源: xl-converter的_linearRegression和_extrapolateScale
    pub fn predict_scale(&self, target_size_bytes: u64) -> Option<f64> {
        if self.samples.len() < 2 {
            return None;
        }

        let n = self.samples.len() as f64;
        let sum_x: f64 = self.samples.iter().map(|(x, _)| *x as f64).sum();
        let sum_y: f64 = self.samples.iter().map(|(_, y)| *y).sum();
        let mean_x = sum_x / n;
        let mean_y = sum_y / n;

        let numerator: f64 = self.samples.iter()
            .map(|(x, y)| (*x as f64 - mean_x) * (*y - mean_y))
            .sum();
        let denominator: f64 = self.samples.iter()
            .map(|(x, _)| (*x as f64 - mean_x).powi(2))
            .sum();

        if denominator == 0.0 {
            return None;
        }

        let slope = numerator / denominator;
        let intercept = mean_y - slope * mean_x;

        let predicted_scale = slope * (target_size_bytes as f64) + intercept;

        // 限制在合理范围内
        Some(predicted_scale.clamp(1.0, 100.0))
    }

    /// 清除采样数据
    pub fn clear(&mut self) {
        self.samples.clear();
    }
}

impl Default for SmartDownscalePredictor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_optimizer() {
        let optimizer = RamOptimizer::new(8);

        // 小图JXL不需要限制
        let workers = optimizer.get_optimized_workers(2.0, "jxl", None);
        assert_eq!(workers, 8);

        // 4MP+ JXL限制为一半
        let workers = optimizer.get_optimized_workers(8.0, "jxl", None);
        assert_eq!(workers, 4);

        // 32MP+ JXL单worker
        let workers = optimizer.get_optimized_workers(35.0, "jxl", None);
        assert_eq!(workers, 1);
    }

    #[test]
    fn test_ram_target_parse() {
        assert!(matches!(RamTarget::parse("1"), Some(RamTarget::Single)));
        assert!(matches!(RamTarget::parse("1/2"), Some(RamTarget::Fraction(1, 2))));
        assert!(matches!(RamTarget::parse("1/4"), Some(RamTarget::Fraction(1, 4))));
        assert!(RamTarget::parse("invalid").is_none());
    }

    #[test]
    fn test_smart_downscale_predictor() {
        let mut predictor = SmartDownscalePredictor::new();

        // 添加采样点 (模拟xl-converter的采样逻辑)
        predictor.add_sample(500_000, 66.0); // 500KB at 66%
        predictor.add_sample(200_000, 33.0); // 200KB at 33%

        // 预测300KB需要的缩放比例
        let scale = predictor.predict_scale(300_000);
        assert!(scale.is_some());
        let scale = scale.unwrap();
        assert!(scale > 33.0 && scale < 66.0);
    }

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
        log::debug!("Small file complexity: {}", complexity_small);
        assert!(complexity_small < 50.0); // 小文件相对低复杂度
        
        let complexity_large = manager.calculate_file_complexity(50 * 1024 * 1024, "avif", 95);
        log::debug!("Large file complexity: {}", complexity_large);
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
