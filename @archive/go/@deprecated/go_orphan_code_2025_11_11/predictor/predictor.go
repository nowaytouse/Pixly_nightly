package predictor

import (
	"fmt"

	"go.uber.org/zap"
)

// PredictorImpl 预测器实现 - 纯特征提取器
// 注意: 格式专用预测器已全部废弃,由AdaptivePredictor替代
// 协调特征提取和参数预测
type PredictorImpl struct {
	logger           *zap.Logger
	featureExtractor *FeatureExtractor
	// 🤖 Note: AdaptivePredictor在smart/engine.go中直接初始化,避免导入循环
}

// NewPredictor 创建主预测器
func NewPredictor(logger *zap.Logger, ffprobePath string) *PredictorImpl {
	return &PredictorImpl{
		logger:           logger,
		featureExtractor: NewFeatureExtractor(logger, ffprobePath),
	}
}

// GetFeatureExtractor 获取特征提取器 (供AdaptivePredictor使用)
func (p *PredictorImpl) GetFeatureExtractor() *FeatureExtractor {
	return p.featureExtractor
}

// PredictOptimalParams 预测最优转换参数
// 这是主入口函数
func (p *PredictorImpl) PredictOptimalParams(filePath string) (*Prediction, error) {
	// 步骤1: 提取特征
	features, err := p.featureExtractor.ExtractFeatures(filePath)
	if err != nil {
		return nil, fmt.Errorf("特征提取失败: %w", err)
	}

	// 步骤2: 根据格式选择预测器
	prediction := p.selectAndPredict(features)

	// 步骤3: 日志记录
	p.logger.Info("预测完成",
		zap.String("file", filePath),
		zap.String("format", features.Format),
		zap.String("target", prediction.Params.TargetFormat),
		zap.Float64("confidence", prediction.Confidence),
		zap.String("method", prediction.Method),
		zap.Float64("expected_saving", prediction.ExpectedSaving*100),
		zap.Bool("should_explore", prediction.ShouldExplore))

	return prediction, nil
}

// selectAndPredict 选择合适的预测策略
// ⚠️ 格式专用预测器已废弃!改为完全依赖ML预测+通用回退
func (p *PredictorImpl) selectAndPredict(features *FileFeatures) *Prediction {
	// 🎯 不再使用硬编码的格式专用预测器
	// 原因:
	// 1. 过度依赖JXL/AVIF,不支持通用场景
	// 2. 无法学习,无法适应新格式
	// 3. 用户指定格式被忽略
	//
	// 新策略:
	// - ML预测器会在engine.go中被调用 (优先级最高)
	// - 这里只提供通用回退 (当ML失败时)

	p.logger.Debug("使用通用回退策略 (ML预测器应在上层调用)",
		zap.String("format", features.Format))

		return p.getDefaultPrediction(features)
}

// getDefaultPrediction 获取默认预测（fallback）
// 当ML预测失败时使用,提供保守的通用策略
func (p *PredictorImpl) getDefaultPrediction(features *FileFeatures) *Prediction {
	p.logger.Warn("使用通用回退策略 (ML预测应由上层AdaptivePredictor提供)",
		zap.String("format", features.Format))

	// 保守的默认参数 (完全基于特征,不依赖格式)
	quality := 85
	effort := 6
	lossless := false

	// 基于文件质量调整
	if features.EstimatedQuality >= 90 {
		quality = 90
		lossless = true
	}

	// 基于文件大小调整effort
	fileSizeMB := float64(features.FileSize) / (1024 * 1024)
	if fileSizeMB > 20 {
		effort = 5
	} else if fileSizeMB < 2 {
		effort = 7
	}

	params := &ConversionParams{
		TargetFormat: "", // 空,由用户明确指定或由上层smart engine决定
		Lossless:     lossless,
		Distance:     float64(100-quality) / 100.0,
		Effort:       effort,
		Quality:      quality,
		Threads:      8,
	}

	return &Prediction{
		Params:            params,
		Confidence:        0.50, // 中等置信度 (基于特征)
		Method:            "fallback_feature_based",
		RuleName:          "FEATURE_BASED_FALLBACK",
		ExpectedSaving:    0.15, // 保守估计15%
		ExpectedSizeBytes: int64(float64(features.FileSize) * 0.85),
		ShouldExplore:     false, // 不需要探索,参数已基于特征调整
	}
}

// GetFeatures 获取文件特征（辅助方法）
// 用于调试和测试
func (p *PredictorImpl) GetFeatures(filePath string) (*FileFeatures, error) {
	return p.featureExtractor.ExtractFeatures(filePath)
}
