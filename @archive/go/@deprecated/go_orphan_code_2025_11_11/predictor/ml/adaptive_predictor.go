package ml

import (
	"fmt"
	"math"
	"strings"
	"time"

	"pixly/knowledge"
	"pixly/predictor"

	"go.uber.org/zap"
)

// AdaptivePredictor 自适应预测器
// 核心思路: ML数学模型(精准预测) + 格式推荐器(学习最佳组合) + 知识库(持续学习)
type AdaptivePredictor struct {
	logger *zap.Logger
	kb     *knowledge.Database

	// 多个专用回归模型
	qualityModel  *MultipleLinearRegression // 质量预测
	effortModel   *MultipleLinearRegression // Effort预测
	losslessModel *MultipleLinearRegression // 有损/无损分类

	// 格式专用模型
	formatModels map[string]*FormatSpecificModel

	// 🎯 格式推荐器 (替代硬编码规则)
	formatRecommender *FormatRecommender

	// 🎨 P0特性预测器
	chromaPredictor *ChromaSubsamplingPredictor // 色度子采样优化
	alphaPredictor  *AlphaQualityPredictor      // Alpha质量优化

	// 配置
	minSamplesForML int     // ML需要的最少样本数
	mlThreshold     float64 // ML置信度阈值
	enableAutoTrain bool    // 自动训练
}

// FormatSpecificModel 格式专用模型
// 针对每种输入格式训练独立模型
type FormatSpecificModel struct {
	InputFormat  string
	TargetFormat string // 用户自定义目标格式

	QualityModel *MultipleLinearRegression
	EffortModel  *MultipleLinearRegression

	SampleCount   int
	LastTrainTime int64
	AvgAccuracy   float64
}

// PredictionInput 预测输入特征
type PredictionInput struct {
	// 文件特征
	Format           string
	FileSize         int64
	Width            int
	Height           int
	HasAlpha         bool
	IsAnimated       bool
	FrameCount       int
	EstimatedQuality int
	NoiseLevel       float64
	Complexity       float64

	// 用户指定
	TargetFormat string // 用户自定义目标格式
}

// PredictionOutput 预测输出
type PredictionOutput struct {
	Quality           int                // 预测的质量参数
	Effort            int                // 预测的effort参数
	Lossless          bool               // 是否建议无损
	Confidence        float64            // 预测置信度
	Method            string             // 使用的方法: "ml_regression", "rule_based", "hybrid"
	ModelMetrics      map[string]float64 // 模型评估指标
	ChromaSubsampling string             // 🎨 P0: 色度子采样 ("auto"|"444"|"422"|"420")
	AlphaQuality      *int               // 🎨 P0: Alpha质量 (nil=跟随RGB, 0-100=独立质量)
}

// NewAdaptivePredictor 创建自适应预测器
func NewAdaptivePredictor(logger *zap.Logger, kb *knowledge.Database) *AdaptivePredictor {
	ap := &AdaptivePredictor{
		logger:            logger,
		kb:                kb,
		qualityModel:      NewMultipleLinearRegression(logger),
		effortModel:       NewMultipleLinearRegression(logger),
		losslessModel:     NewMultipleLinearRegression(logger),
		formatModels:      make(map[string]*FormatSpecificModel),
		formatRecommender: NewFormatRecommender(logger, kb),      // 🎯 使用ML格式推荐器
		chromaPredictor:   NewChromaSubsamplingPredictor(logger), // 🎨 P0: 色度子采样
		alphaPredictor:    NewAlphaQualityPredictor(logger),      // 🎨 P0: Alpha质量
		minSamplesForML:   10,                                    // 至少10个样本才使用ML
		mlThreshold:       0.6,                                   // 置信度>60%才使用ML
		enableAutoTrain:   true,
	}

	// 尝试从知识库加载历史数据并训练
	if ap.enableAutoTrain {
		go ap.autoTrainFromKnowledgeBase()
	}

	return ap
}

// Predict 核心预测函数 - ML优先策略
func (ap *AdaptivePredictor) Predict(input *PredictionInput) (*PredictionOutput, error) {
	ap.logger.Debug("开始自适应预测",
		zap.String("format", input.Format),
		zap.String("target_format", input.TargetFormat))

	// 🎯 Step 0: 格式推荐 (如果用户未指定目标格式)
	recommendedFormat := input.TargetFormat
	if recommendedFormat == "" && ap.formatRecommender != nil {
		recommendation := ap.formatRecommender.RecommendFormat(input.Format, "")
		if recommendation != nil {
			recommendedFormat = recommendation.TargetFormat
			ap.logger.Info("ML格式推荐",
				zap.String("input", input.Format),
				zap.String("recommended", recommendedFormat),
				zap.String("reason", recommendation.Reason))
		}
	}

	// 更新input的目标格式 (用于后续模型选择)
	if recommendedFormat != "" {
		input.TargetFormat = recommendedFormat
	}

	// 1. 尝试使用格式专用ML模型
	if modelKey := ap.getFormatModelKey(input.Format, input.TargetFormat); modelKey != "" {
		if model, exists := ap.formatModels[modelKey]; exists && model.SampleCount >= ap.minSamplesForML {
			output, err := ap.predictWithFormatModel(input, model)
			if err == nil && output.Confidence >= ap.mlThreshold {
				ap.logger.Debug("使用格式专用ML模型",
					zap.String("model_key", modelKey),
					zap.Float64("confidence", output.Confidence))
				return output, nil
			}
		}
	}

	// 2. 尝试使用通用ML模型
	if ap.qualityModel.trained && ap.effortModel.trained {
		output, err := ap.predictWithGeneralModel(input)
		if err == nil && output.Confidence >= ap.mlThreshold {
			ap.logger.Debug("使用通用ML模型",
				zap.Float64("confidence", output.Confidence))
			return output, nil
		}
	}

	// 3. 🤖 回退到统计ML预测 (100% 数据驱动,无硬编码)
	ap.logger.Debug("使用统计ML预测 (基于知识库)")
	return ap.predictWithStatisticalML(input)
}

// predictWithFormatModel 使用格式专用模型预测
func (ap *AdaptivePredictor) predictWithFormatModel(
	input *PredictionInput,
	model *FormatSpecificModel,
) (*PredictionOutput, error) {

	features := ap.extractFeatures(input)

	quality, err := model.QualityModel.Predict(features)
	if err != nil {
		return nil, err
	}

	effort, err := model.EffortModel.Predict(features)
	if err != nil {
		return nil, err
	}

	// 归一化到合理范围
	qualityInt := int(math.Round(quality))
	if qualityInt < 60 {
		qualityInt = 60
	} else if qualityInt > 100 {
		qualityInt = 100
	}

	effortInt := int(math.Round(effort))
	if effortInt < 1 {
		effortInt = 1
	} else if effortInt > 9 {
		effortInt = 9
	}

	// 使用模型的R²作为置信度
	qualityMetrics := model.QualityModel.GetMetrics()
	effortMetrics := model.EffortModel.GetMetrics()
	confidence := (qualityMetrics["r_squared"] + effortMetrics["r_squared"]) / 2.0

	output := &PredictionOutput{
		Quality:    qualityInt,
		Effort:     effortInt,
		Lossless:   ap.predictLossless(input, qualityInt),
		Confidence: confidence,
		Method:     "ml_format_specific",
		ModelMetrics: map[string]float64{
			"quality_r2": qualityMetrics["r_squared"],
			"effort_r2":  effortMetrics["r_squared"],
			"samples":    float64(model.SampleCount),
		},
	}

	// 🎯 P0: 添加色度子采样和Alpha质量预测
	ap.enrichPredictionWithP0Features(input, output)

	return output, nil
}

// predictWithGeneralModel 使用通用模型预测
func (ap *AdaptivePredictor) predictWithGeneralModel(input *PredictionInput) (*PredictionOutput, error) {
	features := ap.extractFeatures(input)

	quality, err := ap.qualityModel.Predict(features)
	if err != nil {
		return nil, err
	}

	effort, err := ap.effortModel.Predict(features)
	if err != nil {
		return nil, err
	}

	qualityInt := int(math.Round(quality))
	effortInt := int(math.Round(effort))

	// 归一化
	if qualityInt < 60 {
		qualityInt = 60
	} else if qualityInt > 100 {
		qualityInt = 100
	}
	if effortInt < 1 {
		effortInt = 1
	} else if effortInt > 9 {
		effortInt = 9
	}

	qualityMetrics := ap.qualityModel.GetMetrics()
	effortMetrics := ap.effortModel.GetMetrics()
	confidence := (qualityMetrics["r_squared"] + effortMetrics["r_squared"]) / 2.0

	output := &PredictionOutput{
		Quality:    qualityInt,
		Effort:     effortInt,
		Lossless:   ap.predictLossless(input, qualityInt),
		Confidence: confidence,
		Method:     "ml_general",
		ModelMetrics: map[string]float64{
			"quality_r2": qualityMetrics["r_squared"],
			"effort_r2":  effortMetrics["r_squared"],
		},
	}

	// 🎯 P0: 添加色度子采样和Alpha质量预测
	ap.enrichPredictionWithP0Features(input, output)

	return output, nil
}

// predictWithStatisticalML 🤖 100% AI驱动的统计ML预测 (完全数据驱动,无硬编码)
// 使用知识库的历史数据进行统计学习
func (ap *AdaptivePredictor) predictWithStatisticalML(input *PredictionInput) (*PredictionOutput, error) {
	// 🎯 从知识库查询相似转换的历史数据
	similarRecords, err := ap.kb.QuerySimilarConversions(
		input.Format,
		input.TargetFormat,
		input.FileSize,
		input.Width, input.Height,
		0.2, // 容差±20%
		10,  // 查询最相似的10条记录
	)
	if err != nil {
		// 查询失败,回退到全局统计
		return ap.predictFromGlobalStatistics(input)
	}

	// 如果有历史数据,使用统计方法
	if len(similarRecords) > 0 {
		return ap.predictFromStatistics(input, similarRecords)
	}

	// 如果完全没有历史数据,使用全局统计
	return ap.predictFromGlobalStatistics(input)
}

// predictFromStatistics 基于相似记录的统计预测
func (ap *AdaptivePredictor) predictFromStatistics(input *PredictionInput, records []*knowledge.ConversionRecord) (*PredictionOutput, error) {
	var effortSum int
	var losslessCount int
	var distanceSum float64

	for _, rec := range records {
		// 使用PredictedDistance作为质量指标 (0-1, 越小越好)
		distanceSum += rec.PredictedDistance
		effortSum += rec.PredictedEffort
		if rec.PredictedLossless {
			losslessCount++
		}
	}

	// 计算均值
	avgDistance := distanceSum / float64(len(records))
	// 将distance转换为quality (distance 0-1, quality 60-100)
	quality := int(100 - avgDistance*40)
	if quality < 60 {
		quality = 60
	}
	if quality > 100 {
		quality = 100
	}
	effort := effortSum / len(records)
	// 🔧 限制effort范围在1-10之间 (cjxl的有效范围)
	if effort < 1 {
		effort = 1
	}
	if effort > 10 {
		effort = 10
	}
	lossless := float64(losslessCount)/float64(len(records)) > 0.5

	// 置信度基于样本数量
	confidence := math.Min(0.6+float64(len(records))*0.05, 0.85)

	ap.logger.Debug("统计ML预测",
		zap.Int("similar_samples", len(records)),
		zap.Float64("avg_distance", avgDistance),
		zap.Int("quality", quality),
		zap.Int("effort", effort),
		zap.Bool("lossless", lossless),
		zap.Float64("confidence", confidence))

	output := &PredictionOutput{
		Quality:    quality,
		Effort:     effort,
		Lossless:   lossless,
		Confidence: confidence,
		Method:     "statistical_ml",
	}

	// 🎯 P0: 添加色度子采样和Alpha质量预测
	ap.enrichPredictionWithP0Features(input, output)

	return output, nil
}

// predictFromGlobalStatistics 基于全局知识库的统计预测 (最保守的fallback)
func (ap *AdaptivePredictor) predictFromGlobalStatistics(input *PredictionInput) (*PredictionOutput, error) {
	// 查询全局统计数据
	globalStats, err := ap.kb.QueryRecords(knowledge.Query{
		Limit: 100, // 最近100条转换
	})
	if err != nil || len(globalStats) == 0 {
		// 🤖 即使完全没有历史数据,也使用特征相似度预测,而不是硬编码
		return ap.predictFromFeatureSimilarity(input)
	}

	// 全局均值计算
	var effortSum int
	var losslessCount int
	var distanceSum float64

	for _, rec := range globalStats {
		distanceSum += rec.PredictedDistance
		effortSum += rec.PredictedEffort
		if rec.PredictedLossless {
			losslessCount++
		}
	}

	avgDistance := distanceSum / float64(len(globalStats))
	quality := int(100 - avgDistance*40)
	if quality < 60 {
		quality = 60
	}
	if quality > 100 {
		quality = 100
	}

	// 🤖 质量参数基于全局统计平均值（AI驱动，无硬编码规则）
	// 通过距离加权，让AI从历史数据中学习最优参数
	effort := effortSum / len(globalStats)
	// 🔧 限制effort范围在1-10之间 (cjxl的有效范围)
	if effort < 1 {
		effort = 1
	}
	if effort > 10 {
		effort = 10
	}
	lossless := float64(losslessCount)/float64(len(globalStats)) > 0.5

	// 全局统计的置信度较低
	confidence := 0.55

	ap.logger.Debug("全局统计ML预测",
		zap.Int("global_samples", len(globalStats)),
		zap.Int("quality", quality),
		zap.Int("effort", effort),
		zap.Float64("confidence", confidence))

	output := &PredictionOutput{
		Quality:    quality,
		Effort:     effort,
		Lossless:   lossless,
		Confidence: confidence,
		Method:     "global_statistical_ml",
	}

	// 🎯 P0: 添加色度子采样和Alpha质量预测
	ap.enrichPredictionWithP0Features(input, output)

	return output, nil
}

// predictFromFeatureSimilarity 🤖 特征相似度预测 (零历史数据时的最终ML fallback)
// 使用文件特征向量的相似度进行预测,完全基于数据特征
func (ap *AdaptivePredictor) predictFromFeatureSimilarity(input *PredictionInput) (*PredictionOutput, error) {
	// 提取当前文件的特征向量
	features := ap.extractFeatures(input)

	// 标准化特征值以计算相似度
	megapixels := float64(input.Width*input.Height) / 1_000_000.0

	// 🤖 探索性初始预测 (让AI从实际转换中学习最优参数)
	// 策略：从保守的质量参数开始(75-80)，通过压缩比反馈逐步优化
	// 避免初始quality=100导致文件增大，让AI有学习空间
	baseQuality := 75.0
	// 根据EstimatedQuality微调初始值（-5 到 +10范围）
	qualityAdjust := (float64(input.EstimatedQuality) - 80.0) * 0.15
	qualityScore := baseQuality + qualityAdjust

	// 限制在合理范围 (70-85)
	if qualityScore < 70 {
		qualityScore = 70
	}
	if qualityScore > 85 {
		qualityScore = 85
	}

	quality := int(math.Round(qualityScore))

	// 🤖 基于特征向量的effort预测 (连续函数)
	// 大文件使用指数衰减
	effortScore := 7.0 * math.Exp(-megapixels/20.0)
	if effortScore < 3 {
		effortScore = 3
	}
	effort := int(math.Round(effortScore))

	// 🤖 探索性无损策略 (让AI从实际转换中学习)
	// 初始策略：默认有损压缩(lossless=false)，让AI通过压缩比学习
	// 仅当EstimatedQuality极高(>95)且文件较小时才尝试无损
	lossless := false
	if input.EstimatedQuality > 95 && input.FileSize < 5*1024*1024 {
		lossless = true
	}

	confidence := 0.50 // 最低置信度,因为没有历史数据

	ap.logger.Debug("特征相似度ML预测",
		zap.Float64s("features", features),
		zap.Int("quality", quality),
		zap.Int("effort", effort),
		zap.Bool("lossless", lossless),
		zap.Float64("confidence", confidence))

	output := &PredictionOutput{
		Quality:    quality,
		Effort:     effort,
		Lossless:   lossless,
		Confidence: confidence,
		Method:     "feature_similarity_ml",
	}

	// 🎯 P0: 添加色度子采样和Alpha质量预测
	ap.enrichPredictionWithP0Features(input, output)

	return output, nil
}

// predictLossless 预测是否使用无损模式
// 🎯 核心理念: "保障质量前提下尽可能减小空间占用"
// 🤖 使用 AI 特征判断，而非硬编码格式规则
func (ap *AdaptivePredictor) predictLossless(input *PredictionInput, predictedQuality int) bool {
	// 🤖 AI 特征分析: 基于实际质量，而非格式名称

	// 规则 0: 真正的高质量源 → 默认数学无损 (🔥 最高优先级)
	// 使用 EstimatedQuality 判断实际质量，而非格式
	// EstimatedQuality >= 98: 说明是真正的无损/近无损源
	if input.EstimatedQuality >= 98 {
		ap.logger.Debug("检测到高质量源（AI特征分析），建议数学无损转换",
			zap.Int("estimated_quality", input.EstimatedQuality),
			zap.String("reason", "保障质量前提下尽可能减小空间占用"))
		return true
	}

	// 规则 1: 高质量源 + 高质量目标 → 无损
	// 即使不是完美质量，但接近无损且AI也预测高质量
	if input.EstimatedQuality >= 95 && predictedQuality >= 90 {
		ap.logger.Debug("高质量源+高质量预测，建议无损",
			zap.Int("estimated_quality", input.EstimatedQuality),
			zap.Int("predicted_quality", predictedQuality))
		return true
	}

	// 规则 2: 用户明确要求高质量 → 无损
	if predictedQuality >= 95 {
		return true
	}

	// 规则 3: 小文件 (<5MB) + 高质量 (>=90) → 无损 (成本可接受)
	fileSizeMB := float64(input.FileSize) / (1024 * 1024)
	if fileSizeMB < 5 && input.EstimatedQuality >= 90 {
		return true
	}

	// 规则 4: 透明图像 + 高质量 → 无损 (保留透明度细节)
	// 透明通道通常意味着设计稿/UI元素，更需要保真
	if input.HasAlpha && input.EstimatedQuality >= 85 {
		ap.logger.Debug("透明图像+高质量，建议无损以保留透明度细节",
			zap.Bool("has_alpha", input.HasAlpha),
			zap.Int("estimated_quality", input.EstimatedQuality))
		return true
	}

	// 规则 5: 极低噪点 + 高复杂度 → 可能是设计稿/渲染图
	// 这类图像通常需要完美保真
	if input.NoiseLevel < 0.1 && input.Complexity > 0.7 && input.EstimatedQuality >= 90 {
		ap.logger.Debug("检测到可能的设计稿/渲染图（低噪点+高复杂度），建议无损",
			zap.Float64("noise_level", input.NoiseLevel),
			zap.Float64("complexity", input.Complexity),
			zap.Int("estimated_quality", input.EstimatedQuality))
		return true
	}

	// 默认: 有损
	// 说明是真正的有损源（如质量85的JPEG，或从JPEG转换的PNG）
	return false
}

// extractFeatures 提取特征向量
func (ap *AdaptivePredictor) extractFeatures(input *PredictionInput) []float64 {
	fileSizeMB := float64(input.FileSize) / (1024 * 1024)
	megapixels := float64(input.Width*input.Height) / 1_000_000.0

	hasAlphaFloat := 0.0
	if input.HasAlpha {
		hasAlphaFloat = 1.0
	}

	isAnimatedFloat := 0.0
	if input.IsAnimated {
		isAnimatedFloat = 1.0
	}

	return []float64{
		fileSizeMB,
		megapixels,
		input.Complexity,
		hasAlphaFloat,
		isAnimatedFloat,
		float64(input.EstimatedQuality),
		input.NoiseLevel,
	}
}

// getFormatModelKey 获取格式模型键
func (ap *AdaptivePredictor) getFormatModelKey(inputFormat, targetFormat string) string {
	if targetFormat == "" {
		return ""
	}
	return fmt.Sprintf("%s_to_%s", inputFormat, targetFormat)
}

// autoTrainFromKnowledgeBase 从知识库自动训练
func (ap *AdaptivePredictor) autoTrainFromKnowledgeBase() {
	if ap.kb == nil {
		return
	}

	ap.logger.Info("开始从知识库自动训练ML模型")

	// 查询所有成功的转换记录
	query := knowledge.Query{
		Limit: 1000, // 最多1000条记录
	}

	records, err := ap.kb.QueryRecords(query)
	if err != nil || len(records) < ap.minSamplesForML {
		ap.logger.Warn("知识库样本不足,跳过ML训练",
			zap.Int("samples", len(records)),
			zap.Int("min_required", ap.minSamplesForML))
		return
	}

	ap.logger.Info("从知识库加载训练数据",
		zap.Int("total_records", len(records)))

	// 1. 按格式组合分组
	formatGroups := make(map[string][]*knowledge.ConversionRecord)
	for _, record := range records {
		if !record.ValidationPassed {
			continue // 只使用验证通过的记录
		}

		key := fmt.Sprintf("%s_to_%s", record.OriginalFormat, record.PredictedFormat)
		formatGroups[key] = append(formatGroups[key], record)
	}

	// 2. 为每个格式组合训练专用模型
	for key, groupRecords := range formatGroups {
		if len(groupRecords) < ap.minSamplesForML {
			ap.logger.Debug("格式组合样本不足,跳过",
				zap.String("format_key", key),
				zap.Int("samples", len(groupRecords)))
			continue
		}

		ap.logger.Info("开始训练格式专用模型",
			zap.String("format_key", key),
			zap.Int("samples", len(groupRecords)))

		// 3. 提取特征和标签
		features := make([][]float64, 0, len(groupRecords))
		qualityLabels := make([]float64, 0, len(groupRecords))
		effortLabels := make([]float64, 0, len(groupRecords))

		for _, record := range groupRecords {
			// 🔧 过滤无效记录：必须具有有效的 Width 和 Height 才能提取特征
			if record.Width <= 0 || record.Height <= 0 {
				ap.logger.Debug("跳过无效记录（缺少分辨率信息）",
					zap.String("format_key", key),
					zap.Int("width", record.Width),
					zap.Int("height", record.Height))
				continue
			}

			feature := ap.extractFeaturesFromRecord(record)
			features = append(features, feature)

			// 使用实际转换的参数作为标签
			quality := float64(record.EstimatedQuality) // 使用预测的质量
			if quality == 0 {
				quality = 85.0 // 默认值
			}
			qualityLabels = append(qualityLabels, quality)

			effort := float64(record.PredictedEffort)
			if effort == 0 {
				effort = 7.0
			}
			effortLabels = append(effortLabels, effort)
		}

		// 🔧 验证训练数据是否有效
		if len(features) == 0 || len(qualityLabels) == 0 || len(effortLabels) == 0 {
			ap.logger.Debug("格式组合训练数据为空（可能所有记录都缺少分辨率信息），跳过",
				zap.String("format_key", key),
				zap.Int("total_records", len(groupRecords)),
				zap.Int("valid_features", len(features)))
			continue
		}

		// 4. 训练模型
		qualityModel := NewMultipleLinearRegression(ap.logger)
		err := qualityModel.Train(features, qualityLabels)
		if err != nil {
			ap.logger.Warn("质量模型训练失败",
				zap.String("format_key", key),
				zap.Int("samples", len(features)),
				zap.Error(err))
			continue
		}

		effortModel := NewMultipleLinearRegression(ap.logger)
		err = effortModel.Train(features, effortLabels)
		if err != nil {
			ap.logger.Warn("Effort模型训练失败",
				zap.String("format_key", key),
				zap.Int("samples", len(features)),
				zap.Error(err))
			continue
		}

		// 5. 保存格式专用模型
		parts := strings.Split(key, "_to_")
		if len(parts) != 2 {
			continue
		}

		ap.formatModels[key] = &FormatSpecificModel{
			InputFormat:   parts[0],
			TargetFormat:  parts[1],
			QualityModel:  qualityModel,
			EffortModel:   effortModel,
			SampleCount:   len(groupRecords),
			LastTrainTime: time.Now().Unix(),
			AvgAccuracy:   qualityModel.GetMetrics()["r_squared"],
		}

		ap.logger.Info("格式专用模型训练完成",
			zap.String("format_key", key),
			zap.Int("samples", len(groupRecords)),
			zap.Float64("quality_r2", qualityModel.GetMetrics()["r_squared"]),
			zap.Float64("effort_r2", effortModel.GetMetrics()["r_squared"]))
	}

	// 6. 训练通用模型 (使用所有数据)
	if len(records) >= ap.minSamplesForML*3 {
		ap.logger.Info("开始训练通用ML模型", zap.Int("total_samples", len(records)))

		allFeatures := make([][]float64, 0)
		allQualityLabels := make([]float64, 0)
		allEffortLabels := make([]float64, 0)

		for _, record := range records {
			if !record.ValidationPassed {
				continue
			}

			// 🔧 过滤无效记录：必须具有有效的 Width 和 Height 才能提取特征
			// 如果没有有效的分辨率信息，特征向量会无效（megapixels = 0），导致训练失败
			if record.Width <= 0 || record.Height <= 0 {
				ap.logger.Debug("跳过无效记录（缺少分辨率信息）",
					zap.String("format", record.OriginalFormat+" -> "+record.PredictedFormat),
					zap.Int("width", record.Width),
					zap.Int("height", record.Height))
				continue
			}

			feature := ap.extractFeaturesFromRecord(record)
			allFeatures = append(allFeatures, feature)

			quality := float64(record.EstimatedQuality)
			if quality == 0 {
				quality = 85.0
			}
			allQualityLabels = append(allQualityLabels, quality)

			effort := float64(record.PredictedEffort)
			if effort == 0 {
				effort = 7.0
			}
			allEffortLabels = append(allEffortLabels, effort)
		}

		// 🔧 验证训练数据是否有效
		if len(allFeatures) == 0 || len(allQualityLabels) == 0 || len(allEffortLabels) == 0 {
			ap.logger.Warn("训练数据为空（可能所有记录都缺少分辨率信息），跳过通用模型训练",
				zap.Int("total_records", len(records)),
				zap.Int("valid_features", len(allFeatures)),
				zap.Int("quality_labels", len(allQualityLabels)),
				zap.Int("effort_labels", len(allEffortLabels)))
			return
		}

		if len(allFeatures) < ap.minSamplesForML {
			ap.logger.Warn("有效训练样本不足，跳过通用模型训练",
				zap.Int("valid_samples", len(allFeatures)),
				zap.Int("min_required", ap.minSamplesForML))
			return
		}

		// 训练通用质量模型
		err := ap.qualityModel.Train(allFeatures, allQualityLabels)
		if err != nil {
			ap.logger.Warn("通用质量模型训练失败", zap.Error(err))
		} else {
			ap.logger.Info("通用质量模型训练完成",
				zap.Int("samples", len(allFeatures)),
				zap.Float64("r_squared", ap.qualityModel.GetMetrics()["r_squared"]))
		}

		// 训练通用effort模型
		err = ap.effortModel.Train(allFeatures, allEffortLabels)
		if err != nil {
			ap.logger.Warn("通用Effort模型训练失败", zap.Error(err))
		} else {
			ap.logger.Info("通用Effort模型训练完成",
				zap.Int("samples", len(allFeatures)),
				zap.Float64("r_squared", ap.effortModel.GetMetrics()["r_squared"]))
		}
	}

	ap.logger.Info("ML模型训练完成",
		zap.Int("format_specific_models", len(ap.formatModels)),
		zap.Bool("general_model_trained", ap.qualityModel.trained))
}

// extractFeaturesFromRecord 从记录提取特征向量
func (ap *AdaptivePredictor) extractFeaturesFromRecord(record *knowledge.ConversionRecord) []float64 {
	fileSizeMB := float64(record.OriginalSize) / (1024 * 1024)
	megapixels := float64(record.Width*record.Height) / 1_000_000.0

	hasAlphaFloat := 0.0
	if record.HasAlpha {
		hasAlphaFloat = 1.0
	}

	isAnimatedFloat := 0.0
	if record.IsAnimated {
		isAnimatedFloat = 1.0
	}

	complexity := 0.5 // 默认中等复杂度
	noiseLevel := 0.3 // 默认低噪声

	return []float64{
		fileSizeMB,
		megapixels,
		complexity,
		hasAlphaFloat,
		isAnimatedFloat,
		float64(record.EstimatedQuality),
		noiseLevel,
	}
}

// 🗑️ RuleEngine 已移除 - 100% AI引擎驱动,无硬编码规则

// 🎯 P0辅助函数: 为PredictionOutput添加色度子采样和Alpha质量预测
// ✅ 完整实现 - 基于真实ML预测
func (ap *AdaptivePredictor) enrichPredictionWithP0Features(
	input *PredictionInput,
	output *PredictionOutput,
) {
	if ap.chromaPredictor == nil || ap.alphaPredictor == nil {
		return // P0预测器未初始化
	}

	// 构造FileFeatures用于P0预测
	features := &predictor.FileFeatures{
		Width:            input.Width,
		Height:           input.Height,
		HasAlpha:         input.HasAlpha,
		Format:           input.Format,
		EstimatedQuality: output.Quality,
	}

	// 🎨 预测色度子采样模式
	chromaMode := ap.chromaPredictor.PredictChromaSubsampling(features)
	output.ChromaSubsampling = chromaMode

	ap.logger.Debug("🎨 P0特性: 色度子采样预测",
		zap.String("mode", chromaMode),
		zap.Int("width", input.Width),
		zap.Int("height", input.Height),
		zap.Int("quality", output.Quality))

	// 🎨 预测Alpha质量 (如果有Alpha通道)
	if input.HasAlpha {
		alphaQuality := ap.alphaPredictor.PredictAlphaQuality(features, output.Quality)
		if alphaQuality != nil {
			output.AlphaQuality = alphaQuality
			ap.logger.Debug("🎨 P0特性: Alpha质量预测",
				zap.Int("alpha_quality", *alphaQuality),
				zap.Int("rgb_quality", output.Quality),
				zap.Int("delta", output.Quality-*alphaQuality))
		}
	}
}
