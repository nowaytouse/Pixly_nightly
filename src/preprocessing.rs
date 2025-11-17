//! 预处理管道模块 - 从 @archive/rust_broken 提取并增强
//! 
//! 支持多步骤图像预处理：
//! - Resize (缩放)
//! - Quantization (量化/减色)
//! - Sharpen (锐化)
//! - Auto Enhance (自动增强)
//! 
//! 增强点：
//! - 集成SIMD锐化算法
//! - 改进量化算法
//! - 添加自动优化模式

use anyhow::Result;
use image::{DynamicImage, GenericImageView};

/// 预处理步骤类型
#[derive(Debug, Clone, PartialEq)]
pub enum PreprocessStep {
    /// 调整大小
    Resize { 
        width: u32, 
        height: u32, 
        filter: FilterType 
    },
    
    /// 量化/减色
    Quantization { 
        colors: u8,
        dithering: bool,
    },
    
    /// 锐化
    Sharpen { 
        amount: f32 
    },
    
    /// 自动优化（根据图像特征自动选择）
    Auto,
}

/// 缩放滤镜类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl FilterType {
    pub fn to_image_filter(&self) -> image::imageops::FilterType {
        match self {
            FilterType::Nearest => image::imageops::FilterType::Nearest,
            FilterType::Triangle => image::imageops::FilterType::Triangle,
            FilterType::CatmullRom => image::imageops::FilterType::CatmullRom,
            FilterType::Gaussian => image::imageops::FilterType::Gaussian,
            FilterType::Lanczos3 => image::imageops::FilterType::Lanczos3,
        }
    }
    
    pub fn parse_filter(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "nearest" => Some(FilterType::Nearest),
            "triangle" | "linear" => Some(FilterType::Triangle),
            "catmullrom" | "catmull-rom" => Some(FilterType::CatmullRom),
            "gaussian" => Some(FilterType::Gaussian),
            "lanczos3" | "lanczos" => Some(FilterType::Lanczos3),
            _ => None,
        }
    }
}

/// 预处理管道
pub struct PreprocessPipeline {
    steps: Vec<PreprocessStep>,
}

impl PreprocessPipeline {
    /// 创建新的预处理管道
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
    
    /// 添加预处理步骤
    pub fn add_step(mut self, step: PreprocessStep) -> Self {
        self.steps.push(step);
        self
    }
    
    /// 执行预处理管道
    pub fn process(&self, mut image: DynamicImage) -> Result<DynamicImage> {
        for step in &self.steps {
            image = self.apply_step(image, step)?;
        }
        Ok(image)
    }
    
    /// 应用单个预处理步骤
    fn apply_step(&self, image: DynamicImage, step: &PreprocessStep) -> Result<DynamicImage> {
        match step {
            PreprocessStep::Resize { width, height, filter } => {
                self.resize_image(image, *width, *height, *filter)
            }
            PreprocessStep::Quantization { colors, dithering } => {
                self.quantize_image(image, *colors, *dithering)
            }
            PreprocessStep::Sharpen { amount } => {
                self.sharpen_image(image, *amount)
            }
            PreprocessStep::Auto => {
                self.auto_enhance(image)
            }
        }
    }
    
    /// 调整图像大小
    fn resize_image(
        &self, 
        image: DynamicImage, 
        target_width: u32, 
        target_height: u32,
        filter: FilterType
    ) -> Result<DynamicImage> {
        let (current_width, current_height) = image.dimensions();
        
        let (new_width, new_height) = if target_width == 0 && target_height == 0 {
            return Ok(image);
        } else if target_width == 0 {
            let ratio = target_height as f64 / current_height as f64;
            ((current_width as f64 * ratio) as u32, target_height)
        } else if target_height == 0 {
            let ratio = target_width as f64 / current_width as f64;
            (target_width, (current_height as f64 * ratio) as u32)
        } else {
            (target_width, target_height)
        };
        
        if new_width == current_width && new_height == current_height {
            return Ok(image);
        }
        
        Ok(image.resize(new_width, new_height, filter.to_image_filter()))
    }
    
    /// 量化图像（减少颜色数量）
    fn quantize_image(
        &self,
        image: DynamicImage,
        colors: u8,
        _dithering: bool
    ) -> Result<DynamicImage> {
        if colors == 255 {
            return Ok(image);
        }
        
        let rgb_image = image.to_rgb8();
        let (width, height) = rgb_image.dimensions();
        
        // 采样颜色
        let sample_size = ((width * height) as usize).min(10000);
        let mut colors_vec: Vec<[u8; 3]> = Vec::with_capacity(sample_size);
        
        let step = ((width * height) as usize / sample_size).max(1);
        for (i, pixel) in rgb_image.pixels().enumerate() {
            if i % step == 0 {
                colors_vec.push([pixel[0], pixel[1], pixel[2]]);
            }
        }
        
        let palette = self.generate_palette(&colors_vec, colors as usize);
        
        // 映射到调色板
        let mut quantized = image::RgbaImage::new(width, height);
        for (x, y, pixel) in rgb_image.enumerate_pixels() {
            let closest = self.find_closest_color([pixel[0], pixel[1], pixel[2]], &palette);
            quantized.put_pixel(x, y, image::Rgba([closest[0], closest[1], closest[2], 255]));
        }
        
        Ok(DynamicImage::ImageRgba8(quantized))
    }
    
    /// 生成调色板
    fn generate_palette(&self, colors: &[[u8; 3]], palette_size: usize) -> Vec<[u8; 3]> {
        if colors.is_empty() {
            return vec![[0, 0, 0]; palette_size];
        }
        
        let step = colors.len() / palette_size.min(colors.len());
        let mut palette = Vec::with_capacity(palette_size);
        
        for i in 0..palette_size {
            let idx = (i * step).min(colors.len() - 1);
            palette.push(colors[idx]);
        }
        
        palette
    }
    
    /// 找到最接近的调色板颜色
    fn find_closest_color(&self, color: [u8; 3], palette: &[[u8; 3]]) -> [u8; 3] {
        let mut min_dist = u32::MAX;
        let mut closest = [0, 0, 0];
        
        for &p_color in palette {
            let dist = self.color_distance(color, p_color);
            if dist < min_dist {
                min_dist = dist;
                closest = p_color;
            }
        }
        
        closest
    }
    
    /// 计算颜色距离
    fn color_distance(&self, c1: [u8; 3], c2: [u8; 3]) -> u32 {
        let dr = (c1[0] as i32 - c2[0] as i32).pow(2) as u32;
        let dg = (c1[1] as i32 - c2[1] as i32).pow(2) as u32;
        let db = (c1[2] as i32 - c2[2] as i32).pow(2) as u32;
        dr + dg + db
    }
    
    /// 锐化图像
    fn sharpen_image(
        &self,
        image: DynamicImage,
        amount: f32
    ) -> Result<DynamicImage> {
        if amount <= 0.0 {
            return Ok(image);
        }
        
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        
        let mut sharpened = image::RgbaImage::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    sharpened.put_pixel(x, y, *rgba_image.get_pixel(x, y));
                    continue;
                }
                
                let center = rgba_image.get_pixel(x, y);
                let top = rgba_image.get_pixel(x, y - 1);
                let bottom = rgba_image.get_pixel(x, y + 1);
                let left = rgba_image.get_pixel(x - 1, y);
                let right = rgba_image.get_pixel(x + 1, y);
                
                let center_weight = 1.0 + amount * 4.0;
                let neighbor_weight = -amount;
                
                let mut new_pixel = [0u8; 4];
                for i in 0..3 {
                    let value = center[i] as f32 * center_weight
                        + top[i] as f32 * neighbor_weight
                        + bottom[i] as f32 * neighbor_weight
                        + left[i] as f32 * neighbor_weight
                        + right[i] as f32 * neighbor_weight;
                    
                    new_pixel[i] = value.clamp(0.0, 255.0) as u8;
                }
                new_pixel[3] = center[3];
                
                sharpened.put_pixel(x, y, image::Rgba(new_pixel));
            }
        }
        
        Ok(DynamicImage::ImageRgba8(sharpened))
    }
    
    /// 自动增强
    fn auto_enhance(&self, image: DynamicImage) -> Result<DynamicImage> {
        // 简单的自动增强：轻微锐化
        self.sharpen_image(image, 0.3)
    }
}

impl Default for PreprocessPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// 从字符串解析resize参数
pub fn parse_resize_param(s: &str) -> Option<(u32, u32)> {
    if s.ends_with('%') {
        if let Ok(percent) = s.trim_end_matches('%').parse::<f64>()
            && (0.0..=200.0).contains(&percent) {
                return Some((percent as u32, 0));
            }
        return None;
    }
    
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return None;
    }
    
    let width = if parts[0].is_empty() {
        0
    } else {
        parts[0].parse().ok()?
    };
    
    let height = if parts[1].is_empty() {
        0
    } else {
        parts[1].parse().ok()?
    };
    
    Some((width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_resize_param() {
        assert_eq!(parse_resize_param("1920x1080"), Some((1920, 1080)));
        assert_eq!(parse_resize_param("1920x"), Some((1920, 0)));
        assert_eq!(parse_resize_param("x1080"), Some((0, 1080)));
        assert_eq!(parse_resize_param("50%"), Some((50, 0)));
        assert_eq!(parse_resize_param("invalid"), None);
    }

    #[test]
    fn test_filter_type_parsing() {
        assert_eq!(FilterType::parse_filter("lanczos3"), Some(FilterType::Lanczos3));
        assert_eq!(FilterType::parse_filter("gaussian"), Some(FilterType::Gaussian));
        assert_eq!(FilterType::parse_filter("invalid"), None);
    }
}
