package predictor

import "time"

// FileFeatures 文件特征信息
// 用于智能预测最优转换参数
type FileFeatures struct {
	// 基本信息
	FilePath string
	Format   string // "png", "jpeg", "gif", etc.
	FileSize int64
	Width    int
	Height   int

	// 格式特征
	HasAlpha   bool
	ColorSpace string // "rgb", "rgba", "grayscale"
	BitDepth   int    // 8, 16, 32
	PixFmt     string // FFprobe的pix_fmt字段

	// 质量特征（主要用于JPEG）
	EstimatedQuality int     // 0-100
	NoiseLevel       float64 // 0-1
	Compression      float64 // 当前压缩率

	// 内容特征
	IsAnimated bool    // 是否动图
	FrameCount int     // 帧数（动画）
	PageCount  int     // 页数（多页图片如TIFF）
	FrameRate  float64 // 帧率

	// 派生特征
	BytesPerPixel float64 // 文件大小/像素数
	Complexity    float64 // 图像复杂度估算 (0-1)
}

// ConversionParams 转换参数
// 由预测器生成的最优转换参数
type ConversionParams struct {
	// 通用参数
	TargetFormat  string // 目标格式
	Quality       int    // 质量参数 (0-100)
	Threads       int    // 线程数
	PreserveAlpha bool   // 保留透明度

	// 无损/有损决策
	Lossless     bool
	Distance     float64 // 0=无损, 1-15=有损
	Effort       int     // 1-10，压缩力度
	LosslessJPEG bool    // JPEG无损重包装

	// AVIF专用参数
	CRF   int // 0-63，AVIF质量参数
	Speed int // 0-10，AVIF编码速度

	// 降采样参数
	DownscaleMode string   // "none", "target_size", "resolution"
	TargetSizeKB  *int64   // 目标文件大小(KB)
	MaxResolution *int     // 最大分辨率
	ScaleRatio    *float64 // 缩放比例

	// 🎨 P0特性 (色度子采样 + Alpha质量优化，来自PIO/Squoosh启发)
	ChromaSubsampling string // 色度子采样模式: "auto"|"444"|"422"|"420"
	AlphaQuality      *int   // 独立Alpha质量 (nil=跟随RGB, 0-100=独立质量)
}

// Prediction 预测结果
// 包含预测的参数和置信度
type Prediction struct {
	Features   *FileFeatures // 文件特征(用于时间估算)
	Params     *ConversionParams
	Confidence float64 // 0-1，预测置信度
	Method     string  // "lookup_table", "rule_based", "regression"
	RuleName   string  // 使用的规则名称（如果是规则预测）

	// 预测的空间节省
	ExpectedSaving    float64 // 预期节省百分比 (0-1)
	ExpectedSizeBytes int64   // 预期文件大小

	// 辅助信息
	ShouldExplore         bool                // 是否需要探索
	ExplorationCandidates []*ConversionParams // 探索候选参数

	// 冲突检测报告(新增)
	ConflictReport *ConflictReport // 冲突检测报告

	PredictionTime time.Duration
}

// PredictionRule 预测规则
type PredictionRule struct {
	Name       string
	Condition  func(*FileFeatures) bool
	Prediction *ConversionParams
	Confidence float64
	Priority   int // 规则优先级，数字越大优先级越高
}

// ExplorationResult 探索结果
type ExplorationResult struct {
	BestParams   *ConversionParams
	BestSize     int64
	TestedParams []ConversionParams
	TestResults  map[string]int64 // 参数描述 -> 文件大小
	ExploreTime  time.Duration
}
