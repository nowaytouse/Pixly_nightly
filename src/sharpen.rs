use anyhow::Result;
use image::DynamicImage;
#[cfg(target_arch = "x86_64")]
use image::{GrayImage, Rgba, RgbaImage};
use tracing::{debug, info};

/// 锐化配置
#[derive(Debug, Clone)]
pub struct SharpenConfig {
    /// 锐化强度 (0.0-10.0)
    pub strength: f32,
    /// 锐化半径 (1-5)
    pub radius: u32,
    /// 阈值（避免过度锐化）
    pub threshold: f32,
    /// 是否使用SIMD优化
    pub use_simd: bool,
}

impl Default for SharpenConfig {
    fn default() -> Self {
        Self {
            strength: 1.0,
            radius: 1,
            threshold: 0.1,
            use_simd: cfg!(target_arch = "x86_64"),
        }
    }
}

/// SIMD优化的锐化处理器
pub struct SimdSharpener {
    config: SharpenConfig,
}

impl SimdSharpener {
    pub fn new(config: SharpenConfig) -> Self {
        Self { config }
    }

    /// 执行锐化
    pub fn sharpen(&self, image: &DynamicImage) -> Result<DynamicImage> {
        #[cfg(target_arch = "x86_64")]
        if self.config.use_simd && is_x86_feature_detected!("avx2") {
            info!("Using SIMD-optimized sharpening");
            return self.sharpen_simd(image);
        }

        info!("Using standard sharpening algorithm");
        self.sharpen_standard(image)
    }

    /// SIMD优化的锐化实现
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn sharpen_simd_impl(&self, pixels: &[u8], width: usize, height: usize) -> Vec<u8> {
        use std::arch::x86_64::*;

        let mut output = vec![0u8; pixels.len()];
        let radius_factor = self.config.radius.max(1) as f32;
        let threshold = self.config.threshold.max(0.0);
        let strength = self.config.strength * radius_factor / (1.0 + threshold);

        // 锐化核（3x3）
        let kernel = [
            -strength,
            -strength,
            -strength,
            -strength,
            8.0 * strength + 1.0,
            -strength,
            -strength,
            -strength,
            -strength,
        ];

        // 处理内部像素（跳过边缘）
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let idx = y * width + x;

                // 使用SIMD处理8个像素
                if x + 8 < width - 1 {
                    let mut sum = _mm256_setzero_ps();

                    // 应用3x3核
                    for ky in 0..3 {
                        for kx in 0..3 {
                            let pixel_idx = (y + ky - 1) * width + (x + kx - 1);
                            let pixel_val = pixels[pixel_idx] as f32;
                            let kernel_val = kernel[ky * 3 + kx];

                            let pixel_vec = _mm256_set1_ps(pixel_val);
                            let kernel_vec = _mm256_set1_ps(kernel_val);
                            let mul = _mm256_mul_ps(pixel_vec, kernel_vec);
                            sum = _mm256_add_ps(sum, mul);
                        }
                    }

                    // 提取结果
                    let mut result = [0f32; 8];
                    _mm256_storeu_ps(result.as_mut_ptr(), sum);

                    // 裁剪到0-255范围
                    for i in 0..8 {
                        output[idx + i] = result[i].max(0.0).min(255.0) as u8;
                    }
                } else {
                    // 非SIMD处理剩余像素
                    let mut sum = 0.0;
                    for ky in 0..3 {
                        for kx in 0..3 {
                            let pixel_idx = (y + ky - 1) * width + (x + kx - 1);
                            sum += pixels[pixel_idx] as f32 * kernel[ky * 3 + kx];
                        }
                    }
                    output[idx] = sum.max(0.0).min(255.0) as u8;
                }
            }
        }

        // 复制边缘像素
        for x in 0..width {
            output[x] = pixels[x];
            output[(height - 1) * width + x] = pixels[(height - 1) * width + x];
        }
        for y in 0..height {
            output[y * width] = pixels[y * width];
            output[y * width + width - 1] = pixels[y * width + width - 1];
        }

        output
    }

    /// SIMD锐化
    #[cfg(target_arch = "x86_64")]
    fn sharpen_simd(&self, image: &DynamicImage) -> Result<DynamicImage> {
        let gray = image.to_luma8();
        let width = gray.width() as usize;
        let height = gray.height() as usize;
        let pixels = gray.as_raw();

        let sharpened = unsafe { self.sharpen_simd_impl(pixels, width, height) };

        let output = GrayImage::from_raw(width as u32, height as u32, sharpened)
            .ok_or_else(|| anyhow::anyhow!("Failed to create image from sharpened data"))?;

        // 转换回原始颜色空间
        let result = if image.color().has_color() {
            let sharp_gray = DynamicImage::ImageLuma8(output);
            self.apply_luminance_only(image, &sharp_gray)?
        } else {
            DynamicImage::ImageLuma8(output)
        };

        Ok(result)
    }

    /// 标准锐化算法
    fn sharpen_standard(&self, image: &DynamicImage) -> Result<DynamicImage> {
        use imageproc::filter;

        let radius_factor = self.config.radius.max(1) as f32;
        let threshold = self.config.threshold.max(0.0);
        let strength = self.config.strength * radius_factor / (1.0 + threshold);

        let kernel = [
            -strength,
            -strength,
            -strength,
            -strength,
            8.0 * strength + 1.0,
            -strength,
            -strength,
            -strength,
            -strength,
        ];

        let rgba = image.to_rgba8();
        let sharpened = filter::filter3x3(&rgba, &kernel);

        Ok(DynamicImage::ImageRgba8(sharpened))
    }

    /// 仅应用亮度锐化（保持颜色）
    #[cfg(target_arch = "x86_64")]
    fn apply_luminance_only(
        &self,
        original: &DynamicImage,
        sharpened_gray: &DynamicImage,
    ) -> Result<DynamicImage> {
        let orig_rgba = original.to_rgba8();
        let sharp_gray = sharpened_gray.to_luma8();
        let orig_gray = original.to_luma8();

        let width = orig_rgba.width();
        let height = orig_rgba.height();
        let mut output = RgbaImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let orig_pixel = orig_rgba.get_pixel(x, y);
                let orig_luma = orig_gray.get_pixel(x, y)[0] as f32;
                let sharp_luma = sharp_gray.get_pixel(x, y)[0] as f32;

                // 计算亮度比例
                let ratio = if orig_luma > 0.0 {
                    sharp_luma / orig_luma
                } else {
                    1.0
                };

                // 应用到RGB通道
                let r = (orig_pixel[0] as f32 * ratio).min(255.0).max(0.0) as u8;
                let g = (orig_pixel[1] as f32 * ratio).min(255.0).max(0.0) as u8;
                let b = (orig_pixel[2] as f32 * ratio).min(255.0).max(0.0) as u8;
                let a = orig_pixel[3];

                output.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }

        Ok(DynamicImage::ImageRgba8(output))
    }

    /// 自适应锐化（根据图像内容调整）
    pub fn adaptive_sharpen(&self, image: &DynamicImage) -> Result<DynamicImage> {
        // 检测图像锐度
        let sharpness = self.measure_sharpness(image);
        debug!("Image sharpness score: {:.2}", sharpness);

        // 根据锐度调整配置
        let mut config = self.config.clone();
        if sharpness < 0.3 {
            // 图像模糊，增强锐化
            config.strength = (self.config.strength * 1.5).min(3.0);
        } else if sharpness > 0.7 {
            // 图像已经很锐利，减弱锐化
            config.strength = (self.config.strength * 0.5).max(0.5);
        }

        let sharpener = SimdSharpener::new(config);
        sharpener.sharpen(image)
    }

    /// 测量图像锐度
    fn measure_sharpness(&self, image: &DynamicImage) -> f32 {
        let gray = image.to_luma8();
        let width = gray.width();
        let height = gray.height();

        let mut total_gradient = 0.0;
        let mut count = 0u64;

        // 计算梯度
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let center = gray.get_pixel(x, y)[0] as f32;
                let right = gray.get_pixel(x + 1, y)[0] as f32;
                let bottom = gray.get_pixel(x, y + 1)[0] as f32;

                let dx = (center - right).abs();
                let dy = (center - bottom).abs();
                let gradient = (dx * dx + dy * dy).sqrt();

                total_gradient += gradient;
                count += 1;
            }
        }

        if count == 0 {
            return 0.0;
        }

        // 归一化到0-1
        let avg_gradient = total_gradient / count as f32;
        (avg_gradient / 255.0).min(1.0)
    }
}
