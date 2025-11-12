package ml

import (
	"fmt"
	"pixly/predictor"

	"go.uber.org/zap"
)

// AlphaQualityPredictor Alpha质量预测器
// 🎯 灵感来自Squoosh的独立Alpha质量控制
// 原理: Alpha通道通常比RGB通道简单,可以用更低质量编码
// 通过独立控制Alpha质量可以节省5-15%文件大小
type AlphaQualityPredictor struct {
	logger *zap.Logger
}

// NewAlphaQualityPredictor 创建Alpha质量预测器
func NewAlphaQualityPredictor(logger *zap.Logger) *AlphaQualityPredictor {
	return &AlphaQualityPredictor{
		logger: logger,
	}
}

// PredictAlphaQuality 预测独立Alpha质量
// 返回: nil (跟随RGB质量) 或 具体质量值 (0-100)
//
// Alpha复杂度分类:
// - 简单蒙版 (0-0.3): 纯透明/不透明,无渐变 → 可用低质量 (RGB-15)
// - 中等复杂度 (0.3-0.7): 部分渐变 → 略低质量 (RGB-5)
// - 高复杂度 (0.7-1.0): 复杂渐变/半透明 → 跟随RGB质量
func (ap *AlphaQualityPredictor) PredictAlphaQuality(
	features *predictor.FileFeatures,
	rgbQuality int,
) *int {
	// 1. 如果没有Alpha通道,返回nil
	if !features.HasAlpha {
		return nil
	}

	// 2. 估算Alpha复杂度
	alphaComplexity := ap.estimateAlphaComplexity(features)

	// 3. 根据复杂度决定Alpha质量
	if alphaComplexity < 0.3 {
		// 简单Alpha (如纯蒙版, UI元素)
		// 可以用显著更低的质量
		alphaQuality := rgbQuality - 15
		if alphaQuality < 60 {
			alphaQuality = 60 // 最低60
		}

		ap.logger.Debug("简单Alpha通道,使用独立低质量",
			zap.Float64("complexity", alphaComplexity),
			zap.Int("rgb_quality", rgbQuality),
			zap.Int("alpha_quality", alphaQuality),
			zap.String("reason", "pure_mask"))

		return &alphaQuality

	} else if alphaComplexity < 0.7 {
		// 中等Alpha (部分渐变, 阴影效果)
		// 可以用略低的质量
		alphaQuality := rgbQuality - 5
		if alphaQuality < 75 {
			alphaQuality = 75 // 最低75
		}

		ap.logger.Debug("中等Alpha通道,使用独立质量",
			zap.Float64("complexity", alphaComplexity),
			zap.Int("rgb_quality", rgbQuality),
			zap.Int("alpha_quality", alphaQuality),
			zap.String("reason", "partial_gradient"))

		return &alphaQuality

	} else {
		// 复杂Alpha (复杂渐变, 半透明效果)
		// 需要跟随RGB质量
		ap.logger.Debug("复杂Alpha通道,跟随RGB质量",
			zap.Float64("complexity", alphaComplexity),
			zap.Int("rgb_quality", rgbQuality),
			zap.String("reason", "complex_gradient"))

		return nil // 跟随RGB质量
	}
}

// estimateAlphaComplexity 估算Alpha复杂度
// 返回: 0.0-1.0 (0=简单蒙版, 1=复杂渐变)
//
// 由于我们目前没有实际分析Alpha通道的能力,
// 这里基于文件特征进行启发式估计
func (ap *AlphaQualityPredictor) estimateAlphaComplexity(
	features *predictor.FileFeatures,
) float64 {
	complexity := 0.3 // 基础复杂度 (保守估计)

	// 1. 基于文件格式推断
	// 不同格式的Alpha使用模式不同
	if features.Format == "png" {
		// PNG的Alpha通常用于:
		// - UI元素: 简单蒙版 (复杂度-0.2)
		// - 图标: 简单蒙版 (复杂度-0.2)
		// - 照片抠图: 复杂边缘 (复杂度+0.2)

		// 如果文件很小 (<1MB) 且分辨率中等,可能是UI/图标
		if features.FileSize < 1024*1024 &&
			features.Width*features.Height < 2_000_000 {
			complexity -= 0.2
		}

	} else if features.Format == "webp" {
		// WebP的Alpha通常用于照片抠图或UI
		// 默认中等复杂度

	} else if features.Format == "gif" {
		// GIF的透明度是1bit (纯透明或纯不透明)
		// 非常简单
		complexity = 0.1
	}

	// 2. 基于图像复杂度
	// 高复杂度的图像,Alpha也可能复杂
	complexity += features.Complexity * 0.3

	// 3. 基于噪声水平
	// 高噪声可能在Alpha边缘产生复杂性
	complexity += features.NoiseLevel * 0.2

	// 4. 归一化到 [0, 1]
	if complexity < 0 {
		complexity = 0
	} else if complexity > 1 {
		complexity = 1
	}

	return complexity
}

// GetAlphaQualityRecommendation 获取Alpha质量建议
// 返回人类可读的建议说明
func (ap *AlphaQualityPredictor) GetAlphaQualityRecommendation(
	features *predictor.FileFeatures,
	rgbQuality int,
) string {
	if !features.HasAlpha {
		return "无Alpha通道"
	}

	alphaQuality := ap.PredictAlphaQuality(features, rgbQuality)

	if alphaQuality == nil {
		return "Alpha质量跟随RGB质量 (复杂Alpha)"
	}

	saving := float64(rgbQuality-*alphaQuality) / float64(rgbQuality) * 100

	return fmt.Sprintf("独立Alpha质量=%d (RGB=%d), 预期节省%.1f%%",
		*alphaQuality, rgbQuality, saving)
}
