package predictor

import (
	"math"
	"time"

	"pixly/knowledge"

	"go.uber.org/zap"
)

// TimeEstimator ML驱动的转换时间估算器
// 结合文件特征和历史数据预测转换时间
type TimeEstimator struct {
	logger        *zap.Logger
	knowledgeBase *knowledge.Database
}

// NewTimeEstimator 创建时间估算器
func NewTimeEstimator(logger *zap.Logger, kb *knowledge.Database) *TimeEstimator {
	return &TimeEstimator{
		logger:        logger,
		knowledgeBase: kb,
	}
}

// EstimateConversionTime 预测单个文件转换时间
// 结合ML特征分析和历史数据学习
func (te *TimeEstimator) EstimateConversionTime(
	features *FileFeatures,
	params *ConversionParams,
) time.Duration {
	// 1. 基于特征的理论估算
	baseTime := te.calculateBaseTime(features, params)

	// 2. 从知识库查询相似转换的历史时间
	historicalTime := te.queryHistoricalTime(features, params)

	// 3. 加权平均 (历史数据权重更高)
	var estimatedTime time.Duration
	if historicalTime > 0 {
		// 70%历史数据 + 30%理论估算
		estimatedTime = time.Duration(
			float64(historicalTime)*0.7 + float64(baseTime)*0.3,
		)
		te.logger.Debug("使用混合时间估算",
			zap.Duration("base", baseTime),
			zap.Duration("historical", historicalTime),
			zap.Duration("estimated", estimatedTime))
	} else {
		// 无历史数据时使用理论估算
		estimatedTime = baseTime
		te.logger.Debug("使用理论时间估算",
			zap.Duration("estimated", estimatedTime))
	}

	return estimatedTime
}

// calculateBaseTime 基于文件特征的理论时间计算
func (te *TimeEstimator) calculateBaseTime(
	features *FileFeatures,
	params *ConversionParams,
) time.Duration {
	// 1. 计算百万像素
	megapixels := float64(features.Width*features.Height) / 1_000_000.0
	if megapixels < 0.1 {
		megapixels = 0.1
	}

	// 2. 格式复杂度因子
	formatFactor := te.getFormatFactor(params.TargetFormat)

	// 3. Effort因子
	effortFactor := 1.0 + float64(params.Effort)*0.15

	// 4. 质量因子
	qualityFactor := 1.0
	if params.Quality > 85 {
		qualityFactor = 1.2
	} else if params.Quality > 95 {
		qualityFactor = 1.5
	}

	// 5. 无损模式因子
	losslessFactor := 1.0
	if params.Lossless {
		losslessFactor = 1.3
	}

	// 6. 降采样因子
	downscaleFactor := 1.0
	if params.ScaleRatio != nil && *params.ScaleRatio < 100 {
		scaleRatio := *params.ScaleRatio / 100.0
		downscaleFactor = scaleRatio * scaleRatio
	}

	// 7. 动画因子
	animationFactor := 1.0
	if features.IsAnimated {
		frameCount := float64(features.FrameCount)
		if frameCount < 1 {
			frameCount = 30
		}
		animationFactor = math.Sqrt(frameCount)
	}

	// 8. CPU并发因子
	cpuFactor := 8.0 / float64(params.Threads)

	// 综合计算
	estimatedSeconds := megapixels *
		formatFactor *
		effortFactor *
		qualityFactor *
		losslessFactor *
		downscaleFactor *
		animationFactor *
		cpuFactor

	baseOverhead := 0.5
	totalSeconds := estimatedSeconds + baseOverhead

	return time.Duration(totalSeconds * float64(time.Second))
}

// getFormatFactor 获取格式复杂度因子
func (te *TimeEstimator) getFormatFactor(format string) float64 {
	factors := map[string]float64{
		"jpg":  0.3,
		"jpeg": 0.3,
		"webp": 0.8,
		"png":  0.5,
		"jxl":  1.5,
		"avif": 2.5,
		"heif": 2.0,
		"gif":  0.6,
	}

	if factor, ok := factors[format]; ok {
		return factor
	}
	return 1.0
}

// queryHistoricalTime 从知识库查询相似转换的历史时间
func (te *TimeEstimator) queryHistoricalTime(
	features *FileFeatures,
	params *ConversionParams,
) time.Duration {
	if te.knowledgeBase == nil {
		return 0
	}

	// 查询相似转换记录 (±10%容差)
	records, err := te.knowledgeBase.QuerySimilarConversions(
		features.Format,
		params.TargetFormat,
		features.FileSize,
		features.Width,
		features.Height,
		0.1, // 10%容差
		10,  // 最多10条记录
	)

	if err != nil || len(records) == 0 {
		te.logger.Debug("未找到历史数据",
			zap.String("format", features.Format),
			zap.Error(err))
		return 0
	}

	// 计算平均处理时间
	var totalTime time.Duration
	validCount := 0

	for _, record := range records {
		if record.ActualConversionTimeMs > 0 {
			totalTime += time.Duration(record.ActualConversionTimeMs) * time.Millisecond
			validCount++
		}
	}

	if validCount == 0 {
		return 0
	}

	avgTime := totalTime / time.Duration(validCount)

	te.logger.Debug("历史时间查询成功",
		zap.Int("records_found", len(records)),
		zap.Int("valid_count", validCount),
		zap.Duration("avg_time", avgTime))

	return avgTime
}

// RecordActualTime 记录实际转换时间到知识库
func (te *TimeEstimator) RecordActualTime(
	features *FileFeatures,
	params *ConversionParams,
	actualTime time.Duration,
) error {
	if te.knowledgeBase == nil {
		return nil
	}

	record := &knowledge.ConversionRecord{
		FilePath:       features.FilePath,
		OriginalFormat: features.Format,
		Width:          features.Width,
		Height:         features.Height,
	}

	return te.knowledgeBase.SaveRecord(record)
}
