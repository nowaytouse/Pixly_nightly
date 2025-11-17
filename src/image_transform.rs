//! 图像变换预处理模块
//! 
//! 实现裁剪、旋转、翻转等图像变换操作

use anyhow::Result;
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use std::f32::consts::PI;

/// 裁剪模式
#[derive(Debug, Clone)]
pub enum CropMode {
    /// 中心裁剪
    Center(u32, u32),
    /// 智能裁剪（基于内容感知）
    Smart(u32, u32),
    /// 自定义区域裁剪
    Custom { x: u32, y: u32, width: u32, height: u32 },
    /// 按比例裁剪
    AspectRatio(f32),
}

/// 旋转模式
#[derive(Debug, Clone)]
pub enum RotateMode {
    /// 90度旋转
    Rotate90,
    /// 180度旋转
    Rotate180,
    /// 270度旋转
    Rotate270,
    /// 自定义角度旋转
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
        
        // 1. 旋转
        if let Some(rotate) = &self.config.rotate {
            result = self.rotate_image(&result, rotate)?;
        }
        
        // 2. 翻转
        if self.config.flip_horizontal {
            result = result.fliph();
        }
        if self.config.flip_vertical {
            result = result.flipv();
        }
        
        // 3. 裁剪（最后执行）
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
                
                let src_x = dx * cos + dy * sin + cx;
                let src_y = -dx * sin + dy * cos + cy;
                
                if src_x >= 0.0 && src_x < width as f32 - 1.0 &&
                   src_y >= 0.0 && src_y < height as f32 - 1.0 {
                    let x0 = src_x.floor() as u32;
                    let y0 = src_y.floor() as u32;
                    let x1 = (x0 + 1).min(width - 1);
                    let y1 = (y0 + 1).min(height - 1);
                    
                    let dx = src_x - x0 as f32;
                    let dy = src_y - y0 as f32;
                    
                    let p00 = rgba.get_pixel(x0, y0);
                    let p01 = rgba.get_pixel(x0, y1);
                    let p10 = rgba.get_pixel(x1, y0);
                    let p11 = rgba.get_pixel(x1, y1);
                    
                    let pixel = self.bilinear_interpolate(p00, p01, p10, p11, dx, dy);
                    output.put_pixel(x, y, pixel);
                }
            }
        }
        
        Ok(DynamicImage::ImageRgba8(output))
    }
    
    /// 双线性插值
    fn bilinear_interpolate(&self, p00: &Rgba<u8>, p01: &Rgba<u8>, 
                           p10: &Rgba<u8>, p11: &Rgba<u8>,
                           dx: f32, dy: f32) -> Rgba<u8> {
        let mut result = [0u8; 4];
        
        for i in 0..4 {
            let v00 = p00[i] as f32;
            let v01 = p01[i] as f32;
            let v10 = p10[i] as f32;
            let v11 = p11[i] as f32;
            
            let v0 = v00 * (1.0 - dx) + v10 * dx;
            let v1 = v01 * (1.0 - dx) + v11 * dx;
            let v = v0 * (1.0 - dy) + v1 * dy;
            
            result[i] = v.round() as u8;
        }
        
        Rgba(result)
    }
    
    /// 查找图像中的重要区域（用于智能裁剪）
    fn find_interesting_region(&self, image: &DynamicImage, 
                               target_width: u32, target_height: u32) -> (u32, u32) {
        let (width, height) = image.dimensions();
        let gray = image.to_luma8();
        let mut max_energy = 0.0;
        let mut best_x = 0;
        let mut best_y = 0;
        
        let step = 10;
        for y in (0..height.saturating_sub(target_height)).step_by(step) {
            for x in (0..width.saturating_sub(target_width)).step_by(step) {
                let energy = self.calculate_region_energy(&gray, x, y, target_width, target_height);
                if energy > max_energy {
                    max_energy = energy;
                    best_x = x;
                    best_y = y;
                }
            }
        }
        
        (best_x, best_y)
    }
    
    /// 计算区域能量（基于边缘强度）
    fn calculate_region_energy(&self, gray: &image::GrayImage, 
                              x: u32, y: u32, width: u32, height: u32) -> f32 {
        let mut energy = 0.0;
        let mut count = 0;
        
        for dy in 1..height-1 {
            for dx in 1..width-1 {
                let px = x + dx;
                let py = y + dy;
                
                let _center = gray.get_pixel(px, py)[0] as f32;
                let left = gray.get_pixel(px - 1, py)[0] as f32;
                let right = gray.get_pixel(px + 1, py)[0] as f32;
                let top = gray.get_pixel(px, py - 1)[0] as f32;
                let bottom = gray.get_pixel(px, py + 1)[0] as f32;
                
                let gx = (right - left).abs();
                let gy = (bottom - top).abs();
                let gradient = (gx * gx + gy * gy).sqrt();
                
                energy += gradient;
                count += 1;
            }
        }
        
        if count > 0 {
            energy / count as f32
        } else {
            0.0
        }
    }
}
