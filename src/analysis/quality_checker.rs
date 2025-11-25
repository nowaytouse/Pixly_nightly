/// qualitycheckmodule
/// 
/// feature：
/// 1. SSIM (structureural similarity) quality validation
/// 2. Before/after conversion comparison
/// 3. Quality report generation
use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView, Rgba};
use std::path::Path;

/// SSIM qualitycheckresult
#[derive(Debug, Clone)]
pub structure QualityCheckResult {
 /// SSIM score (0.0-1.0, 1.0 = Identical)
 pub ssim_score: f64,
 
 /// Whether passed quality check（SSIM >= 0.95）
 pub passed: bool,
 
 /// Quality grade
 pub quality_grade: QualityGrade,
 
 /// Detailed information
 pub details: String,
}

/// Quality grade
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityGrade {
 /// Excellent (SSIM >= 0.98)
 Excellent,
 /// Good (SSIM >= 0.95)
 Good,
 /// 可accept (SSIM >= 0.90)
 Acceptable,
 /// Poor (SSIM < 0.90)
 Poor,
}

impl QualityGrade {
 pub fn from_ssim(ssim: f64) -> Self {
 if ssim >= 0.98 {
 Self::Excellent
 } else if ssim >= 0.95 {
 Self::Good
 } else if ssim >= 0.90 {
 Self::Acceptable
 } else {
 Self::Poor
 }
 }
 
 pub fn as_str(&self) -> &'static str {
 match self {
 Self::Excellent => "Excellent",
 Self::Good => "Good",
 Self::Acceptable => "Acceptable",
 Self::Poor => "Poor",
 }
 }
 
 pub fn emoji(&self) -> &'static str {
 match self {
 Self::Excellent => "🌟",
 Self::Good => "✅",
 Self::Acceptable => "⚠️",
 Self::Poor => "❌",
 }
 }
}

/// SSIM qualitycheck
pub structure QualityChecker {
 /// SSIM threshold（default 0.95）
 threshold: f64,
}

impl QualityChecker {
 /// createdefaultcheck（threshold 0.95）
 pub fn new() -> Self {
 Self { threshold: 0.95 }
 }
 
 /// createcustomthresholdcheck
 pub fn with_threshold(threshold: f64) -> Self {
 Self { threshold }
 }
 
 /// Check conversion quality
 pub fn check_conversion_quality(
 &self,
 original_path: &Path,
 converted_path: &Path,
 ) -> Result<QualityCheckResult> {
 // 1. Load image
 let original = image::open(original_path)
 .with_context(|| format!("Failed to load original: {:?}", original_path))?;
 
 let converted = image::open(converted_path)
 .with_context(|| format!("Failed to load converted: {:?}", converted_path))?;
 
 // 2. checkdimension
 if original.dimensions() != converted.dimensions() {
 anyhow::bail!(
 "Image dimensions mismatch: {:?} vs {:?}",
 original.dimensions(),
 converted.dimensions()
 );
 }
 
 // 3. Calculate SSIM
 let ssim_score = self.calculate_ssim(&original, &converted)?;
 
 // 4. Generate result
 let quality_grade = QualityGrade::from_ssim(ssim_score);
 let passed = ssim_score >= self.threshold;
 
 let details = format!(
 "SSIM: {:.4} | Grade: {} | Threshold: {:.2}",
 ssim_score,
 quality_grade.as_str(),
 self.threshold
 );
 
 Ok(QualityCheckResult {
 ssim_score,
 passed,
 quality_grade,
 details,
 })
 }
 
 /// Calculate SSIM（简version）
 /// 
 /// note：thisisa简 SSIM implementation，for quick quality checking。
 /// full SSIM implementationneedconsideringmorefactors（brightness、contrast、structure）。
 fn calculate_ssim(&self, img1: &DynamicImage, img2: &DynamicImage) -> Result<f64> {
 let (width, height) = img1.dimensions();
 
 // conversionfor RGB
 let img1_rgb = img1.to_rgba8();
 let img2_rgb = img2.to_rgba8();
 
 // samplingcalculation（every 8x8 blocksamplinga次，提highperformance）
 let sample_step = 8;
 let mut total_similarity = 0.0;
 let mut sample_count = 0;
 
 for y in (0..height).step_by(sample_step) {
 for x in (0..width).step_by(sample_step) {
 let pixel1 = img1_rgb.get_pixel(x, y);
 let pixel2 = img2_rgb.get_pixel(x, y);
 
 // Calculate pixel similarity
 let similarity = self.pixel_similarity(pixel1, pixel2);
 total_similarity += similarity;
 sample_count += 1;
 }
 }
 
 if sample_count == 0 {
 return Ok(1.0);
 }
 
 Ok(total_similarity / sample_count as f64)
 }
 
 /// calculation两pixelsimilarity
 fn pixel_similarity(&self, p1: &Rgba<u8>, p2: &Rgba<u8>) -> f64 {
 // Calculate Euclidean distance
 let r_diff = (p1[0] as f64 - p2[0] as f64).powi(2);
 let g_diff = (p1[1] as f64 - p2[1] as f64).powi(2);
 let b_diff = (p1[2] as f64 - p2[2] as f64).powi(2);
 
 let distance = (r_diff + g_diff + b_diff).sqrt();
 let max_distance = (255.0_f64.powi(2) * 3.0).sqrt();
 
 // conversionforsimilarity (0.0-1.0)
 1.0 - (distance / max_distance)
 }
}

impl Default for QualityChecker {
 fn default() -> Self {
 Self::new()
 }
}

#[cfg(test)]
mod tests {
 use super::*;
 use image::Rgba;
 
 #[test]
 fn test_pixel_similarity() {
 let checker = QualityChecker::new();
 
 // Identical pixels
 let p1 = Rgba([100, 150, 200, 255]);
 let p2 = Rgba([100, 150, 200, 255]);
 let sim = checker.pixel_similarity(&p1, &p2);
 assert!((sim - 1.0).abs() < 0.001);
 
 // fullydifferentpixel
 let p1 = Rgba([0, 0, 0, 255]);
 let p2 = Rgba([255, 255, 255, 255]);
 let sim = checker.pixel_similarity(&p1, &p2);
 assert!(sim < 0.5);
 }
 
 #[test]
 fn test_quality_grade() {
 assert_eq!(QualityGrade::from_ssim(0.99), QualityGrade::Excellent);
 assert_eq!(QualityGrade::from_ssim(0.96), QualityGrade::Good);
 assert_eq!(QualityGrade::from_ssim(0.92), QualityGrade::Acceptable);
 assert_eq!(QualityGrade::from_ssim(0.85), QualityGrade::Poor);
 }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Phase 5.2: merged quality_checker_advanced.rs PSNR/MSEfeature
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// fullquality指标（contains SSIM、PSNR、MSE）
#[derive(Debug, Clone)]
pub structure AdvancedQualityMetrics {
 /// structuresimilarity指数 (0.0-1.0, morehighmore好)
 pub ssim: f64,
 /// Peak signal-to-noise ratio (d B, morehighmore好)
 pub psnr: f64,
 /// Mean squared error (morelowmore好)
 pub mse: f64,
 /// qualityevaluate
 pub assessment: QualityGrade,
}

impl QualityChecker {
 /// calculationfullquality指标（SSIM + PSNR + MSE）
 pub fn compare_advanced<P: AsRef<Path>>(
 &self,
 original_path: P,
 converted_path: P,
 ) -> Result<AdvancedQualityMetrics> {
 use image::GenericImageView;
 
 let original = image::open(original_path.as_ref())
 .with_context(|| format!("Failed to open original: {:?}", original_path.as_ref()))?;
 let converted = image::open(converted_path.as_ref())
 .with_context(|| format!("Failed to open converted: {:?}", converted_path.as_ref()))?;
 
 // ensuredimensionsame
 if original.dimensions() != converted.dimensions() {
 anyhow::bail!("Image dimensions don't match");
 }
 
 // calculationSSIM
 let ssim = self.calculate_ssim(&original, &converted)?;
 
 // calculation MSE and PSNR
 let (mse, psnr) = self.calculate_mse_psnr(&original, &converted)?;
 
 Ok(AdvancedQualityMetrics {
 ssim,
 psnr,
 mse,
 assessment: QualityGrade::from_ssim(ssim),
 })
 }
 
 /// calculationMean squared error(MSE) and Peak signal-to-noise ratio(PSNR)
 fn calculate_mse_psnr(&self, img1: &DynamicImage, img2: &DynamicImage) -> Result<(f64, f64)> {
 let rgba1 = img1.to_rgba8();
 let rgba2 = img2.to_rgba8();
 
 let (width, height) = rgba1.dimensions();
 let mut sum_squared_diff = 0.0f64;
 let pixel_count = (width * height) as f64;
 
 for y in 0..height {
 for x in 0..width {
 let p1 = rgba1.get_pixel(x, y);
 let p2 = rgba2.get_pixel(x, y);
 
 // 只calculation RGBchannel（ignorealpha）
 for i in 0..3 {
 let diff = p1[i] as f64 - p2[i] as f64;
 sum_squared_diff += diff * diff;
 }
 }
 }
 
 // MSE = sum(squared_diff) / (width * height * channels)
 let mse = sum_squared_diff / (pixel_count * 3.0);
 
 // PSNR = 10 * log10(MAX^2 / MSE)
 // MAX = 255 for 8-bit images
 let psnr = if mse > 0.0 {
 10.0 * ((255.0 * 255.0) / mse).log10()
 } else {
 f64::INFINITY // completelysame
 };
 
 Ok((mse, psnr))
 }
}


// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// REF-001: SSIMautoqualityoptimization (Inspired by Pio) - 2025-11-22
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Optimal quality parameter result
#[derive(Debug, Clone)]
pub structure OptimalQualityResult {
 /// Optimal quality parameter
 pub quality: u8,
 /// Achieved SSIM score
 pub ssim_score: f64,
 /// filesize（bytes）
 pub file_size: u64,
 /// searchiterationcount
 pub iterations: usize,
 /// Quality grade
 pub grade: QualityGrade,
}

/// SSIMautoqualityoptimization
/// 
/// Inspired by Pio project design，usetwo分searchautofindtoOptimal quality parameter
pub structure SSIMOptimizer {
 /// Target SSIM value (0.0-1.0)
 target_ssim: f64,
 /// Minimum quality parameter
 min_quality: u8,
 /// Maximum quality parameter
 max_quality: u8,
 /// qualitycheck
 checker: QualityChecker,
}

impl SSIMOptimizer {
 /// createoptimization
 /// 
 /// # parameter
 /// - `target_ssim`: Target SSIM value (recommended 0.95)
 /// - `min_quality`: minimumquality (recommended 60)
 /// - `max_quality`: maximumquality (recommended 95)
 pub fn new(target_ssim: f64, min_quality: u8, max_quality: u8) -> Self {
 Self {
 target_ssim,
 min_quality,
 max_quality,
 checker: QualityChecker::with_threshold(target_ssim),
 }
 }
 
 /// create Weboptimizationpreset (SSIM 0.95, qualityrange 70-90)
 pub fn web_optimized() -> Self {
 Self::new(0.95, 70, 90)
 }
 
 /// createhighqualitypreset (SSIM 0.98, qualityrange 85-100)
 pub fn high_quality() -> Self {
 Self::new(0.98, 85, 100)
 }
 
 /// createquickpreviewpreset (SSIM 0.90, qualityrange 60-75)
 pub fn fast_preview() -> Self {
 Self::new(0.90, 60, 75)
 }
 
 /// Find optimal quality parameter
 /// 
 /// usetwo分searchalgorithm，atspecifyqualityrange内findtomeettargetSSIMlowest qualityparameter
 /// 
 /// # parameter
 /// - `original`: Original image
 /// - `encode_fn`: Encoding function，Takes quality parameter，returnencoding after imagedata
 /// 
 /// # return
 /// Optimal quality parameter and relatedinformation
 pub fn find_optimal_quality<F>(
 &self,
 original: &DynamicImage,
 mut encode_fn: F,
 ) -> Result<OptimalQualityResult>
 where
 F: FnMut(u8) -> Result<Vec<u8>>,
 {
 let mut low = self.min_quality;
 let mut high = self.max_quality;
 let mut best_quality = high;
 let mut best_ssim = 0.0;
 let mut best_size = 0u64;
 let mut iterations = 0;
 
 log::info!("🔍 Starting SSIM optimization: target={:.3}, range=[{}, {}]", 
 self.target_ssim, low, high);
 
 while low <= high {
 iterations += 1;
 let mid = (low + high) / 2;
 
 // Encode image
 let encoded_data = encode_fn(mid)
 .with_context(|| format!("Failed to encode with quality {}", mid))?;
 
 let file_size = encoded_data.len() as u64;
 
 // savetemporaryfileforSSIMcalculation
 let temp_path = std::env::temp_dir().join(format!("pixly_ssim_test_q{}.tmp", mid));
 std::fs::write(&temp_path, &encoded_data)?;
 
 // calculationSSIM
 let decoded = image::open(&temp_path)?;
 let ssim = self.checker.calculate_ssim(original, &decoded)?;
 
 // cleanuptemporaryfile
 let _ = std::fs::remove_file(&temp_path);
 
 log::debug!(" Quality {}: SSIM={:.4}, Size={}KB", 
 mid, ssim, file_size / 1024);
 
 if ssim >= self.target_ssim {
 // SSIM meets requirement，try更lowquality
 best_quality = mid;
 best_ssim = ssim;
 best_size = file_size;
 high = mid - 1;
 } else {
 // SSIM not 足，need更highquality
 low = mid + 1;
 }
 }
 
 log::info!("✅ Optimization complete: quality={}, SSIM={:.4}, size={}KB, iterations={}", 
 best_quality, best_ssim, best_size / 1024, iterations);
 
 Ok(OptimalQualityResult {
 quality: best_quality,
 ssim_score: best_ssim,
 file_size: best_size,
 iterations,
 grade: QualityGrade::from_ssim(best_ssim),
 })
 }
}

#[cfg(test)]
mod ssim_optimizer_tests {
 use super::*;
 
 #[test]
 fn test_optimizer_creation() {
 let opt = SSIMOptimizer::web_optimized();
 assert_eq!(opt.target_ssim, 0.95);
 assert_eq!(opt.min_quality, 70);
 assert_eq!(opt.max_quality, 90);
 
 let opt = SSIMOptimizer::high_quality();
 assert_eq!(opt.target_ssim, 0.98);
 
 let opt = SSIMOptimizer::fast_preview();
 assert_eq!(opt.target_ssim, 0.90);
 }
}
