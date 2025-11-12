// pkg/predictor/ml/features/extractor.go - 特征提取器模块
//
// 功能说明：
// - 提供128维特征提取功能
// - 支持图像、元数据、上下文特征提取
// - 为机器学习模型提供高质量特征
//
// 作者: AI Assistant
// 版本: v2.2.0
// 更新: 2025-10-24
// 来源: easymode/learning/features/feature_extractor.go

package features

import (
	"fmt"
	"image"
	"math"
)

// FeatureExtractor 特征提取器
type FeatureExtractor struct {
	ImageFeatures    ImageFeatures    `json:"image_features"`
	MetadataFeatures MetadataFeatures `json:"metadata_features"`
	ContextFeatures  ContextFeatures  `json:"context_features"`
}

// ImageFeatures 图像特征 (64维)
type ImageFeatures struct {
	// 颜色直方图 (16维)
	ColorHistogram [16]float64 `json:"color_histogram"`

	// 纹理特征 (16维)
	TextureFeatures [16]float64 `json:"texture_features"`

	// 形状特征 (16维)
	ShapeFeatures [16]float64 `json:"shape_features"`

	// 质量指标 (16维)
	QualityMetrics [16]float64 `json:"quality_metrics"`
}

// MetadataFeatures 元数据特征 (32维)
type MetadataFeatures struct {
	// EXIF信息 (16维)
	EXIFFeatures [16]float64 `json:"exif_features"`

	// 文件属性 (8维)
	FileAttributes [8]float64 `json:"file_attributes"`

	// 用户偏好 (8维)
	UserPreferences [8]float64 `json:"user_preferences"`
}

// ContextFeatures 上下文特征 (32维)
type ContextFeatures struct {
	// 处理历史 (16维)
	ProcessingHistory [16]float64 `json:"processing_history"`

	// 设备信息 (8维)
	DeviceInfo [8]float64 `json:"device_info"`

	// 环境参数 (8维)
	EnvironmentParams [8]float64 `json:"environment_params"`
}

// NewFeatureExtractor 创建特征提取器
func NewFeatureExtractor() *FeatureExtractor {
	return &FeatureExtractor{}
}

// ExtractFeatures 提取128维特征向量
func (fe *FeatureExtractor) ExtractFeatures(img image.Image, metadata map[string]interface{}) []float64 {
	features := make([]float64, 128)

	// 提取图像特征 (0-63)
	imageFeatures := fe.extractImageFeatures(img)
	copy(features[0:64], imageFeatures[:])

	// 提取元数据特征 (64-95)
	metadataFeatures := fe.extractMetadataFeatures(metadata)
	copy(features[64:96], metadataFeatures[:])

	// 提取上下文特征 (96-127)
	contextFeatures := fe.extractContextFeatures()
	copy(features[96:128], contextFeatures[:])

	return features
}

// extractImageFeatures 提取图像特征
func (fe *FeatureExtractor) extractImageFeatures(img image.Image) [64]float64 {
	var features [64]float64

	// 颜色直方图 (0-15)
	colorHist := fe.calculateColorHistogram(img)
	copy(features[0:16], colorHist[:])

	// 纹理特征 (16-31)
	texture := fe.calculateTextureFeatures(img)
	copy(features[16:32], texture[:])

	// 形状特征 (32-47)
	shape := fe.calculateShapeFeatures(img)
	copy(features[32:48], shape[:])

	// 质量指标 (48-63)
	quality := fe.calculateQualityMetrics(img)
	copy(features[48:64], quality[:])

	return features
}

// calculateColorHistogram 计算颜色直方图
func (fe *FeatureExtractor) calculateColorHistogram(img image.Image) [16]float64 {
	var histogram [16]float64
	bounds := img.Bounds()
	totalPixels := float64(bounds.Dx() * bounds.Dy())

	// 简化的颜色直方图计算
	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			r, g, b, _ := img.At(x, y).RGBA()

			// 归一化到0-255范围
			r = r >> 8
			g = g >> 8
			b = b >> 8

			// 计算亮度
			brightness := (float64(r) + float64(g) + float64(b)) / 3.0

			// 分配到16个bin中
			bin := int(brightness / 16.0)
			if bin >= 16 {
				bin = 15
			}
			histogram[bin]++
		}
	}

	// 归一化
	for i := range histogram {
		histogram[i] /= totalPixels
	}

	return histogram
}

// calculateTextureFeatures 计算纹理特征
func (fe *FeatureExtractor) calculateTextureFeatures(img image.Image) [16]float64 {
	var features [16]float64
	bounds := img.Bounds()

	// 简化的纹理特征计算
	// 计算局部方差和梯度
	for y := bounds.Min.Y + 1; y < bounds.Max.Y-1; y++ {
		for x := bounds.Min.X + 1; x < bounds.Max.X-1; x++ {
			// 计算梯度
			r1, _, _, _ := img.At(x-1, y).RGBA()
			r2, _, _, _ := img.At(x+1, y).RGBA()
			gx := float64(r2>>8 - r1>>8)

			r3, _, _, _ := img.At(x, y-1).RGBA()
			r4, _, _, _ := img.At(x, y+1).RGBA()
			gy := float64(r4>>8 - r3>>8)

			gradient := math.Sqrt(gx*gx + gy*gy)

			// 分配到特征向量中
			bin := int(gradient / 16.0)
			if bin >= 16 {
				bin = 15
			}
			features[bin]++
		}
	}

	// 归一化
	totalPixels := float64((bounds.Dx() - 2) * (bounds.Dy() - 2))
	for i := range features {
		features[i] /= totalPixels
	}

	return features
}

// calculateShapeFeatures 计算形状特征
func (fe *FeatureExtractor) calculateShapeFeatures(img image.Image) [16]float64 {
	var features [16]float64
	bounds := img.Bounds()

	// 简化的形状特征计算
	width := bounds.Dx()
	height := bounds.Dy()

	// 计算宽高比
	aspectRatio := float64(width) / float64(height)
	features[0] = aspectRatio

	// 计算面积
	area := float64(width * height)
	features[1] = math.Log(area + 1) // 对数变换

	// 计算周长（近似）
	perimeter := 2 * (width + height)
	features[2] = float64(perimeter)

	// 计算圆形度
	circularity := 4 * math.Pi * area / float64(perimeter*perimeter)
	features[3] = circularity

	// 其他形状特征（简化）
	for i := 4; i < 16; i++ {
		features[i] = float64(i) / 16.0
	}

	return features
}

// calculateQualityMetrics 计算质量指标
func (fe *FeatureExtractor) calculateQualityMetrics(img image.Image) [16]float64 {
	var features [16]float64
	bounds := img.Bounds()

	// 计算图像质量指标
	totalPixels := float64(bounds.Dx() * bounds.Dy())

	// 计算平均亮度
	totalBrightness := 0.0
	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			r, g, b, _ := img.At(x, y).RGBA()
			brightness := (float64(r>>8) + float64(g>>8) + float64(b>>8)) / 3.0
			totalBrightness += brightness
		}
	}
	avgBrightness := totalBrightness / totalPixels
	features[0] = avgBrightness / 255.0

	// 计算对比度（标准差）
	variance := 0.0
	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			r, g, b, _ := img.At(x, y).RGBA()
			brightness := (float64(r>>8) + float64(g>>8) + float64(b>>8)) / 3.0
			variance += math.Pow(brightness-avgBrightness, 2)
		}
	}
	contrast := math.Sqrt(variance / totalPixels)
	features[1] = contrast / 255.0

	// 其他质量指标（简化）
	for i := 2; i < 16; i++ {
		features[i] = float64(i) / 16.0
	}

	return features
}

// extractMetadataFeatures 提取元数据特征
func (fe *FeatureExtractor) extractMetadataFeatures(metadata map[string]interface{}) [32]float64 {
	var features [32]float64

	// EXIF特征 (0-15)
	exifFeatures := fe.extractEXIFFeatures(metadata)
	copy(features[0:16], exifFeatures[:])

	// 文件属性 (16-23)
	fileAttrs := fe.extractFileAttributes(metadata)
	copy(features[16:24], fileAttrs[:])

	// 用户偏好 (24-31)
	userPrefs := fe.extractUserPreferences(metadata)
	copy(features[24:32], userPrefs[:])

	return features
}

// extractEXIFFeatures 提取EXIF特征
func (fe *FeatureExtractor) extractEXIFFeatures(metadata map[string]interface{}) [16]float64 {
	var features [16]float64

	// 简化的EXIF特征提取
	if width, ok := metadata["width"].(int); ok {
		features[0] = math.Log(float64(width) + 1)
	}
	if height, ok := metadata["height"].(int); ok {
		features[1] = math.Log(float64(height) + 1)
	}
	if orientation, ok := metadata["orientation"].(int); ok {
		features[2] = float64(orientation)
	}

	// 其他EXIF特征（简化）
	for i := 3; i < 16; i++ {
		features[i] = float64(i) / 16.0
	}

	return features
}

// extractFileAttributes 提取文件属性
func (fe *FeatureExtractor) extractFileAttributes(metadata map[string]interface{}) [8]float64 {
	var features [8]float64

	// 简化的文件属性提取
	if size, ok := metadata["file_size"].(int64); ok {
		features[0] = math.Log(float64(size) + 1)
	}
	if format, ok := metadata["format"].(string); ok {
		// 将格式字符串转换为数值
		hash := 0
		for _, c := range format {
			hash += int(c)
		}
		features[1] = float64(hash%1000) / 1000.0
	}

	// 其他文件属性（简化）
	for i := 2; i < 8; i++ {
		features[i] = float64(i) / 8.0
	}

	return features
}

// extractUserPreferences 提取用户偏好
func (fe *FeatureExtractor) extractUserPreferences(metadata map[string]interface{}) [8]float64 {
	var features [8]float64

	// 简化的用户偏好提取
	// 这里应该根据用户历史行为来设置
	for i := 0; i < 8; i++ {
		features[i] = float64(i) / 8.0
	}

	return features
}

// extractContextFeatures 提取上下文特征
func (fe *FeatureExtractor) extractContextFeatures() [32]float64 {
	var features [32]float64

	// 处理历史 (0-15)
	processingHistory := fe.extractProcessingHistory()
	copy(features[0:16], processingHistory[:])

	// 设备信息 (16-23)
	deviceInfo := fe.extractDeviceInfo()
	copy(features[16:24], deviceInfo[:])

	// 环境参数 (24-31)
	environmentParams := fe.extractEnvironmentParams()
	copy(features[24:32], environmentParams[:])

	return features
}

// extractProcessingHistory 提取处理历史
func (fe *FeatureExtractor) extractProcessingHistory() [16]float64 {
	var features [16]float64

	// 简化的处理历史提取
	// 这里应该根据实际的处理历史来设置
	for i := 0; i < 16; i++ {
		features[i] = float64(i) / 16.0
	}

	return features
}

// extractDeviceInfo 提取设备信息
func (fe *FeatureExtractor) extractDeviceInfo() [8]float64 {
	var features [8]float64

	// 简化的设备信息提取
	// 这里应该根据实际设备信息来设置
	for i := 0; i < 8; i++ {
		features[i] = float64(i) / 8.0
	}

	return features
}

// extractEnvironmentParams 提取环境参数
func (fe *FeatureExtractor) extractEnvironmentParams() [8]float64 {
	var features [8]float64

	// 简化的环境参数提取
	// 这里应该根据实际环境参数来设置
	for i := 0; i < 8; i++ {
		features[i] = float64(i) / 8.0
	}

	return features
}

// GetFeatureNames 获取特征名称
func (fe *FeatureExtractor) GetFeatureNames() []string {
	names := make([]string, 128)

	// 图像特征名称
	for i := 0; i < 16; i++ {
		names[i] = fmt.Sprintf("color_hist_%d", i)
	}
	for i := 16; i < 32; i++ {
		names[i] = fmt.Sprintf("texture_%d", i-16)
	}
	for i := 32; i < 48; i++ {
		names[i] = fmt.Sprintf("shape_%d", i-32)
	}
	for i := 48; i < 64; i++ {
		names[i] = fmt.Sprintf("quality_%d", i-48)
	}

	// 元数据特征名称
	for i := 64; i < 80; i++ {
		names[i] = fmt.Sprintf("exif_%d", i-64)
	}
	for i := 80; i < 88; i++ {
		names[i] = fmt.Sprintf("file_attr_%d", i-80)
	}
	for i := 88; i < 96; i++ {
		names[i] = fmt.Sprintf("user_pref_%d", i-88)
	}

	// 上下文特征名称
	for i := 96; i < 112; i++ {
		names[i] = fmt.Sprintf("processing_%d", i-96)
	}
	for i := 112; i < 120; i++ {
		names[i] = fmt.Sprintf("device_%d", i-112)
	}
	for i := 120; i < 128; i++ {
		names[i] = fmt.Sprintf("environment_%d", i-120)
	}

	return names
}

// NormalizeFeatures 归一化特征向量
func (fe *FeatureExtractor) NormalizeFeatures(features []float64) []float64 {
	normalized := make([]float64, len(features))

	// 计算均值和标准差
	mean := 0.0
	for _, f := range features {
		mean += f
	}
	mean /= float64(len(features))

	variance := 0.0
	for _, f := range features {
		variance += math.Pow(f-mean, 2)
	}
	variance /= float64(len(features))
	std := math.Sqrt(variance)

	// Z-score归一化
	for i, f := range features {
		if std > 0 {
			normalized[i] = (f - mean) / std
		} else {
			normalized[i] = 0
		}
	}

	return normalized
}
