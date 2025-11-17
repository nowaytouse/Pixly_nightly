//! Pixly Unified AI Prediction Kernel (Root)
//! 
//! 纯本地化统一 AI 预测内核，实现自 @archive/core/fusion_v3/rust_engine
//! 的 `unified_ai_predictor.rs`，在项目根目录作为标准内核暴露。
//! 
//! 设计约束（来自 PROJECT_QUALITY_MANIFESTO）：
//! - 纯本地计算，无任何网络依赖
//! - 不提供“跳过 AI”的 fallback 通路
//! - 不做演示/模拟代码，所有逻辑为真实可用实现
//! - 内核只负责“参数预测 + 体积估算”，不做文件 IO / 编码执行

use anyhow::Result;
use image::DynamicImage;
#[cfg(any(test, target_arch = "x86_64"))]
use image::{Rgba, RgbaImage};
#[cfg(target_arch = "x86_64")]
use image::GrayImage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use tracing::{debug, info};

/// 质量模式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QualityMode {
    /// 速度优先
    Speed,
    /// 平衡模式
    Balanced,
    /// 质量优先
    Quality,
    /// 无损模式
    Lossless,
}

impl QualityMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            QualityMode::Speed => "speed",
            QualityMode::Balanced => "balanced",
            QualityMode::Quality => "quality",
            QualityMode::Lossless => "lossless",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "speed" => Some(QualityMode::Speed),
            "balanced" => Some(QualityMode::Balanced),
            "quality" => Some(QualityMode::Quality),
            "lossless" => Some(QualityMode::Lossless),
            _ => None,
        }
    }
}

/// 图像特征数据 (标准化结构)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFeatures {
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub format: String,
    pub has_alpha: bool,
    pub is_animated: bool,
    pub complexity: f64,
}

impl ImageFeatures {
    /// 像素总数
    pub fn pixels(&self) -> u64 {
        (self.width as u64) * (self.height as u64)
    }

    /// 文件大小(MB)
    pub fn size_mb(&self) -> f64 {
        self.file_size as f64 / (1024.0 * 1024.0)
    }

    /// 宽高比
    pub fn aspect_ratio(&self) -> f64 {
        if self.height > 0 {
            self.width as f64 / self.height as f64
        } else {
            1.0
        }
    }

    /// 是否为高分辨率图像 (>= 2K)
    pub fn is_high_resolution(&self) -> bool {
        self.width >= 2048 || self.height >= 2048
    }

    /// 是否为大图像 (>4MP)
    pub fn is_large_image(&self) -> bool {
        self.pixels() > 4_000_000
    }

    /// 是否为中等图像 (1MP-4MP)
    pub fn is_medium_image(&self) -> bool {
        let pixels = self.pixels();
        pixels >= 1_000_000 && pixels <= 4_000_000
    }

    /// 是否为小图像 (<1MP)
    pub fn is_small_image(&self) -> bool {
        self.pixels() < 1_000_000
    }

    /// 归一化后的压缩复杂度（从 Python ExtendedImageFeatures.compression_complexity 迁移）
    pub fn effective_complexity(&self) -> f64 {
        let mut base = self.complexity;

        if self.is_large_image() {
            base *= 1.3;
        }

        if self.is_high_resolution() {
            base *= 1.2;
        }

        if self.is_animated {
            base *= 1.5;
        }

        if self.has_alpha {
            base *= 1.1;
        }

        base.min(1.0)
    }
}

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
            info!("使用SIMD优化锐化");
            return self.sharpen_simd(image);
        }

        info!("使用标准锐化算法");
        self.sharpen_standard(image)
    }

    /// SIMD优化的锐化实现
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn sharpen_simd_impl(&self, pixels: &[u8], width: usize, height: usize) -> Vec<u8> {
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
        debug!("图像锐度评分: {:.2}", sharpness);

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

/// 预测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    pub quality: u32,
    pub speed: u32,
    pub lossless: bool,
    pub format_options: HashMap<String, String>,
    pub estimated_size: u64,
    pub estimated_ratio: f64,
    pub algorithm_version: String,
    pub predictor_version: String,
}

/// 预测请求（方便跨语言调用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionRequest {
    pub features: ImageFeatures,
    pub target_format: String,
    pub quality_mode: QualityMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionWithConfidence {
    pub core: PredictionResult,
    pub confidence: f64,
}

/// 统一AI预测器 - 标准化算法
pub struct UnifiedAIPredictor {
    version: String,
    algorithm_version: String,
}

impl UnifiedAIPredictor {
    /// 创建新的预测器实例
    pub fn new() -> Self {
        Self {
            version: "3.0.0-unified".to_string(),
            algorithm_version: "unified-v1".to_string(),
        }
    }

    /// 统一参数预测算法
    pub fn predict_parameters(
        &self,
        features: &ImageFeatures,
        target_format: &str,
        quality_mode: QualityMode,
    ) -> (u32, u32, bool, HashMap<String, String>) {
        let target_format = target_format.to_lowercase();

        match target_format.as_str() {
            "avif" => self.predict_avif(features, quality_mode),
            "jxl" | "jpegxl" => self.predict_jxl(features, quality_mode),
            "webp" => self.predict_webp(features, quality_mode),
            "png" => self.predict_png(features, quality_mode),
            "jpeg" | "jpg" => self.predict_jpeg(features, quality_mode),
            _ => self.predict_default(features, quality_mode),
        }
    }

    /// 统一AVIF预测算法
    fn predict_avif(
        &self,
        features: &ImageFeatures,
        mode: QualityMode,
    ) -> (u32, u32, bool, HashMap<String, String>) {
        let complexity = features.effective_complexity();
        let base_quality = match mode {
            QualityMode::Speed => 70,
            QualityMode::Balanced => 80,
            QualityMode::Quality => 85,
            QualityMode::Lossless => 100,
        };

        let mut quality_adjustment = 0i32;

        if features.is_large_image() {
            quality_adjustment -= 8;
        } else if features.is_small_image() {
            quality_adjustment += 5;
        }

        if complexity > 0.7 {
            quality_adjustment += 3;
        } else if complexity < 0.3 {
            quality_adjustment -= 2;
        }

        if features.has_alpha {
            quality_adjustment += 2;
        }

        if features.size_mb() > 10.0 {
            quality_adjustment -= 3;
        } else if features.size_mb() < 0.5 {
            quality_adjustment += 2;
        }

        let final_quality = ((base_quality as i32 + quality_adjustment).max(50).min(100)) as u32;

        let base_speed = match mode {
            QualityMode::Speed => 2,
            QualityMode::Balanced => 4,
            QualityMode::Quality => 6,
            QualityMode::Lossless => 8,
        };

        let speed = if features.size_mb() > 5.0 {
            (base_speed - 2).max(1)
        } else if features.size_mb() < 1.0 {
            (base_speed + 2).min(10)
        } else {
            base_speed
        };

        let lossless = mode == QualityMode::Lossless;

        let mut format_options = HashMap::new();
        format_options.insert("speed".to_string(), speed.to_string());
        format_options.insert(
            "tiles".to_string(),
            if features.is_large_image() {
                "4x4".to_string()
            } else {
                "2x2".to_string()
            },
        );

        (final_quality, speed, lossless, format_options)
    }

    /// 统一JXL预测算法
    fn predict_jxl(
        &self,
        features: &ImageFeatures,
        mode: QualityMode,
    ) -> (u32, u32, bool, HashMap<String, String>) {
        let base_quality = match mode {
            QualityMode::Speed => 75,
            QualityMode::Balanced => 85,
            QualityMode::Quality => 90,
            QualityMode::Lossless => 100,
        };

        let mut quality_adjustment = 0i32;

        if features.is_large_image() {
            quality_adjustment -= 5;
        } else if features.is_small_image() {
            quality_adjustment += 3;
        }

        if features.has_alpha {
            quality_adjustment += 5;
        }

        let final_quality = ((base_quality as i32 + quality_adjustment).max(60).min(100)) as u32;

        let effort = match mode {
            QualityMode::Speed => 3,
            QualityMode::Balanced => 6,
            QualityMode::Quality => 8,
            QualityMode::Lossless => 9,
        };

        let effort = if features.size_mb() > 10.0 {
            (effort - 2).max(1)
        } else {
            effort
        };

        let lossless = mode == QualityMode::Lossless;

        let mut format_options = HashMap::new();
        format_options.insert("effort".to_string(), effort.to_string());
        format_options.insert(
            "progressive".to_string(),
            if features.is_large_image() {
                "true".to_string()
            } else {
                "false".to_string()
            },
        );

        (final_quality, effort, lossless, format_options)
    }

    /// 统一WebP预测算法
    fn predict_webp(
        &self,
        features: &ImageFeatures,
        mode: QualityMode,
    ) -> (u32, u32, bool, HashMap<String, String>) {
        let complexity = features.effective_complexity();
        let base_quality = match mode {
            QualityMode::Speed => 75,
            QualityMode::Balanced => 80,
            QualityMode::Quality => 88,
            QualityMode::Lossless => 100,
        };

        let quality_adjustment = ((complexity * 8.0) as i32) - 4;
        let quality_adjustment = if features.has_alpha {
            quality_adjustment + 3
        } else {
            quality_adjustment
        };

        let final_quality = ((base_quality as i32 + quality_adjustment).max(60).min(100)) as u32;

        let method = match mode {
            QualityMode::Speed => 1,
            QualityMode::Balanced => 4,
            QualityMode::Quality => 6,
            QualityMode::Lossless => 6,
        };

        let lossless = mode == QualityMode::Lossless
            || (features.size_mb() < 2.0
                && features.pixels() < 1_000_000
                && complexity < 0.4);

        let mut format_options = HashMap::new();
        format_options.insert("method".to_string(), method.to_string());
        format_options.insert("lossless".to_string(), lossless.to_string());
        format_options.insert(
            "alpha_compression".to_string(),
            if features.has_alpha { "1" } else { "0" }.to_string(),
        );

        (final_quality, method, lossless, format_options)
    }

    /// 统一PNG预测算法
    fn predict_png(
        &self,
        features: &ImageFeatures,
        mode: QualityMode,
    ) -> (u32, u32, bool, HashMap<String, String>) {
        let quality = 100;
        let lossless = true;

        let mut compression = match mode {
            QualityMode::Speed => 3,
            QualityMode::Balanced => 6,
            QualityMode::Quality => 9,
            QualityMode::Lossless => 9,
        };

        if features.size_mb() > 20.0 {
            compression = 9;
        } else if features.size_mb() < 1.0 {
            compression = compression.min(6);
        }

        let mut format_options = HashMap::new();
        format_options.insert("compression".to_string(), compression.to_string());
        format_options.insert(
            "interlaced".to_string(),
            if features.is_large_image() {
                "true".to_string()
            } else {
                "false".to_string()
            },
        );

        (quality, compression, lossless, format_options)
    }

    /// 统一JPEG预测算法
    fn predict_jpeg(
        &self,
        features: &ImageFeatures,
        mode: QualityMode,
    ) -> (u32, u32, bool, HashMap<String, String>) {
        let base_quality = match mode {
            QualityMode::Speed => 78,
            QualityMode::Balanced => 85,
            QualityMode::Quality => 92,
            QualityMode::Lossless => 98,
        };

        let mut quality_adjustment = 0i32;

        if features.is_large_image() {
            quality_adjustment -= 4;
        } else if features.is_small_image() {
            quality_adjustment += 3;
        }

        let final_quality = ((base_quality as i32 + quality_adjustment).max(65).min(98)) as u32;

        let optimization = 4;

        let mut format_options = HashMap::new();
        format_options.insert("optimize".to_string(), "true".to_string());
        format_options.insert(
            "progressive".to_string(),
            if features.pixels() > 500_000 {
                "true".to_string()
            } else {
                "false".to_string()
            },
        );

        (final_quality, optimization, false, format_options)
    }

    /// 默认预测算法
    fn predict_default(
        &self,
        _features: &ImageFeatures,
        mode: QualityMode,
    ) -> (u32, u32, bool, HashMap<String, String>) {
        let quality = match mode {
            QualityMode::Speed => 70,
            QualityMode::Balanced => 80,
            QualityMode::Quality => 85,
            QualityMode::Lossless => 95,
        };

        (quality, 4, false, HashMap::new())
    }

    /// 统一文件大小估算
    pub fn estimate_output_size(
        &self,
        features: &ImageFeatures,
        target_format: &str,
        quality: u32,
    ) -> u64 {
        let complexity = features.effective_complexity();
        let (min_ratio, max_ratio) = match target_format.to_lowercase().as_str() {
            "avif" => (0.12, 0.45),
            "jxl" => (0.20, 0.60),
            "webp" => (0.25, 0.75),
            "jpeg" | "jpg" => (0.15, 0.80),
            "png" => (0.70, 0.90),
            _ => (0.50, 0.80),
        };

        let mut ratio = min_ratio + (max_ratio - min_ratio) * (quality as f64 / 100.0);

        if complexity > 0.7 {
            ratio *= 1.2;
        } else if complexity < 0.3 {
            ratio *= 0.8;
        }

        if features.has_alpha {
            ratio *= 1.15;
        }

        ((features.file_size as f64) * ratio) as u64
    }

    fn calculate_confidence(&self, features: &ImageFeatures, target_format: &str) -> f64 {
        let mut confidence = 0.8f64;
        let complexity = features.effective_complexity();

        match target_format.to_lowercase().as_str() {
            "jpeg" | "jpg" | "png" => {
                confidence += 0.1;
            }
            "webp" => {
                confidence += 0.05;
            }
            "avif" | "jxl" => {
                confidence -= 0.05;
            }
            _ => {}
        }

        if features.is_large_image() {
            confidence -= 0.05;
        }

        if features.is_animated {
            confidence -= 0.1;
        }

        if complexity > 0.8 {
            confidence -= 0.05;
        }

        if confidence < 0.3 {
            confidence = 0.3;
        }
        if confidence > 0.95 {
            confidence = 0.95;
        }

        confidence
    }

    /// 完整预测接口
    pub fn predict(
        &self,
        features: &ImageFeatures,
        target_format: &str,
        quality_mode: QualityMode,
    ) -> PredictionResult {
        let (quality, speed, lossless, format_options) =
            self.predict_parameters(features, target_format, quality_mode);

        let estimated_size = self.estimate_output_size(features, target_format, quality);
        let estimated_ratio = if features.file_size > 0 {
            estimated_size as f64 / features.file_size as f64
        } else {
            0.5
        };

        PredictionResult {
            quality,
            speed,
            lossless,
            format_options,
            estimated_size,
            estimated_ratio,
            algorithm_version: self.algorithm_version.clone(),
            predictor_version: self.version.clone(),
        }
    }

    pub fn predict_with_confidence(
        &self,
        features: &ImageFeatures,
        target_format: &str,
        quality_mode: QualityMode,
    ) -> PredictionWithConfidence {
        let core = self.predict(features, target_format, quality_mode);
        let confidence = self.calculate_confidence(features, target_format);

        PredictionWithConfidence { core, confidence }
    }

    /// 从标准请求结构执行预测（适合跨语言桥接）
    pub fn predict_from_request(&self, request: &PredictionRequest) -> PredictionWithConfidence {
        self.predict_with_confidence(
            &request.features,
            &request.target_format,
            request.quality_mode,
        )
    }

    /// 图像锐化（基于 SIMD / 标准算法）
    pub fn sharpen_image(
        &self,
        image: &DynamicImage,
        config: SharpenConfig,
    ) -> Result<DynamicImage> {
        let sharpener = SimdSharpener::new(config);
        sharpener.sharpen(image)
    }

    /// 自适应图像锐化
    pub fn adaptive_sharpen_image(
        &self,
        image: &DynamicImage,
        config: SharpenConfig,
    ) -> Result<DynamicImage> {
        let sharpener = SimdSharpener::new(config);
        sharpener.adaptive_sharpen(image)
    }
}

impl Default for UnifiedAIPredictor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avif_prediction() {
        let predictor = UnifiedAIPredictor::new();
        let features = ImageFeatures {
            width: 1920,
            height: 1080,
            file_size: 2_000_000,
            format: "jpeg".to_string(),
            has_alpha: false,
            is_animated: false,
            complexity: 0.6,
        };

        let result = predictor.predict(&features, "avif", QualityMode::Balanced);
        assert!(result.quality > 0 && result.quality <= 100);
        assert!(result.speed > 0);
    }

    #[test]
    fn test_quality_modes() {
        let predictor = UnifiedAIPredictor::new();
        let features = ImageFeatures {
            width: 1920,
            height: 1080,
            file_size: 2_000_000,
            format: "jpeg".to_string(),
            has_alpha: false,
            is_animated: false,
            complexity: 0.6,
        };

        for mode in &[
            QualityMode::Speed,
            QualityMode::Balanced,
            QualityMode::Quality,
            QualityMode::Lossless,
        ] {
            let result = predictor.predict(&features, "webp", *mode);
            assert!(result.quality > 0);
        }
    }

    #[test]
    fn test_image_features_helpers() {
        let features = ImageFeatures {
            width: 1920,
            height: 1080,
            file_size: 2_000_000,
            format: "jpeg".to_string(),
            has_alpha: false,
            is_animated: false,
            complexity: 0.6,
        };

        assert_eq!(features.pixels(), 1920 * 1080);
        assert!(features.size_mb() > 1.9 && features.size_mb() < 2.0);
        assert!(features.is_medium_image());
        assert!(!features.is_small_image());
        assert!(!features.is_large_image());
    }

    #[test]
    fn test_effective_complexity_scaling_and_clamp() {
        let base = ImageFeatures {
            width: 1000,
            height: 1000,
            file_size: 1_000_000,
            format: "png".to_string(),
            has_alpha: false,
            is_animated: false,
            complexity: 0.5,
        };

        // 基本情况下，effective_complexity 等于原始复杂度
        let base_eff = base.effective_complexity();
        assert!((base_eff - 0.5).abs() < 1e-6);

        // 大图 + 高分辨率 + 动画 + alpha 应显著放大复杂度，但不会超过 1.0
        let boosted = ImageFeatures {
            width: 4000,
            height: 3000,
            file_size: 20_000_000,
            format: "png".to_string(),
            has_alpha: true,
            is_animated: true,
            complexity: 0.8,
        };

        assert!(boosted.is_large_image());
        assert!(boosted.is_high_resolution());

        let boosted_eff = boosted.effective_complexity();
        assert!(boosted_eff >= 0.8);
        assert!(boosted_eff <= 1.0);
    }

    #[test]
    fn test_predict_from_request_matches_direct_call() {
        let predictor = UnifiedAIPredictor::new();
        let features = ImageFeatures {
            width: 1920,
            height: 1080,
            file_size: 4_000_000,
            format: "jpeg".to_string(),
            has_alpha: false,
            is_animated: false,
            complexity: 0.6,
        };

        let request = PredictionRequest {
            features: features.clone(),
            target_format: "webp".to_string(),
            quality_mode: QualityMode::Balanced,
        };

        let via_request = predictor.predict_from_request(&request);
        let direct = predictor.predict_with_confidence(
            &features,
            "webp",
            QualityMode::Balanced,
        );

        assert_eq!(via_request.core.quality, direct.core.quality);
        assert_eq!(via_request.core.speed, direct.core.speed);
        assert_eq!(via_request.core.lossless, direct.core.lossless);
        assert_eq!(via_request.core.estimated_size, direct.core.estimated_size);
        assert_eq!(via_request.core.estimated_ratio, direct.core.estimated_ratio);
        assert!((via_request.confidence - direct.confidence).abs() < 1e-6);
    }

    #[test]
    fn test_confidence_range() {
        let predictor = UnifiedAIPredictor::new();
        let features = ImageFeatures {
            width: 8000,
            height: 4000,
            file_size: 50_000_000,
            format: "jpeg".to_string(),
            has_alpha: false,
            is_animated: true,
            complexity: 0.9,
        };

        let extended = predictor.predict_with_confidence(&features, "avif", QualityMode::Quality);
        assert!(extended.confidence >= 0.3 && extended.confidence <= 0.95);
    }

    #[test]
    fn test_sharpen_image_preserves_size() {
        let predictor = UnifiedAIPredictor::new();
        let config = SharpenConfig::default();

        let mut img = RgbaImage::new(64, 64);
        for y in 0..64 {
            for x in 0..64 {
                let v = if (x + y) % 2 == 0 { 64 } else { 192 };
                img.put_pixel(x, y, Rgba([v, v, v, 255]));
            }
        }

        let dyn_img = DynamicImage::ImageRgba8(img);
        let sharpened = predictor
            .sharpen_image(&dyn_img, config)
            .expect("sharpen_image failed");

        assert_eq!(sharpened.width(), dyn_img.width());
        assert_eq!(sharpened.height(), dyn_img.height());
    }
}
