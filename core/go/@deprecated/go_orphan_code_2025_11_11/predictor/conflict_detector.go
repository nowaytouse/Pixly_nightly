package predictor

import (
	"fmt"

	"go.uber.org/zap"
)

// ConflictDetector 冲突检测器
// 在ML预测前检测并预警可能的转换冲突
type ConflictDetector struct {
	logger *zap.Logger
}

// NewConflictDetector 创建冲突检测器
func NewConflictDetector(logger *zap.Logger) *ConflictDetector {
	return &ConflictDetector{
		logger: logger,
	}
}

// Conflict 冲突信息
type Conflict struct {
	Type       string // 冲突类型
	Severity   string // 严重程度: Critical, Warning, Info
	Message    string // 冲突描述
	Suggestion string // 建议解决方案
}

// ConflictReport 冲突检测报告
type ConflictReport struct {
	Conflicts   []Conflict
	HasCritical bool // 是否有严重冲突
	CanProceed  bool // 是否可以继续转换
}

// CheckConflicts 检测转换冲突
// 在ML预测后执行,用于验证预测结果的可行性
func (cd *ConflictDetector) CheckConflicts(
	features *FileFeatures,
	prediction *Prediction,
) (*ConflictReport, error) {
	conflicts := make([]Conflict, 0)

	// 1. 格式组合冲突检测
	formatConflicts := cd.checkFormatCombination(features, prediction)
	conflicts = append(conflicts, formatConflicts...)

	// 2. 多页图片检测
	multipageConflicts := cd.checkMultipage(features)
	conflicts = append(conflicts, multipageConflicts...)

	// 3. 动画降采样冲突
	animationConflicts := cd.checkAnimationDownscale(features, prediction)
	conflicts = append(conflicts, animationConflicts...)

	// 4. 资源消耗预警
	resourceConflicts := cd.checkResourceUsage(features, prediction)
	conflicts = append(conflicts, resourceConflicts...)

	// 5. 质量参数冲突
	qualityConflicts := cd.checkQualityParams(features, prediction)
	conflicts = append(conflicts, qualityConflicts...)

	// 生成报告
	report := &ConflictReport{
		Conflicts:   conflicts,
		HasCritical: cd.hasCriticalConflict(conflicts),
	}
	report.CanProceed = len(conflicts) == 0 || !report.HasCritical

	if len(conflicts) > 0 {
		cd.logger.Warn("检测到转换冲突",
			zap.Int("conflict_count", len(conflicts)),
			zap.Bool("has_critical", report.HasCritical),
			zap.Bool("can_proceed", report.CanProceed))
	}

	return report, nil
}

// checkFormatCombination 检测格式组合冲突
func (cd *ConflictDetector) checkFormatCombination(
	features *FileFeatures,
	prediction *Prediction,
) []Conflict {
	conflicts := make([]Conflict, 0)

	// 定义有效的格式转换路径
	validRoutes := map[string][]string{
		"gif":  {"jxl", "webp"}, // GIF只能转JXL或WebP
		"apng": {"jxl"},         // APNG只能转JXL
	}

	if validFormats, exists := validRoutes[features.Format]; exists {
		targetFormat := prediction.Params.TargetFormat
		isValid := false
		for _, format := range validFormats {
			if format == targetFormat {
				isValid = true
				break
			}
		}

		if !isValid {
			conflicts = append(conflicts, Conflict{
				Type:     "InvalidFormatCombination",
				Severity: "Critical",
				Message: fmt.Sprintf("%s -> %s 转换不支持",
					features.Format, targetFormat),
				Suggestion: fmt.Sprintf("推荐格式: %v", validFormats),
			})
		}
	}

	return conflicts
}

// checkMultipage 检测多页图片
func (cd *ConflictDetector) checkMultipage(features *FileFeatures) []Conflict {
	conflicts := make([]Conflict, 0)

	if features.PageCount > 1 {
		conflicts = append(conflicts, Conflict{
			Type:       "MultiPageImage",
			Severity:   "Warning",
			Message:    fmt.Sprintf("检测到%d页图片", features.PageCount),
			Suggestion: "多页图片可能只转换第一页,或转换失败",
		})
	}

	return conflicts
}

// checkAnimationDownscale 检测动画降采样冲突
func (cd *ConflictDetector) checkAnimationDownscale(
	features *FileFeatures,
	prediction *Prediction,
) []Conflict {
	conflicts := make([]Conflict, 0)

	if features.IsAnimated && prediction.Params.DownscaleMode != "" &&
		prediction.Params.DownscaleMode != "none" {
		conflicts = append(conflicts, Conflict{
			Type:       "AnimationDownscaleConflict",
			Severity:   "Critical",
			Message:    "动画文件不支持降采样",
			Suggestion: "请禁用降采样或选择静态图片",
		})
	}

	return conflicts
}

// checkResourceUsage 检测资源消耗预警
func (cd *ConflictDetector) checkResourceUsage(
	features *FileFeatures,
	prediction *Prediction,
) []Conflict {
	conflicts := make([]Conflict, 0)

	// 超大文件 + 高质量 = OOM风险
	fileSizeMB := float64(features.FileSize) / (1024 * 1024)
	if fileSizeMB > 100 && prediction.Params.Quality > 90 {
		conflicts = append(conflicts, Conflict{
			Type:     "OOMRisk",
			Severity: "Warning",
			Message: fmt.Sprintf("超大文件(%.1fMB) + 高质量(%d)可能导致内存溢出",
				fileSizeMB, prediction.Params.Quality),
			Suggestion: "建议降低质量参数或启用降采样",
		})
	}

	// 超高分辨率预警
	megapixels := float64(features.Width*features.Height) / 1_000_000.0
	if megapixels > 50 {
		conflicts = append(conflicts, Conflict{
			Type:     "HighResolution",
			Severity: "Info",
			Message: fmt.Sprintf("超高分辨率图片(%.1fMP),转换可能耗时较长",
				megapixels),
			Suggestion: "建议启用降采样以提高转换速度",
		})
	}

	return conflicts
}

// checkQualityParams 检测质量参数冲突
func (cd *ConflictDetector) checkQualityParams(
	features *FileFeatures,
	prediction *Prediction,
) []Conflict {
	conflicts := make([]Conflict, 0)

	// 无损模式 + 质量参数
	if prediction.Params.Lossless && prediction.Params.Quality < 100 {
		conflicts = append(conflicts, Conflict{
			Type:       "LosslessQualityConflict",
			Severity:   "Warning",
			Message:    "无损模式下质量参数将被忽略",
			Suggestion: "无损模式会自动使用100%质量",
		})
	}

	// JPEG转JXL lossless特殊处理
	if features.Format == "jpeg" || features.Format == "jpg" {
		if prediction.Params.TargetFormat == "jxl" && !prediction.Params.Lossless {
			conflicts = append(conflicts, Conflict{
				Type:       "JPEGLosslessOpportunity",
				Severity:   "Info",
				Message:    "JPEG可使用--lossless_jpeg=1实现完美无损转换",
				Suggestion: "建议启用lossless_jpeg模式,可节省16-22%空间",
			})
		}
	}

	return conflicts
}

// hasCriticalConflict 检查是否有严重冲突
func (cd *ConflictDetector) hasCriticalConflict(conflicts []Conflict) bool {
	for _, conflict := range conflicts {
		if conflict.Severity == "Critical" {
			return true
		}
	}
	return false
}

// AdjustPredictionForConflicts 根据冲突自动调整预测策略
func (cd *ConflictDetector) AdjustPredictionForConflicts(
	features *FileFeatures,
	prediction *Prediction,
	report *ConflictReport,
) *Prediction {
	if !report.HasCritical {
		return prediction
	}

	adjustedPrediction := *prediction

	for _, conflict := range report.Conflicts {
		switch conflict.Type {
		case "InvalidFormatCombination":
			// 调整为有效的格式
			if features.Format == "gif" {
				adjustedPrediction.Params.TargetFormat = "jxl"
				cd.logger.Info("自动调整格式: GIF -> JXL")
			} else if features.Format == "apng" {
				adjustedPrediction.Params.TargetFormat = "jxl"
				cd.logger.Info("自动调整格式: APNG -> JXL")
			}

		case "AnimationDownscaleConflict":
			// 禁用降采样
			adjustedPrediction.Params.DownscaleMode = "none"
			adjustedPrediction.Params.TargetSizeKB = nil
			adjustedPrediction.Params.ScaleRatio = nil
			cd.logger.Info("自动禁用动画降采样")

		case "OOMRisk":
			// 降低质量或启用降采样
			if adjustedPrediction.Params.Quality > 85 {
				adjustedPrediction.Params.Quality = 85
				cd.logger.Info("自动降低质量参数: 85")
			}
		}
	}

	return &adjustedPrediction
}
