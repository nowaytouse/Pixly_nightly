use std::collections::HashMap;

use anyhow::Result;
use image::DynamicImage;

use crate::sharpen::{SharpenConfig, SimdSharpener};
use crate::types::{ImageFeatures, PredictionRequest, PredictionResult, PredictionWithConfidence, QualityMode};

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

        if features.size_mb() > 10.0 {
            quality_adjustment -= 3;
        } else if features.size_mb() < 0.5 {
            quality_adjustment += 2;
        }

        let (animation_penalty, alpha_bonus) = Self::format_adjustments_for("avif");
        if features.is_animated {
            quality_adjustment -= animation_penalty;
        }
        if features.has_alpha {
            quality_adjustment += alpha_bonus;
        }

        let final_quality = (base_quality + quality_adjustment).clamp(50, 100) as u32;

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

        let (animation_penalty, alpha_bonus) = Self::format_adjustments_for("jxl");
        if features.is_animated {
            quality_adjustment -= animation_penalty;
        }
        if features.has_alpha {
            quality_adjustment += alpha_bonus;
        }

        let final_quality = (base_quality + quality_adjustment).clamp(60, 100) as u32;

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

    /// 统一WebP预测算法（带质量下限保护）
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

        let mut quality_adjustment = ((complexity * 8.0) as i32) - 4;

        let (animation_penalty, alpha_bonus) = Self::format_adjustments_for("webp");
        if features.is_animated {
            quality_adjustment -= animation_penalty;
        }
        if features.has_alpha {
            quality_adjustment += alpha_bonus;
        }

        let mut final_quality = (base_quality + quality_adjustment).clamp(60, 100) as u32;
        
        // 质量下限保护
        final_quality = match mode {
            QualityMode::Quality => final_quality.max(85),  // 质量模式最低85
            QualityMode::Balanced => final_quality.max(75), // 平衡模式最低75
            QualityMode::Speed => final_quality.max(70),    // 速度模式最低70
            QualityMode::Lossless => 100,
        };

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

        let final_quality = (base_quality + quality_adjustment).clamp(65, 98) as u32;

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

    fn format_adjustments_for(target_format: &str) -> (i32, i32) {
        match target_format {
            "avif" => (15, 3),
            "jxl" | "jpegxl" => (10, 5),
            "webp" => (5, 2),
            _ => (8, 2),
        }
    }

    /// 归档启发式：根据输入/目标格式决定是否启用无损模式
    ///
    /// 规则来源（精炼自 ai_parameter_provider.rs::decide_lossless_heuristic）：
    /// - JPEG/JPG → JXL: 优先无损（利用 JXL 对 JPEG 的无损重新打包能力）
    /// - PNG 输入 → 只要目标格式支持无损（JXL/PNG/WebP/AVIF），则启用无损
    /// - 其他组合：不强制无损，由各格式内部策略决定
    fn decide_lossless_heuristic(input_format: &str, target_format: &str) -> bool {
        let input_lower = input_format.to_lowercase();
        let target_lower = target_format.to_lowercase();

        let target_supports_lossless = matches!(
            target_lower.as_str(),
            "jxl" | "jpegxl" | "png" | "webp" | "avif"
        );

        if (input_lower == "jpeg" || input_lower == "jpg")
            && (target_lower == "jxl" || target_lower == "jpegxl")
        {
            return true;
        }

        if input_lower == "png" && target_supports_lossless {
            return true;
        }

        false
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

        confidence.clamp(0.3, 0.95)
    }

    /// 完整预测接口
    pub fn predict(
        &self,
        features: &ImageFeatures,
        target_format: &str,
        quality_mode: QualityMode,
    ) -> PredictionResult {
        let (quality, speed, mut lossless, format_options) =
            self.predict_parameters(features, target_format, quality_mode);

        // 叠加归档中的无损启发式（只会将 lossless 从 false 覆盖为 true）
        if !lossless
            && Self::decide_lossless_heuristic(&features.format, target_format)
        {
            lossless = true;
        }

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

        PredictionWithConfidence { 
            core, 
            confidence,
            method: format!("unified-ai-v{}", self.algorithm_version)
        }
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
    fn test_lossless_heuristic_for_jpeg_to_jxl() {
        let predictor = UnifiedAIPredictor::new();
        let features = ImageFeatures {
            width: 1920,
            height: 1080,
            file_size: 3_000_000,
            format: "jpeg".to_string(),
            has_alpha: false,
            is_animated: false,
            complexity: 0.5,
        };

        let result = predictor.predict(&features, "jxl", QualityMode::Balanced);
        assert!(result.lossless);
    }

    #[test]
    fn test_lossless_heuristic_for_png_to_webp() {
        let predictor = UnifiedAIPredictor::new();
        let features = ImageFeatures {
            width: 800,
            height: 600,
            file_size: 500_000,
            format: "png".to_string(),
            has_alpha: true,
            is_animated: false,
            complexity: 0.4,
        };

        let result = predictor.predict(&features, "webp", QualityMode::Balanced);
        assert!(result.lossless);
    }

    #[test]
    fn test_lossless_heuristic_for_png_to_jpeg_is_disabled() {
        let predictor = UnifiedAIPredictor::new();
        let features = ImageFeatures {
            width: 800,
            height: 600,
            file_size: 500_000,
            format: "png".to_string(),
            has_alpha: true,
            is_animated: false,
            complexity: 0.4,
        };

        let result = predictor.predict(&features, "jpeg", QualityMode::Balanced);
        assert!(!result.lossless);
    }

    #[test]
    fn test_webp_prediction_matches_python_heuristic() {
        let predictor = UnifiedAIPredictor::new();
        let features = ImageFeatures {
            width: 1920,
            height: 1080,
            file_size: 2_500_000,
            format: "jpeg".to_string(),
            has_alpha: false,
            is_animated: false,
            complexity: 0.6,
        };

        let (quality, method, lossless, options) =
            predictor.predict_parameters(&features, "webp", QualityMode::Balanced);

        assert_eq!(quality, 80);
        assert_eq!(method, 4);
        assert!(!lossless);
        assert_eq!(options.get("method"), Some(&"4".to_string()));
        assert_eq!(options.get("lossless"), Some(&"false".to_string()));
    }

    #[test]
    fn test_jpeg_prediction_matches_python_heuristic() {
        let predictor = UnifiedAIPredictor::new();
        let features = ImageFeatures {
            width: 1920,
            height: 1080,
            file_size: 2_000_000,
            format: "jpeg".to_string(),
            has_alpha: false,
            is_animated: false,
            complexity: 0.5,
        };

        let (quality, _speed, lossless, options) =
            predictor.predict_parameters(&features, "jpeg", QualityMode::Balanced);

        assert_eq!(quality, 85);
        assert!(!lossless);
        assert_eq!(options.get("optimize"), Some(&"true".to_string()));
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
        let direct = predictor.predict_with_confidence(&features, "webp", QualityMode::Balanced);

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
    fn test_confidence_matches_python_logic_for_complex_animated_avif() {
        // 该场景对应 Python unified_ai_prediction_logic.py 中的逻辑：
        // base 0.8
        // + avif: -0.05 => 0.75
        // + large image: -0.05 => 0.70
        // + animated: -0.10 => 0.60
        // + high complexity (>0.8): -0.05 => 0.55
        let predictor = UnifiedAIPredictor::new();
        let features = ImageFeatures {
            width: 4000,
            height: 3000,
            file_size: 20_000_000,
            format: "jpeg".to_string(),
            has_alpha: false,
            is_animated: true,
            complexity: 0.9,
        };

        let extended = predictor.predict_with_confidence(&features, "avif", QualityMode::Quality);
        let expected = 0.55f64;
        assert!((extended.confidence - expected).abs() < 1e-6);
    }

    #[test]
    fn test_sharpen_image_preserves_size() {
        use image::{DynamicImage, Rgba, RgbaImage};

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
