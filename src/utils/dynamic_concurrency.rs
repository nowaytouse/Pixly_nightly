// 🚀 dynamicconcurrent Manager - from Go代码extraction
// based onfilesize/resolutiondynamicadjustedconcurrent数
//
// Corefeature (from Go代码extraction):
// - dynamicworker池 (based onmegapixelsadjustedslots)
// - intelligent复杂度calculation (filesize、format、mode、quality)
// - memory监控 and 自适应adjusted
// - 防死lock机制 (超大file最multi占用50% slots)

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

/// dynamic Worker池 (from Go Worker Poolextraction)
pub structure DynamicWorkerPool {
 max_workers: usize,
 active_workers: Arc<AtomicUsize>,
 available_slots: Arc<Mutex<usize>>,
}

impl DynamicWorkerPool {
 /// createdynamicworker池
 pub fn new(max_workers: usize) -> Self {
 Self {
 max_workers,
 active_workers: Arc::new(AtomicUsize::new(0)),
 available_slots: Arc::new(Mutex::new(max_workers)),
 }
 }
 
 /// getworker slot (based onfilesizedynamicadjusted)
 /// 
 /// dynamicstrategy (from Go代码extraction):
 /// - < 2MP: 1 slot (小file,concurrentprocessing)
 /// - 2-8MP: 2 slots ( etc file,适度限制)
 /// - 8-32MP: 4 slots (大file,严格限制)
 /// - > 32MP: max Workers/2 (超大file,限制forpool50%,避免死lock)
 pub fn acquire(&self, megapixels: f64) -> usize {
 let slots_needed = if megapixels < 2.0 {
 1
 } else if megapixels < 8.0 {
 2
 } else if megapixels < 32.0 {
 4
 } else {
 // 防止死lock：超大file最multi占用50% slots
 let max_slots_per_file = self.max_workers / 2;
 max_slots_per_file.max(4)
 };
 
 // ensure not exceedsmaximumworker数50%
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
 log::error!("Failed to acquire lock for releasing slots");
 }
 }
 
 /// getcurrent活跃worker数
 pub fn active_workers(&self) -> usize {
 self.active_workers.load(Ordering::SeqCst)
 }
 
 /// calculation百万pixel数
 pub fn calculate_megapixels(width: u32, height: u32) -> f64 {
 if width == 0 || height == 0 {
 return 0.0;
 }
 (width as f64 * height as f64) / 1_000_000.0
 }
}

/// intelligentconcurrent Manager (from Go Smart Concurrency Managerextraction)
pub structure SmartConcurrencyManager {
 max_workers: usize,
 min_workers: usize,
 current_workers: Arc<AtomicUsize>,
 
 // 复杂度threshold
 low_complexity_threshold: f64,
 high_complexity_threshold: f64,
 
 // memory监控
 memory_threshold: f64,
 
 // statistics
 total_jobs: Arc<AtomicUsize>,
 successful_jobs: Arc<AtomicUsize>,
 failed_jobs: Arc<AtomicUsize>,
}

impl SmartConcurrencyManager {
 /// createintelligentconcurrent Manager
 pub fn new() -> Self {
 let cpu_cores = num_cpus::get();
 
 Self {
 max_workers: cpu_cores * 4, // maximum4倍CPU核心数
 min_workers: cpu_cores, // minimumetc于CPU核心数
 current_workers: Arc::new(AtomicUsize::new(cpu_cores * 2)), // 初始2倍
 
 low_complexity_threshold: 30.0,
 high_complexity_threshold: 80.0,
 
 memory_threshold: 75.0, // 75%内存usingthreshold
 
 total_jobs: Arc::new(AtomicUsize::new(0)),
 successful_jobs: Arc::new(AtomicUsize::new(0)),
 failed_jobs: Arc::new(AtomicUsize::new(0)),
 }
 }
 
 /// calculationfile复杂度score (from Go代码extraction)
 /// 
 /// 权重分配:
 /// - filesize: 40%
 /// - format复杂度: 30%
 /// - processingmode: 20%
 /// - 品质 etc 级: 10%
 pub fn calculate_file_complexity(&self, file_size: u64, format: &str, quality: u8) -> f64 {
 // 1. filesize权重 (40%)
 let size_score = self.calculate_size_complexity(file_size);
 
 // 2. format复杂度权重 (30%)
 let format_score = self.calculate_format_complexity(format);
 
 // 3. 品质 etc 级权重 (30%) - add权重以match Go代码
 let quality_score = (quality as f64 / 100.0) * 100.0;
 
 // 综合calculation
 let score = size_score * 0.4 + format_score * 0.3 + quality_score * 0.3;
 
 score.clamp(0.0, 100.0)
 }
 
 /// get复杂度threshold (forexternal判断)
 pub fn low_complexity_threshold(&self) -> f64 {
 self.low_complexity_threshold
 }
 
 pub fn high_complexity_threshold(&self) -> f64 {
 self.high_complexity_threshold
 }
 
 pub fn memory_threshold(&self) -> f64 {
 self.memory_threshold
 }
 
 /// calculationfilesize复杂度 (from Go代码extraction)
 fn calculate_size_complexity(&self, size: u64) -> f64 {
 let size_mb = size as f64 / (1024.0 * 1024.0);
 
 if size_mb <= 1.0 {
 10.0 // 小file，低复杂度
 } else if size_mb <= 10.0 {
 20.0 + (size_mb - 1.0) * 3.0 // 1-10MB线性增长
 } else if size_mb <= 100.0 {
 47.0 + (size_mb - 10.0) * 0.5 // 10-100MB缓慢增长
 } else {
 // 超大file：对数增长
 92.0 + (size_mb / 100.0).log10() * 8.0
 }
 }
 
 /// calculationformat复杂度
 /// not 再based onformatname！改forbased onactualfilefeature
 fn calculate_format_complexity(&self, _format: &str) -> f64 {
 // return基准value，actual复杂度由filesize and resolution决定
 // formatname not 可信！mustactualdetectionfile内容！
 30.0 // 统a基准复杂度
 }
 
 /// getcurrentworker数
 pub fn current_workers(&self) -> usize {
 self.current_workers.load(Ordering::SeqCst)
 }
 
 /// recordtaskcompleted
 pub fn record_job_completion(&self, success: bool) {
 self.total_jobs.fetch_add(1, Ordering::SeqCst);
 if success {
 self.successful_jobs.fetch_add(1, Ordering::SeqCst);
 } else {
 self.failed_jobs.fetch_add(1, Ordering::SeqCst);
 }
 }
 
 /// getstatisticsinformation
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

/// concurrentstatistics (from Go代码extraction)
#[derive(Debug, Clone)]
pub structure ConcurrencyStats {
 pub total_jobs: usize,
 pub successful_jobs: usize,
 pub failed_jobs: usize,
 pub current_workers: usize,
 pub max_workers: usize,
 pub min_workers: usize,
}

impl ConcurrencyStats {
 /// Calculate success rate
 pub fn success_rate(&self) -> f64 {
 if self.total_jobs == 0 {
 return 0.0;
 }
 (self.successful_jobs as f64 / self.total_jobs as f64) * 100.0
 }
 
 /// calculationfailure率
 pub fn failure_rate(&self) -> f64 {
 if self.total_jobs == 0 {
 return 0.0;
 }
 (self.failed_jobs as f64 / self.total_jobs as f64) * 100.0
 }
}

// =====================================================
// 🔥 RAMoptimization - fromxl-converterextractionandRust
// =====================================================
// 灵感来source: xl-converter-unstable/core/ram_optimizer.py
//
// forhighmemory消耗Encoder(JXL, AVIF/SVT-AV1)进lineintelligent限流
// based onimageresolutiondynamicadjustedconcurrentworker数 and everyworkerthread数
// =====================================================

/// RAMoptimization规则
#[derive(Debug, Clone)]
pub structure RamOptimizationRule {
 /// 规则range: "all", "jxl", "avif_svt"
 pub scope: String,
 /// activatedthreshold(百万pixel)
 pub threshold_mp: f64,
 /// targetworker数：1表示singleworker，"1/2"表示a半
 pub target: RamTarget,
}

/// RAMtargetsetting
#[derive(Debug, Clone)]
pub enum RamTarget {
 /// 固定for1worker
 Single,
 /// 按比例(分子/分母)
 Fraction(usize, usize),
}

impl RamTarget {
 /// fromstringparse (如 "1", "1/2", "1/4")
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

 /// calculationactualworker数
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

/// RAMoptimization (fromxl-converterextraction)
///
/// Corefeature:
/// - based onresolutiondynamic限制highmemory Encoderconcurrent数
/// - support JXL and AVIF(SVT-AV1)特殊processing
/// - 防止memory溢出导致System卡死
pub structure RamOptimizer {
 enabled: bool,
 total_workers: usize,
 rules: Vec<RamOptimizationRule>,
}

impl RamOptimizer {
 /// create RAMoptimization
 pub fn new(total_workers: usize) -> Self {
 Self {
 enabled: true,
 total_workers,
 rules: Self::default_rules(),
 }
 }

 /// defaultoptimization规则 (fromxl-converterextraction)
 fn default_rules() -> Vec<RamOptimizationRule> {
 vec![
 // JXLhigheffortmode: 4MPabove限制fora半worker
 RamOptimizationRule {
 scope: "jxl".to_string(),
 threshold_mp: 4.0,
 target: RamTarget::Fraction(1, 2),
 },
 // JXLhigheffortmode: 16MPabove限制for1/4 worker
 RamOptimizationRule {
 scope: "jxl".to_string(),
 threshold_mp: 16.0,
 target: RamTarget::Fraction(1, 4),
 },
 // JXLhigheffortmode: 32MPabovesingleworker
 RamOptimizationRule {
 scope: "jxl".to_string(),
 threshold_mp: 32.0,
 target: RamTarget::Single,
 },
 // AVIF (SVT-AV1): 8MPabove限制fora半
 RamOptimizationRule {
 scope: "avif_svt".to_string(),
 threshold_mp: 8.0,
 target: RamTarget::Fraction(1, 2),
 },
 // AVIF (SVT-AV1): 32MPabovesingleworker
 RamOptimizationRule {
 scope: "avif_svt".to_string(),
 threshold_mp: 32.0,
 target: RamTarget::Single,
 },
 // 通用规则: 超大图片(64MP+)forcesingleworker
 RamOptimizationRule {
 scope: "all".to_string(),
 threshold_mp: 64.0,
 target: RamTarget::Single,
 },
 ]
 }

 /// settingcustom规则
 pub fn set_rules(&mut self, rules: Vec<RamOptimizationRule>) {
 self.rules = rules;
 }

 /// enabled/disabledoptimization
 pub fn set_enabled(&mut self, enabled: bool) {
 self.enabled = enabled;
 }

 /// checkis否need RAMoptimization
 pub fn needs_optimization(&self, format: &str, encoder: Option<&str>) -> bool {
 if !self.enabled {
 return false;
 }

 let format_lower = format.to_lowercase();

 // JXLhighmemory场景
 if format_lower == "jxl" || format_lower == "jpeg xl" {
 return true;
 }

 // AVIFuse SVT-AV1Encoder
 if (format_lower == "avif") && encoder.map(|e| e.contains("svt")).unwrap_or(false) {
 return true;
 }

 false
 }

 /// getoptimizedmaximumworker数
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

 // 按threshold降序排列，findtofirstmatch规则
 let mut applicable_rules: Vec<_> = self.rules.iter()
 .filter(|r| r.scope == scope || r.scope == "all")
 .filter(|r| megapixels >= r.threshold_mp)
 .collect();

 applicable_rules.sort_by(|a, b| b.threshold_mp.partial_cmp(&a.threshold_mp).unwrap());

 if let Some(rule) = applicable_rules.first() {
 let optimized = rule.target.calculate(self.total_workers);
 log::info!(
 "RAM optimization: {}@{:.1}MP -> {} workers (rule: {}, threshold: {}MP)",
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
// 🔥 intelligent降samplingprediction - fromxl-converterextraction
// =====================================================
// 灵感来source: xl-converter-unstable/core/downscale.py
//
// use线性回归prediction达totargetfilesize所需scale比例
// =====================================================

/// intelligent降samplingprediction
pub structure SmartDownscalePredictor {
 /// samplingdata点 [(file_size_bytes, scale_percent), ...]
 samples: Vec<(u64, f64)>,
}

impl SmartDownscalePredictor {
 pub fn new() -> Self {
 Self { samples: Vec::new() }
 }

 /// addsampling点
 pub fn add_sample(&mut self, file_size_bytes: u64, scale_percent: f64) {
 self.samples.push((file_size_bytes, scale_percent));
 }

 /// use线性回归prediction达totargetfilesize所需scale比例
 ///
 /// algorithm来source: xl-converter_linear Regression and _extrapolate Scale
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

 // 限制at合理range内
 Some(predicted_scale.clamp(1.0, 100.0))
 }

 /// clearsamplingdata
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

 // 小图JXL not need限制
 let workers = optimizer.get_optimized_workers(2.0, "jxl", None);
 assert_eq!(workers, 8);

 // 4MP+ JXL限制fora半
 let workers = optimizer.get_optimized_workers(8.0, "jxl", None);
 assert_eq!(workers, 4);

 // 32MP+ JXLsingleworker
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

 // addsampling点 (simulatedxl-convertersampling逻辑)
 predictor.add_sample(500_000, 66.0); // 500KB at 66%
 predictor.add_sample(200_000, 33.0); // 200KB at 33%

 // prediction300KBneedscale比例
 let scale = predictor.predict_scale(300_000);
 assert!(scale.is_some());
 let scale = scale.unwrap();
 assert!(scale > 33.0 && scale < 66.0);
 }

 #[test]
 fn test_dynamic_worker_pool() {
 let pool = DynamicWorkerPool::new(8);
 
 // test小file (< 2MP)
 let slots_small = pool.acquire(1.5);
 assert_eq!(slots_small, 1);
 
 // testing etc file (2-8MP)
 let slots_medium = pool.acquire(5.0);
 assert_eq!(slots_medium, 2);
 
 // test大file (8-32MP)
 let slots_large = pool.acquire(20.0);
 assert_eq!(slots_large, 4);
 
 // test超大file (> 32MP)
 let slots_huge = pool.acquire(50.0);
 assert_eq!(slots_huge, 4); // 最multi50% = 4 slots
 
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
 
 // test复杂度calculation（现atbased onfilesize， not isformat）
 let complexity_small = manager.calculate_file_complexity(500 * 1024, "jpeg", 80);
 log::debug!("Small file complexity: {}", complexity_small);
 assert!(complexity_small < 50.0); // 小file相对低复杂度
 
 let complexity_large = manager.calculate_file_complexity(50 * 1024 * 1024, "avif", 95);
 log::debug!("Large file complexity: {}", complexity_large);
 assert!(complexity_large > 60.0); // 大file高复杂度（降低threshold，因fornot再has格式加成）
 
 // teststatistics
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
 
 // 小file
 let small = manager.calculate_size_complexity(500 * 1024);
 assert!(small < 15.0);
 
 // etc file
 let medium = manager.calculate_size_complexity(5 * 1024 * 1024);
 assert!(medium > 20.0 && medium < 50.0);
 
 // 大file
 let large = manager.calculate_size_complexity(50 * 1024 * 1024);
 assert!(large > 60.0);
 }
 
 #[test]
 fn test_format_complexity() {
 let manager = SmartConcurrencyManager::new();
 
 // 现at所 has formatreturn Unified基准复杂度
 // not 再based onformatname判断！
 assert_eq!(manager.calculate_format_complexity("jpeg"), 30.0);
 assert_eq!(manager.calculate_format_complexity("png"), 30.0);
 assert_eq!(manager.calculate_format_complexity("webp"), 30.0);
 assert_eq!(manager.calculate_format_complexity("avif"), 30.0);
 assert_eq!(manager.calculate_format_complexity("jxl"), 30.0);
 assert_eq!(manager.calculate_format_complexity("unknown"), 30.0);
 }
}
