/**
 * Phase 47.19 (R-001): imagequant优化Quantization
 * 使用libimagequant进行高质量颜色量化
 */

use anyhow::{Result, Context};
use image::{DynamicImage, RgbaImage, ImageBuffer, Rgba};
use std::path::Path;
use tracing::{info, debug, warn};

#[cfg(feature = "imagequant")]
use imagequant::{new as liq_new, Image as LiqImage};

/// 颜色量化配置
#[derive(Debug, Clone)]
pub struct QuantizationConfig {
    /// 最大颜色数 (2-256)
    pub max_colors: u32,
    /// 质量范围 (0-100)
    pub min_quality: u8,
    pub max_quality: u8,
    /// 速度 (1-10, 1最慢但质量最好)
    pub speed: i32,
    /// 是否使用抖动
    pub dithering: bool,
}

impl Default for QuantizationConfig {
    fn default() -> Self {
        Self {
            max_colors: 256,
            min_quality: 70,
            max_quality: 100,
            speed: 3,
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
    
    /// 使用imagequant进行颜色量化
    #[cfg(feature = "imagequant")]
    pub fn quantize_with_imagequant(&self, image: &DynamicImage) -> Result<DynamicImage> {
        info!("🎨 使用imagequant进行颜色量化");
        debug!("   最大颜色数: {}", self.config.max_colors);
        debug!("   质量范围: {}-{}", self.config.min_quality, self.config.max_quality);
        
        // 转换为RGBA
        let rgba_image = image.to_rgba8();
        let width = rgba_image.width();
        let height = rgba_image.height();
        
        // 创建imagequant实例
        let mut liq = liq_new();
        liq.set_max_colors(self.config.max_colors)?;
        liq.set_quality(self.config.min_quality, self.config.max_quality)?;
        liq.set_speed(self.config.speed)?;
        
        // 创建imagequant图像
        let liq_image = liq.new_image(
            rgba_image.as_raw(),
            width as usize,
            height as usize,
            0.0,  // gamma
        )?;
        
        // 执行量化
        let mut res = liq.quantize(&liq_image)?;
        
        // 设置抖动
        if self.config.dithering {
            res.set_dithering_level(1.0)?;
        }
        
        // 重新映射颜色
        let (palette, pixels) = res.remapped(&liq_image)?;
        
        // 转换回RGBA
        let mut output = RgbaImage::new(width, height);
        for (i, &index) in pixels.iter().enumerate() {
            let x = (i % width as usize) as u32;
            let y = (i / width as usize) as u32;
            let color = palette[index as usize];
            output.put_pixel(x, y, Rgba([color.r, color.g, color.b, color.a]));
        }
        
        info!("✅ 颜色量化完成，使用 {} 种颜色", palette.len());
        
        Ok(DynamicImage::ImageRgba8(output))
    }
    
    /// 简单的颜色量化（不依赖imagequant）
    #[cfg(not(feature = "imagequant"))]
    pub fn quantize_simple(&self, image: &DynamicImage) -> Result<DynamicImage> {
        warn!("⚠️ imagequant未启用，使用简单量化算法");
        
        let rgba_image = image.to_rgba8();
        let width = rgba_image.width();
        let height = rgba_image.height();
        
        // 简单的位深度减少
        let bit_depth = match self.config.max_colors {
            c if c <= 16 => 4,
            c if c <= 64 => 6,
            _ => 8,
        };
        
        let mask = (0xFF >> (8 - bit_depth)) << (8 - bit_depth);
        
        let mut output = RgbaImage::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);
                let quantized = Rgba([
                    pixel[0] & mask,
                    pixel[1] & mask,
                    pixel[2] & mask,
                    pixel[3],
                ]);
                output.put_pixel(x, y, quantized);
            }
        }
        
        Ok(DynamicImage::ImageRgba8(output))
    }
    
    /// 执行颜色量化
    pub fn quantize(&self, image: &DynamicImage) -> Result<DynamicImage> {
        #[cfg(feature = "imagequant")]
        {
            self.quantize_with_imagequant(image)
        }
        
        #[cfg(not(feature = "imagequant"))]
        {
            self.quantize_simple(image)
        }
    }
    
    /// 自适应量化（根据图像特征调整参数）
    pub fn adaptive_quantize(&self, image: &DynamicImage) -> Result<DynamicImage> {
        // 分析图像特征
        let (color_count, has_transparency) = self.analyze_image(image);
        
        debug!("图像分析: {} 种颜色, 透明度: {}", color_count, has_transparency);
        
        // 根据特征调整配置
        let mut config = self.config.clone();
        
        if color_count < 256 {
            // 图像颜色较少，不需要太多量化
            config.max_colors = color_count.min(256);
            config.dithering = false;
        } else if color_count > 10000 {
            // 图像颜色丰富，使用更多颜色
            config.max_colors = 256;
            config.dithering = true;
        }
        
        // 如果有透明度，保留更多颜色用于边缘
        if has_transparency {
            config.min_quality = 80;
            config.max_quality = 100;
        }
        
        // 使用调整后的配置进行量化
        let quantizer = ColorQuantizer::new(config);
        quantizer.quantize(image)
    }
    
    /// 分析图像特征
    fn analyze_image(&self, image: &DynamicImage) -> (u32, bool) {
        use std::collections::HashSet;
        
        let rgba = image.to_rgba8();
        let mut colors = HashSet::new();
        let mut has_transparency = false;
        
        // 采样分析（每10个像素采样一个）
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
    fn test_quantization_config() {
        let config = QuantizationConfig::default();
        assert_eq!(config.max_colors, 256);
        assert_eq!(config.min_quality, 70);
    }
    
    #[test]
    fn test_simple_quantization() {
        let image = DynamicImage::new_rgba8(100, 100);
        let quantizer = ColorQuantizer::new(QuantizationConfig::default());
        let result = quantizer.quantize(&image);
        assert!(result.is_ok());
    }
}
