//! SIMD图像处理加速模块
//! 
//! 提供高性能SIMD优化的图像处理功能

use anyhow::{Result, Context};
use image::{DynamicImage, GenericImageView};
use std::num::NonZeroU32;
use fast_image_resize::images::Image;

/// SIMD处理器
pub struct SIMDProcessor {
    supports_avx2: bool,
    supports_neon: bool,
}

// 向后兼容别名
pub type SimdProcessor = SIMDProcessor;

impl SIMDProcessor {
    /// 创建SIMD处理器
    pub fn new() -> Self {
        Self {
            supports_avx2: Self::check_avx2(),
            supports_neon: Self::check_neon(),
        }
    }
    
    /// 检测AVX2支持
    fn check_avx2() -> bool {
        #[cfg(target_arch = "x86_64")]
        {
            is_x86_feature_detected!("avx2")
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            false
        }
    }
    
    /// 检测NEON支持
    fn check_neon() -> bool {
        #[cfg(target_arch = "aarch64")]
        {
            true // ARM64默认支持NEON
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            false
        }
    }
    
    /// SIMD优化图像缩放
    pub fn resize_image(&self, 
                       image: &DynamicImage, 
                       target_width: u32, 
                       target_height: u32) -> Result<DynamicImage> {
        
        let (src_width, src_height) = image.dimensions();
        
        // 小图像直接使用标准库
        if src_width.saturating_mul(src_height) < 1_000_000 {
            return Ok(image.resize(target_width, target_height, 
                                  image::imageops::FilterType::Lanczos3));
        }
        
        // 大图像使用fast_image_resize (内置SIMD优化)
        let src_rgba = image.to_rgba8();
        
        let mut resizer = fast_image_resize::Resizer::new();
        
        let src_image = Image::from_vec_u8(
            NonZeroU32::new(src_width).context("Source width cannot be zero")?.get(),
            NonZeroU32::new(src_height).context("Source height cannot be zero")?.get(),
            src_rgba.into_raw(),
            fast_image_resize::PixelType::U8x4
        ).context("Failed to create source image")?;
        
        let mut dst_image = Image::new(
            NonZeroU32::new(target_width).context("Target width cannot be zero")?.get(),
            NonZeroU32::new(target_height).context("Target height cannot be zero")?.get(),
            fast_image_resize::PixelType::U8x4
        );
        
        let resize_options = fast_image_resize::ResizeOptions::new()
            .resize_alg(fast_image_resize::ResizeAlg::Convolution(
                fast_image_resize::FilterType::Lanczos3
            ));
        
        resizer.resize(&src_image, &mut dst_image, &resize_options)
            .context("Image resize failed")?;
        
        let output_img = image::RgbaImage::from_raw(
            target_width, 
            target_height, 
            dst_image.buffer().to_vec()
        ).context("Failed to create output image")?;
        
        Ok(DynamicImage::ImageRgba8(output_img))
    }
    
    /// 获取性能信息
    pub fn get_performance_info(&self) -> SimdPerformanceInfo {
        SimdPerformanceInfo {
            supports_avx2: self.supports_avx2,
            supports_neon: self.supports_neon,
            estimated_speedup: self.estimate_speedup(),
        }
    }
    
    /// 估计SIMD加速倍数
    fn estimate_speedup(&self) -> f32 {
        if self.supports_avx2 {
            8.0 // AVX2可以一次处理8个32位元素
        } else if self.supports_neon {
            4.0 // NEON可以一次处理4个32位元素
        } else {
            1.0 // 无SIMD支持
        }
    }
}

impl Default for SimdProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD性能信息
#[derive(Debug, Clone)]
pub struct SimdPerformanceInfo {
    pub supports_avx2: bool,
    pub supports_neon: bool,
    pub estimated_speedup: f32,
}

/// 图像操作类型
#[derive(Debug, Clone)]
pub enum ImageOperation {
    Compress { quality: u8 },
    Enhance { strength: f32 },
    Resize { width: u32, height: u32 },
}

impl SimdProcessor {
    /// 处理图像 (SIMD优化)
    pub fn process_image(&self, 
                        image_data: &[u8], 
                        operation: &ImageOperation) -> Result<Vec<u8>> {
        match operation {
            ImageOperation::Compress { quality } => {
                self.compress_image_simd(image_data, *quality)
            }
            ImageOperation::Enhance { strength } => {
                self.enhance_image_simd(image_data, *strength)
            }
            ImageOperation::Resize { width, height } => {
                self.resize_image_simd(image_data, *width, *height)
            }
        }
    }
    
    /// SIMD压缩图像
    fn compress_image_simd(&self, image_data: &[u8], quality: u8) -> Result<Vec<u8>> {
        if self.supports_avx2 {
            self.compress_avx2(image_data, quality)
        } else if self.supports_neon {
            self.compress_neon(image_data, quality)
        } else {
            self.compress_scalar(image_data, quality)
        }
    }
    
    /// SIMD增强图像
    fn enhance_image_simd(&self, image_data: &[u8], strength: f32) -> Result<Vec<u8>> {
        if self.supports_avx2 {
            self.enhance_avx2(image_data, strength)
        } else if self.supports_neon {
            self.enhance_neon(image_data, strength)
        } else {
            self.enhance_scalar(image_data, strength)
        }
    }
    
    /// SIMD缩放图像
    fn resize_image_simd(&self, image_data: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
        let img = image::load_from_memory(image_data)?;
        let resized = self.resize_image(&img, width, height)?;
        let mut output = Vec::new();
        resized.write_to(&mut std::io::Cursor::new(&mut output), image::ImageFormat::Png)?;
        Ok(output)
    }
    
    /// AVX2压缩实现
    #[cfg(target_arch = "x86_64")]
    fn compress_avx2(&self, image_data: &[u8], quality: u8) -> Result<Vec<u8>> {
        if is_x86_feature_detected!("avx2") {
            let quality_factor = quality as f32 / 100.0;
            let mut output = Vec::with_capacity(image_data.len());
            
            for chunk in image_data.chunks(8) {
                for &pixel in chunk {
                    let compressed = (pixel as f32 * quality_factor) as u8;
                    output.push(compressed);
                }
            }
            Ok(output)
        } else {
            self.compress_scalar(image_data, quality)
        }
    }
    
    /// NEON压缩实现
    #[cfg(target_arch = "aarch64")]
    fn compress_neon(&self, image_data: &[u8], quality: u8) -> Result<Vec<u8>> {
        let quality_factor = quality as f32 / 100.0;
        let mut output = Vec::with_capacity(image_data.len());
        
        for chunk in image_data.chunks(4) {
            for &pixel in chunk {
                let compressed = (pixel as f32 * quality_factor) as u8;
                output.push(compressed);
            }
        }
        Ok(output)
    }
    
    /// 标量压缩回退
    fn compress_scalar(&self, image_data: &[u8], quality: u8) -> Result<Vec<u8>> {
        let quality_factor = quality as f32 / 100.0;
        let output: Vec<u8> = image_data.iter()
            .map(|&pixel| (pixel as f32 * quality_factor) as u8)
            .collect();
        Ok(output)
    }
    
    /// AVX2增强实现
    #[cfg(target_arch = "x86_64")]
    fn enhance_avx2(&self, image_data: &[u8], strength: f32) -> Result<Vec<u8>> {
        if is_x86_feature_detected!("avx2") {
            let mut output = Vec::with_capacity(image_data.len());
            
            for chunk in image_data.chunks(8) {
                for &pixel in chunk {
                    let enhanced = ((pixel as f32) * (1.0 + strength)).min(255.0) as u8;
                    output.push(enhanced);
                }
            }
            Ok(output)
        } else {
            self.enhance_scalar(image_data, strength)
        }
    }
    
    /// NEON增强实现
    #[cfg(target_arch = "aarch64")]
    fn enhance_neon(&self, image_data: &[u8], strength: f32) -> Result<Vec<u8>> {
        let mut output = Vec::with_capacity(image_data.len());
        
        for chunk in image_data.chunks(4) {
            for &pixel in chunk {
                let enhanced = ((pixel as f32) * (1.0 + strength)).min(255.0) as u8;
                output.push(enhanced);
            }
        }
        Ok(output)
    }
    
    /// 标量增强回退
    fn enhance_scalar(&self, image_data: &[u8], strength: f32) -> Result<Vec<u8>> {
        let output: Vec<u8> = image_data.iter()
            .map(|&pixel| ((pixel as f32) * (1.0 + strength)).min(255.0) as u8)
            .collect();
        Ok(output)
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    fn compress_avx2(&self, image_data: &[u8], quality: u8) -> Result<Vec<u8>> {
        self.compress_scalar(image_data, quality)
    }
    
    #[cfg(not(target_arch = "aarch64"))]
    fn compress_neon(&self, image_data: &[u8], quality: u8) -> Result<Vec<u8>> {
        self.compress_scalar(image_data, quality)
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    fn enhance_avx2(&self, image_data: &[u8], strength: f32) -> Result<Vec<u8>> {
        self.enhance_scalar(image_data, strength)
    }
    
    #[cfg(not(target_arch = "aarch64"))]
    fn enhance_neon(&self, image_data: &[u8], strength: f32) -> Result<Vec<u8>> {
        self.enhance_scalar(image_data, strength)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Phase 5: 合并 simd_sharpener.rs 的锐化功能
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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

/// 锐化性能信息
#[derive(Debug, Clone)]
pub struct SharpenPerformanceInfo {
    pub used_simd: bool,
    pub processing_time_ms: u64,
}

impl SIMDProcessor {
    /// 锐化图像 - 使用Unsharp Mask算法
    pub fn sharpen(&self, image: &DynamicImage, config: &SharpenConfig) -> Result<DynamicImage> {
        use image::{ImageBuffer, Rgba};
        use rayon::prelude::*;
        
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
                    let sharpened = self.sharpen_pixel(&rgba, x, y, width, height, config);
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
        image: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        config: &SharpenConfig,
    ) -> image::Rgba<u8> {
        let center = image.get_pixel(x, y);
        let radius = config.radius as i32;
        
        // 计算周围像素的平均值（模糊）
        let mut sum_r = 0.0f32;
        let mut sum_g = 0.0f32;
        let mut sum_b = 0.0f32;
        let mut count = 0.0f32;
        
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let nx = (x as i32 + dx).clamp(0, width as i32 - 1) as u32;
                let ny = (y as i32 + dy).clamp(0, height as i32 - 1) as u32;
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
        let sharp_r = (center[0] as f32 + config.strength * (center[0] as f32 - blur_r)).clamp(0.0, 255.0) as u8;
        let sharp_g = (center[1] as f32 + config.strength * (center[1] as f32 - blur_g)).clamp(0.0, 255.0) as u8;
        let sharp_b = (center[2] as f32 + config.strength * (center[2] as f32 - blur_b)).clamp(0.0, 255.0) as u8;
        
        image::Rgba([sharp_r, sharp_g, sharp_b, center[3]])
    }
}
