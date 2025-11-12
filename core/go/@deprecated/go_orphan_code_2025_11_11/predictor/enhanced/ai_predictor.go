package enhanced

import (
	"pixly/ai"
	"pixly/predictor"

	"go.uber.org/zap"
)

// AIEnhancedPredictor AI增强预测器
// 在现有预测器基础上添加Python AI服务支持
// 三种模式：基础/高级/混合
type AIEnhancedPredictor struct {
	basePredictor predictor.Predictor
	aiClient      *ai.Client
	logger        *zap.Logger
	mode          AIMode
	enabled       bool
}

// AIMode AI预测模式
type AIMode string

const (
	ModeBasic    AIMode = "basic"    // 基础优化（规则+种子库）
	ModeAdvanced AIMode = "advanced" // 高级AI（SWT+LightGBM+PPO）
	ModeHybrid   AIMode = "hybrid"   // 混合模式（智能路由）
)

// NewAIEnhancedPredictor 创建AI增强预测器
func NewAIEnhancedPredictor(
	base predictor.Predictor,
	aiServerAddr string,
	logger *zap.Logger,
) (*AIEnhancedPredictor, error) {

	// 尝试连接AI服务
	aiClient, err := ai.NewClient(aiServerAddr)
	if err != nil {
		logger.Warn("AI服务连接失败，将使用基础预测器",
			zap.Error(err))

		return &AIEnhancedPredictor{
			basePredictor: base,
			logger:        logger,
			mode:          ModeBasic,
			enabled:       false,
		}, nil
	}

	logger.Info("✅ AI增强预测器初始化成功",
		zap.String("server", aiServerAddr))

	return &AIEnhancedPredictor{
		basePredictor: base,
		aiClient:      aiClient,
		logger:        logger,
		mode:          ModeHybrid, // 默认混合模式
		enabled:       true,
	}, nil
}

// SetMode 设置AI模式
func (p *AIEnhancedPredictor) SetMode(mode AIMode) {
	p.mode = mode
	if p.aiClient != nil {
		p.aiClient.SetMode(ai.PredictionMode(mode))
	}

	p.logger.Info("切换AI预测模式",
		zap.String("mode", string(mode)))
}

// PredictOptimalParams 预测最优参数
// 实现 predictor.Predictor 接口
func (p *AIEnhancedPredictor) PredictOptimalParams(filePath string) (*predictor.Prediction, error) {
	// 如果AI服务可用
	if p.enabled && p.aiClient != nil {
		return p.predictWithAI(filePath)
	}

	// 回退到基础预测器
	p.logger.Debug("使用基础预测器",
		zap.String("file", filePath))

	return p.basePredictor.PredictOptimalParams(filePath)
}

// predictWithAI 使用AI服务预测
func (p *AIEnhancedPredictor) predictWithAI(filePath string) (*predictor.Prediction, error) {
	// 1. 获取图像信息
	// 注意：这里应该使用现有的特征提取器
	features, err := p.basePredictor.GetFeatureExtractor().ExtractFeatures(filePath)
	if err != nil {
		p.logger.Warn("特征提取失败，回退到基础预测",
			zap.Error(err))
		return p.basePredictor.PredictOptimalParams(filePath)
	}

	// 2. 构建AI请求
	imageInfo := &ai.ImageInfo{
		FilePath:   filePath,
		Width:      features.Width,
		Height:     features.Height,
		Size:       features.FileSize,
		Format:     features.Format,
		IsAnimated: features.IsAnimated,
		HasAlpha:   features.HasAlpha,
	}

	// 3. 调用AI服务
	tool := p.selectTool(features.TargetFormat)
	result, err := p.aiClient.Predict(imageInfo, tool)
	if err != nil {
		p.logger.Warn("AI预测失败，回退到基础预测",
			zap.Error(err))
		return p.basePredictor.PredictOptimalParams(filePath)
	}

	// 4. 转换AI结果为predictor.Prediction
	prediction := p.convertAIResult(result, features)

	p.logger.Info("🎯 AI预测成功",
		zap.String("mode", string(p.mode)),
		zap.Int("quality", result.Params.CjxlQuality),
		zap.Int("effort", result.Params.CjxlEffort),
		zap.Float32("confidence", result.Confidence),
		zap.String("reason", result.Reason))

	return prediction, nil
}

// convertAIResult 将AI结果转换为predictor.Prediction
func (p *AIEnhancedPredictor) convertAIResult(
	aiResult *ai.PredictResult,
	features *predictor.FileFeatures,
) *predictor.Prediction {

	return &predictor.Prediction{
		Params: predictor.ConversionParams{
			TargetFormat: features.TargetFormat,
			Quality:      aiResult.Params.CjxlQuality,
			Effort:       aiResult.Params.CjxlEffort,
			Distance:     aiResult.Params.CjxlDistance,
			Lossless:     aiResult.Params.CjxlLossless,
			Modular:      aiResult.Params.CjxlModular,
			Progressive:  aiResult.Params.CjxlProgressive,
		},
		Confidence:     float64(aiResult.Confidence),
		Method:         "ai-" + aiResult.ModeUsed,
		ExpectedSaving: float64(aiResult.EstimatedCompression) / 100,
		Reason:         aiResult.Reason,
		ShouldExplore:  aiResult.Confidence < 0.8,
	}
}

// selectTool 根据目标格式选择工具
func (p *AIEnhancedPredictor) selectTool(targetFormat string) string {
	tools := map[string]string{
		"jxl":  "cjxl",
		"avif": "avifenc",
		"webp": "cwebp",
	}

	if tool, ok := tools[targetFormat]; ok {
		return tool
	}
	return "cjxl"
}

// Close 关闭AI客户端
func (p *AIEnhancedPredictor) Close() error {
	if p.aiClient != nil {
		return p.aiClient.Close()
	}
	return nil
}

// GetFeatureExtractor 获取特征提取器（实现接口）
func (p *AIEnhancedPredictor) GetFeatureExtractor() *predictor.FeatureExtractor {
	return p.basePredictor.GetFeatureExtractor()
}
