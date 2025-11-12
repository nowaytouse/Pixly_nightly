package ai

import (
	"fmt"
	"math"
)

// ============================================================================
// 双精度模式架构 v4.2.0
// ============================================================================

// PrecisionMode 精度模式
type PrecisionMode int

const (
	// PrecisionBasic 基础模式：贝叶斯优化（仅工具参数）
	PrecisionBasic PrecisionMode = iota
	// PrecisionAdvanced 进阶模式：多阶段智能执行（工具+可选项+顺序优化）
	PrecisionAdvanced
)

func (pm PrecisionMode) String() string {
	switch pm {
	case PrecisionBasic:
		return "basic"
	case PrecisionAdvanced:
		return "advanced"
	default:
		return "unknown"
	}
}

// ============================================================================
// 基础模式：贝叶斯参数优化
// ============================================================================

// BayesianOptimizer 贝叶斯优化器（基础模式）
// 特点：仅优化工具参数（quality, distance, effort等）
type BayesianOptimizer struct {
	tool              string        // 工具名称 (jxl/avif/webp)
	qualityRange      [2]int        // 质量范围 [min, max]
	observations      []Observation // 历史观测
	priorMean         float64       // 先验均值
	priorStd          float64       // 先验标准差
	explorationFactor float64       // 探索因子 (UCB)
}

// Observation 观测记录
type Observation struct {
	Quality    int     `json:"quality"`     // 质量参数
	Distance   float64 `json:"distance"`    // Distance参数
	Reward     float64 `json:"reward"`      // 奖励值（基于SSIM+压缩率）
	FileSize   int64   `json:"file_size"`   // 文件大小
	SSIM       float64 `json:"ssim"`        // SSIM值
	TargetMode string  `json:"target_mode"` // 目标模式 (size/balanced/quality)
}

// NewBayesianOptimizer 创建贝叶斯优化器
func NewBayesianOptimizer(tool string, qualityRange [2]int) *BayesianOptimizer {
	return &BayesianOptimizer{
		tool:              tool,
		qualityRange:      qualityRange,
		observations:      make([]Observation, 0),
		priorMean:         float64(qualityRange[0]+qualityRange[1]) / 2,
		priorStd:          float64(qualityRange[1]-qualityRange[0]) / 4,
		explorationFactor: 2.0, // UCB探索因子
	}
}

// AddObservation 添加观测
func (bo *BayesianOptimizer) AddObservation(obs Observation) {
	bo.observations = append(bo.observations, obs)
}

// PredictOptimal 预测最优参数（贝叶斯后验）
func (bo *BayesianOptimizer) PredictOptimal(targetMode string) (int, float64, error) {
	if len(bo.observations) == 0 {
		// 无历史数据，返回先验均值
		quality := int(bo.priorMean)
		distance := bo.qualityToDistance(quality)
		return quality, distance, nil
	}

	// 计算后验分布（高斯过程简化版）
	// 使用UCB（Upper Confidence Bound）选择探索/利用平衡点
	bestQuality := bo.qualityRange[0]
	bestScore := math.Inf(-1)

	// 遍历质量范围，找到UCB最大值
	for q := bo.qualityRange[0]; q <= bo.qualityRange[1]; q++ {
		// 计算均值（已观测点的加权平均）
		mean := bo.computePosteriorMean(q, targetMode)

		// 计算标准差（探索不确定性）
		std := bo.computePosteriorStd(q)

		// UCB = mean + exploration_factor * std
		ucb := mean + bo.explorationFactor*std

		if ucb > bestScore {
			bestScore = ucb
			bestQuality = q
		}
	}

	distance := bo.qualityToDistance(bestQuality)
	return bestQuality, distance, nil
}

// computePosteriorMean 计算后验均值（基于历史观测）
func (bo *BayesianOptimizer) computePosteriorMean(quality int, targetMode string) float64 {
	if len(bo.observations) == 0 {
		return bo.priorMean
	}

	// 加权平均：距离越近，权重越大
	totalWeight := 0.0
	weightedSum := 0.0

	for _, obs := range bo.observations {
		if obs.TargetMode != targetMode {
			continue // 只考虑同一目标模式的观测
		}

		// RBF核函数：exp(-d^2 / (2 * sigma^2))
		diff := float64(obs.Quality - quality)
		sigma := 10.0 // 带宽参数
		weight := math.Exp(-diff * diff / (2 * sigma * sigma))

		weightedSum += obs.Reward * weight
		totalWeight += weight
	}

	if totalWeight > 0 {
		return weightedSum / totalWeight
	}

	return bo.priorMean
}

// computePosteriorStd 计算后验标准差（探索不确定性）
func (bo *BayesianOptimizer) computePosteriorStd(quality int) float64 {
	if len(bo.observations) == 0 {
		return bo.priorStd
	}

	// 计算该点附近的观测密度
	// 观测越多，标准差越小（更确定）
	nearbyCount := 0.0
	sigma := 10.0

	for _, obs := range bo.observations {
		diff := float64(obs.Quality - quality)
		nearbyCount += math.Exp(-diff * diff / (2 * sigma * sigma))
	}

	// 标准差随观测增加而减小
	return bo.priorStd / math.Sqrt(1+nearbyCount)
}

// qualityToDistance 质量转distance（简化映射）
func (bo *BayesianOptimizer) qualityToDistance(quality int) float64 {
	if quality >= 95 {
		return 0.0
	} else if quality >= 85 {
		return 0.5
	} else {
		return 1.0
	}
}

// ComputeReward 计算奖励函数
func ComputeReward(ssim float64, compressionRatio float64, targetMode string) float64 {
	// 多目标奖励函数
	switch targetMode {
	case "size":
		// 体积优先：压缩率权重大
		return 0.3*ssim + 0.7*(1.0-compressionRatio)
	case "quality":
		// 质量优先：SSIM权重大
		return 0.8*ssim + 0.2*(1.0-compressionRatio)
	case "balanced":
		// 平衡模式
		return 0.5*ssim + 0.5*(1.0-compressionRatio)
	default:
		return 0.5*ssim + 0.5*(1.0-compressionRatio)
	}
}

// ============================================================================
// 进阶模式：多阶段智能执行
// ============================================================================

// AdvancedPipeline 进阶模式流水线
// 特点：工具参数 + 可选项 + 最佳顺序执行
type AdvancedPipeline struct {
	tool          string                 // 工具名称
	stages        []Stage                // 执行阶段
	imageFeatures *ImageFeaturesAdvanced // 图像特征（详细）
	bayesianOpt   *BayesianOptimizer     // 内嵌贝叶斯优化器
}

// Stage 执行阶段
type Stage struct {
	Name        string                            `json:"name"`
	Description string                            `json:"description"`
	Order       int                               `json:"order"`    // 执行顺序
	Options     map[string]interface{}            `json:"options"`  // 阶段选项
	Required    bool                              `json:"required"` // 是否必须
	Condition   func(*ImageFeaturesAdvanced) bool `json:"-"`        // 执行条件
}

// ImageFeaturesAdvanced 进阶图像特征
type ImageFeaturesAdvanced struct {
	// 基础特征
	Width      int  `json:"width"`
	Height     int  `json:"height"`
	HasAlpha   bool `json:"has_alpha"`
	IsAnimated bool `json:"is_animated"`
	FrameCount int  `json:"frame_count"`

	// SWT小波特征（9维）
	EdgeStrength      float64 `json:"edge_strength"`
	TextureComplexity float64 `json:"texture_complexity"`
	NoiseLevel        float64 `json:"noise_level"`
	DetailLevel       float64 `json:"detail_level"`
	HighFreqEnergy    float64 `json:"high_freq_energy"`
	MidFreqEnergy     float64 `json:"mid_freq_energy"`
	LowFreqEnergy     float64 `json:"low_freq_energy"`
	OverallQuality    float64 `json:"overall_quality"`
	CompressionScore  float64 `json:"compression_score"`

	// 颜色特征
	ColorSpace string  `json:"color_space"`
	ColorRange float64 `json:"color_range"` // 颜色范围
	Saturation float64 `json:"saturation"`  // 饱和度
	Brightness float64 `json:"brightness"`  // 亮度
	Contrast   float64 `json:"contrast"`    // 对比度

	// 内容特征
	IsPhoto      bool `json:"is_photo"`      // 是否照片
	IsDocument   bool `json:"is_document"`   // 是否文档
	IsScreenshot bool `json:"is_screenshot"` // 是否截图
	HasText      bool `json:"has_text"`      // 是否包含文字
}

// NewAdvancedPipeline 创建进阶流水线
func NewAdvancedPipeline(tool string, features *ImageFeaturesAdvanced) *AdvancedPipeline {
	qualityRange := [2]int{70, 100}

	return &AdvancedPipeline{
		tool:          tool,
		stages:        buildStages(tool, features),
		imageFeatures: features,
		bayesianOpt:   NewBayesianOptimizer(tool, qualityRange),
	}
}

// Execute 执行流水线（最佳顺序）
func (ap *AdvancedPipeline) Execute(targetMode string) (*AdvancedResult, error) {
	result := &AdvancedResult{
		Tool:   ap.tool,
		Mode:   targetMode,
		Stages: make([]StageResult, 0),
	}

	// 阶段1: 贝叶斯参数预测
	quality, distance, err := ap.bayesianOpt.PredictOptimal(targetMode)
	if err != nil {
		return nil, err
	}
	result.BaseQuality = quality
	result.BaseDistance = distance

	// 阶段2-N: 执行各个阶段（按顺序）
	for _, stage := range ap.stages {
		// 检查是否应该执行此阶段
		if stage.Condition != nil && !stage.Condition(ap.imageFeatures) {
			continue
		}

		stageResult := StageResult{
			Name:     stage.Name,
			Order:    stage.Order,
			Executed: true,
			Options:  stage.Options,
		}

		result.Stages = append(result.Stages, stageResult)
	}

	// 阶段最终: 参数微调
	result.FinalQuality = ap.fineTuneQuality(result.BaseQuality, targetMode)
	result.FinalDistance = ap.fineTuneDistance(result.BaseDistance, targetMode)

	return result, nil
}

// fineTuneQuality 根据图像特征微调质量
func (ap *AdvancedPipeline) fineTuneQuality(baseQuality int, targetMode string) int {
	adjustment := 0

	// 高纹理复杂度 → 质量+5
	if ap.imageFeatures.TextureComplexity > 80 {
		adjustment += 5
	}

	// 高噪声 → 质量-3（噪声图像不需要太高质量）
	if ap.imageFeatures.NoiseLevel > 30 {
		adjustment -= 3
	}

	// 文档/截图 → 质量+3（需要保持锐度）
	if ap.imageFeatures.IsDocument || ap.imageFeatures.IsScreenshot {
		adjustment += 3
	}

	finalQuality := baseQuality + adjustment

	// 限制在合理范围
	if finalQuality < 70 {
		finalQuality = 70
	}
	if finalQuality > 100 {
		finalQuality = 100
	}

	return finalQuality
}

// fineTuneDistance 根据图像特征微调distance
func (ap *AdvancedPipeline) fineTuneDistance(baseDistance float64, targetMode string) float64 {
	adjustment := 0.0

	// 高可压缩性 → distance+0.2
	if ap.imageFeatures.CompressionScore > 0.8 {
		adjustment += 0.2
	}

	// 低噪声+高边缘 → distance-0.1（可以更精确）
	if ap.imageFeatures.NoiseLevel < 10 && ap.imageFeatures.EdgeStrength > 60 {
		adjustment -= 0.1
	}

	finalDistance := baseDistance + adjustment

	// 限制在合理范围
	if finalDistance < 0 {
		finalDistance = 0
	}
	if finalDistance > 2.0 {
		finalDistance = 2.0
	}

	return finalDistance
}

// buildStages 构建执行阶段
func buildStages(tool string, features *ImageFeaturesAdvanced) []Stage {
	stages := make([]Stage, 0)

	// 阶段1: 色彩空间优化
	stages = append(stages, Stage{
		Name:        "色彩空间优化",
		Description: "根据图像特性选择最优色彩空间",
		Order:       1,
		Required:    false,
		Options: map[string]interface{}{
			"color_space": "auto",
		},
		Condition: func(f *ImageFeaturesAdvanced) bool {
			return f.IsPhoto // 仅照片需要
		},
	})

	// 阶段2: 预处理增强
	stages = append(stages, Stage{
		Name:        "预处理增强",
		Description: "降噪、锐化等预处理",
		Order:       2,
		Required:    false,
		Options: map[string]interface{}{
			"denoise":  features.NoiseLevel > 20,
			"sharpen":  features.EdgeStrength < 40,
			"contrast": features.Contrast < 0.5,
		},
		Condition: func(f *ImageFeaturesAdvanced) bool {
			return f.NoiseLevel > 20 || f.EdgeStrength < 40
		},
	})

	// 阶段3: 工具特定优化
	if tool == "jxl" {
		stages = append(stages, Stage{
			Name:        "JXL高级选项",
			Description: "Modular、Progressive、Gaborish等",
			Order:       3,
			Required:    true,
			Options: map[string]interface{}{
				"modular":     features.IsDocument || features.HasText,
				"progressive": features.Width*features.Height > 2000000,
				"responsive":  true,
				"gaborish":    features.EdgeStrength > 60,
			},
		})
	} else if tool == "avif" {
		stages = append(stages, Stage{
			Name:        "AVIF高级选项",
			Description: "Tiles、Speed、Chroma等",
			Order:       3,
			Required:    true,
			Options: map[string]interface{}{
				"tiles":  features.Width*features.Height > 4000000,
				"speed":  6,
				"chroma": "444",
			},
		})
	} else if tool == "webp" {
		stages = append(stages, Stage{
			Name:        "WebP高级选项",
			Description: "Method、Filter、Preprocessing等",
			Order:       3,
			Required:    true,
			Options: map[string]interface{}{
				"method":        6,
				"auto_filter":   true,
				"preprocessing": 4,
			},
		})
	}

	// 阶段4: SSIM验证
	stages = append(stages, Stage{
		Name:        "SSIM质量验证",
		Description: "转换后验证SSIM，低于阈值重试",
		Order:       4,
		Required:    true,
		Options: map[string]interface{}{
			"enabled":  true,
			"min_ssim": 0.94,
		},
	})

	return stages
}

// AdvancedResult 进阶模式结果
type AdvancedResult struct {
	Tool          string        `json:"tool"`
	Mode          string        `json:"mode"`
	BaseQuality   int           `json:"base_quality"`
	BaseDistance  float64       `json:"base_distance"`
	FinalQuality  int           `json:"final_quality"`
	FinalDistance float64       `json:"final_distance"`
	Stages        []StageResult `json:"stages"`
}

// StageResult 阶段结果
type StageResult struct {
	Name     string                 `json:"name"`
	Order    int                    `json:"order"`
	Executed bool                   `json:"executed"`
	Options  map[string]interface{} `json:"options"`
}

// ============================================================================
// 统一接口
// ============================================================================

// PrecisionPredictor 精度预测器（统一接口）
type PrecisionPredictor struct {
	mode     PrecisionMode
	basic    *BayesianOptimizer
	advanced *AdvancedPipeline
}

// NewPrecisionPredictor 创建精度预测器
func NewPrecisionPredictor(mode PrecisionMode, tool string, features *ImageFeaturesAdvanced) *PrecisionPredictor {
	qualityRange := [2]int{70, 100}

	return &PrecisionPredictor{
		mode:     mode,
		basic:    NewBayesianOptimizer(tool, qualityRange),
		advanced: NewAdvancedPipeline(tool, features),
	}
}

// Predict 预测（根据模式）
func (pp *PrecisionPredictor) Predict(targetMode string) (interface{}, error) {
	switch pp.mode {
	case PrecisionBasic:
		quality, distance, err := pp.basic.PredictOptimal(targetMode)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"mode":     "basic",
			"quality":  quality,
			"distance": distance,
		}, nil

	case PrecisionAdvanced:
		result, err := pp.advanced.Execute(targetMode)
		if err != nil {
			return nil, err
		}
		return result, nil

	default:
		return nil, fmt.Errorf("unknown precision mode: %v", pp.mode)
	}
}
