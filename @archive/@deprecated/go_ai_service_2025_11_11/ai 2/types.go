package ai

// ImageFeatures 图像特征结构
// 包含基础特征和高级特征（SWT）
type ImageFeatures struct {
	// 基础特征（与旧版本兼容）
	Width       int     `json:"width"`
	Height      int     `json:"height"`
	Size        int64   `json:"size"`
	Format      string  `json:"format"`
	HasAlpha    bool    `json:"has_alpha"`
	IsAnimated  bool    `json:"is_animated"`
	FrameCount  int     `json:"frame_count,omitempty"`
	PixelCount  int64   `json:"pixel_count"`
	AspectRatio float64 `json:"aspect_ratio"`

	// SWT 小波特征（新增）
	EdgeStrength      float64 `json:"edge_strength,omitempty"`
	TextureComplexity float64 `json:"texture_complexity,omitempty"`
	NoiseLevel        float64 `json:"noise_level,omitempty"`
	DetailLevel       float64 `json:"detail_level,omitempty"`
	HighFreqEnergy    float64 `json:"high_freq_energy,omitempty"`
	MidFreqEnergy     float64 `json:"mid_freq_energy,omitempty"`
	LowFreqEnergy     float64 `json:"low_freq_energy,omitempty"`
	EntropyScore      float64 `json:"entropy_score,omitempty"`
	SmoothRegionRatio float64 `json:"smooth_region_ratio,omitempty"`
}

// PredictionParams 预测的转换参数
type PredictionParams struct {
	// 格式选择
	Format string `json:"format"` // jxl, avif, webp

	// JXL 参数
	Quality      int     `json:"quality,omitempty"`       // 1-100
	Effort       int     `json:"effort,omitempty"`        // 1-9
	Distance     float64 `json:"distance,omitempty"`      // 0.0-15.0
	Lossless     bool    `json:"lossless,omitempty"`      // 无损模式
	LosslessJPEG bool    `json:"lossless_jpeg,omitempty"` // JPEG 无损重编码
	Progressive  bool    `json:"progressive,omitempty"`   // 渐进式
	ModularLossy bool    `json:"modular_lossy,omitempty"` // 模块化有损

	// AVIF 参数
	Quantizer int `json:"quantizer,omitempty"` // 0-63
	Speed     int `json:"speed,omitempty"`     // 0-10

	// WebP 参数
	Method int `json:"method,omitempty"` // 0-6

	// 通用参数
	Preset string `json:"preset,omitempty"` // photo, drawing, icon, text

	// 预测元数据
	Confidence     float64 `json:"confidence"`         // 预测置信度 0-1
	PredictorName  string  `json:"predictor_name"`     // basic, ai, ensemble
	PredictionTime int64   `json:"prediction_time_ms"` // 预测耗时(ms)

	// 预期效果
	ExpectedSSIM   float64 `json:"expected_ssim,omitempty"`   // 预期 SSIM
	ExpectedSize   int64   `json:"expected_size,omitempty"`   // 预期文件大小
	ExpectedSaving float64 `json:"expected_saving,omitempty"` // 预期节省百分比
}

// AIConfig AI 预测器配置
type AIConfig struct {
	// 预测模式（已弃用，改用OptimizeMode）
	// Mode PredictionMode `json:"mode"`

	// 模型路径
	LightGBMModelPath string `json:"lightgbm_model_path,omitempty"`

	// 特征提取选项
	EnableSWT bool `json:"enable_swt"` // 启用 SWT 特征提取
	SWTLevels int  `json:"swt_levels"` // SWT 分解层数 (默认3)

	// 性能选项
	MaxPredictionTime int64 `json:"max_prediction_time_ms"` // 最大预测时间(ms)
	// FallbackOnError 已移除 - 违反质量宣言：禁止fallback代码
	// 错误时应响亮报错，而非静默降级

	// 调试选项
	Debug       bool `json:"debug"`
	LogFeatures bool `json:"log_features"`
}

// TrainingData 训练数据结构
type TrainingData struct {
	ID        string        `json:"id"`
	ImagePath string        `json:"image_path"`
	Features  ImageFeatures `json:"features"`

	// 最优参数（多个格式）
	OptimalJXL  *PredictionParams `json:"optimal_jxl,omitempty"`
	OptimalAVIF *PredictionParams `json:"optimal_avif,omitempty"`
	OptimalWebP *PredictionParams `json:"optimal_webp,omitempty"`

	// 质量指标
	SourceFormat string  `json:"source_format"`
	ResultSSIM   float64 `json:"result_ssim,omitempty"`
	ResultPSNR   float64 `json:"result_psnr,omitempty"`
	ResultSize   int64   `json:"result_size,omitempty"`

	// 元数据
	CreatedAt string `json:"created_at"`
	Source    string `json:"source"` // eagle, manual, grid_search
}

// TrainingDataset 训练数据集
type TrainingDataset struct {
	Version     string         `json:"version"`
	Description string         `json:"description"`
	CreatedAt   string         `json:"created_at"`
	SampleCount int            `json:"sample_count"`
	Samples     []TrainingData `json:"samples"`
}

// ============================================================================
// 🆕 新架构类型定义（无预设模式，基于可选参数）
// ============================================================================

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
	Features *ImageFeatures `json:"features,omitempty"`

	// 元数据
	Confidence float64 `json:"confidence"`
	UsedPPO    bool    `json:"used_ppo,omitempty"`
}
