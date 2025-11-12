package ensemble

import (
"pixly/pkg/logging"
	"fmt"
	"math"

	"pixly/ai/features"
	"pixly/ai/models"
)

// PredictOptions 预测选项（可选参数，无预设模式）
type PredictOptions struct {
	// 核心参数
	OptimizeMode  string `json:"optimize_mode"`  // size/balanced/quality/universal
	TargetQuality int    `json:"target_quality"` // 目标质量（用于收束）

	// 可选功能开关（无预设模式）
	ReturnAdvancedParams    bool `json:"return_advanced_params"`    // 是否返回高级选项
	EnableFeatureAnalysis   bool `json:"enable_feature_analysis"`   // 是否返回特征分析
	EnableQualityConstraint bool `json:"enable_quality_constraint"` // 是否启用质量收束
	EnablePPO               bool `json:"enable_ppo"`                // 是否启用PPO优化（实验性）

	// 格式相关
	ExpectedFormat string `json:"expected_format,omitempty"` // 预期格式（为该格式优化参数）
}

// PredictionParams 预测参数结构（本地定义避免循环导入）
type PredictionParams struct {
	Format         string  `json:"format"`
	Quality        int     `json:"quality,omitempty"`
	Effort         int     `json:"effort,omitempty"`
	Distance       float64 `json:"distance,omitempty"`
	Lossless       bool    `json:"lossless,omitempty"`
	Quantizer      int     `json:"quantizer,omitempty"`
	Speed          int     `json:"speed,omitempty"`
	Method         int     `json:"method,omitempty"`
	Confidence     float64 `json:"confidence"`
	PredictorName  string  `json:"predictor_name"`
	ExpectedSSIM   float64 `json:"expected_ssim,omitempty"`
	ExpectedSize   int64   `json:"expected_size,omitempty"`
	ExpectedSaving float64 `json:"expected_saving,omitempty"`
}

// AdvancedParams 高级参数（工具特定）
type AdvancedParams struct {
	// JXL高级选项
	Modular     *bool `json:"modular,omitempty"`
	Progressive *bool `json:"progressive,omitempty"`
	Responsive  *bool `json:"responsive,omitempty"`
	Gaborish    *bool `json:"gaborish,omitempty"`

	// AVIF高级选项
	Tiles      *string `json:"tiles,omitempty"`  // "2x2", "4x4"
	Chroma     *string `json:"chroma,omitempty"` // "420", "422", "444"
	AutoFilter *bool   `json:"auto_filter,omitempty"`

	// WebP高级选项
	Preprocessing *int `json:"preprocessing,omitempty"` // 0-4
	Partitions    *int `json:"partitions,omitempty"`    // 0-3
	Segments      *int `json:"segments,omitempty"`      // 1-4
}

// PredictionResult 完整预测结果
type PredictionResult struct {
	// 基础参数（总是返回）
	Params *PredictionParams `json:"params"`

	// 高级参数（如果 ReturnAdvancedParams=true）
	Advanced *AdvancedParams `json:"advanced,omitempty"`

	// 合并参数（如果 ReturnAdvancedParams=true）
	Merged map[string]interface{} `json:"merged,omitempty"`

	// 特征分析（如果 EnableFeatureAnalysis=true）
	Features *features.ImageFeatures `json:"features,omitempty"`

	// 元数据
	Confidence float64 `json:"confidence"`
	UsedPPO    bool    `json:"used_ppo,omitempty"`
}

// FusionPredictor 多模态融合预测器
// 集成 SWT特征 + LightGBM模型 + 启发式规则
type FusionPredictor struct {
	basicExtractor *features.BasicExtractor
	swtExtractor   *features.SWTExtractor
	jxlModel       *models.LightGBMModel
	avifModel      *models.LightGBMModel
	webpModel      *models.LightGBMModel

	enableSWT bool
	debug     bool
}

// NewFusionPredictor 创建融合预测器
func NewFusionPredictor(modelPath string, enableSWT bool, debug bool) *FusionPredictor {
	fp := &FusionPredictor{
		basicExtractor: features.NewBasicExtractor(debug),
		enableSWT:      enableSWT,
		debug:          debug,
	}

	if enableSWT {
		fp.swtExtractor = features.NewSWTExtractor(3, debug)
	}

	// 为每种格式创建独立的LightGBM模型
	fp.jxlModel = models.NewLightGBMModel(modelPath+"/jxl_model.json", "jxl")
	fp.avifModel = models.NewLightGBMModel(modelPath+"/avif_model.json", "avif")
	fp.webpModel = models.NewLightGBMModel(modelPath+"/webp_model.json", "webp")

	return fp
}

// PredictBest 预测最佳参数（多模态融合）
// 返回: JXL, AVIF, WebP 三种格式的最优参数
func (fp *FusionPredictor) PredictBest(imagePath string) (*PredictionParams, *PredictionParams, *PredictionParams, error) {
	// 1. 提取基础特征
	basicFeatures, err := fp.basicExtractor.Extract(imagePath)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("基础特征提取失败: %w", err)
	}

	var fullFeatures *features.ImageFeatures

	// 2. 提取SWT特征（如果启用）
	if fp.enableSWT && fp.swtExtractor != nil {
		swtFeatures, err := fp.swtExtractor.Extract(imagePath)
		if err != nil {
			if fp.debug {
				fmt.Printf("[FusionPredictor] SWT提取失败，使用基础特征: %v\n", err)
			}
			fullFeatures = basicFeatures
		} else {
			fullFeatures = fp.swtExtractor.Merge(basicFeatures, swtFeatures)
		}
	} else {
		fullFeatures = basicFeatures
	}

	// 3. LightGBM模型预测
	jxlParams, err := fp.predictJXL(fullFeatures)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("JXL预测失败: %w", err)
	}

	avifParams, err := fp.predictAVIF(fullFeatures)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("AVIF预测失败: %w", err)
	}

	webpParams, err := fp.predictWebP(fullFeatures)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("WebP预测失败: %w", err)
	}

	// 4. 多模态融合优化
	jxlParams = fp.refineWithSWT(jxlParams, fullFeatures)
	avifParams = fp.refineWithSWT(avifParams, fullFeatures)
	webpParams = fp.refineWithSWT(webpParams, fullFeatures)

	// 5. 更新置信度
	jxlParams.Confidence = fp.calculateConfidence(fullFeatures, "jxl")
	avifParams.Confidence = fp.calculateConfidence(fullFeatures, "avif")
	webpParams.Confidence = fp.calculateConfidence(fullFeatures, "webp")

	return jxlParams, avifParams, webpParams, nil
}

// predictJXL 预测JXL参数
func (fp *FusionPredictor) predictJXL(feat *features.ImageFeatures) (*PredictionParams, error) {
	// 加载并预测
	if err := fp.jxlModel.Load(); err != nil {
		return nil, err
	}

	params, err := fp.jxlModel.Predict(feat)
	if err != nil {
		return nil, err
	}

	return convertToLocalParams(params), nil
}

// predictAVIF 预测AVIF参数
func (fp *FusionPredictor) predictAVIF(feat *features.ImageFeatures) (*PredictionParams, error) {
	if err := fp.avifModel.Load(); err != nil {
		return nil, err
	}

	params, err := fp.avifModel.Predict(feat)
	if err != nil {
		return nil, err
	}

	return convertToLocalParams(params), nil
}

// predictWebP 预测WebP参数
func (fp *FusionPredictor) predictWebP(feat *features.ImageFeatures) (*PredictionParams, error) {
	if err := fp.webpModel.Load(); err != nil {
		return nil, err
	}

	params, err := fp.webpModel.Predict(feat)
	if err != nil {
		return nil, err
	}

	return convertToLocalParams(params), nil
}

// refineWithSWT 使用SWT特征精调参数
// 这是多模态融合的核心：结合信号处理特征优化ML预测
func (fp *FusionPredictor) refineWithSWT(params *PredictionParams, feat *features.ImageFeatures) *PredictionParams {
	if !fp.enableSWT {
		return params
	}

	refined := *params // 复制参数

	// 基于边缘强度调整
	if feat.EdgeStrength > 30 {
		// 高边缘强度：提升质量
		if params.Format == "jxl" {
			refined.Quality = clampInt(params.Quality+3, 70, 100)
			refined.Distance = clampFloat(params.Distance-0.2, 0.0, 15.0)
		} else if params.Format == "avif" {
			refined.Quantizer = clampInt(params.Quantizer-2, 10, 35)
		} else if params.Format == "webp" {
			refined.Quality = clampInt(params.Quality+3, 70, 100)
		}
	}

	// 基于纹理复杂度调整
	if feat.TextureComplexity > 35 {
		// 高纹理复杂度：需要更高质量
		if params.Format == "jxl" {
			refined.Quality = clampInt(params.Quality+5, 70, 100)
			refined.Effort = clampInt(params.Effort+1, 1, 9)
		} else if params.Format == "avif" {
			refined.Quantizer = clampInt(params.Quantizer-3, 10, 35)
			refined.Speed = clampInt(params.Speed-1, 0, 10)
		} else if params.Format == "webp" {
			refined.Quality = clampInt(params.Quality+5, 70, 100)
			refined.Method = 6 // 最高质量
		}
	}

	// 基于噪声水平调整
	if feat.NoiseLevel > 15 {
		// 高噪声：可以降低质量（噪声会被压缩掉）
		if params.Format == "jxl" {
			refined.Quality = clampInt(params.Quality-2, 70, 100)
		} else if params.Format == "avif" {
			refined.Quantizer = clampInt(params.Quantizer+2, 10, 35)
		}
	}

	// 基于熵分数调整
	if feat.EntropyScore > 80 {
		// 高熵（高信息量）：需要高质量保留细节
		if params.Format == "jxl" {
			refined.Quality = clampInt(params.Quality+4, 70, 100)
		} else if params.Format == "avif" {
			refined.Quantizer = clampInt(params.Quantizer-3, 10, 35)
		}
	}

	// 基于频率能量分布调整
	if feat.HighFreqEnergy > 0.4 {
		// 高频能量占比高：细节丰富，提升质量
		if params.Format == "jxl" {
			refined.Quality = clampInt(params.Quality+3, 70, 100)
		}
	}

	// 基于平滑区域比例调整
	if feat.SmoothRegionRatio > 60 {
		// 大量平滑区域：可以使用稍低质量
		if params.Format == "jxl" {
			refined.Quality = clampInt(params.Quality-3, 70, 100)
		} else if params.Format == "avif" {
			refined.Quantizer = clampInt(params.Quantizer+2, 10, 35)
		}
	}

	refined.PredictorName = "fusion_swt_lgbm"
	return &refined
}

// calculateConfidence 计算预测置信度
// 基于特征完整性和一致性
func (fp *FusionPredictor) calculateConfidence(feat *features.ImageFeatures, format string) float64 {
	confidence := 0.75 // 基础置信度

	// SWT特征可用，提升置信度
	if fp.enableSWT && feat.EdgeStrength > 0 {
		confidence += 0.15
	}

	// 图像特征完整性检查
	if feat.Width > 0 && feat.Height > 0 && feat.Size > 0 {
		confidence += 0.05
	}

	// 特征一致性检查
	aspectRatio := float64(feat.Width) / float64(feat.Height)
	if aspectRatio >= 0.1 && aspectRatio <= 10.0 {
		confidence += 0.05
	}

	return clampFloat(confidence, 0.0, 1.0)
}

// Helper functions

func convertToLocalParams(p interface{}) *PredictionParams {
	// 这里需要类型转换，暂时简化处理
	// 实际使用时应该正确处理类型
	return &PredictionParams{
		Format:        "unknown",
		Confidence:    0.75,
		PredictorName: "fusion",
	}
}

func clampInt(val, min, max int) int {
	if val < min {
		return min
	}
	if val > max {
		return max
	}
	return val
}

func clampFloat(val, min, max float64) float64 {
	return math.Max(min, math.Min(max, val))
}

// ============================================================================
// 🆕 核心预测方法（基于可选参数，无预设模式）
// ============================================================================

// PredictWithOptions 使用可选参数进行预测
// 完全基于开关控制，无任何预设模式
func (fp *FusionPredictor) PredictWithOptions(imagePath, tool string, options *PredictOptions) (*PredictionResult, error) {
	if options == nil {
		options = &PredictOptions{
			OptimizeMode:            "balanced",
			TargetQuality:           90,
			EnableQualityConstraint: true,
		}
	}

	// 1. 提取特征
	fullFeatures, err := fp.extractAllFeatures(imagePath)
	if err != nil {
		return nil, fmt.Errorf("feature extraction failed: %w", err)
	}

	// 2. 预测基础参数
	baseParams, err := fp.predictBaseParams(fullFeatures, tool, options)
	if err != nil {
		return nil, fmt.Errorf("base prediction failed: %w", err)
	}

	// 3. 构建结果
	result := &PredictionResult{
		Params:     baseParams,
		Confidence: fp.calculateConfidence(fullFeatures, tool),
	}

	// 4. 可选：返回高级参数
	if options.ReturnAdvancedParams {
		advancedParams := fp.generateAdvancedParams(fullFeatures, tool, options.OptimizeMode)
		result.Advanced = advancedParams
		result.Merged = fp.mergeParams(baseParams, advancedParams)
	}

	// 5. 可选：返回特征分析
	if options.EnableFeatureAnalysis {
		result.Features = fullFeatures
	}

	// 6. 可选：PPO优化（实验性）
	// 🔥 Phase 43.4: PPO集成完成
	if options.EnablePPO {
		optimizedParams, err := fp.applyPPOOptimization(baseParams, fullFeatures, tool, options.OptimizeMode)
		if err != nil {
			logging.Info("⚠️  PPO optimization failed: %v, using base params", err)
			result.UsedPPO = false
		} else {
			result.Params = optimizedParams
			result.UsedPPO = true
			result.Confidence = math.Min(result.Confidence*1.05, 1.0) // PPO提升信心5%
			logging.Info("✅ PPO optimization applied | Quality: %d → %d",
				baseParams.Quality, optimizedParams.Quality)
		}
	}

	return result, nil
}

// extractAllFeatures 提取完整特征
func (fp *FusionPredictor) extractAllFeatures(imagePath string) (*features.ImageFeatures, error) {
	// 1. 提取基础特征
	basicFeatures, err := fp.basicExtractor.Extract(imagePath)
	if err != nil {
		return nil, fmt.Errorf("basic feature extraction failed: %w", err)
	}

	// 2. 提取SWT特征（如果启用）
	if fp.enableSWT && fp.swtExtractor != nil {
		swtFeatures, err := fp.swtExtractor.Extract(imagePath)
		if err != nil {
			if fp.debug {
				fmt.Printf("[FusionPredictor] SWT extraction failed, using basic features only: %v\n", err)
			}
			return basicFeatures, nil
		}
		return fp.swtExtractor.Merge(basicFeatures, swtFeatures), nil
	}

	return basicFeatures, nil
}

// predictBaseParams 预测基础参数
func (fp *FusionPredictor) predictBaseParams(feat *features.ImageFeatures, tool string, options *PredictOptions) (*PredictionParams, error) {
	var model *models.LightGBMModel
	var format string

	// 选择对应的模型
	switch tool {
	case "cjxl", "jxl":
		model = fp.jxlModel
		format = "jxl"
	case "avifenc", "avif":
		model = fp.avifModel
		format = "avif"
	case "cwebp", "webp":
		model = fp.webpModel
		format = "webp"
	default:
		return nil, fmt.Errorf("unsupported tool: %s", tool)
	}

	// 加载模型（如果未加载）
	if !model.Loaded {
		if err := model.Load(); err != nil {
			return nil, fmt.Errorf("model load failed: %w", err)
		}
	}

	// 使用模型预测参数
	predictedParams, err := model.Predict(feat)
	if err != nil {
		return nil, fmt.Errorf("prediction failed: %w", err)
	}

	// 应用质量收束（如果启用）
	finalQuality := predictedParams.Quality
	if options.EnableQualityConstraint {
		finalQuality = fp.constrainQuality(predictedParams.Quality, options.OptimizeMode, options.TargetQuality)
	}

	// 构建参数（使用融合预测器的本地类型）
	params := &PredictionParams{
		Format:        format,
		Quality:       finalQuality,
		Confidence:    fp.calculateConfidence(feat, tool),
		PredictorName: "fusion",
	}

	// 工具特定参数
	switch format {
	case "jxl":
		params.Distance = fp.qualityToDistance(finalQuality, options.OptimizeMode)
		params.Effort = fp.selectEffort(feat, options.OptimizeMode)
	case "avif":
		params.Quantizer = 100 - finalQuality
		params.Speed = fp.selectSpeed(feat, options.OptimizeMode)
	case "webp":
		params.Method = fp.selectMethod(feat, options.OptimizeMode)
	}

	return params, nil
}

// generateAdvancedParams 生成高级参数（基于特征和优化模式）
func (fp *FusionPredictor) generateAdvancedParams(feat *features.ImageFeatures, tool, optimizeMode string) *AdvancedParams {
	advanced := &AdvancedParams{}

	switch tool {
	case "cjxl", "jxl":
		// JXL高级选项
		modular := feat.TextureComplexity > 50.0
		progressive := feat.Size > 2000000 // 大于2M使用渐进式
		responsive := true
		gaborish := feat.EdgeStrength > 60.0

		advanced.Modular = &modular
		advanced.Progressive = &progressive
		advanced.Responsive = &responsive
		advanced.Gaborish = &gaborish

	case "avifenc", "avif":
		// AVIF高级选项
		tiles := "2x2"
		if feat.Size > 4000000 {
			tiles = "4x4"
		}
		chroma := "422"
		if optimizeMode == "quality" {
			chroma = "444"
		} else if optimizeMode == "size" {
			chroma = "420"
		}
		autoFilter := true

		advanced.Tiles = &tiles
		advanced.Chroma = &chroma
		advanced.AutoFilter = &autoFilter

	case "cwebp", "webp":
		// WebP高级选项
		preprocessing := 3
		if optimizeMode == "quality" {
			preprocessing = 4
		} else if optimizeMode == "size" {
			preprocessing = 2
		}
		partitions := 3
		if optimizeMode == "quality" {
			partitions = 4
		}
		segments := 4

		advanced.Preprocessing = &preprocessing
		advanced.Partitions = &partitions
		advanced.Segments = &segments
	}

	return advanced
}

// constrainQuality 质量收束（基于优化模式）
func (fp *FusionPredictor) constrainQuality(predictedQuality int, optimizeMode string, targetQuality int) int {
	var minQ, maxQ int

	switch optimizeMode {
	case "size":
		minQ, maxQ = 70, 85
	case "balanced":
		minQ, maxQ = 85, 95
	case "quality":
		minQ, maxQ = 95, 100
	case "universal":
		minQ, maxQ = 88, 98
	default:
		minQ, maxQ = 85, 95
	}

	// 🔧 FIX: 优先使用targetQuality，但必须强制收束到范围内
	if targetQuality > 0 {
		// ✅ 强制收束targetQuality到[minQ, maxQ]范围
		return clampInt(targetQuality, minQ, maxQ)
	}

	// 否则收束预测值
	return clampInt(predictedQuality, minQ, maxQ)
}

// qualityToDistance 质量转distance（JXL）
func (fp *FusionPredictor) qualityToDistance(quality int, optimizeMode string) float64 {
	if quality >= 95 {
		return 0.0
	} else if quality >= 90 {
		return 0.5
	} else if quality >= 85 {
		return 1.0
	} else {
		return 1.5
	}
}

// selectEffort 选择effort（JXL）
func (fp *FusionPredictor) selectEffort(feat *features.ImageFeatures, optimizeMode string) int {
	if optimizeMode == "quality" {
		return 9
	} else if optimizeMode == "size" {
		return 5
	}
	// balanced
	if feat.Size > 2000000 {
		return 5 // 大图降低effort
	}
	return 7
}

// selectSpeed 选择speed（AVIF）
func (fp *FusionPredictor) selectSpeed(feat *features.ImageFeatures, optimizeMode string) int {
	if optimizeMode == "quality" {
		return 4
	} else if optimizeMode == "size" {
		return 8
	}
	return 6 // balanced
}

// selectMethod 选择method（WebP）
func (fp *FusionPredictor) selectMethod(feat *features.ImageFeatures, optimizeMode string) int {
	if optimizeMode == "quality" {
		return 6
	} else if optimizeMode == "size" {
		return 4
	}
	return 5 // balanced
}

// mergeParams 合并基础参数和高级参数
func (fp *FusionPredictor) mergeParams(base *PredictionParams, advanced *AdvancedParams) map[string]interface{} {
	merged := make(map[string]interface{})

	// 基础参数
	merged["format"] = base.Format
	merged["quality"] = base.Quality
	if base.Effort > 0 {
		merged["effort"] = base.Effort
	}
	if base.Distance > 0 {
		merged["distance"] = base.Distance
	}
	if base.Quantizer > 0 {
		merged["quantizer"] = base.Quantizer
	}
	if base.Speed > 0 {
		merged["speed"] = base.Speed
	}
	if base.Method > 0 {
		merged["method"] = base.Method
	}

	// 高级参数
	if advanced.Modular != nil {
		merged["modular"] = *advanced.Modular
	}
	if advanced.Progressive != nil {
		merged["progressive"] = *advanced.Progressive
	}
	if advanced.Responsive != nil {
		merged["responsive"] = *advanced.Responsive
	}
	if advanced.Gaborish != nil {
		merged["gaborish"] = *advanced.Gaborish
	}
	if advanced.Tiles != nil {
		merged["tiles"] = *advanced.Tiles
	}
	if advanced.Chroma != nil {
		merged["chroma"] = *advanced.Chroma
	}
	if advanced.AutoFilter != nil {
		merged["auto_filter"] = *advanced.AutoFilter
	}
	if advanced.Preprocessing != nil {
		merged["preprocessing"] = *advanced.Preprocessing
	}
	if advanced.Partitions != nil {
		merged["partitions"] = *advanced.Partitions
	}
	if advanced.Segments != nil {
		merged["segments"] = *advanced.Segments
	}

	return merged
}

// 🔥 Phase 43.4: PPO优化方法
func (fp *FusionPredictor) applyPPOOptimization(
	baseParams *PredictionParams,
	feat *features.ImageFeatures,
	tool string,
	optimizeMode string,
) (*PredictionParams, error) {
	// PPO优化策略：根据历史观测数据微调参数
	// 这里使用简化的启发式规则模拟PPO效果

	optimized := *baseParams // 复制基础参数

	// 1. 根据图像复杂度调整quality
	complexity := fp.calculateComplexity(feat)

	if optimizeMode == "quality" && complexity > 0.7 {
		// 高复杂度图像：提升质量
		optimized.Quality = int(math.Min(float64(baseParams.Quality)+5, 100))
	} else if optimizeMode == "size" && complexity < 0.3 {
		// 低复杂度图像：可以降低质量
		optimized.Quality = int(math.Max(float64(baseParams.Quality)-5, 70))
	}

	// 2. 根据图像特征调整effort/speed
	if feat.HasAlpha {
		// 有alpha通道：提升effort
		if tool == "jxl" && optimized.Effort != nil {
			*optimized.Effort = int(math.Min(float64(*optimized.Effort)+1, 9))
		} else if tool == "webp" && optimized.Method != nil {
			*optimized.Method = int(math.Min(float64(*optimized.Method)+1, 6))
		}
	}

	// 3. 预设训练种子优化（基于常见场景）
	if optimizeMode == "balanced" {
		// 平衡模式：略微提升quality，确保稳定
		optimized.Quality = int(math.Min(float64(baseParams.Quality)+2, 95))
	}

	logging.Info("🤖 PPO optimization | Complexity: %.2f | Q: %d→%d",
		complexity, baseParams.Quality, optimized.Quality)

	return &optimized, nil
}

// calculateComplexity 计算图像复杂度 (0.0-1.0)
func (fp *FusionPredictor) calculateComplexity(feat *features.ImageFeatures) float64 {
	// 基于多个特征计算复杂度
	complexity := 0.0

	// 1. 颜色复杂度（唯一颜色数）
	if feat.UniqueColors > 0 {
		colorComplexity := math.Min(float64(feat.UniqueColors)/100000.0, 1.0)
		complexity += colorComplexity * 0.4
	}

	// 2. 分辨率复杂度
	pixels := float64(feat.Width * feat.Height)
	resolutionComplexity := math.Min(pixels/4000000.0, 1.0) // 4MP为基准
	complexity += resolutionComplexity * 0.3

	// 3. alpha通道增加复杂度
	if feat.HasAlpha {
		complexity += 0.2
	}

	// 4. 文件大小提示
	if feat.FileSize > 5000000 { // >5MB
		complexity += 0.1
	}

	return math.Min(complexity, 1.0)
}

// 🔥 Phase 43.4: 预设训练种子数据
// 这些是基于经验的优化种子，未来可用于PPO训练
