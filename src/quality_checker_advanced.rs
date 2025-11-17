//! 高级质量检查器
//! 
//! 提供SSIM、PSNR、MSE等质量指标计算

use anyhow::{Result, Context};
use image::{DynamicImage, GenericImageView};
use std::path::Path;

/// 质量指标
#[derive(Debug, Clone)]
pub struct QualityMetrics {
    /// 结构相似性指数 (0.0-1.0, 越高越好)
    pub ssim: f64,
    /// 峰值信噪比 (dB, 越高越好)
    pub psnr: f64,
    /// 均方误差 (越低越好)
    pub mse: f64,
    /// 质量评估
    pub assessment: QualityAssessment,
}

/// 质量评估等级
#[derive(Debug, Clone, PartialEq)]
pub enum QualityAssessment {
    Excellent,      // SSIM > 0.95
    Good,           // SSIM > 0.90
    Acceptable,     // SSIM > 0.85
    Poor,           // SSIM > 0.75
    Unacceptable,   // SSIM <= 0.75
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
}

/// 质量检查器
pub struct QualityChecker {
    min_ssim: f64,
    min_psnr: f64,
}

impl QualityChecker {
    pub fn new() -> Self {
        Self {
            min_ssim: 0.85,
            min_psnr: 30.0,
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
    
    /// 比较两个图像并计算质量指标
    pub fn compare<P: AsRef<Path>>(&self, original: P, converted: P) -> Result<QualityMetrics> {
        let img1 = image::open(original.as_ref())
            .with_context(|| format!("Cannot open original image: {:?}", original.as_ref()))?;
        let img2 = image::open(converted.as_ref())
            .with_context(|| format!("Cannot open converted image: {:?}", converted.as_ref()))?;
        
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
    
    /// 标准化图像尺寸
    fn normalize_dimensions(&self, img1: DynamicImage, img2: DynamicImage) -> (DynamicImage, DynamicImage) {
        let (w1, h1) = img1.dimensions();
        let (w2, h2) = img2.dimensions();
        
        if w1 != w2 || h1 != h2 {
            let target_w = w1.min(w2);
            let target_h = h1.min(h2);
            (
                img1.resize_exact(target_w, target_h, image::imageops::FilterType::Lanczos3),
                img2.resize_exact(target_w, target_h, image::imageops::FilterType::Lanczos3)
            )
        } else {
            (img1, img2)
        }
    }
    
    /// 计算均方误差
    fn calculate_mse(&self, img1: &DynamicImage, img2: &DynamicImage) -> f64 {
        let rgba1 = img1.to_rgba8();
        let rgba2 = img2.to_rgba8();
        
        let mut sum = 0.0;
        let mut count = 0;
        
        for (p1, p2) in rgba1.pixels().zip(rgba2.pixels()) {
            for i in 0..3 {
                let diff = p1[i] as f64 - p2[i] as f64;
                sum += diff * diff;
                count += 1;
            }
        }
        
        sum / count as f64
    }
    
    /// 计算峰值信噪比
    fn calculate_psnr(&self, mse: f64) -> f64 {
        if mse == 0.0 {
            f64::INFINITY
        } else {
            20.0 * (255.0_f64).log10() - 10.0 * mse.log10()
        }
    }
    
    /// 计算结构相似性指数
    fn calculate_ssim(&self, img1: &DynamicImage, img2: &DynamicImage) -> f64 {
        let rgba1 = img1.to_rgba8();
        let rgba2 = img2.to_rgba8();
        
        let (width, height) = rgba1.dimensions();
        let window_size = 11;
        let k1 = 0.01;
        let k2 = 0.03;
        let l = 255.0;
        let c1 = (k1 * l) * (k1 * l);
        let c2 = (k2 * l) * (k2 * l);
        
        let mut ssim_sum = 0.0;
        let mut count = 0;
        
        for y in 0..height.saturating_sub(window_size) {
            for x in 0..width.saturating_sub(window_size) {
                let (mean1, var1) = self.calculate_window_stats(&rgba1, x, y, window_size);
                let (mean2, var2) = self.calculate_window_stats(&rgba2, x, y, window_size);
                let covar = self.calculate_covariance(&rgba1, &rgba2, x, y, window_size, mean1, mean2);
                
                let numerator = (2.0 * mean1 * mean2 + c1) * (2.0 * covar + c2);
                let denominator = (mean1 * mean1 + mean2 * mean2 + c1) * (var1 + var2 + c2);
                
                ssim_sum += numerator / denominator;
                count += 1;
            }
        }
        
        if count > 0 {
            ssim_sum / count as f64
        } else {
            1.0
        }
    }
    
    /// 计算窗口统计
    fn calculate_window_stats(&self, img: &image::RgbaImage, x: u32, y: u32, size: u32) -> (f64, f64) {
        let mut sum = 0.0;
        let mut count = 0;
        
        for dy in 0..size {
            for dx in 0..size {
                let pixel = img.get_pixel(x + dx, y + dy);
                let gray = (pixel[0] as f64 + pixel[1] as f64 + pixel[2] as f64) / 3.0;
                sum += gray;
                count += 1;
            }
        }
        
        let mean = sum / count as f64;
        
        let mut var_sum = 0.0;
        for dy in 0..size {
            for dx in 0..size {
                let pixel = img.get_pixel(x + dx, y + dy);
                let gray = (pixel[0] as f64 + pixel[1] as f64 + pixel[2] as f64) / 3.0;
                var_sum += (gray - mean) * (gray - mean);
            }
        }
        
        let variance = var_sum / count as f64;
        (mean, variance)
    }
    
    /// 计算协方差
    #[allow(clippy::too_many_arguments)]
    fn calculate_covariance(&self, img1: &image::RgbaImage, img2: &image::RgbaImage, 
                           x: u32, y: u32, size: u32, mean1: f64, mean2: f64) -> f64 {
        let mut sum = 0.0;
        let mut count = 0;
        
        for dy in 0..size {
            for dx in 0..size {
                let p1 = img1.get_pixel(x + dx, y + dy);
                let p2 = img2.get_pixel(x + dx, y + dy);
                let gray1 = (p1[0] as f64 + p1[1] as f64 + p1[2] as f64) / 3.0;
                let gray2 = (p2[0] as f64 + p2[1] as f64 + p2[2] as f64) / 3.0;
                sum += (gray1 - mean1) * (gray2 - mean2);
                count += 1;
            }
        }
        
        sum / count as f64
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
    }
    
    #[test]
    fn test_psnr_calculation() {
        let checker = QualityChecker::new();
        let psnr = checker.calculate_psnr(100.0);
        assert!(psnr > 0.0);
    }
}
