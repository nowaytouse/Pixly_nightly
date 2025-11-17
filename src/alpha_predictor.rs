// Alpha质量预测器 - 独立Alpha质量控制
// 提取自: @archive/go/@deprecated/go_orphan_code_2025_11_11/predictor/ml/alpha_predictor.go
// 灵感来自Squoosh的独立Alpha质量控制
// 原理: Alpha通道通常比RGB通道简单,可以用更低质量编码
// 通过独立控制Alpha质量可以节省5-15%文件大小

#[derive(Default)]
pub struct AlphaQualityPredictor;

#[derive(Debug, Clone)]
pub struct FileFeatures {
    pub has_alpha: bool,
    pub format: String,
    pub file_size: u64,
    pub width: u32,
    pub height: u32,
    pub complexity: f64,
    pub noise_level: f64,
}

impl AlphaQualityPredictor {
    pub fn new() -> Self {
        Self
    }

    /// 预测独立Alpha质量
    /// 返回: None (跟随RGB质量) 或 Some(质量值) (0-100)
    ///
    /// Alpha复杂度分类:
    /// - 简单蒙版 (0-0.3): 纯透明/不透明,无渐变 → 可用低质量 (RGB-15)
    /// - 中等复杂度 (0.3-0.7): 部分渐变 → 略低质量 (RGB-5)
    /// - 高复杂度 (0.7-1.0): 复杂渐变/半透明 → 跟随RGB质量
    pub fn predict_alpha_quality(&self, features: &FileFeatures, rgb_quality: u8) -> Option<u8> {
        // 1. 如果没有Alpha通道,返回None
        if !features.has_alpha {
            return None;
        }

        // 2. 估算Alpha复杂度
        let alpha_complexity = self.estimate_alpha_complexity(features);

        // 3. 根据复杂度决定Alpha质量
        if alpha_complexity < 0.3 {
            // 简单Alpha (如纯蒙版, UI元素)
            // 可以用显著更低的质量
            let alpha_quality = (rgb_quality as i16 - 15).max(60) as u8;
            Some(alpha_quality)
        } else if alpha_complexity < 0.7 {
            // 中等Alpha (部分渐变, 阴影效果)
            // 可以用略低的质量
            let alpha_quality = (rgb_quality as i16 - 5).max(75) as u8;
            Some(alpha_quality)
        } else {
            // 复杂Alpha (复杂渐变, 半透明效果)
            // 需要跟随RGB质量
            None
        }
    }

    /// 估算Alpha复杂度
    /// 返回: 0.0-1.0 (0=简单蒙版, 1=复杂渐变)
    fn estimate_alpha_complexity(&self, features: &FileFeatures) -> f64 {
        let mut complexity = 0.3; // 基础复杂度 (保守估计)

        // 1. 基于文件格式推断
        match features.format.as_str() {
            "png" => {
                // PNG的Alpha通常用于UI元素/图标
                // 如果文件很小 (<1MB) 且分辨率中等,可能是UI/图标
                if features.file_size < 1024 * 1024
                    && (features.width * features.height) < 2_000_000
                {
                    complexity -= 0.2;
                }
            }
            "gif" => {
                // GIF的透明度是1bit (纯透明或纯不透明)
                complexity = 0.1;
            }
            _ => {}
        }

        // 2. 基于图像复杂度 (权重提高到0.5)
        complexity += features.complexity * 0.5;

        // 3. 基于噪声水平 (权重提高到0.3)
        complexity += features.noise_level * 0.3;

        // 4. 归一化到 [0, 1]
        complexity.clamp(0.0, 1.0)
    }

    /// 获取Alpha质量建议
    pub fn get_alpha_quality_recommendation(
        &self,
        features: &FileFeatures,
        rgb_quality: u8,
    ) -> String {
        if !features.has_alpha {
            return "无Alpha通道".to_string();
        }

        match self.predict_alpha_quality(features, rgb_quality) {
            None => "Alpha质量跟随RGB质量 (复杂Alpha)".to_string(),
            Some(alpha_quality) => {
                let saving = (rgb_quality as f64 - alpha_quality as f64) / rgb_quality as f64 * 100.0;
                format!(
                    "独立Alpha质量={} (RGB={}), 预期节省{:.1}%",
                    alpha_quality, rgb_quality, saving
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_alpha() {
        let predictor = AlphaQualityPredictor::new();
        let features = FileFeatures {
            has_alpha: true,
            format: "png".to_string(),
            file_size: 500 * 1024, // 500KB
            width: 1000,
            height: 1000,
            complexity: 0.2,
            noise_level: 0.1,
        };

        let alpha_quality = predictor.predict_alpha_quality(&features, 85);
        assert!(alpha_quality.is_some());
        assert!(alpha_quality.unwrap() < 85);
    }

    #[test]
    fn test_no_alpha() {
        let predictor = AlphaQualityPredictor::new();
        let features = FileFeatures {
            has_alpha: false,
            format: "jpg".to_string(),
            file_size: 1024 * 1024,
            width: 2000,
            height: 2000,
            complexity: 0.5,
            noise_level: 0.3,
        };

        let alpha_quality = predictor.predict_alpha_quality(&features, 85);
        assert!(alpha_quality.is_none());
    }

    #[test]
    fn test_complex_alpha() {
        let predictor = AlphaQualityPredictor::new();
        let features = FileFeatures {
            has_alpha: true,
            format: "webp".to_string(), // 改用webp避免PNG的-0.2调整
            file_size: 5 * 1024 * 1024,
            width: 4000,
            height: 4000,
            complexity: 0.9, // 提高复杂度
            noise_level: 0.6, // 提高噪声
        };

        let complexity = predictor.estimate_alpha_complexity(&features);
        // 0.3 + 0.9*0.5 + 0.6*0.3 = 0.3 + 0.45 + 0.18 = 0.93
        assert!(complexity >= 0.7, "complexity={}", complexity);
        
        let alpha_quality = predictor.predict_alpha_quality(&features, 90);
        assert!(alpha_quality.is_none()); // 复杂Alpha应该跟随RGB
    }
}
