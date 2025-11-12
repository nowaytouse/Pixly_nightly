package ml

import (
	"pixly/predictor"

	"go.uber.org/zap"
)

// ChromaSubsamplingPredictor 色度子采样预测器
// 🎯 灵感来自PIO的自动色度子采样
// 原理: 人眼对色度(Chroma)的敏感度低于亮度(Luma)
// 通过降低色度分辨率可以节省20-30%文件大小,且人眼几乎看不出差异
type ChromaSubsamplingPredictor struct {
	logger *zap.Logger
}

// NewChromaSubsamplingPredictor 创建色度子采样预测器
func NewChromaSubsamplingPredictor(logger *zap.Logger) *ChromaSubsamplingPredictor {
	return &ChromaSubsamplingPredictor{
		logger: logger,
	}
}

// PredictChromaSubsampling 预测最佳色度子采样模式
// 返回: "auto" | "444" | "422" | "420"
//
// 色度子采样说明:
// - 4:4:4 (无子采样): 完整色度信息,文件最大,质量最高
// - 4:2:2 (水平2:1):  水平方向色度减半,节省~17%
// - 4:2:0 (水平垂直2:1): 水平垂直色度都减半,节省~25%
//
// 决策规则:
// 1. 高色度复杂度图像 (如彩色图表、UI截图) → 4:4:4
// 2. 中等复杂度图像 (如人像、风景) → 4:2:2
// 3. 低色度复杂度图像 (如自然照片) → 4:2:0
func (cp *ChromaSubsamplingPredictor) PredictChromaSubsampling(
	features *predictor.FileFeatures,
) string {
	// 1. 如果是无损模式,强制使用4:4:4
	if features.EstimatedQuality >= 99 {
		cp.logger.Debug("无损模式,使用4:4:4色度子采样")
		return "444"
	}

	// 3. 基于图像特征预测色度复杂度
	chromaComplexity := cp.estimateChromaComplexity(features)

	// 4. 根据复杂度选择子采样模式
	if chromaComplexity > 0.8 {
		// 高色度细节 → 4:4:4 (无子采样)
		// 适用: UI截图、彩色图表、文字图像
		cp.logger.Debug("高色度复杂度,使用4:4:4",
			zap.Float64("complexity", chromaComplexity))
		return "444"
	} else if chromaComplexity > 0.5 {
		// 中等色度细节 → 4:2:2
		// 适用: 人像、建筑物、混合场景
		cp.logger.Debug("中等色度复杂度,使用4:2:2",
			zap.Float64("complexity", chromaComplexity))
		return "422"
	} else {
		// 低色度细节 → 4:2:0 (最aggressive)
		// 适用: 自然风景、天空、海洋、模糊背景
		cp.logger.Debug("低色度复杂度,使用4:2:0",
			zap.Float64("complexity", chromaComplexity))
		return "420"
	}
}

// estimateChromaComplexity 估算色度复杂度
// 返回: 0.0-1.0 (0=低复杂度, 1=高复杂度)
func (cp *ChromaSubsamplingPredictor) estimateChromaComplexity(
	features *predictor.FileFeatures,
) float64 {
	complexity := 0.0

	// 1. 基于图像复杂度 (高复杂度通常意味着高色度细节)
	complexity += features.Complexity * 0.4

	// 2. 基于噪声水平 (高噪声需要更多色度信息)
	complexity += features.NoiseLevel * 0.2

	// 3. 基于估计质量 (高质量源通常有更多色度细节)
	qualityFactor := float64(features.EstimatedQuality) / 100.0
	complexity += qualityFactor * 0.2

	// 4. 基于文件类型推断
	// 某些格式已知有特定色度特征
	if features.Format == "png" {
		// PNG通常用于UI/图表,可能有高色度复杂度
		complexity += 0.1
	} else if features.Format == "jpg" || features.Format == "jpeg" {
		// JPEG照片通常色度复杂度中等
		// (除非是高ISO噪点照片)
		if features.NoiseLevel < 0.3 {
			complexity -= 0.1 // 降低复杂度估计
		}
	}

	// 5. 基于分辨率 (超高分辨率可以用更激进的子采样)
	megapixels := float64(features.Width*features.Height) / 1_000_000.0
	if megapixels > 20 {
		// 超高分辨率 (>20MP) → 可以更激进
		complexity -= 0.15
	} else if megapixels < 2 {
		// 低分辨率 (<2MP) → 需要保守
		complexity += 0.1
	}

	// 6. 归一化到 [0, 1]
	if complexity < 0 {
		complexity = 0
	} else if complexity > 1 {
		complexity = 1
	}

	return complexity
}

// GetChromaSubsamplingRecommendation 获取色度子采样建议
// 返回人类可读的建议说明
func (cp *ChromaSubsamplingPredictor) GetChromaSubsamplingRecommendation(
	features *predictor.FileFeatures,
) string {
	mode := cp.PredictChromaSubsampling(features)

	recommendations := map[string]string{
		"444":  "使用4:4:4 (无子采样) - 保留完整色度信息,适合UI/图表",
		"422":  "使用4:2:2 (水平子采样) - 平衡质量与大小,适合人像/建筑",
		"420":  "使用4:2:0 (水平垂直子采样) - 最大压缩,适合自然风景",
		"auto": "使用自动模式 - 由编码器根据内容决定",
	}

	return recommendations[mode]
}
