// 🚀 图像质量检查器
// 从 @archive/rust_broken/src/converter/quality.rs 提取核心功能
//
// 核心功能:
// - SSIM计算
// - PSNR计算
// - MSE计算
// - 质量评估

use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView};
use std::path::Path;
use serde::{Deserialize, Serialize};

/// 质量指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub ssim: f64,
    pub psnr: f64,
    pub mse: f64,
    pub assessment: QualityAssessment,
}

/// 质量评估
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QualityAssessment {
    Excellent,
    Good,
    Acceptable,
    Poor,
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

/// 质量检查器
pub struct QualityChecker {
    min_ssim: f64,
    min_psnr: f64,
    auto_rollback: bool,
}

impl QualityChecker {
    pub fn new() -> Self {
        Self {
            min_ssim: 0.85,
            min_psnr: 30.0,
            auto_rollback: true,
        }
    }
    
    pub fn with_min_ssim(mut self, min_ssim: f64) -> Self {
        self.min_ssim = min_ssim.clamp(0.0, 1.0);
        self
    }
    
    pub fn with_min_psnr(mut self, min_psnr: f64) -> Self {
        self.min_psnr = min_psnr.max(0.0);
        self
    }
    
    pub fn with_auto_rollback(mut self, enabled: bool) -> Self {
        self.auto_rollback = enabled;
        self
    }
    
    pub fn compare<P: AsRef<Path>>(&self, original: P, converted: P) -> Result<QualityMetrics> {
        let img1 = image::open(original.as_ref())
            .with_context(|| "Failed to open original image")?;
        let img2 = image::open(converted.as_ref())
            .with_context(|| "Failed to open converted image")?;
        
        let (img1, img2) = self.normalize_dimensions(img1, img2);
        let mse = self.calculate_mse(&img1, &img2);
        let psnr = self.calculate_psnr(mse);
        let ssim = self.calculate_ssim(&img1, &img2);
        let assessment = QualityAssessment::from_ssim(ssim);
        
        Ok(QualityMetrics {
            ssim,
            psnr,
            mse,
            assessment,
        })
    }
    
    pub fn is_acceptable(&self, metrics: &QualityMetrics) -> bool {
        metrics.ssim >= self.min_ssim && metrics.psnr >= self.min_psnr
    }
    
    fn normalize_dimensions(&self, img1: DynamicImage, img2: DynamicImage) -> (DynamicImage, DynamicImage) {
        let (w1, h1) = img1.dimensions();
        let (w2, h2) = img2.dimensions();
        
        if (w1, h1) == (w2, h2) {
            return (img1, img2);
        }
        
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
    
    fn calculate_psnr(&self, mse: f64) -> f64 {
        if mse < 1e-10 {
            return 100.0;
        }
        
        let max_pixel_value = 255.0;
        20.0 * (max_pixel_value / mse.sqrt()).log10()
    }
    
    fn calculate_ssim(&self, img1: &DynamicImage, img2: &DynamicImage) -> f64 {
        let rgba1 = img1.to_rgba8();
        let rgba2 = img2.to_rgba8();
        
        let k1 = 0.01_f64;
        let k2 = 0.03_f64;
        let l = 255.0_f64;
        let c1 = (k1 * l).powi(2);
        let c2 = (k2 * l).powi(2);
        
        let (mean1, mean2) = self.calculate_means(&rgba1, &rgba2);
        let (var1, var2, cov) = self.calculate_variances(&rgba1, &rgba2, mean1, mean2);
        
        let numerator = (2.0 * mean1 * mean2 + c1) * (2.0 * cov + c2);
        let denominator = (mean1 * mean1 + mean2 * mean2 + c1) * (var1 + var2 + c2);
        
        (numerator / denominator).clamp(0.0, 1.0)
    }
    
    fn calculate_means(&self, img1: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, img2: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> (f64, f64) {
        let (width, height) = img1.dimensions();
        let mut sum1 = 0.0;
        let mut sum2 = 0.0;
        let count = (width * height) as f64;
        
        for y in 0..height {
            for x in 0..width {
                let p1 = img1.get_pixel(x, y);
                let p2 = img2.get_pixel(x, y);
                sum1 += (p1[0] as f64 + p1[1] as f64 + p1[2] as f64) / 3.0;
                sum2 += (p2[0] as f64 + p2[1] as f64 + p2[2] as f64) / 3.0;
            }
        }
        
        (sum1 / count, sum2 / count)
    }
    
    fn calculate_variances(&self, img1: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, img2: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, mean1: f64, mean2: f64) -> (f64, f64, f64) {
        let (width, height) = img1.dimensions();
        let mut var1 = 0.0;
        let mut var2 = 0.0;
        let mut cov = 0.0;
        let count = (width * height) as f64;
        
        for y in 0..height {
            for x in 0..width {
                let p1 = img1.get_pixel(x, y);
                let p2 = img2.get_pixel(x, y);
                let lum1 = (p1[0] as f64 + p1[1] as f64 + p1[2] as f64) / 3.0;
                let lum2 = (p2[0] as f64 + p2[1] as f64 + p2[2] as f64) / 3.0;
                
                let diff1 = lum1 - mean1;
                let diff2 = lum2 - mean2;
                
                var1 += diff1 * diff1;
                var2 += diff2 * diff2;
                cov += diff1 * diff2;
            }
        }
        
        (var1 / count, var2 / count, cov / count)
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
    
    #[test]
    fn test_quality_assessment() {
        assert_eq!(QualityAssessment::from_ssim(0.96), QualityAssessment::Excellent);
        assert_eq!(QualityAssessment::from_ssim(0.92), QualityAssessment::Good);
        assert_eq!(QualityAssessment::from_ssim(0.87), QualityAssessment::Acceptable);
        assert_eq!(QualityAssessment::from_ssim(0.78), QualityAssessment::Poor);
        assert_eq!(QualityAssessment::from_ssim(0.70), QualityAssessment::Unacceptable);
    }
    
    #[test]
    fn test_quality_checker_creation() {
        let checker = QualityChecker::new();
        assert_eq!(checker.min_ssim, 0.85);
        assert_eq!(checker.min_psnr, 30.0);
        assert!(checker.auto_rollback);
    }
}
