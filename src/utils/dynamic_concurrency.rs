// 🚀 dynamicconcurrent Manager - from Goextraction
// based onfilesize/resolutiondynamicadjustedconcurrent
//
// Corefeature (from Goextraction):
// - dynamicworkerpool (based onmegapixelsadjustedslots)
// - intelligentcomplexitycalculation (filesize、format、mode、quality)
// - memory and shouldadjusted
// - lockmachine (largefilemostmulti50% slots)

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

/// dynamic Workerpool (from Go Worker Poolextraction)
pub struct DynamicWorkerPool {
 max_workers: usize,
 active_workers: Arc<AtomicUsize>,
 available_slots: Arc<Mutex<usize>>,
}

impl DynamicWorkerPool {
/// createdynamicworkerpool
 pub fn new(max_workers: usize) -> Self {
 Self {
 max_workers,
 active_workers: Arc::new(AtomicUsize::new(0)),
 available_slots: Arc::new(Mutex::new(max_workers)),
 }
 }

/// getworker slot (based onfilesizedynamicadjusted)
///
/// dynamicstrategy (from Goextraction):
/// - < 2MP: 1 slot (smallfile,concurrentprocessing)
/// - 2-8MP: 2 slots ( etc file,degreelimit)
/// - 8-32MP: 4 slots (largefile,limit)
/// - > 32MP: max Workers/2 (largefile,limitforpool50%,lock)
 pub fn acquire(&self, megapixels: f64) -> usize {
 let slots_needed = if megapixels < 2.0 {
 1
 } else if megapixels < 8.0 {
 2
 } else if megapixels < 32.0 {
 4
 } else {
// lock：largefilemostmulti50% slots
 let max_slots_per_file = self.max_workers / 2;
 max_slots_per_file.max(4)
 };

// ensure not exceedsmaximumworker50%
 let max_slots_per_file = self.max_workers / 2;
 let final_slots = slots_needed.min(max_slots_per_file.max(1));

// slots
 let mut available = self.available_slots.lock()
 .expect("Dynamic concurrency lock poisoned");
 if *available >= final_slots {
 *available -= final_slots;
 self.active_workers.fetch_add(final_slots, Ordering::SeqCst);
 }

 final_slots
 }

/// worker slots
 pub fn release(&self, slots: usize) {
 if let Ok(mut available) = self.available_slots.lock() {
 *available += slots;
 self.active_workers.fetch_sub(slots, Ordering::SeqCst);
 } else {
 log::error!("Failed to acquire lock for releasing slots");
 }
 }

/// getcurrentworker
 pub fn active_workers(&self) -> usize {
 self.active_workers.load(Ordering::SeqCst)
 }

/// calculationhundredten thousandpixel
 pub fn calculate_megapixels(width: u32, height: u32) -> f64 {
 if width == 0 || height == 0 {
 return 0.0;
 }
 (width as f64 * height as f64) / 1_000_000.0
 }
}

/// intelligentconcurrent Manager (from Go Smart Concurrency Managerextraction)
pub struct SmartConcurrencyManager {
 max_workers: usize,
 min_workers: usize,
 current_workers: Arc<AtomicUsize>,

// complexitythreshold
 low_complexity_threshold: f64,
 high_complexity_threshold: f64,

// memory
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
 max_workers: cpu_cores * 4, // maximum4CPUcore
 min_workers: cpu_cores, // minimumetcatCPUcore
 current_workers: Arc::new(AtomicUsize::new(cpu_cores * 2)), // 2

 low_complexity_threshold: 30.0,
 high_complexity_threshold: 80.0,

 memory_threshold: 75.0, // 75%memoryusingthreshold

 total_jobs: Arc::new(AtomicUsize::new(0)),
 successful_jobs: Arc::new(AtomicUsize::new(0)),
 failed_jobs: Arc::new(AtomicUsize::new(0)),
 }
 }

/// calculationfilecomplexityscore (from Goextraction)
///
/// weight:
/// - filesize: 40%
/// - formatcomplexity: 30%
/// - processingmode: 20%
/// -  etc level: 10%
 pub fn calculate_file_complexity(&self, file_size: u64, format: &str, quality: u8) -> f64 {
// 1. filesizeweight (40%)
 let size_score = self.calculate_size_complexity(file_size);

// 2. formatcomplexityweight (30%)
 let format_score = self.calculate_format_complexity(format);

// 3.  etc levelweight (30%) - addweightbymatch Go
 let quality_score = (quality as f64 / 100.0) * 100.0;

// comprehensivecalculation
 let score = size_score * 0.4 + format_score * 0.3 + quality_score * 0.3;

 score.clamp(0.0, 100.0)
 }

/// getcomplexitythreshold (forexternal)
 pub fn low_complexity_threshold(&self) -> f64 {
 self.low_complexity_threshold
 }

 pub fn high_complexity_threshold(&self) -> f64 {
 self.high_complexity_threshold
 }

 pub fn memory_threshold(&self) -> f64 {
 self.memory_threshold
 }

/// calculationfilesizecomplexity (from Goextraction)
 fn calculate_size_complexity(&self, size: u64) -> f64 {
 let size_mb = size as f64 / (1024.0 * 1024.0);

 if size_mb <= 1.0 {
 10.0 // smallfile，lowcomplexity
 } else if size_mb <= 10.0 {
 20.0 + (size_mb - 1.0) * 3.0 // 1-10MBlinelong
 } else if size_mb <= 100.0 {
 47.0 + (size_mb - 10.0) * 0.5 // 10-100MBslowlong
 } else {
// largefile：pairlong
 92.0 + (size_mb / 100.0).log10() * 8.0
 }
 }

/// calculationformatcomplexity
/// not againbased onformatname！forbased onactualfilefeature
 fn calculate_format_complexity(&self, _format: &str) -> f64 {
// returnvalue，actualcomplexityfilesize and resolution
// formatname not can！mustactualdetectionfileinside！
 30.0 // acomplexity
 }

/// getcurrentworker
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

/// concurrentstatistics (from Goextraction)
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
/// Calculate success rate
 pub fn success_rate(&self) -> f64 {
 if self.total_jobs == 0 {
 return 0.0;
 }
 (self.successful_jobs as f64 / self.total_jobs as f64) * 100.0
 }

/// calculationfailure
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
// source: xl-converter-unstable/core/ram_optimizer.py
//
// forhighmemoryEncoder(JXL, AVIF/SVT-AV1)lineintelligent
// based onimageresolutiondynamicadjustedconcurrentworker and everyworkerthread
// =====================================================

/// RAMoptimizationthen
#[derive(Debug, Clone)]
pub struct RamOptimizationRule {
/// thenrange: "all", "jxl", "avif_svt"
 pub scope: String,
/// activatedthreshold(hundredten thousandpixel)
 pub threshold_mp: f64,
/// targetworker：1singleworker，"1/2"ahalf
 pub target: RamTarget,
}

/// RAMtargetsetting
#[derive(Debug, Clone)]
pub enum RamTarget {
/// for1worker
 Single,
/// (sub/母)
 Fraction(usize, usize),
}

impl RamTarget {
/// fromstringparse (like "1", "1/2", "1/4")
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

/// calculationactualworker
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
/// - based onresolutiondynamiclimithighmemory Encoderconcurrent
/// - support JXL and AVIF(SVT-AV1)processing
/// - memorySystem
pub struct RamOptimizer {
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

/// defaultoptimizationthen (fromxl-converterextraction)
 fn default_rules() -> Vec<RamOptimizationRule> {
 vec![
// JXLhigheffortmode: 4MPabovelimitforahalfworker
 RamOptimizationRule {
 scope: "jxl".to_string(),
 threshold_mp: 4.0,
 target: RamTarget::Fraction(1, 2),
 },
// JXLhigheffortmode: 16MPabovelimitfor1/4 worker
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
// AVIF (SVT-AV1): 8MPabovelimitforahalf
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
// 通用then: large(64MP+)forcesingleworker
 RamOptimizationRule {
 scope: "all".to_string(),
 threshold_mp: 64.0,
 target: RamTarget::Single,
 },
 ]
 }

/// settingcustomthen
 pub fn set_rules(&mut self, rules: Vec<RamOptimizationRule>) {
 self.rules = rules;
 }

/// enabled/disabledoptimization
 pub fn set_enabled(&mut self, enabled: bool) {
 self.enabled = enabled;
 }

/// checkisnoneed RAMoptimization
 pub fn needs_optimization(&self, format: &str, encoder: Option<&str>) -> bool {
 if !self.enabled {
 return false;
 }

 let format_lower = format.to_lowercase();

// JXLhighmemory
 if format_lower == "jxl" || format_lower == "jpeg xl" {
 return true;
 }

// AVIFuse SVT-AV1Encoder
 if (format_lower == "avif") && encoder.map(|e| e.contains("svt")).unwrap_or(false) {
 return true;
 }

 false
 }

/// getoptimizedmaximumworker
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

// threshold，findtofirstmatchthen
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
// 🔥 intelligentsamplingprediction - fromxl-converterextraction
// =====================================================
// source: xl-converter-unstable/core/downscale.py
//
// uselineregressionpredictiontotargetfilesizeneedscale
// =====================================================

/// intelligentsamplingprediction
pub struct SmartDownscalePredictor {
/// samplingdatapoint [(file_size_bytes, scale_percent), ...]
 samples: Vec<(u64, f64)>,
}

impl SmartDownscalePredictor {
 pub fn new() -> Self {
 Self { samples: Vec::new() }
 }

/// addsamplingpoint
 pub fn add_sample(&mut self, file_size_bytes: u64, scale_percent: f64) {
 self.samples.push((file_size_bytes, scale_percent));
 }

/// uselineregressionpredictiontotargetfilesizeneedscale
///
/// algorithmsource: xl-converter_linear Regression and _extrapolate Scale
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

// limitatrangeinside
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

// smallJXL not needlimit
 let workers = optimizer.get_optimized_workers(2.0, "jxl", None);
 assert_eq!(workers, 8);

// 4MP+ JXLlimitforahalf
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

// addsamplingpoint (simulatedxl-convertersampling)
 predictor.add_sample(500_000, 66.0); // 500KB at 66%
 predictor.add_sample(200_000, 33.0); // 200KB at 33%

// prediction300KBneedscale
 let scale = predictor.predict_scale(300_000);
 assert!(scale.is_some());
 let scale = scale.unwrap();
 assert!(scale > 33.0 && scale < 66.0);
 }

 #[test]
 fn test_dynamic_worker_pool() {
 let pool = DynamicWorkerPool::new(8);

// testsmallfile (< 2MP)
 let slots_small = pool.acquire(1.5);
 assert_eq!(slots_small, 1);

// testing etc file (2-8MP)
 let slots_medium = pool.acquire(5.0);
 assert_eq!(slots_medium, 2);

// testlargefile (8-32MP)
 let slots_large = pool.acquire(20.0);
 assert_eq!(slots_large, 4);

// testlargefile (> 32MP)
 let slots_huge = pool.acquire(50.0);
 assert_eq!(slots_huge, 4); // mostmulti50% = 4 slots

// slots
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

// testcomplexitycalculation（atbased onfilesize， not isformat）
 let complexity_small = manager.calculate_file_complexity(500 * 1024, "jpeg", 80);
 log::debug!("Small file complexity: {}", complexity_small);
 assert!(complexity_small < 50.0); // smallfilepairlowcomplexity

 let complexity_large = manager.calculate_file_complexity(50 * 1024 * 1024, "avif", 95);
 log::debug!("Large file complexity: {}", complexity_large);
 assert!(complexity_large > 60.0); // largefilehighcomplexity（lowthreshold，becausefornotagainhasformat）

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

// smallfile
 let small = manager.calculate_size_complexity(500 * 1024);
 assert!(small < 15.0);

// etc file
 let medium = manager.calculate_size_complexity(5 * 1024 * 1024);
 assert!(medium > 20.0 && medium < 50.0);

// largefile
 let large = manager.calculate_size_complexity(50 * 1024 * 1024);
 assert!(large > 60.0);
 }

 #[test]
 fn test_format_complexity() {
 let manager = SmartConcurrencyManager::new();

// at has formatreturn Unifiedcomplexity
// not againbased onformatname！
 assert_eq!(manager.calculate_format_complexity("jpeg"), 30.0);
 assert_eq!(manager.calculate_format_complexity("png"), 30.0);
 assert_eq!(manager.calculate_format_complexity("webp"), 30.0);
 assert_eq!(manager.calculate_format_complexity("avif"), 30.0);
 assert_eq!(manager.calculate_format_complexity("jxl"), 30.0);
 assert_eq!(manager.calculate_format_complexity("unknown"), 30.0);
 }
}
