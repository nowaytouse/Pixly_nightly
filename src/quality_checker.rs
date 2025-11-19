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
    use image::Rgba;
    
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

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Phase 5.2: 合并 quality_checker_advanced.rs 的PSNR/MSE功能
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// 完整质量指标（包含SSIM、PSNR、MSE）
#[derive(Debug, Clone)]
pub struct AdvancedQualityMetrics {
    /// 结构相似性指数 (0.0-1.0, 越高越好)
    pub ssim: f64,
    /// 峰值信噪比 (dB, 越高越好)
    pub psnr: f64,
    /// 均方误差 (越低越好)
    pub mse: f64,
    /// 质量评估
    pub assessment: QualityGrade,
}

impl QualityChecker {
    /// 计算完整质量指标（SSIM + PSNR + MSE）
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
        
        // 确保尺寸相同
        if original.dimensions() != converted.dimensions() {
            anyhow::bail!("Image dimensions don't match");
        }
        
        // 计算SSIM
        let ssim = self.calculate_ssim(&original, &converted)?;
        
        // 计算MSE和PSNR
        let (mse, psnr) = self.calculate_mse_psnr(&original, &converted)?;
        
        Ok(AdvancedQualityMetrics {
            ssim,
            psnr,
            mse,
            assessment: QualityGrade::from_ssim(ssim),
        })
    }
    
    /// 计算均方误差(MSE)和峰值信噪比(PSNR)
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
                
                // 只计算RGB通道（忽略alpha）
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
            f64::INFINITY // 完全相同
        };
        
        Ok((mse, psnr))
    }
}
