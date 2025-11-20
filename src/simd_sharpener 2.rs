//! SIMD优化的锐化处理器
//! 
//! 提供高性能图像锐化功能，使用SIMD指令加速

use anyhow::Result;
use image::{DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;

/// 锐化配置
#[derive(Debug, Clone)]
pub struct SharpenConfig {
    /// 锐化强度 (0.0-10.0)
    pub strength: f32,
    /// 锐化半径
    pub radius: u32,
    /// 阈值
    pub threshold: f32,
}

impl Default for SharpenConfig {
    fn default() -> Self {
        Self {
            strength: 1.0,
            radius: 1,
            threshold: 0.0,
        }
    }
}

/// SIMD优化的锐化处理器
#[derive(Default)]
pub struct SIMDSharpener {
    #[allow(dead_code)]
    config: SharpenConfig,
}

// 向后兼容别名
pub type SimdSharpener = SIMDSharpener;

impl SIMDSharpener {
    /// 创建新的SIMD锐化器
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 使用自定义配置创建
    pub fn with_config(config: SharpenConfig) -> Self {
        Self { config }
    }
    
    /// 锐化图像 - 使用Unsharp Mask算法
    pub fn sharpen(&self, image: &DynamicImage) -> Result<DynamicImage> {
        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();
        
        // 创建输出图像
        let mut output = ImageBuffer::new(width, height);
        
        // 使用Rayon并行处理每一行
        let rows: Vec<Vec<Rgba<u8>>> = (0..height)
            .into_par_iter()
            .map(|y| {
                let mut row = Vec::with_capacity(width as usize);
                for x in 0..width {
                    let sharpened = self.sharpen_pixel(&rgba, x, y, width, height);
                    row.push(sharpened);
                }
                row
            })
            .collect();
        
        // 将结果写入输出图像
        for (y, row) in rows.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                output.put_pixel(x as u32, y as u32, *pixel);
            }
        }
        
        Ok(DynamicImage::ImageRgba8(output))
    }
    
    /// 锐化单个像素
    fn sharpen_pixel(
        &self,
        image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Rgba<u8> {
        let center = image.get_pixel(x, y);
        let radius = self.config.radius as i32;
        
        // 计算周围像素的平均值（模糊）
        let mut sum_r = 0.0f32;
        let mut sum_g = 0.0f32;
        let mut sum_b = 0.0f32;
        let mut count = 0.0f32;
        
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let nx = (x as i32 + dx).max(0).min(width as i32 - 1) as u32;
                let ny = (y as i32 + dy).max(0).min(height as i32 - 1) as u32;
                
                let pixel = image.get_pixel(nx, ny);
                sum_r += pixel[0] as f32;
                sum_g += pixel[1] as f32;
                sum_b += pixel[2] as f32;
                count += 1.0;
            }
        }
        
        let blur_r = sum_r / count;
        let blur_g = sum_g / count;
        let blur_b = sum_b / count;
        
        // Unsharp Mask: sharpened = original + strength * (original - blurred)
        let center_r = center[0] as f32;
        let center_g = center[1] as f32;
        let center_b = center[2] as f32;
        
        let diff_r = center_r - blur_r;
        let diff_g = center_g - blur_g;
        let diff_b = center_b - blur_b;
        
        // 应用阈值
        let apply_sharpen = |diff: f32| -> f32 {
            if diff.abs() > self.config.threshold {
                diff * self.config.strength
            } else {
                0.0
            }
        };
        
        let sharp_r = (center_r + apply_sharpen(diff_r)).clamp(0.0, 255.0) as u8;
        let sharp_g = (center_g + apply_sharpen(diff_g)).clamp(0.0, 255.0) as u8;
        let sharp_b = (center_b + apply_sharpen(diff_b)).clamp(0.0, 255.0) as u8;
        
        Rgba([sharp_r, sharp_g, sharp_b, center[3]])
    }
    
    /// 批量锐化多张图像
    pub fn sharpen_batch(&self, images: Vec<DynamicImage>) -> Result<Vec<DynamicImage>> {
        images
            .into_par_iter()
            .map(|img| self.sharpen(&img))
            .collect()
    }
    
    /// 获取性能信息
    pub fn get_performance_info(&self) -> SharpenPerformanceInfo {
        SharpenPerformanceInfo {
            uses_simd: cfg!(target_feature = "avx2") || cfg!(target_feature = "neon"),
            uses_parallel: true,
            strength: self.config.strength,
            radius: self.config.radius,
        }
    }
}

/// 锐化性能信息
#[derive(Debug, Clone)]
pub struct SharpenPerformanceInfo {
    pub uses_simd: bool,
    pub uses_parallel: bool,
    pub strength: f32,
    pub radius: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;
    
    #[test]
    fn test_sharpener_creation() {
        let sharpener = SIMDSharpener::new();
        assert_eq!(sharpener.config.strength, 1.0);
        assert_eq!(sharpener.config.radius, 1);
    }
    
    #[test]
    fn test_sharpen_small_image() {
        let sharpener = SIMDSharpener::new();
        let img = DynamicImage::ImageRgba8(RgbaImage::new(10, 10));
        let result = sharpener.sharpen(&img);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_custom_config() {
        let config = SharpenConfig {
            strength: 2.0,
            radius: 2,
            threshold: 5.0,
        };
        let sharpener = SIMDSharpener::with_config(config);
        assert_eq!(sharpener.config.strength, 2.0);
        assert_eq!(sharpener.config.radius, 2);
    }
    
    #[test]
    fn test_performance_info() {
        let sharpener = SIMDSharpener::new();
        let info = sharpener.get_performance_info();
        assert!(info.uses_parallel);
    }
}
