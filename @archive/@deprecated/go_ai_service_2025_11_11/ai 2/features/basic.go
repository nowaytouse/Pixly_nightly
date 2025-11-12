package features

import (
	"fmt"
	"image"
	"image/gif"
	_ "image/jpeg"
	_ "image/png"
	"os"
	"path/filepath"
	"strings"
)

// ImageFeatures 图像特征结构（本地定义，避免循环导入）
type ImageFeatures struct {
	// 基础特征
	Width       int     `json:"width"`
	Height      int     `json:"height"`
	Size        int64   `json:"size"`
	Format      string  `json:"format"`
	HasAlpha    bool    `json:"has_alpha"`
	IsAnimated  bool    `json:"is_animated"`
	FrameCount  int     `json:"frame_count,omitempty"`
	PixelCount  int64   `json:"pixel_count"`
	AspectRatio float64 `json:"aspect_ratio"`

	// SWT 小波特征
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

// BasicExtractor 基础特征提取器
// 提取图像的基本属性（宽、高、大小、格式等）
type BasicExtractor struct {
	debug bool
}

// NewBasicExtractor 创建基础特征提取器
func NewBasicExtractor(debug bool) *BasicExtractor {
	return &BasicExtractor{
		debug: debug,
	}
}

// Extract 提取图像的基础特征
func (e *BasicExtractor) Extract(imagePath string) (*ImageFeatures, error) {
	// 获取文件信息
	fileInfo, err := os.Stat(imagePath)
	if err != nil {
		return nil, fmt.Errorf("无法获取文件信息: %w", err)
	}

	// 打开图像文件
	file, err := os.Open(imagePath)
	if err != nil {
		return nil, fmt.Errorf("无法打开图像: %w", err)
	}
	defer file.Close()

	// 解码图像配置（不解码完整图像，节省内存）
	config, format, err := image.DecodeConfig(file)
	if err != nil {
		return nil, fmt.Errorf("无法解码图像配置: %w", err)
	}

	// 计算像素数
	pixelCount := int64(config.Width) * int64(config.Height)

	// 计算宽高比
	aspectRatio := float64(config.Width) / float64(config.Height)

	// 检测是否有透明通道
	hasAlpha := e.detectAlpha(format)

	// 创建特征结构
	features := &ImageFeatures{
		Width:       config.Width,
		Height:      config.Height,
		Size:        fileInfo.Size(),
		Format:      e.normalizeFormat(format),
		HasAlpha:    hasAlpha,
		IsAnimated:  e.isAnimatedFormat(imagePath),
		PixelCount:  pixelCount,
		AspectRatio: aspectRatio,
	}

	if e.debug {
		fmt.Printf("[BasicExtractor] 提取特征: %dx%d, 格式=%s, 大小=%d bytes\n",
			features.Width, features.Height, features.Format, features.Size)
	}

	return features, nil
}

// detectAlpha 检测图像是否有透明通道
func (e *BasicExtractor) detectAlpha(format string) bool {
	// PNG 和 GIF 可能有透明通道
	format = strings.ToLower(format)
	if format == "png" || format == "gif" {
		return true
	}

	// WebP 也可能有透明通道
	if format == "webp" {
		return true
	}

	return false
}

// normalizeFormat 标准化格式名称
func (e *BasicExtractor) normalizeFormat(format string) string {
	format = strings.ToLower(format)
	switch format {
	case "jpeg":
		return "jpg"
	default:
		return format
	}
}

// isAnimatedFormat 检测是否是动画格式
func (e *BasicExtractor) isAnimatedFormat(imagePath string) bool {
	ext := strings.ToLower(filepath.Ext(imagePath))

	// GIF 动画检测
	if ext == ".gif" {
		return e.isAnimatedGIF(imagePath)
	}

	// WebP 和 AVIF 也可能是动画（暂时返回false，需要进一步实现）
	if ext == ".webp" || ext == ".avif" {
		return false
	}

	return false
}

// isAnimatedGIF 检测GIF是否为动画（多帧）
func (e *BasicExtractor) isAnimatedGIF(imagePath string) bool {
	file, err := os.Open(imagePath)
	if err != nil {
		return false
	}
	defer file.Close()

	// 解码GIF
	gifImg, err := gif.DecodeAll(file)
	if err != nil {
		return false
	}

	// 如果有多于1帧，则为动画
	return len(gifImg.Image) > 1
}

// ExtractBatch 批量提取特征
func (e *BasicExtractor) ExtractBatch(imagePaths []string) ([]*ImageFeatures, error) {
	features := make([]*ImageFeatures, 0, len(imagePaths))

	for i, path := range imagePaths {
		feat, err := e.Extract(path)
		if err != nil {
			if e.debug {
				fmt.Printf("[BasicExtractor] 跳过 %s: %v\n", path, err)
			}
			continue
		}
		features = append(features, feat)

		if e.debug && (i+1)%10 == 0 {
			fmt.Printf("[BasicExtractor] 已处理 %d/%d 张图像\n", i+1, len(imagePaths))
		}
	}

	return features, nil
}
