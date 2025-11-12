package ml

import (
	"fmt"
	"sort"
	"strings"

	"pixly/knowledge"

	"go.uber.org/zap"
)

// FormatRecommender 格式推荐器 (基于历史数据学习)
// 🎯 不硬编码格式,完全从知识库学习哪些格式组合效果最好
type FormatRecommender struct {
	logger *zap.Logger
	kb     *knowledge.Database

	// 格式组合效果缓存
	formatScores map[string]float64 // "jpg_to_jxl" -> score
}

// FormatRecommendation 格式推荐结果
type FormatRecommendation struct {
	TargetFormat   string  // 推荐的目标格式
	Confidence     float64 // 推荐置信度
	ExpectedSaving float64 // 预期节省比例
	SampleCount    int     // 历史样本数量
	Reason         string  // 推荐理由
}

// NewFormatRecommender 创建格式推荐器
func NewFormatRecommender(logger *zap.Logger, kb *knowledge.Database) *FormatRecommender {
	return &FormatRecommender{
		logger:       logger,
		kb:           kb,
		formatScores: make(map[string]float64),
	}
}

// RecommendFormat 推荐最佳目标格式
// 🎯 完全基于历史数据,不依赖硬编码规则
func (fr *FormatRecommender) RecommendFormat(inputFormat string, targetFormat string) *FormatRecommendation {
	// 🔥 ML智能推荐：即使用户指定了格式，也从历史数据中验证合理性
	if targetFormat != "" {
		// 从知识库查询此格式组合的历史效果
		recommendation := fr.evaluateFormatChoice(inputFormat, targetFormat)
		
		// 如果历史数据显示这是不好的选择，返回更好的推荐
		if recommendation != nil && recommendation.TargetFormat != targetFormat {
			fr.logger.Warn("🤖 ML检测到次优格式选择，推荐更佳方案",
				zap.String("input", inputFormat),
				zap.String("user_requested", targetFormat),
				zap.String("ml_recommendation", recommendation.TargetFormat),
				zap.Float64("confidence", recommendation.Confidence),
				zap.String("reason", recommendation.Reason))
			return recommendation // 返回基于历史数据的ML推荐
		}
		
		fr.logger.Debug("用户已指定目标格式，ML验证通过",
			zap.String("input", inputFormat),
			zap.String("target", targetFormat))
		return &FormatRecommendation{
			TargetFormat: targetFormat,
			Confidence:   1.0,
			Reason:       "用户明确指定",
		}
	}

	// 从知识库查询此格式的历史转换记录
	candidates := fr.getCandidateFormats(inputFormat)

	if len(candidates) == 0 {
		// 🎯 无特定格式的历史数据时,尝试从全局历史中推荐
		fr.logger.Debug("无此格式的历史数据,尝试全局推荐",
			zap.String("input_format", inputFormat))

		globalCandidates := fr.getGlobalBestFormats()
		if len(globalCandidates) > 0 {
			best := globalCandidates[0]
			fr.logger.Info("全局ML格式推荐",
				zap.String("input", inputFormat),
				zap.String("recommended", best.TargetFormat),
				zap.Float64("confidence", best.Confidence),
				zap.Int("global_samples", best.SampleCount))
			return best
		}

		// 🤖 完全没有历史数据时,基于输入格式特征进行ML推荐
		fr.logger.Info("知识库为空,使用基于特征的ML格式推荐",
			zap.String("input_format", inputFormat))
		return fr.recommendByFeatures(inputFormat)
	}

	// 返回得分最高的格式
	best := candidates[0]
	fr.logger.Info("ML格式推荐",
		zap.String("input", inputFormat),
		zap.String("recommended", best.TargetFormat),
		zap.Float64("confidence", best.Confidence),
		zap.Float64("expected_saving", best.ExpectedSaving),
		zap.Int("sample_count", best.SampleCount))

	return best
}

// getGlobalBestFormats 从全局历史数据中推荐最佳格式 (不限输入格式)
func (fr *FormatRecommender) getGlobalBestFormats() []*FormatRecommendation {
	if fr.kb == nil {
		return nil
	}

	// 查询所有成功转换记录
	query := knowledge.Query{
		Limit: 1000,
	}

	records, err := fr.kb.QueryRecords(query)
	if err != nil || len(records) == 0 {
		return nil
	}

	// 按目标格式分组统计
	formatStats := make(map[string]*formatStat)

	for _, record := range records {
		if !record.ValidationPassed {
			continue
		}

		targetFmt := record.PredictedFormat
		if targetFmt == "" {
			continue
		}

		if _, exists := formatStats[targetFmt]; !exists {
			formatStats[targetFmt] = &formatStat{
				targetFormat: targetFmt,
				samples:      make([]float64, 0),
			}
		}

		stat := formatStats[targetFmt]
		stat.count++
		savingRatio := 0.0
		if record.OriginalSize > 0 {
			savingRatio = float64(record.OriginalSize-record.ActualOutputSize) / float64(record.OriginalSize)
		}
		stat.totalSaving += savingRatio
		stat.samples = append(stat.samples, savingRatio)
		stat.totalTime += float64(record.ActualConversionTimeMs)
	}

	// 计算每个格式的综合得分
	candidates := make([]*FormatRecommendation, 0, len(formatStats))

	for _, stat := range formatStats {
		if stat.count < 5 {
			continue // 全局推荐需要更多样本
		}

		avgSaving := stat.totalSaving / float64(stat.count)
		confidence := calculateConfidence(stat.count, stat.samples)

		candidates = append(candidates, &FormatRecommendation{
			TargetFormat:   stat.targetFormat,
			Confidence:     confidence * 0.8, // 全局推荐置信度打8折
			ExpectedSaving: avgSaving,
			SampleCount:    stat.count,
			Reason: fmt.Sprintf("全局ML推荐: %d个样本, 平均节省%.1f%%",
				stat.count, avgSaving*100),
		})
	}

	// 按综合得分排序
	sort.Slice(candidates, func(i, j int) bool {
		return candidates[i].Confidence*candidates[i].ExpectedSaving >
			candidates[j].Confidence*candidates[j].ExpectedSaving
	})

	return candidates
}

// getCandidateFormats 获取候选格式列表 (基于历史数据)
func (fr *FormatRecommender) getCandidateFormats(inputFormat string) []*FormatRecommendation {
	if fr.kb == nil {
		return nil
	}

	// 查询此输入格式的所有成功转换记录
	query := knowledge.Query{
		SourceFormat: inputFormat,
		Limit:        1000,
	}

	records, err := fr.kb.QueryRecords(query)
	if err != nil || len(records) == 0 {
		return nil
	}

	// 按目标格式分组统计
	formatStats := make(map[string]*formatStat)

	for _, record := range records {
		if !record.ValidationPassed {
			continue // 只使用验证通过的记录
		}

		targetFmt := record.PredictedFormat
		if targetFmt == "" {
			continue
		}

		if _, exists := formatStats[targetFmt]; !exists {
			formatStats[targetFmt] = &formatStat{
				targetFormat: targetFmt,
				samples:      make([]float64, 0),
			}
		}

		stat := formatStats[targetFmt]
		stat.count++
		// 计算SavingRatio: (OriginalSize - ActualOutputSize) / OriginalSize
		savingRatio := 0.0
		if record.OriginalSize > 0 {
			savingRatio = float64(record.OriginalSize-record.ActualOutputSize) / float64(record.OriginalSize)
		}
		stat.totalSaving += savingRatio
		stat.samples = append(stat.samples, savingRatio)
		stat.totalTime += float64(record.ActualConversionTimeMs)
	}

	// 计算每个格式的综合得分
	candidates := make([]*FormatRecommendation, 0, len(formatStats))

	for _, stat := range formatStats {
		if stat.count < 3 {
			continue // 样本太少,不可信
		}

		avgSaving := stat.totalSaving / float64(stat.count)
		avgTime := stat.totalTime / float64(stat.count)

		// 综合得分 = 节省率 * 样本权重 / 时间惩罚
		sampleWeight := calculateSampleWeight(stat.count)
		timePenalty := calculateTimePenalty(avgTime)
		score := avgSaving * sampleWeight / timePenalty

		// 计算置信度
		confidence := calculateConfidence(stat.count, stat.samples)

		candidates = append(candidates, &FormatRecommendation{
			TargetFormat:   stat.targetFormat,
			Confidence:     confidence,
			ExpectedSaving: avgSaving,
			SampleCount:    stat.count,
			Reason: fmt.Sprintf("ML学习: %d个样本, 平均节省%.1f%%, 置信度%.0f%%",
				stat.count, avgSaving*100, confidence*100),
		})

		fr.logger.Debug("格式候选",
			zap.String("format", stat.targetFormat),
			zap.Int("samples", stat.count),
			zap.Float64("avg_saving", avgSaving),
			zap.Float64("score", score))
	}

	// 按综合得分排序
	sort.Slice(candidates, func(i, j int) bool {
		return candidates[i].Confidence*candidates[i].ExpectedSaving >
			candidates[j].Confidence*candidates[j].ExpectedSaving
	})

	return candidates
}

// formatStat 格式统计
type formatStat struct {
	targetFormat string
	count        int
	totalSaving  float64
	totalTime    float64
	samples      []float64 // 用于计算标准差
}

// calculateSampleWeight 计算样本权重 (样本越多越可信)
func calculateSampleWeight(count int) float64 {
	if count >= 100 {
		return 1.0
	}
	if count >= 50 {
		return 0.95
	}
	if count >= 20 {
		return 0.90
	}
	if count >= 10 {
		return 0.85
	}
	return 0.80
}

// calculateTimePenalty 计算时间惩罚 (越快越好)
func calculateTimePenalty(avgTimeMs float64) float64 {
	if avgTimeMs < 1000 {
		return 1.0 // <1秒,无惩罚
	}
	if avgTimeMs < 5000 {
		return 1.1 // <5秒,轻微惩罚
	}
	if avgTimeMs < 10000 {
		return 1.2 // <10秒,中等惩罚
	}
	return 1.3 // >10秒,较高惩罚
}

// calculateConfidence 计算置信度 (基于样本数量和方差)
func calculateConfidence(count int, samples []float64) float64 {
	if count < 3 {
		return 0.3
	}

	// 基础置信度 (基于样本数)
	baseConfidence := 0.5
	if count >= 100 {
		baseConfidence = 0.95
	} else if count >= 50 {
		baseConfidence = 0.90
	} else if count >= 20 {
		baseConfidence = 0.85
	} else if count >= 10 {
		baseConfidence = 0.75
	} else if count >= 5 {
		baseConfidence = 0.65
	}

	// 计算方差 (方差越小,越稳定,置信度越高)
	mean := 0.0
	for _, s := range samples {
		mean += s
	}
	mean /= float64(len(samples))

	variance := 0.0
	for _, s := range samples {
		variance += (s - mean) * (s - mean)
	}
	variance /= float64(len(samples))

	// 标准差
	stdDev := 0.0
	if variance > 0 {
		stdDev = 1.0 / (1.0 + variance*10) // 方差越大,置信度越低
	} else {
		stdDev = 1.0
	}

	return baseConfidence * stdDev
}
// recommendByFeatures 当完全没有历史数据时的回退
// �� 现在有预设种子数据,这个方法应该永远不会被调用
func (fr *FormatRecommender) recommendByFeatures(inputFormat string) *FormatRecommendation {
	// 如果真的到这里,说明种子数据导入失败或数据库有严重问题
	fr.logger.Error("🚨 FormatRecommender回退到零历史模式,这不应该发生!请检查种子数据是否正确导入",
		zap.String("input_format", inputFormat))
	
	// 返回nil,让调用者知道推荐失败
	return nil
}


// GetFormatKey 获取格式组合键
func GetFormatKey(inputFormat, targetFormat string) string {
	return fmt.Sprintf("%s_to_%s",
		strings.ToLower(inputFormat),
		strings.ToLower(targetFormat))
}

// evaluateFormatChoice 基于历史数据评估用户的格式选择
// 🤖 纯ML方法：从知识库学习，无硬编码规则
func (fr *FormatRecommender) evaluateFormatChoice(inputFormat, targetFormat string) *FormatRecommendation {
	if fr.kb == nil {
		// 无知识库，无法评估，接受用户选择
		return nil
	}
	
	input := strings.ToLower(inputFormat)
	target := strings.ToLower(targetFormat)
	
	// 🔥 统一格式命名：jpeg <-> jpg
	if target == "jpeg" {
		target = "jpg"
	}
	if input == "jpeg" {
		input = "jpg"
	}
	
	// 1. 查询用户指定的格式组合的历史效果
	userChoiceQuery := knowledge.Query{
		SourceFormat: input,
		Limit:        100,
	}
	
	allRecords, err := fr.kb.QueryRecords(userChoiceQuery)
	if err != nil || len(allRecords) == 0 {
		// 无历史数据，无法评估，接受用户选择
		return nil
	}
	
	// 2. 按目标格式分组统计
	formatStats := make(map[string]*formatStat)
	
	for _, record := range allRecords {
		if !record.ValidationPassed {
			continue
		}
		
		targetFmt := strings.ToLower(record.PredictedFormat)
		if targetFmt == "" {
			continue
		}
		
		// 🔥 统一格式命名：确保历史数据中的格式名也被统一
		if targetFmt == "jpeg" {
			targetFmt = "jpg"
		}
		
		if _, exists := formatStats[targetFmt]; !exists {
			formatStats[targetFmt] = &formatStat{
				targetFormat: targetFmt,
				samples:      make([]float64, 0),
			}
		}
		
		stat := formatStats[targetFmt]
		stat.count++
		
		// 计算压缩效果
		savingRatio := 0.0
		if record.OriginalSize > 0 {
			savingRatio = float64(record.OriginalSize-record.ActualOutputSize) / float64(record.OriginalSize)
		}
		stat.totalSaving += savingRatio
		stat.samples = append(stat.samples, savingRatio)
	}
	
	// 3. 检查是否有足够的历史数据来评估
	userChoiceStat, hasUserChoice := formatStats[target]
	if !hasUserChoice || userChoiceStat.count < 1 {
		// 用户选择的格式没有历史数据，无法评估
		// 🔥 阈值降低到1：因为种子数据是高质量的，1条也足够
		return nil
	}
	
	// 4. 查找历史数据中效果更好的格式
	userAvgSaving := userChoiceStat.totalSaving / float64(userChoiceStat.count)
	
	var bestAlternative *FormatRecommendation
	bestScore := userAvgSaving
	
	for format, stat := range formatStats {
		if format == target || stat.count < 1 {
			continue
		}

		avgSaving := stat.totalSaving / float64(stat.count)
		confidence := calculateConfidence(stat.count, stat.samples)

		// 只推荐明显更好的格式
		if avgSaving > userAvgSaving+0.05 && avgSaving > bestScore {
			bestScore = avgSaving
			bestAlternative = &FormatRecommendation{
				TargetFormat:   format,
				Confidence:     confidence,
				ExpectedSaving: avgSaving,
				SampleCount:    stat.count,
				Reason: fmt.Sprintf("🤖 ML推荐：基于%d个历史样本，%s格式平均压缩%.1f%%（比%s好%.1f%%）",
					stat.count, strings.ToUpper(format), avgSaving*100,
					strings.ToUpper(target), (avgSaving-userAvgSaving)*100),
			}
		}
	}
	
	// 5. 检查用户选择是否会导致质量下降或体积增大
	if userAvgSaving < -0.05 {
		// 用户选择会导致体积增大5%以上 → 这是不合理的转换
		fr.logger.Error("❌ ML检测到不合理的格式转换",
			zap.String("input", input),
			zap.String("user_choice", target),
			zap.Float64("avg_size_increase", -userAvgSaving*100),
			zap.Int("samples", userChoiceStat.count))
		
		// 返回错误推荐，让Rust层报错
		return &FormatRecommendation{
			TargetFormat:   "",  // 空字符串表示拒绝转换
			Confidence:     0.0,
			ExpectedSaving: userAvgSaving,
			SampleCount:    userChoiceStat.count,
			Reason: fmt.Sprintf("❌ 智能模式拒绝：根据%d个历史样本，%s→%s会导致体积增大%.1f%%，无法在保持质量前提下优化。建议保持原%s格式或切换到手动模式。",
				userChoiceStat.count, 
				strings.ToUpper(input), 
				strings.ToUpper(target),
				-userAvgSaving*100,
				strings.ToUpper(input)),
		}
	}
	
	// 6. 如果有明显更好的选择，返回推荐
	if bestAlternative != nil {
		fr.logger.Info("🤖 ML发现更优格式",
			zap.String("input", input),
			zap.String("user_choice", target),
			zap.Float64("user_avg_saving", userAvgSaving),
			zap.String("ml_choice", bestAlternative.TargetFormat),
			zap.Float64("ml_avg_saving", bestAlternative.ExpectedSaving),
			zap.Int("ml_samples", bestAlternative.SampleCount))
		return bestAlternative
	}
	
	// 7. 用户的选择是合理的
	return nil
}
