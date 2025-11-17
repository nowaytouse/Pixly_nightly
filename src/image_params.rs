// 🚀 图像参数分析器
// 从 @archive/rust_broken/src/converter/params.rs 提取核心功能
//
// 核心功能:
// - 图像特征分析
// - 复杂度计算
// - 参数优化建议

use anyhow::{Context, Result};
use std::path::Path;
use serde::{Deserialize, Serialize};
use image::GenericImageView;
use std::collections::HashSet;

/// 图像特征
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageCharacteristics {
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub format: String,
    pub has_alpha: bool,
    pub is_animated: bool,
    pub complexity: f32,
    pub path: Option<String>,
}

/// 优化参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedParams {
    pub quality: u8,
    pub speed: u8,
    pub lossless: bool,
    pub format_options: Vec<(String, String)>,
    pub estimated_size: u64,
    pub estimated_ratio: f32,
    pub reason: String,
}

/// 图像参数分析器
pub struct ImageParamAnalyzer;

impl ImageParamAnalyzer {
    pub fn analyze_image<P: AsRef<Path>>(path: P) -> Result<ImageCharacteristics> {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)
            .with_context(|| format!("Failed to read file metadata: {}", path.display()))?;

        let file_size = metadata.len();
        let format = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let img = image::open(path)
            .with_context(|| format!("Failed to open image: {}", path.display()))?;

        let (width, height) = img.dimensions();
        let has_alpha = img.color().has_alpha();
        let complexity = Self::calculate_complexity(&img);
        let is_animated = Self::detect_animation(path)?;

        Ok(ImageCharacteristics {
            width,
            height,
            file_size,
            format,
            has_alpha,
            is_animated,
            complexity: complexity as f32,
            path: Some(path.to_string_lossy().to_string()),
        })
    }
    
    fn calculate_complexity(img: &image::DynamicImage) -> f64 {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let total_pixels = (width * height) as usize;
        let sample_size = total_pixels.min(1000);
        let step = (total_pixels / sample_size).max(1);
        
        let mut unique_colors = HashSet::new();
        for (sampled, (_, _, pixel)) in rgba.enumerate_pixels().enumerate() {
            if sampled >= sample_size {
                break;
            }
            if sampled % step == 0 {
                let quantized = (
                    pixel[0] & 0xF0,
                    pixel[1] & 0xF0,
                    pixel[2] & 0xF0,
                );
                unique_colors.insert(quantized);
            }
        }
        
        let color_diversity = (unique_colors.len() as f64 / sample_size as f64).min(1.0);
        let color_score = color_diversity * 0.4;
        
        let gray = img.to_luma8();
        let mut edge_count = 0;
        let edge_sample = 200.min(total_pixels);
        
        for i in 0..edge_sample {
            let x = (i * width as usize / edge_sample) as u32;
            let y = (i * height as usize / edge_sample) as u32;
            
            if x > 0 && y > 0 && x < width - 1 && y < height - 1 {
                let center = gray.get_pixel(x, y)[0] as i32;
                let right = gray.get_pixel(x + 1, y)[0] as i32;
                let bottom = gray.get_pixel(x, y + 1)[0] as i32;
                
                let gx = (right - center).abs();
                let gy = (bottom - center).abs();
                let gradient = (gx + gy) / 2;
                
                if gradient > 30 {
                    edge_count += 1;
                }
            }
        }
        
        let edge_density = edge_count as f64 / edge_sample as f64;
        let edge_score = edge_density * 0.3;
        
        let mut luminance_sum = 0u64;
        let mut luminance_sq_sum = 0u64;
        let luma_sample = 500.min(total_pixels);
        
        for i in 0..luma_sample {
            let x = (i * width as usize / luma_sample) as u32;
            let y = (i * height as usize / luma_sample) as u32;
            
            if x < width && y < height {
                let luma = gray.get_pixel(x, y)[0] as u64;
                luminance_sum += luma;
                luminance_sq_sum += luma * luma;
            }
        }
        
        let mean = luminance_sum as f64 / luma_sample as f64;
        let variance = (luminance_sq_sum as f64 / luma_sample as f64) - (mean * mean);
        let std_dev = variance.sqrt();
        let texture_score = (std_dev / 128.0).min(1.0) * 0.3;
        
        (color_score + edge_score + texture_score).min(1.0)
    }
    
    fn detect_animation(path: &Path) -> Result<bool> {
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
        
        Ok(matches!(ext.as_str(), "gif" | "apng" | "webp"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_image_characteristics() {
        let chars = ImageCharacteristics {
            width: 1920,
            height: 1080,
            file_size: 1024000,
            format: "jpg".to_string(),
            has_alpha: false,
            is_animated: false,
            complexity: 0.5,
            path: None,
        };
        
        assert_eq!(chars.width, 1920);
        assert_eq!(chars.height, 1080);
        assert!(!chars.has_alpha);
    }
}
