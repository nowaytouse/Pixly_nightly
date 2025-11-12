/**
 * Preprocessing Pipeline Module
 * 
 * 参考Rimage的预处理管道设计
 * 支持多步骤图像预处理：
 * - Resize (缩放)
 * - Quantization (量化/减色)
 * - Sharpen (锐化)
 * - Color Profile (色彩配置)
 */

use image::{DynamicImage, GenericImageView, ImageError};

pub mod quantization;
pub mod simd_sharpen;
pub mod transform;

/// 预处理步骤类型
#[derive(Debug, Clone, PartialEq)]
pub enum PreprocessStep {
    /// 调整大小
    /// width: 目标宽度 (0表示自动计算)
    /// height: 目标高度 (0表示自动计算)
    /// filter: 缩放滤镜类型
    Resize { 
        width: u32, 
        height: u32, 
        filter: FilterType 
    },
    
    /// 量化/减色
    /// colors: 目标颜色数量 (1-256)
    /// dithering: 是否使用抖动
    Quantization { 
        colors: u8,
        dithering: bool,
    },
    
    /// 锐化
    /// amount: 锐化强度 (0.0-2.0)
    Sharpen { 
        amount: f32 
    },
    
    /// 自动优化（根据图像特征自动选择）
    Auto,
}

/// 缩放滤镜类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    /// 最近邻 - 最快，但质量最低
    Nearest,
    /// 三角形/线性 - 快速，质量中等
    Triangle,
    /// Catmull-Rom - 平衡质量和速度
    CatmullRom,
    /// Gaussian - 高质量
    Gaussian,
    /// Lanczos3 - 最高质量，但最慢
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
    
    pub fn from_str(s: &str) -> Option<Self> {
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
    pub fn process(&self, mut image: DynamicImage) -> Result<DynamicImage, ImageError> {
        println!("📋 Preprocessing pipeline: {} steps", self.steps.len());
        
        for (i, step) in self.steps.iter().enumerate() {
            println!("  Step {}: {:?}", i + 1, step);
            image = self.apply_step(image, step)?;
        }
        
        println!("✅ Preprocessing complete");
        Ok(image)
    }
    
    /// 应用单个预处理步骤
    fn apply_step(&self, image: DynamicImage, step: &PreprocessStep) -> Result<DynamicImage, ImageError> {
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
                // 自动优化暂时直接返回
                Ok(image)
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
    ) -> Result<DynamicImage, ImageError> {
        let (current_width, current_height) = image.dimensions();
        
        // 计算实际目标尺寸
        let (new_width, new_height) = if target_width == 0 && target_height == 0 {
            // 两个都是0，不缩放
            return Ok(image);
        } else if target_width == 0 {
            // 只指定高度，保持纵横比
            let ratio = target_height as f64 / current_height as f64;
            ((current_width as f64 * ratio) as u32, target_height)
        } else if target_height == 0 {
            // 只指定宽度，保持纵横比
            let ratio = target_width as f64 / current_width as f64;
            (target_width, (current_height as f64 * ratio) as u32)
        } else {
            // 两个都指定
            (target_width, target_height)
        };
        
        if new_width == current_width && new_height == current_height {
            println!("    ℹ️  No resize needed (already {}x{})", current_width, current_height);
            return Ok(image);
        }
        
        println!("    🔄 Resizing: {}x{} → {}x{} ({})", 
                 current_width, current_height, 
                 new_width, new_height,
                 format!("{:?}", filter));
        
        Ok(image.resize(new_width, new_height, filter.to_image_filter()))
    }
    
    /// 量化图像（减少颜色数量）
    fn quantize_image(
        &self,
        image: DynamicImage,
        colors: u8,
        _dithering: bool
    ) -> Result<DynamicImage, ImageError> {
        println!("    🎨 Quantizing: {} colors", colors);
        
        // 将图像转换为RGB8
        let rgb_image = image.to_rgb8();
        let (width, height) = rgb_image.dimensions();
        
        // 简单的颜色量化实现（使用color-quant库的思路）
        // Phase 46.14: 基础实现，未来可使用imagequant库优化
        
        // 如果目标颜色数大于等于256，直接返回
        if colors >= 255 {
            println!("    ℹ️  No quantization needed (colors >= 255)");
            return Ok(DynamicImage::ImageRgb8(rgb_image));
        }
        
        // 使用简单的中位切分算法（Median Cut）
        // 这里使用基础实现，性能和质量中等
        use image::Rgba;
        
        // 采样颜色（避免处理过多像素）
        let sample_size = ((width * height) as usize).min(10000);
        let mut colors_vec: Vec<[u8; 3]> = Vec::with_capacity(sample_size);
        
        let step = ((width * height) as usize / sample_size).max(1);
        for (i, pixel) in rgb_image.pixels().enumerate() {
            if i % step == 0 {
                colors_vec.push([pixel[0], pixel[1], pixel[2]]);
            }
        }
        
        // 简单的调色板生成（取平均值）
        let palette = self.generate_palette(&colors_vec, colors as usize);
        
        // 映射到调色板
        let mut quantized = image::RgbaImage::new(width, height);
        for (x, y, pixel) in rgb_image.enumerate_pixels() {
            let closest = self.find_closest_color([pixel[0], pixel[1], pixel[2]], &palette);
            quantized.put_pixel(x, y, Rgba([closest[0], closest[1], closest[2], 255]));
        }
        
        println!("    ✅ Quantization complete");
        Ok(DynamicImage::ImageRgba8(quantized))
    }
    
    /// 生成调色板（简单实现）
    fn generate_palette(&self, colors: &[[u8; 3]], palette_size: usize) -> Vec<[u8; 3]> {
        if colors.is_empty() {
            return vec![[0, 0, 0]; palette_size];
        }
        
        // 简单的K-means聚类思路（这里用更简单的均匀采样）
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
    
    /// 计算颜色距离（欧氏距离的平方）
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
    ) -> Result<DynamicImage, ImageError> {
        if amount <= 0.0 {
            return Ok(image);
        }
        
        println!("    ✨ Sharpening: amount = {}", amount);
        
        // Unsharp Mask算法
        // Phase 46.14: 使用image crate的内置方法
        
        // 将图像转换为RGBA8
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        
        // 应用锐化滤镜（3x3 Laplacian kernel）
        // 锐化kernel: 
        // [ 0, -1,  0]
        // [-1,  5, -1]  (中心权重 = 5 + amount的调整)
        // [ 0, -1,  0]
        
        let mut sharpened = image::RgbaImage::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let pixel = rgba_image.get_pixel(x, y);
                
                // 边缘像素直接复制
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    sharpened.put_pixel(x, y, *pixel);
                    continue;
                }
                
                // 获取周围像素
                let center = rgba_image.get_pixel(x, y);
                let top = rgba_image.get_pixel(x, y - 1);
                let bottom = rgba_image.get_pixel(x, y + 1);
                let left = rgba_image.get_pixel(x - 1, y);
                let right = rgba_image.get_pixel(x + 1, y);
                
                // 应用锐化滤镜（只处理RGB，保持Alpha不变）
                let center_weight = 1.0 + amount * 4.0;
                let neighbor_weight = -amount;
                
                let mut new_pixel = [0u8; 4];
                for i in 0..3 {  // RGB channels
                    let value = center[i] as f32 * center_weight
                        + top[i] as f32 * neighbor_weight
                        + bottom[i] as f32 * neighbor_weight
                        + left[i] as f32 * neighbor_weight
                        + right[i] as f32 * neighbor_weight;
                    
                    new_pixel[i] = value.clamp(0.0, 255.0) as u8;
                }
                new_pixel[3] = center[3];  // 保持Alpha不变
                
                sharpened.put_pixel(x, y, image::Rgba(new_pixel));
            }
        }
        
        println!("    ✅ Sharpening complete");
        Ok(DynamicImage::ImageRgba8(sharpened))
    }
}

impl Default for PreprocessPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// 从字符串解析resize参数
/// 支持格式:
/// - "1920x1080" - 指定宽高
/// - "1920x" - 只指定宽度
/// - "x1080" - 只指定高度
/// - "50%" - 百分比缩放
pub fn parse_resize_param(s: &str) -> Option<(u32, u32)> {
    // 百分比缩放
    if s.ends_with('%') {
        if let Ok(percent) = s.trim_end_matches('%').parse::<f64>() {
            if percent > 0.0 && percent <= 200.0 {
                // 返回特殊值，调用者需要根据原图尺寸计算
                return Some((percent as u32, 0));
            }
        }
        return None;
    }
    
    // 宽x高格式
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
    fn test_filter_type_from_str() {
        assert_eq!(FilterType::from_str("nearest"), Some(FilterType::Nearest));
        assert_eq!(FilterType::from_str("Lanczos3"), Some(FilterType::Lanczos3));
        assert_eq!(FilterType::from_str("invalid"), None);
    }
}
