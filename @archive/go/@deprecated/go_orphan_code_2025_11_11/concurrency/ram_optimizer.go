package concurrency

import (
	"fmt"
	"os/exec"
	"strconv"
	"strings"

	"go.uber.org/zap"
)

// ResolutionRule RAM优化规则 - 基于图片分辨率
type ResolutionRule struct {
	ThresholdMP    float64 // 百万像素阈值
	MaxConcurrency int     // 最大并发数
	Description    string  // 规则说明
}

// RAMOptimizer RAM优化器
// 根据图片分辨率动态调整并发数，防止大图OOM
type RAMOptimizer struct {
	logger *zap.Logger
	rules  []ResolutionRule
}

// NewRAMOptimizer 创建RAM优化器
func NewRAMOptimizer(logger *zap.Logger) *RAMOptimizer {
	return &RAMOptimizer{
		logger: logger,
		rules: []ResolutionRule{
			// 超高分辨率：>50MP（如8K图片）
			{
				ThresholdMP:    50.0,
				MaxConcurrency: 1,
				Description:    "超高分辨率(>50MP) - 单线程处理",
			},
			// 高分辨率：25-50MP（如6K图片）
			{
				ThresholdMP:    25.0,
				MaxConcurrency: 2,
				Description:    "高分辨率(25-50MP) - 最多2并发",
			},
			// 中高分辨率：10-25MP（如4K图片）
			{
				ThresholdMP:    10.0,
				MaxConcurrency: 4,
				Description:    "中高分辨率(10-25MP) - 最多4并发",
			},
			// 标准分辨率：5-10MP
			{
				ThresholdMP:    5.0,
				MaxConcurrency: 8,
				Description:    "标准分辨率(5-10MP) - 最多8并发",
			},
			// 低分辨率：<5MP
			{
				ThresholdMP:    0.0,
				MaxConcurrency: 16,
				Description:    "低分辨率(<5MP) - 最多16并发",
			},
		},
	}
}

// OptimizeWorkerCount 基于图片分辨率优化工作线程数
//
// 参数:
//   - imagePath: 图片文件路径
//   - baseWorkers: 基础工作线程数
//
// 返回:
//   - int: 优化后的工作线程数
//   - error: 如果无法获取图片分辨率
func (ro *RAMOptimizer) OptimizeWorkerCount(imagePath string, baseWorkers int) (int, error) {
	// 获取图片分辨率(百万像素)
	resMp, err := ro.getImageResolutionMP(imagePath)
	if err != nil {
		ro.logger.Warn("无法获取图片分辨率，使用基础并发数",
			zap.String("image", imagePath),
			zap.Error(err))
		return baseWorkers, nil // 失败时使用基础值
	}

	// 应用分辨率规则
	optimizedWorkers := ro.applyResolutionRules(resMp, baseWorkers)

	if optimizedWorkers != baseWorkers {
		ro.logger.Info("基于分辨率优化并发数",
			zap.Float64("resolution_mp", resMp),
			zap.Int("base_workers", baseWorkers),
			zap.Int("optimized_workers", optimizedWorkers),
			zap.String("rule", ro.getAppliedRule(resMp)))
	}

	return optimizedWorkers, nil
}

// applyResolutionRules 应用分辨率规则
func (ro *RAMOptimizer) applyResolutionRules(resMp float64, baseWorkers int) int {
	for _, rule := range ro.rules {
		if resMp >= rule.ThresholdMP {
			// 取基础并发数和规则上限的较小值
			if baseWorkers > rule.MaxConcurrency {
				return rule.MaxConcurrency
			}
			return baseWorkers
		}
	}

	// 默认返回基础值
	return baseWorkers
}

// getAppliedRule 获取应用的规则说明
func (ro *RAMOptimizer) getAppliedRule(resMp float64) string {
	for _, rule := range ro.rules {
		if resMp >= rule.ThresholdMP {
			return rule.Description
		}
	}
	return "默认规则"
}

// getImageResolutionMP 获取图片分辨率(百万像素)
func (ro *RAMOptimizer) getImageResolutionMP(imagePath string) (float64, error) {
	// 尝试多个工具获取分辨率

	// 1. 尝试ImageMagick identify
	if resMp, err := ro.getResolutionWithIdentify(imagePath); err == nil {
		return resMp, nil
	}

	// 2. 尝试exiftool
	if resMp, err := ro.getResolutionWithExiftool(imagePath); err == nil {
		return resMp, nil
	}

	// 3. 尝试ffprobe（视频/动图）
	if resMp, err := ro.getResolutionWithFFprobe(imagePath); err == nil {
		return resMp, nil
	}

	return 0, fmt.Errorf("无法获取图片分辨率，所有工具都失败")
}

// getResolutionWithIdentify 使用ImageMagick identify获取分辨率
func (ro *RAMOptimizer) getResolutionWithIdentify(imagePath string) (float64, error) {
	// identify -format "%w %h" image.jpg
	cmd := exec.Command("identify", "-format", "%w %h", imagePath)
	output, err := cmd.Output()
	if err != nil {
		return 0, err
	}

	return ro.parseResolution(string(output))
}

// getResolutionWithExiftool 使用exiftool获取分辨率
func (ro *RAMOptimizer) getResolutionWithExiftool(imagePath string) (float64, error) {
	// exiftool -ImageWidth -ImageHeight -s3 image.jpg
	cmd := exec.Command("exiftool", "-ImageWidth", "-ImageHeight", "-s3", imagePath)
	output, err := cmd.Output()
	if err != nil {
		return 0, err
	}

	return ro.parseResolution(string(output))
}

// getResolutionWithFFprobe 使用ffprobe获取分辨率（视频/动图）
func (ro *RAMOptimizer) getResolutionWithFFprobe(imagePath string) (float64, error) {
	// ffprobe -v error -select_streams v:0 -show_entries stream=width,height -of csv=s=x:p=0
	cmd := exec.Command("ffprobe",
		"-v", "error",
		"-select_streams", "v:0",
		"-show_entries", "stream=width,height",
		"-of", "csv=s=x:p=0",
		imagePath)
	output, err := cmd.Output()
	if err != nil {
		return 0, err
	}

	return ro.parseResolutionFromFFprobe(string(output))
}

// parseResolution 解析分辨率字符串（格式: "width height"）
func (ro *RAMOptimizer) parseResolution(resStr string) (float64, error) {
	parts := strings.Fields(strings.TrimSpace(resStr))
	if len(parts) < 2 {
		return 0, fmt.Errorf("无效的分辨率格式: %s", resStr)
	}

	width, err := strconv.ParseFloat(parts[0], 64)
	if err != nil {
		return 0, fmt.Errorf("无法解析宽度: %v", err)
	}

	height, err := strconv.ParseFloat(parts[1], 64)
	if err != nil {
		return 0, fmt.Errorf("无法解析高度: %v", err)
	}

	// 计算百万像素
	megapixels := (width * height) / 1_000_000.0

	ro.logger.Debug("解析图片分辨率",
		zap.Float64("width", width),
		zap.Float64("height", height),
		zap.Float64("megapixels", megapixels))

	return megapixels, nil
}

// parseResolutionFromFFprobe 解析ffprobe输出（格式: "widthxheight"）
func (ro *RAMOptimizer) parseResolutionFromFFprobe(resStr string) (float64, error) {
	parts := strings.Split(strings.TrimSpace(resStr), "x")
	if len(parts) < 2 {
		return 0, fmt.Errorf("无效的分辨率格式: %s", resStr)
	}

	width, err := strconv.ParseFloat(parts[0], 64)
	if err != nil {
		return 0, fmt.Errorf("无法解析宽度: %v", err)
	}

	height, err := strconv.ParseFloat(parts[1], 64)
	if err != nil {
		return 0, fmt.Errorf("无法解析高度: %v", err)
	}

	megapixels := (width * height) / 1_000_000.0

	ro.logger.Debug("解析图片分辨率(ffprobe)",
		zap.Float64("width", width),
		zap.Float64("height", height),
		zap.Float64("megapixels", megapixels))

	return megapixels, nil
}

// GetRecommendedWorkerCount 获取推荐的工作线程数（供外部使用）
func (ro *RAMOptimizer) GetRecommendedWorkerCount(imagePath string) (int, float64, error) {
	resMp, err := ro.getImageResolutionMP(imagePath)
	if err != nil {
		return 0, 0, err
	}

	// 应用规则
	for _, rule := range ro.rules {
		if resMp >= rule.ThresholdMP {
			return rule.MaxConcurrency, resMp, nil
		}
	}

	// 默认返回最大并发数
	return ro.rules[len(ro.rules)-1].MaxConcurrency, resMp, nil
}

// EstimateMemoryUsage 估算图片处理内存使用（MB）
// 粗略估算：width * height * 4（RGBA） * 3（解码+处理+编码）/ 1024 / 1024
func (ro *RAMOptimizer) EstimateMemoryUsage(imagePath string) (float64, error) {
	resMp, err := ro.getImageResolutionMP(imagePath)
	if err != nil {
		return 0, err
	}

	// 估算公式：MP * 4 bytes(RGBA) * 3(buffers) = MB
	estimatedMB := resMp * 4.0 * 3.0

	ro.logger.Debug("估算内存使用",
		zap.Float64("resolution_mp", resMp),
		zap.Float64("estimated_mb", estimatedMB))

	return estimatedMB, nil
}
