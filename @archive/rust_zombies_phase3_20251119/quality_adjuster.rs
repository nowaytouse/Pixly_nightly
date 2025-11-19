// 质量调整器 - 基于质量分析的自适应参数调整
// 提取自: @archive/go/@deprecated/go_orphan_code_2025_11_11/predictor/quality_adjuster.go

use crate::quality_reporter::QualityMetrics;

#[derive(Debug, Clone)]
pub struct PredictionParams {
    pub target_format: String,
    pub quality: u8,
    pub effort: u8,
    pub lossless: bool,
    pub lossless_jpeg: bool,
    pub crf: u8,
}

#[derive(Default)]
pub struct QualityAdjuster;

impl QualityAdjuster {
    pub fn new() -> Self {
        Self
    }

    /// 根据质量分析调整预测参数
    pub fn adjust_params(
        &self,
        mut params: PredictionParams,
        metrics: &QualityMetrics,
    ) -> PredictionParams {
        match metrics.format.as_str() {
            "png" => self.adjust_png_params(&mut params, metrics),
            "jpg" | "jpeg" => self.adjust_jpeg_params(&mut params, metrics),
            "gif" => self.adjust_gif_params(&mut params, metrics),
            "webp" => self.adjust_webp_params(&mut params, metrics),
            _ => {}
        }

        params
    }

    /// Adjust PNG parameters
    fn adjust_png_params(&self, params: &mut PredictionParams, metrics: &QualityMetrics) {
        // High quality PNG uses higher effort
        if (metrics.quality_class == "Extremely High" || metrics.quality_class == "High" || 
            metrics.quality_class == "极高" || metrics.quality_class == "高") && params.effort < 9 {
            params.effort = 9;
        }

        // Large files use lower effort for speed
        if (metrics.size_class == "Extremely Large" || metrics.size_class == "Large" ||
            metrics.size_class == "极大" || metrics.size_class == "大") && params.effort > 5 {
            params.effort = 5;
        }

        // Small files use maximum effort
        if metrics.size_class == "Small" || metrics.size_class == "Extremely Small" ||
           metrics.size_class == "小" || metrics.size_class == "极小" {
            params.effort = 9;
        }
    }

    /// 调整JPEG参数
    fn adjust_jpeg_params(&self, params: &mut PredictionParams, metrics: &QualityMetrics) {
        // JPEG 4:4:4采样有更大压缩潜力
        if metrics.pixel_format.contains("444") || metrics.pixel_format.contains("yuvj444p") {
            // 预期节省35%
        }

        // JPEG 4:2:0采样压缩潜力较小
        if metrics.pixel_format.contains("420") || metrics.pixel_format.contains("yuvj420p") {
            // 预期节省18%
        }

        // 照片类型使用无损JPEG转换
        if metrics.content_type == "photo" {
            params.lossless_jpeg = true;
        }
    }

    /// 调整GIF参数
    fn adjust_gif_params(&self, params: &mut PredictionParams, metrics: &QualityMetrics) {
        // Adjust CRF based on size
        if metrics.size_class == "Extremely Large" || metrics.size_class == "Large" ||
           metrics.size_class == "极大" || metrics.size_class == "大" {
            // Large GIF uses slightly higher CRF for speed
            if params.crf == 0 {
                params.crf = 32;
            }
        }
    }

    /// 调整WebP参数
    fn adjust_webp_params(&self, params: &mut PredictionParams, metrics: &QualityMetrics) {
        // WebP processing similar to PNG/JPEG
        if metrics.bytes_per_pixel > 2.0 {
            // High quality WebP
            params.effort = 8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_png_params() {
        let adjuster = QualityAdjuster::new();
        let params = PredictionParams {
            target_format: "jxl".to_string(),
            quality: 85,
            effort: 5,
            lossless: false,
            lossless_jpeg: false,
            crf: 0,
        };

        let metrics = QualityMetrics {
            file_path: "test.png".to_string(),
            file_size: 1024,
            format: "png".to_string(),
            media_type: "image".to_string(),
            width: 100,
            height: 100,
            pixel_count: 10000,
            has_alpha: true,
            pixel_format: "rgba8".to_string(),
            bit_depth: 8,
            color_space: "sRGB".to_string(),
            bytes_per_pixel: 0.1024,
            estimated_quality: 85,
            complexity_score: 0.5,
            noise_level: 0.1,
            content_type: "photo".to_string(),
            compression_potential: 0.7,
            is_already_compressed: false,
            compression_ratio: 1.0,
            quality_class: "Extremely High".to_string(),
            size_class: "Small".to_string(),
        };

        let adjusted = adjuster.adjust_params(params, &metrics);
        assert_eq!(adjusted.effort, 9);
    }

    #[test]
    fn test_adjust_jpeg_params() {
        let adjuster = QualityAdjuster::new();
        let params = PredictionParams {
            target_format: "jxl".to_string(),
            quality: 85,
            effort: 7,
            lossless: false,
            lossless_jpeg: false,
            crf: 0,
        };

        let metrics = QualityMetrics {
            file_path: "test.jpg".to_string(),
            file_size: 2048,
            format: "jpeg".to_string(),
            media_type: "image".to_string(),
            width: 200,
            height: 200,
            pixel_count: 40000,
            has_alpha: false,
            pixel_format: "yuvj420p".to_string(),
            bit_depth: 8,
            color_space: "sRGB".to_string(),
            bytes_per_pixel: 0.0512,
            estimated_quality: 80,
            complexity_score: 0.6,
            noise_level: 0.2,
            content_type: "photo".to_string(),
            compression_potential: 0.5,
            is_already_compressed: true,
            compression_ratio: 0.8,
            quality_class: "High".to_string(),
            size_class: "Small".to_string(),
        };

        let adjusted = adjuster.adjust_params(params, &metrics);
        assert!(adjusted.lossless_jpeg);
    }
}
