//! 图像变换处理器
//! 
//! 提供裁剪、旋转、翻转等图像变换操作

use anyhow::Result;
use image::{DynamicImage, RgbaImage, GenericImageView};
use std::f32::consts::PI;

/// 裁剪模式
#[derive(Debug, Clone)]
pub enum CropMode {
    /// 中心裁剪
    Center(u32, u32),
    /// 智能裁剪
    Smart(u32, u32),
    /// 自定义区域
    Custom { x: u32, y: u32, width: u32, height: u32 },
    /// 按比例裁剪
    AspectRatio(f32),
}

/// 旋转模式
#[derive(Debug, Clone)]
pub enum RotateMode {
    Rotate90,
    Rotate180,
    Rotate270,
    Custom(f32),
}

/// 图像变换配置
#[derive(Debug, Clone, Default)]
pub struct TransformConfig {
    pub crop: Option<CropMode>,
    pub rotate: Option<RotateMode>,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

/// 图像变换处理器
pub struct ImageTransformer {
    config: TransformConfig,
}

impl ImageTransformer {
    pub fn new(config: TransformConfig) -> Self {
        Self { config }
    }
    
    /// 执行所有变换
    pub fn transform(&self, image: &DynamicImage) -> Result<DynamicImage> {
        let mut result = image.clone();
        
        if let Some(rotate) = &self.config.rotate {
            result = self.rotate_image(&result, rotate)?;
        }
        
        if self.config.flip_horizontal {
            result = result.fliph();
        }
        
        if self.config.flip_vertical {
            result = result.flipv();
        }
        
        if let Some(crop) = &self.config.crop {
            result = self.crop_image(&result, crop)?;
        }
        
        Ok(result)
    }
    
    /// 裁剪图像
    fn crop_image(&self, image: &DynamicImage, mode: &CropMode) -> Result<DynamicImage> {
        let (img_width, img_height) = image.dimensions();
        
        let (x, y, width, height) = match mode {
            CropMode::Center(w, h) => {
                let x = img_width.saturating_sub(*w) / 2;
                let y = img_height.saturating_sub(*h) / 2;
                (x, y, *w, *h)
            }
            CropMode::Smart(w, h) => {
                let (x, y) = self.find_interesting_region(image, *w, *h);
                (x, y, *w, *h)
            }
            CropMode::Custom { x, y, width, height } => {
                (*x, *y, *width, *height)
            }
            CropMode::AspectRatio(ratio) => {
                let current_ratio = img_width as f32 / img_height as f32;
                if current_ratio > *ratio {
                    let new_width = (img_height as f32 * ratio) as u32;
                    let x = (img_width - new_width) / 2;
                    (x, 0, new_width, img_height)
                } else {
                    let new_height = (img_width as f32 / ratio) as u32;
                    let y = (img_height - new_height) / 2;
                    (0, y, img_width, new_height)
                }
            }
        };
        
        let x = x.min(img_width.saturating_sub(1));
        let y = y.min(img_height.saturating_sub(1));
        let width = width.min(img_width - x);
        let height = height.min(img_height - y);
        
        Ok(image.crop_imm(x, y, width, height))
    }
    
    /// 旋转图像
    fn rotate_image(&self, image: &DynamicImage, mode: &RotateMode) -> Result<DynamicImage> {
        let result = match mode {
            RotateMode::Rotate90 => image.rotate90(),
            RotateMode::Rotate180 => image.rotate180(),
            RotateMode::Rotate270 => image.rotate270(),
            RotateMode::Custom(angle) => self.rotate_custom(image, *angle)?,
        };
        
        Ok(result)
    }
    
    /// 自定义角度旋转
    fn rotate_custom(&self, image: &DynamicImage, angle: f32) -> Result<DynamicImage> {
        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();
        
        let radians = angle * PI / 180.0;
        let cos = radians.cos().abs();
        let sin = radians.sin().abs();
        
        let new_width = (width as f32 * cos + height as f32 * sin).ceil() as u32;
        let new_height = (width as f32 * sin + height as f32 * cos).ceil() as u32;
        
        let mut output = RgbaImage::new(new_width, new_height);
        
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let new_cx = new_width as f32 / 2.0;
        let new_cy = new_height as f32 / 2.0;
        
        let cos = radians.cos();
        let sin = radians.sin();
        
        for y in 0..new_height {
            for x in 0..new_width {
                let dx = x as f32 - new_cx;
                let dy = y as f32 - new_cy;
                
                let src_x = (dx * cos + dy * sin + cx) as i32;
                let src_y = (-dx * sin + dy * cos + cy) as i32;
                
                if src_x >= 0 && src_x < width as i32 && src_y >= 0 && src_y < height as i32 {
                    let pixel = rgba.get_pixel(src_x as u32, src_y as u32);
                    output.put_pixel(x, y, *pixel);
                }
            }
        }
        
        Ok(DynamicImage::ImageRgba8(output))
    }
    
    /// 找到感兴趣区域
    fn find_interesting_region(&self, image: &DynamicImage, target_w: u32, target_h: u32) -> (u32, u32) {
        let (img_w, img_h) = image.dimensions();
        let rgba = image.to_rgba8();
        
        let mut max_variance = 0.0;
        let mut best_x = 0;
        let mut best_y = 0;
        
        let step = 10;
        for y in (0..img_h.saturating_sub(target_h)).step_by(step) {
            for x in (0..img_w.saturating_sub(target_w)).step_by(step) {
                let variance = self.calculate_region_variance(&rgba, x, y, target_w, target_h);
                if variance > max_variance {
                    max_variance = variance;
                    best_x = x;
                    best_y = y;
                }
            }
        }
        
        (best_x, best_y)
    }
    
    /// 计算区域方差
    fn calculate_region_variance(&self, image: &RgbaImage, x: u32, y: u32, w: u32, h: u32) -> f32 {
        let mut sum = 0.0;
        let mut count = 0;
        
        for dy in 0..h.min(10) {
            for dx in 0..w.min(10) {
                if let Some(pixel) = image.get_pixel_checked(x + dx, y + dy) {
                    let gray = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
                    sum += gray;
                    count += 1;
                }
            }
        }
        
        if count == 0 {
            return 0.0;
        }
        
        let mean = sum / count as f32;
        let mut variance = 0.0;
        
        for dy in 0..h.min(10) {
            for dx in 0..w.min(10) {
                if let Some(pixel) = image.get_pixel_checked(x + dx, y + dy) {
                    let gray = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
                    variance += (gray - mean).powi(2);
                }
            }
        }
        
        variance / count as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transformer_creation() {
        let config = TransformConfig::default();
        let transformer = ImageTransformer::new(config);
        assert!(!transformer.config.flip_horizontal);
    }
}
