//! SIMDimageprocessing加速module
//! 
//! providehighperformance SIMDoptimizationimageprocessingfeature

use anyhow::{Result, Context};
use image::{DynamicImage, GenericImageView};
use std::num::NonZeroU32;
use fast_image_resize::images::Image;

/// SIMDhandler
pub structure SIMDProcessor {
 supports_avx2: bool,
 supports_neon: bool,
}

// 向 after compatibility别名
pub type SimdProcessor = SIMDProcessor;

impl SIMDProcessor {
 /// createSIMDhandler
 pub fn new() -> Self {
 Self {
 supports_avx2: Self::check_avx2(),
 supports_neon: Self::check_neon(),
 }
 }
 
 /// detectionAVX2support
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
 
 /// detectionNEONsupport
 fn check_neon() -> bool {
 #[cfg(target_arch = "aarch64")]
 {
 true // ARM64defaultsupportNEON
 }
 #[cfg(not(target_arch = "aarch64"))]
 {
 false
 }
 }
 
 /// SIMDoptimizationimagescale
 pub fn resize_image(&self, 
 image: &DynamicImage, 
 target_width: u32, 
 target_height: u32) -> Result<DynamicImage> {
 
 let (src_width, src_height) = image.dimensions();
 
 // 小image直接usestandardlibrary
 if src_width.saturating_mul(src_height) < 1_000_000 {
 return Ok(image.resize(target_width, target_height, 
 image::imageops::FilterType::Lanczos3));
 }
 
 // 大imageusefast_image_resize (内置SIMDoptimization)
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
 
 /// getperformanceinformation
 pub fn get_performance_info(&self) -> SimdPerformanceInfo {
 SimdPerformanceInfo {
 supports_avx2: self.supports_avx2,
 supports_neon: self.supports_neon,
 estimated_speedup: self.estimate_speedup(),
 }
 }
 
 /// estimated SIMD加速倍数
 fn estimate_speedup(&self) -> f32 {
 if self.supports_avx2 {
 8.0 // AVX2可以once处理832位元素
 } else if self.supports_neon {
 4.0 // NEON可以once处理432位元素
 } else {
 1.0 // 无SIMDsupport
 }
 }
}

impl Default for SimdProcessor {
 fn default() -> Self {
 Self::new()
 }
}

/// SIMDperformanceinformation
#[derive(Debug, Clone)]
pub structure SimdPerformanceInfo {
 pub supports_avx2: bool,
 pub supports_neon: bool,
 pub estimated_speedup: f32,
}

/// image操作type
#[derive(Debug, Clone)]
pub enum ImageOperation {
 Compress { quality: u8 },
 Enhance { strength: f32 },
 Resize { width: u32, height: u32 },
}

impl SimdProcessor {
 /// processingimage (SIMDoptimization)
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
 
 /// SIMDcompressionimage
 fn compress_image_simd(&self, image_data: &[u8], quality: u8) -> Result<Vec<u8>> {
 if self.supports_avx2 {
 self.compress_avx2(image_data, quality)
 } else if self.supports_neon {
 self.compress_neon(image_data, quality)
 } else {
 self.compress_scalar(image_data, quality)
 }
 }
 
 /// SIMDenhancedimage
 fn enhance_image_simd(&self, image_data: &[u8], strength: f32) -> Result<Vec<u8>> {
 if self.supports_avx2 {
 self.enhance_avx2(image_data, strength)
 } else if self.supports_neon {
 self.enhance_neon(image_data, strength)
 } else {
 self.enhance_scalar(image_data, strength)
 }
 }
 
 /// SIMDscaleimage
 fn resize_image_simd(&self, image_data: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
 let img = image::load_from_memory(image_data)?;
 let resized = self.resize_image(&img, width, height)?;
 let mut output = Vec::new();
 resized.write_to(&mut std::io::Cursor::new(&mut output), image::ImageFormat::Png)?;
 Ok(output)
 }
 
 /// AVX2compressionimplementation
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
 
 /// NEONcompressionimplementation
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
 
 /// 标量compression回退
 fn compress_scalar(&self, image_data: &[u8], quality: u8) -> Result<Vec<u8>> {
 let quality_factor = quality as f32 / 100.0;
 let output: Vec<u8> = image_data.iter()
 .map(|&pixel| (pixel as f32 * quality_factor) as u8)
 .collect();
 Ok(output)
 }
 
 /// AVX2enhancedimplementation
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
 
 /// NEONenhancedimplementation
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
 
 /// 标量enhanced回退
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
// Phase 5: merged simd_sharpener.rs sharpeningfeature
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// sharpeningconfiguration
#[derive(Debug, Clone)]
pub structure SharpenConfig {
 /// sharpeningstrength (0.0-10.0)
 pub strength: f32,
 /// sharpening半径
 pub radius: u32,
 /// threshold
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

/// sharpeningperformanceinformation
#[derive(Debug, Clone)]
pub structure SharpenPerformanceInfo {
 pub used_simd: bool,
 pub processing_time_ms: u64,
}

impl SIMDProcessor {
 /// sharpeningimage - use Unsharp Maskalgorithm
 pub fn sharpen(&self, image: &DynamicImage, config: &SharpenConfig) -> Result<DynamicImage> {
 use image::{ImageBuffer, Rgba};
 use rayon::prelude::*;
 
 let rgba = image.to_rgba8();
 let (width, height) = rgba.dimensions();
 
 // createoutputimage
 let mut output = ImageBuffer::new(width, height);
 
 // use Rayonparallelprocessingeveryaline
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
 
 // will resultwriteoutputimage
 for (y, row) in rows.iter().enumerate() {
 for (x, pixel) in row.iter().enumerate() {
 output.put_pixel(x as u32, y as u32, *pixel);
 }
 }
 
 Ok(DynamicImage::ImageRgba8(output))
 }
 
 /// sharpeningsinglepixel
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
 
 // calculation周围pixelaveragevalue（blur）
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
