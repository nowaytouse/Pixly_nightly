use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView, Rgba};
use std::path::Path;
// 🔧 统一日志系统
use tracing::{info, warn, debug};


/// Quality metrics for image comparison
#[derive(Debug, Clone)]
pub struct QualityMetrics {
    /// Structural Similarity Index (0.0-1.0, higher is better)
    pub ssim: f64,
    /// Peak Signal-to-Noise Ratio (dB, higher is better)
    pub psnr: f64,
    /// Mean Squared Error (lower is better)
    pub mse: f64,
    /// Quality assessment
    pub assessment: QualityAssessment,
}

/// Quality assessment levels
#[derive(Debug, Clone, PartialEq)]
pub enum QualityAssessment {
    /// Excellent quality (SSIM > 0.95)
    Excellent,
    /// Good quality (SSIM > 0.90)
    Good,
    /// Acceptable quality (SSIM > 0.85)
    Acceptable,
    /// Poor quality (SSIM > 0.75)
    Poor,
    /// Unacceptable quality (SSIM <= 0.75)
    Unacceptable,
}

impl QualityAssessment {
    pub fn from_ssim(ssim: f64) -> Self {
        if ssim > 0.95 {
            Self::Excellent
        } else if ssim > 0.90 {
            Self::Good
        } else if ssim > 0.85 {
            Self::Acceptable
        } else if ssim > 0.75 {
            Self::Poor
        } else {
            Self::Unacceptable
        }
    }
    
    pub fn should_warn(&self) -> bool {
        matches!(self, Self::Poor | Self::Unacceptable)
    }
    
    pub fn should_rollback(&self) -> bool {
        matches!(self, Self::Unacceptable)
    }
}

/// Quality checker for image conversion
pub struct QualityChecker {
    /// Minimum acceptable SSIM (default: 0.85)
    min_ssim: f64,
    /// Minimum acceptable PSNR (default: 30.0 dB)
    min_psnr: f64,
    /// Enable automatic rollback on poor quality
    auto_rollback: bool,
}

impl QualityChecker {
    /// Create a new quality checker with default thresholds
    pub fn new() -> Self {
        Self {
            min_ssim: 0.85,
            min_psnr: 30.0,
            auto_rollback: true,
        }
    }
    
    /// Set minimum SSIM threshold
    pub fn with_min_ssim(mut self, min_ssim: f64) -> Self {
        self.min_ssim = min_ssim.clamp(0.0, 1.0);
        self
    }
    
    /// Set minimum PSNR threshold
    pub fn with_min_psnr(mut self, min_psnr: f64) -> Self {
        self.min_psnr = min_psnr.max(0.0);
        self
    }
    
    /// Enable/disable automatic rollback
    pub fn with_auto_rollback(mut self, enabled: bool) -> Self {
        self.auto_rollback = enabled;
        self
    }
    
    /// Compare two images and calculate quality metrics
    /// 
    /// # Arguments
    /// - `original`: Path to original image
    /// - `converted`: Path to converted image
    /// 
    /// # Returns
    /// Quality metrics including SSIM, PSNR, MSE
    pub fn compare<P: AsRef<Path>>(&self, original: P, converted: P) -> Result<QualityMetrics> {
        let original_path = original.as_ref();
        let converted_path = converted.as_ref();
        
        info!("🔍 Comparing image quality...");
        debug!("  Original: {:?}", original_path);
        debug!("  Converted: {:?}", converted_path);
        
        // Check if converted format is supported for quality check
        if let Some(ext) = converted_path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if matches!(ext_str.as_str(), "avif" | "heic" | "heif") {
                warn!("⚠️  Format {} not fully supported for quality check (requires system codec)", ext_str);
                info!("   Skipping quality check for this format");
                anyhow::bail!("Quality check not supported for {} format (missing system codec)", ext_str);
            }
        }
        
        // Load images
        let img1 = image::open(original_path)
            .with_context(|| format!("Failed to open original image: {:?}", original_path))?;
        let img2 = image::open(converted_path)
            .with_context(|| format!("Failed to open converted image: {:?}", converted_path))?;
        
        // Ensure same dimensions (resize if needed)
        let (img1, img2) = self.normalize_dimensions(img1, img2);
        
        // Calculate MSE
        let mse = self.calculate_mse(&img1, &img2);
        
        // Calculate PSNR
        let psnr = self.calculate_psnr(mse);
        
        // Calculate SSIM
        let ssim = self.calculate_ssim(&img1, &img2);
        
        // Assess quality
        let assessment = QualityAssessment::from_ssim(ssim);
        
        info!("📊 Quality Metrics:");
        info!("   SSIM: {:.4} ({})", ssim, self.ssim_description(ssim));
        info!("   PSNR: {:.2} dB", psnr);
        info!("   MSE: {:.2}", mse);
        info!("   Assessment: {:?}", assessment);
        
        Ok(QualityMetrics {
            ssim,
            psnr,
            mse,
            assessment,
        })
    }
    
    /// Check if quality meets thresholds
    pub fn is_acceptable(&self, metrics: &QualityMetrics) -> bool {
        metrics.ssim >= self.min_ssim && metrics.psnr >= self.min_psnr
    }
    
    /// Normalize image dimensions (resize smaller to match larger)
    fn normalize_dimensions(&self, img1: DynamicImage, img2: DynamicImage) -> (DynamicImage, DynamicImage) {
        let (w1, h1) = img1.dimensions();
        let (w2, h2) = img2.dimensions();
        
        if (w1, h1) == (w2, h2) {
            return (img1, img2);
        }
        
        warn!("⚠️  Image dimensions differ: {}x{} vs {}x{}, resizing...", w1, h1, w2, h2);
        
        // Use the larger dimensions
        let (target_w, target_h) = (w1.max(w2), h1.max(h2));
        
        let img1_resized = if (w1, h1) != (target_w, target_h) {
            img1.resize_exact(target_w, target_h, image::imageops::FilterType::Lanczos3)
        } else {
            img1
        };
        
        let img2_resized = if (w2, h2) != (target_w, target_h) {
            img2.resize_exact(target_w, target_h, image::imageops::FilterType::Lanczos3)
        } else {
            img2
        };
        
        (img1_resized, img2_resized)
    }
    
    /// Calculate Mean Squared Error
    fn calculate_mse(&self, img1: &DynamicImage, img2: &DynamicImage) -> f64 {
        let rgba1 = img1.to_rgba8();
        let rgba2 = img2.to_rgba8();
        
        let (width, height) = rgba1.dimensions();
        let mut sum = 0.0;
        let mut count = 0u64;
        
        for y in 0..height {
            for x in 0..width {
                let p1 = rgba1.get_pixel(x, y);
                let p2 = rgba2.get_pixel(x, y);
                
                for i in 0..4 {
                    let diff = p1[i] as f64 - p2[i] as f64;
                    sum += diff * diff;
                    count += 1;
                }
            }
        }
        
        sum / count as f64
    }
    
    /// Calculate Peak Signal-to-Noise Ratio
    fn calculate_psnr(&self, mse: f64) -> f64 {
        if mse < 1e-10 {
            // Images are identical or nearly identical
            return 100.0;
        }
        
        let max_pixel_value = 255.0;
        20.0 * (max_pixel_value / mse.sqrt()).log10()
    }
    
    /// Calculate Structural Similarity Index
    /// 
    /// Simplified SSIM implementation using luminance comparison
    /// For full SSIM, consider integrating with GO service
    fn calculate_ssim(&self, img1: &DynamicImage, img2: &DynamicImage) -> f64 {
        let rgba1 = img1.to_rgba8();
        let rgba2 = img2.to_rgba8();
        
        let (_width, _height) = rgba1.dimensions();
        
        // Constants for SSIM calculation
        let k1 = 0.01_f64;
        let k2 = 0.03_f64;
        let l = 255.0_f64; // Dynamic range
        let c1 = (k1 * l).powi(2);
        let c2 = (k2 * l).powi(2);
        
        // Calculate means
        let (mean1, mean2) = self.calculate_means(&rgba1, &rgba2);
        
        // Calculate variances and covariance
        let (var1, var2, cov) = self.calculate_variances(&rgba1, &rgba2, mean1, mean2);
        
        // SSIM formula
        let numerator = (2.0 * mean1 * mean2 + c1) * (2.0 * cov + c2);
        let denominator = (mean1.powi(2) + mean2.powi(2) + c1) * (var1 + var2 + c2);
        
        (numerator / denominator).clamp(0.0, 1.0)
    }
    
    /// Calculate mean luminance of images
    fn calculate_means(&self, rgba1: &image::RgbaImage, rgba2: &image::RgbaImage) -> (f64, f64) {
        let (width, height) = rgba1.dimensions();
        let mut sum1 = 0.0;
        let mut sum2 = 0.0;
        let count = (width * height) as f64;
        
        for y in 0..height {
            for x in 0..width {
                let p1 = rgba1.get_pixel(x, y);
                let p2 = rgba2.get_pixel(x, y);
                
                // Convert to luminance (Y)
                sum1 += self.rgb_to_luminance(p1);
                sum2 += self.rgb_to_luminance(p2);
            }
        }
        
        (sum1 / count, sum2 / count)
    }
    
    /// Calculate variances and covariance
    fn calculate_variances(
        &self,
        rgba1: &image::RgbaImage,
        rgba2: &image::RgbaImage,
        mean1: f64,
        mean2: f64,
    ) -> (f64, f64, f64) {
        let (width, height) = rgba1.dimensions();
        let mut var1 = 0.0;
        let mut var2 = 0.0;
        let mut cov = 0.0;
        let count = (width * height) as f64;
        
        for y in 0..height {
            for x in 0..width {
                let p1 = rgba1.get_pixel(x, y);
                let p2 = rgba2.get_pixel(x, y);
                
                let lum1 = self.rgb_to_luminance(p1);
                let lum2 = self.rgb_to_luminance(p2);
                
                let diff1 = lum1 - mean1;
                let diff2 = lum2 - mean2;
                
                var1 += diff1 * diff1;
                var2 += diff2 * diff2;
                cov += diff1 * diff2;
            }
        }
        
        (var1 / count, var2 / count, cov / count)
    }
    
    /// Convert RGB to luminance
    fn rgb_to_luminance(&self, pixel: &Rgba<u8>) -> f64 {
        // ITU-R BT.709 luma coefficients
        0.2126 * pixel[0] as f64 + 0.7152 * pixel[1] as f64 + 0.0722 * pixel[2] as f64
    }
    
    /// Get SSIM description
    fn ssim_description(&self, ssim: f64) -> &'static str {
        if ssim > 0.95 {
            "Excellent"
        } else if ssim > 0.90 {
            "Good"
        } else if ssim > 0.85 {
            "Acceptable"
        } else if ssim > 0.75 {
            "Poor"
        } else {
            "Unacceptable"
        }
    }
}

impl Default for QualityChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Quality check result with action recommendation
#[derive(Debug)]
pub struct QualityCheckResult {
    pub metrics: QualityMetrics,
    pub passed: bool,
    pub should_warn: bool,
    pub should_rollback: bool,
    pub message: String,
}

impl QualityChecker {
    /// Perform quality check and return action recommendation
    pub fn check<P: AsRef<Path>>(&self, original: P, converted: P) -> Result<QualityCheckResult> {
        let metrics = self.compare(original, converted)?;
        let passed = self.is_acceptable(&metrics);
        let should_warn = metrics.assessment.should_warn();
        let should_rollback = self.auto_rollback && metrics.assessment.should_rollback();
        
        let message = if should_rollback {
            format!(
                "Quality unacceptable (SSIM: {:.4} < {:.2}). Automatic rollback recommended.",
                metrics.ssim, self.min_ssim
            )
        } else if should_warn {
            format!(
                "Quality warning (SSIM: {:.4}). Review recommended.",
                metrics.ssim
            )
        } else if passed {
            format!(
                "Quality check passed (SSIM: {:.4}, PSNR: {:.2} dB)",
                metrics.ssim, metrics.psnr
            )
        } else {
            format!(
                "Quality check failed (SSIM: {:.4} < {:.2}, PSNR: {:.2} < {:.2})",
                metrics.ssim, self.min_ssim, metrics.psnr, self.min_psnr
            )
        };
        
        Ok(QualityCheckResult {
            metrics,
            passed,
            should_warn,
            should_rollback,
            message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quality_assessment() {
        assert_eq!(QualityAssessment::from_ssim(0.96), QualityAssessment::Excellent);
        assert_eq!(QualityAssessment::from_ssim(0.92), QualityAssessment::Good);
        assert_eq!(QualityAssessment::from_ssim(0.87), QualityAssessment::Acceptable);
        assert_eq!(QualityAssessment::from_ssim(0.80), QualityAssessment::Poor);
        assert_eq!(QualityAssessment::from_ssim(0.70), QualityAssessment::Unacceptable);
    }
    
    #[test]
    fn test_should_warn() {
        assert!(!QualityAssessment::Excellent.should_warn());
        assert!(!QualityAssessment::Good.should_warn());
        assert!(!QualityAssessment::Acceptable.should_warn());
        assert!(QualityAssessment::Poor.should_warn());
        assert!(QualityAssessment::Unacceptable.should_warn());
    }
    
    #[test]
    fn test_should_rollback() {
        assert!(!QualityAssessment::Excellent.should_rollback());
        assert!(!QualityAssessment::Poor.should_rollback());
        assert!(QualityAssessment::Unacceptable.should_rollback());
    }

    #[test]
    fn test_quality_checker_default() {
        let checker = QualityChecker::default();
        assert_eq!(checker.min_ssim, 0.85);
        assert_eq!(checker.min_psnr, 30.0);
        assert!(checker.auto_rollback);
    }

    #[test]
    fn test_quality_checker_custom() {
        let checker = QualityChecker::new()
            .with_min_ssim(0.90)
            .with_min_psnr(35.0)
            .with_auto_rollback(false);
        
        assert_eq!(checker.min_ssim, 0.90);
        assert_eq!(checker.min_psnr, 35.0);
        assert!(!checker.auto_rollback);
    }

    #[test]
    fn test_mse_psnr_relationship() {
        let checker = QualityChecker::new();
        
        // Test MSE to PSNR conversion
        let mse = 100.0;
        let psnr = checker.calculate_psnr(mse);
        
        // PSNR = 10 * log10(MAX_I^2 / MSE)
        // where MAX_I = 255 for 8-bit images
        let expected_psnr = 10.0 * (255.0 * 255.0 / mse).log10();
        assert!((psnr - expected_psnr).abs() < 0.001);
    }

    #[test]
    fn test_is_acceptable() {
        let checker = QualityChecker::new()
            .with_min_ssim(0.85)
            .with_min_psnr(30.0);
        
        // Both metrics pass
        let metrics1 = QualityMetrics {
            ssim: 0.90,
            psnr: 35.0,
            mse: 10.0,
            assessment: QualityAssessment::Good,
        };
        assert!(checker.is_acceptable(&metrics1));
        
        // SSIM fails
        let metrics2 = QualityMetrics {
            ssim: 0.80,
            psnr: 35.0,
            mse: 10.0,
            assessment: QualityAssessment::Poor,
        };
        assert!(!checker.is_acceptable(&metrics2));
        
        // PSNR fails
        let metrics3 = QualityMetrics {
            ssim: 0.90,
            psnr: 25.0,
            mse: 100.0,
            assessment: QualityAssessment::Good,
        };
        assert!(!checker.is_acceptable(&metrics3));
    }

    #[test]
    fn test_rgb_to_luminance() {
        let checker = QualityChecker::new();
        
        // Test pure white
        let white = Rgba([255, 255, 255, 255]);
        let lum_white = checker.rgb_to_luminance(&white);
        assert!((lum_white - 255.0).abs() < 0.1);
        
        // Test pure black
        let black = Rgba([0, 0, 0, 255]);
        let lum_black = checker.rgb_to_luminance(&black);
        assert_eq!(lum_black, 0.0);
        
        // Test gray (should be close to input value)
        let gray = Rgba([128, 128, 128, 255]);
        let lum_gray = checker.rgb_to_luminance(&gray);
        assert!((lum_gray - 128.0).abs() < 1.0);
    }

    #[test]
    fn test_ssim_description() {
        let checker = QualityChecker::new();
        
        assert_eq!(checker.ssim_description(0.96), "Excellent");
        assert_eq!(checker.ssim_description(0.92), "Good");
        assert_eq!(checker.ssim_description(0.87), "Acceptable");
        assert_eq!(checker.ssim_description(0.80), "Poor");
        assert_eq!(checker.ssim_description(0.70), "Unacceptable");
    }

    #[test]
    fn test_quality_metrics_display() {
        let metrics = QualityMetrics {
            ssim: 0.9234,
            psnr: 34.567,
            mse: 12.345,
            assessment: QualityAssessment::Good,
        };
        
        // Verify metrics are in expected ranges
        assert!(metrics.ssim >= 0.0 && metrics.ssim <= 1.0);
        assert!(metrics.psnr > 0.0);
        assert!(metrics.mse >= 0.0);
    }
}
