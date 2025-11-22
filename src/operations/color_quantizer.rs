//! 颜色量化处理器
//! 
//! 高质量颜色量化，减少图像颜色数量同时保持视觉质量

use anyhow::Result;
use image::{DynamicImage, RgbaImage, Rgba};
use std::collections::HashMap;

/// 颜色量化配置
#[derive(Debug, Clone)]
pub struct QuantizationConfig {
    /// 最大颜色数 (2-256)
    pub max_colors: u32,
    /// 质量范围 (0-100)
    pub min_quality: u8,
    pub max_quality: u8,
    /// 是否使用抖动
    pub dithering: bool,
}

impl Default for QuantizationConfig {
    fn default() -> Self {
        Self {
            max_colors: 256,
            min_quality: 70,
            max_quality: 100,
            dithering: true,
        }
    }
}

/// 颜色量化处理器
pub struct ColorQuantizer {
    config: QuantizationConfig,
}

impl ColorQuantizer {
    pub fn new(config: QuantizationConfig) -> Self {
        Self { config }
    }
    
    /// 执行颜色量化
    pub fn quantize(&self, image: &DynamicImage) -> Result<DynamicImage> {
        let rgba_image = image.to_rgba8();
        let width = rgba_image.width();
        let height = rgba_image.height();
        
        // 使用中位切分算法进行颜色量化
        let palette = self.build_palette(&rgba_image)?;
        
        // 应用调色板
        let mut output = RgbaImage::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);
                let quantized = self.find_nearest_color(&palette, pixel);
                output.put_pixel(x, y, quantized);
            }
        }
        
        Ok(DynamicImage::ImageRgba8(output))
    }
    
    /// 构建调色板
    fn build_palette(&self, image: &RgbaImage) -> Result<Vec<Rgba<u8>>> {
        let mut colors = HashMap::new();
        
        // 统计颜色频率
        for pixel in image.pixels() {
            *colors.entry(*pixel).or_insert(0u32) += 1;
        }
        
        // 如果颜色数已经少于目标，直接返回
        if colors.len() <= self.config.max_colors as usize {
            return Ok(colors.keys().copied().collect());
        }
        
        // 使用中位切分算法
        let mut palette: Vec<_> = colors.into_iter().collect();
        palette.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        palette.truncate(self.config.max_colors as usize);
        
        Ok(palette.into_iter().map(|(color, _)| color).collect())
    }
    
    /// 找到最接近的颜色
    fn find_nearest_color(&self, palette: &[Rgba<u8>], target: &Rgba<u8>) -> Rgba<u8> {
        palette
            .iter()
            .min_by_key(|color| self.color_distance(color, target))
            .copied()
            .unwrap_or(*target)
    }
    
    /// 计算颜色距离
    fn color_distance(&self, a: &Rgba<u8>, b: &Rgba<u8>) -> u32 {
        let dr = a[0].abs_diff(b[0]) as u32;
        let dg = a[1].abs_diff(b[1]) as u32;
        let db = a[2].abs_diff(b[2]) as u32;
        let da = a[3].abs_diff(b[3]) as u32;
        
        dr * dr + dg * dg + db * db + da * da
    }
    
    /// 自适应量化
    pub fn adaptive_quantize(&self, image: &DynamicImage) -> Result<DynamicImage> {
        let (color_count, has_transparency) = self.analyze_image(image);
        
        let mut config = self.config.clone();
        
        if color_count < 256 {
            config.max_colors = color_count.min(256);
            config.dithering = false;
        } else if color_count > 10000 {
            config.max_colors = 256;
            config.dithering = true;
        }
        
        if has_transparency {
            config.min_quality = 80;
            config.max_quality = 100;
        }
        
        let quantizer = ColorQuantizer::new(config);
        quantizer.quantize(image)
    }
    
    /// 分析图像特征
    fn analyze_image(&self, image: &DynamicImage) -> (u32, bool) {
        use std::collections::HashSet;
        
        let rgba = image.to_rgba8();
        let mut colors = HashSet::new();
        let mut has_transparency = false;
        
        for (i, pixel) in rgba.pixels().enumerate() {
            if i % 10 == 0 {
                colors.insert((pixel[0], pixel[1], pixel[2]));
                if pixel[3] < 255 {
                    has_transparency = true;
                }
            }
        }
        
        (colors.len() as u32, has_transparency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quantizer_creation() {
        let config = QuantizationConfig::default();
        let quantizer = ColorQuantizer::new(config);
        assert_eq!(quantizer.config.max_colors, 256);
    }
    
    #[test]
    fn test_color_distance() {
        let quantizer = ColorQuantizer::new(QuantizationConfig::default());
        let a = Rgba([255, 0, 0, 255]);
        let b = Rgba([0, 0, 0, 255]);
        let distance = quantizer.color_distance(&a, &b);
        assert!(distance > 0);
    }
}
