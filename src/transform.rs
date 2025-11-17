//! 🔄 图像变换模块
//!
//! 提供裁剪、旋转、翻转等图像变换操作
//!
//! ## 功能特性
//!
//! - **智能裁剪**: 基于内容感知的自动裁剪
//! - **多种旋转**: 90°/180°/270°/自定义角度
//! - **翻转操作**: 水平/垂直翻转
//! - **比例裁剪**: 按目标宽高比自动裁剪
//!
//! ## 使用示例
//!
//! ```rust
//! use pixly_kernel::transform::{TransformBuilder, CropMode, RotateMode};
//!
//! # fn example() -> anyhow::Result<()> {
//! # let image = image::DynamicImage::new_rgb8(100, 100);
//! let transformer = TransformBuilder::new()
//!     .crop(CropMode::Center(800, 600))
//!     .rotate(RotateMode::Rotate90)
//!     .flip_horizontal()
//!     .build();
//!
//! let result = transformer.transform(&image)?;
//! # Ok(())
//! # }
//! ```

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
    /// 创建新的变换处理器
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
            CropMode::Custom { x, y, width, height } => (*x, *y, *width, *height),
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

        // 确保裁剪区域在图像范围内
        let x = x.min(img_width.saturating_sub(1));
        let y = y.min(img_height.saturating_sub(1));
        let width = width.min(img_width - x);
        let height = height.min(img_height - y);

        tracing::info!("📐 Cropping image: {}x{} -> {}x{}", img_width, img_height, width, height);

        Ok(image.crop_imm(x, y, width, height))
    }

    /// 旋转图像
    fn rotate_image(&self, image: &DynamicImage, mode: &RotateMode) -> Result<DynamicImage> {
        let result = match mode {
            RotateMode::Rotate90 => {
                tracing::info!("🔄 Rotating 90 degrees");
                image.rotate90()
            }
            RotateMode::Rotate180 => {
                tracing::info!("🔄 Rotating 180 degrees");
                image.rotate180()
            }
            RotateMode::Rotate270 => {
                tracing::info!("🔄 Rotating 270 degrees");
                image.rotate270()
            }
            RotateMode::Custom(angle) => {
                tracing::info!("🔄 Rotating {} degrees", angle);
                self.rotate_custom(image, *angle)?
            }
        };

        Ok(result)
    }

    /// 自定义角度旋转
    fn rotate_custom(&self, image: &DynamicImage, angle: f32) -> Result<DynamicImage> {
        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();

        // 计算旋转后的图像尺寸
        let radians = angle * PI / 180.0;
        let cos = radians.cos().abs();
        let sin = radians.sin().abs();

        let new_width = (width as f32 * cos + height as f32 * sin).ceil() as u32;
        let new_height = (width as f32 * sin + height as f32 * cos).ceil() as u32;

        let mut output = RgbaImage::new(new_width, new_height);

        // 计算中心点
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let new_cx = new_width as f32 / 2.0;
        let new_cy = new_height as f32 / 2.0;

        // 旋转矩阵
        let cos = radians.cos();
        let sin = radians.sin();

        for y in 0..new_height {
            for x in 0..new_width {
                // 反向映射
                let dx = x as f32 - new_cx;
                let dy = y as f32 - new_cy;

                let src_x = dx * cos + dy * sin + cx;
                let src_y = -dx * sin + dy * cos + cy;

                // 双线性插值
                if src_x >= 0.0
                    && src_x < width as f32 - 1.0
                    && src_y >= 0.0
                    && src_y < height as f32 - 1.0
                {
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
    fn bilinear_interpolate(
        &self,
        p00: &Rgba<u8>,
        p01: &Rgba<u8>,
        p10: &Rgba<u8>,
        p11: &Rgba<u8>,
        dx: f32,
        dy: f32,
    ) -> Rgba<u8> {
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
    fn find_interesting_region(
        &self,
        image: &DynamicImage,
        target_width: u32,
        target_height: u32,
    ) -> (u32, u32) {
        let (width, height) = image.dimensions();

        // 简单的边缘检测来找到重要区域
        let gray = image.to_luma8();
        let mut max_energy = 0.0;
        let mut best_x = 0;
        let mut best_y = 0;

        // 滑动窗口寻找能量最高的区域
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

        tracing::debug!(
            "智能裁剪：找到最佳区域 ({}, {}), 能量值: {:.2}",
            best_x,
            best_y,
            max_energy
        );

        (best_x, best_y)
    }

    /// 计算区域能量（基于边缘强度）
    fn calculate_region_energy(
        &self,
        gray: &image::GrayImage,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> f32 {
        let mut energy = 0.0;
        let mut count = 0;

        // Sobel边缘检测的简化版本
        for dy in 1..height.saturating_sub(1) {
            for dx in 1..width.saturating_sub(1) {
                let px = x + dx;
                let py = y + dy;

                if px >= gray.width() - 1 || py >= gray.height() - 1 {
                    continue;
                }

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

/// 变换构建器
pub struct TransformBuilder {
    config: TransformConfig,
}

impl TransformBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: TransformConfig::default(),
        }
    }

    /// 设置裁剪模式
    pub fn crop(mut self, mode: CropMode) -> Self {
        self.config.crop = Some(mode);
        self
    }

    /// 设置旋转模式
    pub fn rotate(mut self, mode: RotateMode) -> Self {
        self.config.rotate = Some(mode);
        self
    }

    /// 启用水平翻转
    pub fn flip_horizontal(mut self) -> Self {
        self.config.flip_horizontal = true;
        self
    }

    /// 启用垂直翻转
    pub fn flip_vertical(mut self) -> Self {
        self.config.flip_vertical = true;
        self
    }

    /// 构建变换处理器
    pub fn build(self) -> ImageTransformer {
        ImageTransformer::new(self.config)
    }
}

impl Default for TransformBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_config_default() {
        let config = TransformConfig::default();
        assert!(config.crop.is_none());
        assert!(config.rotate.is_none());
        assert!(!config.flip_horizontal);
        assert!(!config.flip_vertical);
    }

    #[test]
    fn test_transform_builder() {
        let transformer = TransformBuilder::new()
            .crop(CropMode::Center(800, 600))
            .rotate(RotateMode::Rotate90)
            .flip_horizontal()
            .build();

        assert!(transformer.config.crop.is_some());
        assert!(transformer.config.rotate.is_some());
        assert!(transformer.config.flip_horizontal);
    }

    #[test]
    fn test_center_crop() {
        let image = DynamicImage::new_rgb8(1000, 800);
        let transformer = TransformBuilder::new()
            .crop(CropMode::Center(500, 400))
            .build();

        let result = transformer.transform(&image).unwrap();
        assert_eq!(result.dimensions(), (500, 400));
    }

    #[test]
    fn test_rotate_90() {
        let image = DynamicImage::new_rgb8(100, 200);
        let transformer = TransformBuilder::new()
            .rotate(RotateMode::Rotate90)
            .build();

        let result = transformer.transform(&image).unwrap();
        assert_eq!(result.dimensions(), (200, 100));
    }

    #[test]
    fn test_flip_operations() {
        let image = DynamicImage::new_rgb8(100, 100);
        let transformer = TransformBuilder::new()
            .flip_horizontal()
            .flip_vertical()
            .build();

        let result = transformer.transform(&image);
        assert!(result.is_ok());
    }

    #[test]
    fn test_aspect_ratio_crop() {
        let image = DynamicImage::new_rgb8(1600, 900);
        let transformer = TransformBuilder::new()
            .crop(CropMode::AspectRatio(1.0)) // 1:1 square
            .build();

        let result = transformer.transform(&image).unwrap();
        let (w, h) = result.dimensions();
        assert_eq!(w, h); // Should be square
    }
}
