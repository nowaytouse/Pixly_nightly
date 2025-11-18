/// 质量检查模块
/// 
/// 功能：
/// 1. SSIM（结构相似性）质量验证
/// 2. 转换前后对比
/// 3. 质量报告生成

use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView, Rgba};
use std::path::Path;

/// SSIM 质量检查结果
#[derive(Debug, Clone)]
pub struct QualityCheckResult {
    /// SSIM 分数 (0.0-1.0, 1.0 = 完全相同)
    pub ssim_score: f64,
    
    /// 是否通过质量检查（SSIM >= 0.95）
    pub passed: bool,
    
    /// 质量等级
    pub quality_grade: QualityGrade,
    
    /// 详细信息
    pub details: String,
}

/// 质量等级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityGrade {
    /// 优秀 (SSIM >= 0.98)
    Excellent,
    /// 良好 (SSIM >= 0.95)
    Good,
    /// 可接受 (SSIM >= 0.90)
    Acceptable,
    /// 较差 (SSIM < 0.90)
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
            Self::Excellent => "优秀",
            Self::Good => "良好",
            Self::Acceptable => "可接受",
            Self::Poor => "较差",
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

/// SSIM 质量检查器
pub struct QualityChecker {
    /// SSIM 阈值（默认 0.95）
    threshold: f64,
}

impl QualityChecker {
    /// 创建默认检查器（阈值 0.95）
    pub fn new() -> Self {
        Self { threshold: 0.95 }
    }
    
    /// 创建自定义阈值的检查器
    pub fn with_threshold(threshold: f64) -> Self {
        Self { threshold }
    }
    
    /// 检查转换质量
    pub fn check_conversion_quality(
        &self,
        original_path: &Path,
        converted_path: &Path,
    ) -> Result<QualityCheckResult> {
        // 1. 加载图像
        let original = image::open(original_path)
            .with_context(|| format!("Failed to load original: {:?}", original_path))?;
        
        let converted = image::open(converted_path)
            .with_context(|| format!("Failed to load converted: {:?}", converted_path))?;
        
        // 2. 检查尺寸
        if original.dimensions() != converted.dimensions() {
            anyhow::bail!(
                "Image dimensions mismatch: {:?} vs {:?}",
                original.dimensions(),
                converted.dimensions()
            );
        }
        
        // 3. 计算 SSIM
        let ssim_score = self.calculate_ssim(&original, &converted)?;
        
        // 4. 生成结果
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
    
    /// 计算 SSIM（简化版本）
    /// 
    /// 注意：这是一个简化的 SSIM 实现，用于快速质量检查。
    /// 完整的 SSIM 实现需要考虑更多因素（亮度、对比度、结构）。
    fn calculate_ssim(&self, img1: &DynamicImage, img2: &DynamicImage) -> Result<f64> {
        let (width, height) = img1.dimensions();
        
        // 转换为 RGB
        let img1_rgb = img1.to_rgba8();
        let img2_rgb = img2.to_rgba8();
        
        // 采样计算（每 8x8 块采样一次，提高性能）
        let sample_step = 8;
        let mut total_similarity = 0.0;
        let mut sample_count = 0;
        
        for y in (0..height).step_by(sample_step) {
            for x in (0..width).step_by(sample_step) {
                let pixel1 = img1_rgb.get_pixel(x, y);
                let pixel2 = img2_rgb.get_pixel(x, y);
                
                // 计算像素相似度
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
    
    /// 计算两个像素的相似度
    fn pixel_similarity(&self, p1: &Rgba<u8>, p2: &Rgba<u8>) -> f64 {
        // 计算欧氏距离
        let r_diff = (p1[0] as f64 - p2[0] as f64).powi(2);
        let g_diff = (p1[1] as f64 - p2[1] as f64).powi(2);
        let b_diff = (p1[2] as f64 - p2[2] as f64).powi(2);
        
        let distance = (r_diff + g_diff + b_diff).sqrt();
        let max_distance = (255.0_f64.powi(2) * 3.0).sqrt();
        
        // 转换为相似度 (0.0-1.0)
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
    use image::{ImageBuffer, Rgba};
    
    #[test]
    fn test_pixel_similarity() {
        let checker = QualityChecker::new();
        
        // 完全相同的像素
        let p1 = Rgba([100, 150, 200, 255]);
        let p2 = Rgba([100, 150, 200, 255]);
        let sim = checker.pixel_similarity(&p1, &p2);
        assert!((sim - 1.0).abs() < 0.001);
        
        // 完全不同的像素
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
