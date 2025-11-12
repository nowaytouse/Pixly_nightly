// 🎯 Pixly统一常量配置 (Go)
//
// Phase 46.8: 三端共享的常量定义
// 确保Go/Rust/JS使用相同的参数范围和阈值

package ai

import "fmt"

// ParamRanges 参数范围常量
type ParamRanges struct{}

var (
	// Quality参数范围 (1-100)
	QualityMin     = 1
	QualityMax     = 100
	QualityDefault = 85

	// Speed/Effort参数范围 (0-10)
	SpeedMin     = 0
	SpeedMax     = 10
	SpeedDefault = 6

	// AVIF Quantizer范围 (0-63)
	AVIFQuantizerMin = 0
	AVIFQuantizerMax = 63

	// JXL Distance范围 (0.0-15.0)
	JXLDistanceMin = 0.0
	JXLDistanceMax = 15.0

	// WebP Method范围 (0-6)
	WebPMethodMin = 0
	WebPMethodMax = 6

	// FFmpeg CRF范围 (0-51)
	FFmpegCRFMin = 0
	FFmpegCRFMax = 51

	// 图像尺寸范围
	ImageDimMin = 1
	ImageDimMax = 65535

	// 图像总像素数限制（10亿像素）
	ImageMaxPixels uint64 = 1_000_000_000

	// 文件大小限制 (100MB)
	FileSizeMax uint64 = 100 * 1024 * 1024
)

// AIConstants AI相关常量
type AIConstants struct{}

var (
	// AI置信度阈值
	AIConfidenceRejectThreshold = 0.5 // 低于此值拒绝
	AIConfidenceWarnThreshold   = 0.7 // 低于此值警告

	// AI服务超时（秒）
	AITimeoutSeconds = 30

	// AI最大重试次数
	AIMaxRetries = 3
)

// SupportedFormats 支持的格式列表
var (
	// 输入格式
	SupportedInputFormats = []string{
		"jpg", "jpeg", "png", "webp", "gif", "bmp",
		"tiff", "tif", "avif", "jxl", "heic", "heif",
	}

	// 输出格式
	SupportedOutputFormats = []string{
		"jpg", "jpeg", "png", "webp", "avif", "jxl", "gif", "heic",
	}

	// 支持无损模式的格式
	LosslessFormats = []string{
		"png", "webp", "avif", "jxl",
	}
)

// SupportedTools 工具名称常量
var (
	ToolCJXL    = "cjxl"
	ToolDJXL    = "djxl"
	ToolAVIFENC = "avifenc"
	ToolAVIFDEC = "avifdec"
	ToolCWEBP   = "cwebp"
	ToolDWEBP   = "dwebp"
	ToolFFMPEG  = "ffmpeg"
	ToolMAGICK  = "magick"

	// 所有支持的工具
	AllTools = []string{
		ToolCJXL, ToolDJXL, ToolAVIFENC, ToolAVIFDEC,
		ToolCWEBP, ToolDWEBP, ToolFFMPEG, ToolMAGICK,
	}
)

// ValidationMessages 验证消息常量
var (
	MsgQualityOutOfRange  = "Quality参数超出范围"
	MsgSpeedOutOfRange    = "Speed参数超出范围"
	MsgImageTooLarge      = "图像尺寸超限"
	MsgFileNotFound       = "文件不存在"
	MsgFormatUnsupported  = "格式不支持"
	MsgAIConfidenceLow    = "AI置信度过低"
)

// PerformanceConstants 性能相关常量
var (
	DefaultConcurrency = 4
	MaxConcurrency     = 16
	ProcessingTimeoutSeconds = 300
)

// ValidateQuality 验证quality参数
func ValidateQuality(value int) error {
	if value < QualityMin || value > QualityMax {
		return NewValidationError(
			ErrValInvalidRange,
			formatRangeError(MsgQualityOutOfRange, value, QualityMin, QualityMax),
		)
	}
	return nil
}

// ValidateSpeed 验证speed参数
func ValidateSpeed(value int) error {
	if value < SpeedMin || value > SpeedMax {
		return NewValidationError(
			ErrValInvalidRange,
			formatRangeError(MsgSpeedOutOfRange, value, SpeedMin, SpeedMax),
		)
	}
	return nil
}

// ValidateImageDimensions 验证图像尺寸
func ValidateImageDimensions(width, height int) error {
	if width < ImageDimMin || width > ImageDimMax ||
		height < ImageDimMin || height > ImageDimMax {
		return NewValidationError(
			ErrValInvalidRange,
			formatDimensionError(width, height),
		)
	}

	totalPixels := uint64(width) * uint64(height)
	if totalPixels > ImageMaxPixels {
		return NewValidationError(
			ErrValInvalidRange,
			formatPixelError(totalPixels),
		)
	}

	return nil
}

// IsInputFormatSupported 检查输入格式是否支持
func IsInputFormatSupported(format string) bool {
	return containsString(SupportedInputFormats, format)
}

// IsOutputFormatSupported 检查输出格式是否支持
func IsOutputFormatSupported(format string) bool {
	return containsString(SupportedOutputFormats, format)
}

// SupportsLossless 检查格式是否支持无损模式
func SupportsLossless(format string) bool {
	return containsString(LosslessFormats, format)
}

// IsValidTool 检查工具名称是否有效
func IsValidTool(tool string) bool {
	return containsString(AllTools, tool)
}

// 辅助函数
func containsString(slice []string, item string) bool {
	for _, s := range slice {
		if s == item {
			return true
		}
	}
	return false
}

func formatRangeError(msg string, value, min, max int) string {
	return fmt.Sprintf("%s: %d (应为%d-%d)", msg, value, min, max)
}

func formatDimensionError(width, height int) string {
	return fmt.Sprintf("%s: %dx%d (应为%d-%d)", MsgImageTooLarge, width, height, ImageDimMin, ImageDimMax)
}

func formatPixelError(pixels uint64) string {
	return fmt.Sprintf("%s: 总像素 %d (最大: %d)", MsgImageTooLarge, pixels, ImageMaxPixels)
}
