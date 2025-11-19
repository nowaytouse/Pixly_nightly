// 色度子采样预测器 - 自动色度子采样优化
// 提取自: @archive/go/@deprecated/go_orphan_code_2025_11_11/predictor/ml/chroma_predictor.go
// 灵感来自PIO的自动色度子采样
// 原理: 人眼对色度(Chroma)的敏感度低于亮度(Luma)
// 通过降低色度分辨率可以节省20-30%文件大小,且人眼几乎看不出差异

use crate::alpha_predictor::FileFeatures;

#[derive(Default)]
pub struct ChromaSubsamplingPredictor;

impl ChromaSubsamplingPredictor {
    pub fn new() -> Self {
        Self
    }

    /// 预测最佳色度子采样模式
    /// 返回: "auto" | "444" | "422" | "420"
    ///
    /// 色度子采样说明:
    /// - 4:4:4 (无子采样): 完整色度信息,文件最大,质量最高
    /// - 4:2:2 (水平2:1):  水平方向色度减半,节省~17%
    /// - 4:2:0 (水平垂直2:1): 水平垂直色度都减半,节省~25%
    ///
    /// 决策规则:
    /// 1. 高色度复杂度图像 (如彩色图表、UI截图) → 4:4:4
    /// 2. 中等复杂度图像 (如人像、风景) → 4:2:2
    /// 3. 低色度复杂度图像 (如自然照片) → 4:2:0
    pub fn predict_chroma_subsampling(&self, features: &FileFeatures, estimated_quality: u8) -> String {
        // 1. 如果是无损模式,强制使用4:4:4
        if estimated_quality >= 99 {
            return "444".to_string();
        }

        // 2. 基于图像特征预测色度复杂度
        let chroma_complexity = self.estimate_chroma_complexity(features, estimated_quality);

        // 3. 根据复杂度选择子采样模式
        if chroma_complexity > 0.8 {
            // 高色度细节 → 4:4:4 (无子采样)
            // 适用: UI截图、彩色图表、文字图像
            "444".to_string()
        } else if chroma_complexity > 0.5 {
            // 中等色度细节 → 4:2:2
            // 适用: 人像、建筑物、混合场景
            "422".to_string()
        } else {
            // 低色度细节 → 4:2:0 (最aggressive)
            // 适用: 自然风景、天空、海洋、模糊背景
            "420".to_string()
        }
    }

    /// 估算色度复杂度
    /// 返回: 0.0-1.0 (0=低复杂度, 1=高复杂度)
    fn estimate_chroma_complexity(&self, features: &FileFeatures, estimated_quality: u8) -> f64 {
        let mut complexity = 0.0;

        // 1. 基于图像复杂度 (高复杂度通常意味着高色度细节)
        complexity += features.complexity * 0.4;

        // 2. 基于噪声水平 (高噪声需要更多色度信息)
        complexity += features.noise_level * 0.2;

        // 3. 基于估计质量 (高质量源通常有更多色度细节)
        let quality_factor = estimated_quality as f64 / 100.0;
        complexity += quality_factor * 0.2;

        // 4. 基于实际文件特征推断（不信任格式名！）
        // 检查是否有透明通道 - 透明图像通常是UI/图表
        if features.has_alpha {
            // 透明图像通常用于UI/图表,可能有高色度复杂度
            complexity += 0.1;
        }
        
        // 基于实际噪声水平判断（不是格式！）
        if features.noise_level < 0.3 {
            // 低噪声图像（可能是照片或清晰图形）
            complexity -= 0.1;
        }

        // 5. 基于分辨率 (超高分辨率可以用更激进的子采样)
        let megapixels = (features.width * features.height) as f64 / 1_000_000.0;
        if megapixels > 20.0 {
            // 超高分辨率 (>20MP) → 可以更激进
            complexity -= 0.15;
        } else if megapixels < 2.0 {
            // 低分辨率 (<2MP) → 需要保守
            complexity += 0.1;
        }

        // 6. 归一化到 [0, 1]
        complexity.clamp(0.0, 1.0)
    }

    /// 获取色度子采样建议
    pub fn get_chroma_subsampling_recommendation(&self, features: &FileFeatures, estimated_quality: u8) -> String {
        let mode = self.predict_chroma_subsampling(features, estimated_quality);

        match mode.as_str() {
            "444" => "使用4:4:4 (无子采样) - 保留完整色度信息,适合UI/图表".to_string(),
            "422" => "使用4:2:2 (水平子采样) - 平衡质量与大小,适合人像/建筑".to_string(),
            "420" => "Using 4:2:0 (horizontal and vertical subsampling) - Maximum compression, suitable for natural landscapes".to_string(),
            "auto" => "Using auto mode - Encoder decides based on content".to_string(),
            _ => format!("Using {} mode", mode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_quality_444() {
        let predictor = ChromaSubsamplingPredictor::new();
        let features = FileFeatures {
            has_alpha: false,
            format: "png".to_string(),
            file_size: 1024 * 1024,
            width: 2000,
            height: 2000,
            complexity: 0.9,
            noise_level: 0.1,
        };

        let mode = predictor.predict_chroma_subsampling(&features, 99);
        assert_eq!(mode, "444");
    }

    #[test]
    fn test_photo_420() {
        let predictor = ChromaSubsamplingPredictor::new();
        let features = FileFeatures {
            has_alpha: false,
            format: "jpg".to_string(),
            file_size: 2 * 1024 * 1024,
            width: 4000,
            height: 3000,
            complexity: 0.3,
            noise_level: 0.2,
        };

        let mode = predictor.predict_chroma_subsampling(&features, 85);
        assert_eq!(mode, "420");
    }

    #[test]
    fn test_ui_screenshot() {
        let predictor = ChromaSubsamplingPredictor::new();
        let features = FileFeatures {
            has_alpha: true,
            format: "png".to_string(),
            file_size: 500 * 1024,
            width: 1920,
            height: 1080,
            complexity: 0.9, // 高复杂度
            noise_level: 0.3, // 中等噪声
        };

        let mode = predictor.predict_chroma_subsampling(&features, 95);
        // UI截图通常会得到422或444，取决于具体复杂度
        assert!(mode == "422" || mode == "444");
    }
}
