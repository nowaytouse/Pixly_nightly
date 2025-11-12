// Package ml 提供机器学习辅助的质量优化
package ml

import (
	"fmt"
	"image"
	_ "image/gif"
	_ "image/jpeg"
	_ "image/png"
	"math"
	"os"
	
	"go.uber.org/zap"
)

// VisualQualityScorer 视觉质量评分器
type VisualQualityScorer struct {
	logger *zap.Logger
}

// NewVisualQualityScorer 创建视觉质量评分器
func NewVisualQualityScorer(logger *zap.Logger) *VisualQualityScorer {
	return &VisualQualityScorer{
		logger: logger,
	}
}

// ImageFeatures 图像特征
type ImageFeatures struct {
	// 基础特征
	Width      int     `json:"width"`
	Height     int     `json:"height"`
	Pixels     int     `json:"pixels"`
	AspectRatio float64 `json:"aspect_ratio"`
	
	// 色彩特征
	ColorComplexity  float64 `json:"color_complexity"`   // 色彩复杂度 (0-1)
	Brightness       float64 `json:"brightness"`         // 平均亮度 (0-255)
	Contrast         float64 `json:"contrast"`           // 对比度 (0-1)
	Saturation       float64 `json:"saturation"`         // 饱和度 (0-1)
	
	// 纹理特征
	EdgeDensity      float64 `json:"edge_density"`       // 边缘密度 (0-1)
	TextureComplexity float64 `json:"texture_complexity"` // 纹理复杂度 (0-1)
	NoiseLevel       float64 `json:"noise_level"`        // 噪声水平 (0-1)
	
	// 内容特征
	IsPhoto          bool    `json:"is_photo"`           // 是否为照片
	IsGraphic        bool    `json:"is_graphic"`         // 是否为图形/设计稿
	IsScreenshot     bool    `json:"is_screenshot"`      // 是否为截图
	HasTransparency  bool    `json:"has_transparency"`   // 是否有透明度
	
	// 质量估计
	EstimatedQuality int     `json:"estimated_quality"`  // 估计的原始质量 (0-100)
	RecommendedQuality int   `json:"recommended_quality"` // 推荐的转换质量 (0-100)
}

// QualityRecommendation 质量推荐
type QualityRecommendation struct {
	Quality    int     `json:"quality"`     // 推荐质量 (0-100)
	Effort     int     `json:"effort"`      // 推荐努力 (1-9)
	Lossless   bool    `json:"lossless"`    // 是否推荐无损
	Confidence float64 `json:"confidence"`  // 推荐置信度 (0-1)
	Reasoning  string  `json:"reasoning"`   // 推荐理由
}

// AnalyzeImage 分析图像并提取特征
func (vqs *VisualQualityScorer) AnalyzeImage(filePath string) (*ImageFeatures, error) {
	// 打开图像文件
	file, err := os.Open(filePath)
	if err != nil {
		return nil, fmt.Errorf("无法打开图像: %w", err)
	}
	defer file.Close()
	
	// 解码图像
	img, format, err := image.Decode(file)
	if err != nil {
		return nil, fmt.Errorf("无法解码图像: %w", err)
	}
	
	vqs.logger.Debug("图像解码成功",
		zap.String("format", format),
		zap.Int("width", img.Bounds().Dx()),
		zap.Int("height", img.Bounds().Dy()))
	
	// 提取特征
	features := &ImageFeatures{
		Width:  img.Bounds().Dx(),
		Height: img.Bounds().Dy(),
	}
	features.Pixels = features.Width * features.Height
	features.AspectRatio = float64(features.Width) / float64(features.Height)
	
	// 分析像素数据
	vqs.analyzePixels(img, features)
	
	// 内容分类
	vqs.classifyContent(features)
	
	// 估计质量和推荐
	vqs.estimateQuality(features)
	
	return features, nil
}

// analyzePixels 分析像素数据
func (vqs *VisualQualityScorer) analyzePixels(img image.Image, features *ImageFeatures) {
	bounds := img.Bounds()
	width, height := bounds.Dx(), bounds.Dy()
	
	// 采样像素（为了性能，不分析所有像素）
	sampleStep := 1
	if width*height > 1_000_000 { // 超过 1MP，采样
		sampleStep = int(math.Sqrt(float64(width*height) / 1_000_000))
	}
	
	var (
		totalR, totalG, totalB float64
		minBrightness = 255.0
		maxBrightness = 0.0
		edgeCount     = 0
		sampleCount   = 0
		hasAlpha      = false
	)
	
	// 遍历采样像素
	for y := bounds.Min.Y; y < bounds.Max.Y; y += sampleStep {
		for x := bounds.Min.X; x < bounds.Max.X; x += sampleStep {
			r, g, b, a := img.At(x, y).RGBA()
			
			// 转换为 0-255
			rr, gg, bb := float64(r>>8), float64(g>>8), float64(b>>8)
			
			// 累加RGB
			totalR += rr
			totalG += gg
			totalB += bb
			
			// 计算亮度
			brightness := 0.299*rr + 0.587*gg + 0.114*bb
			if brightness < minBrightness {
				minBrightness = brightness
			}
			if brightness > maxBrightness {
				maxBrightness = brightness
			}
			
			// 检查透明度
			if a < 65535 {
				hasAlpha = true
			}
			
			// 简单的边缘检测（检查与右侧和下侧像素的差异）
			if x < bounds.Max.X-sampleStep && y < bounds.Max.Y-sampleStep {
				r2, g2, b2, _ := img.At(x+sampleStep, y).RGBA()
				r3, g3, b3, _ := img.At(x, y+sampleStep).RGBA()
				
				diffRight := math.Abs(float64(r-r2)) + math.Abs(float64(g-g2)) + math.Abs(float64(b-b2))
				diffDown := math.Abs(float64(r-r3)) + math.Abs(float64(g-g3)) + math.Abs(float64(b-b3))
				
				// 如果差异超过阈值，认为是边缘
				threshold := 20000.0
				if diffRight > threshold || diffDown > threshold {
					edgeCount++
				}
			}
			
			sampleCount++
		}
	}
	
	// 计算特征
	if sampleCount > 0 {
		avgR := totalR / float64(sampleCount)
		avgG := totalG / float64(sampleCount)
		avgB := totalB / float64(sampleCount)
		
		// 平均亮度
		features.Brightness = 0.299*avgR + 0.587*avgG + 0.114*avgB
		
		// 对比度
		features.Contrast = (maxBrightness - minBrightness) / 255.0
		
		// 色彩饱和度（简化计算）
		maxChannel := math.Max(math.Max(avgR, avgG), avgB)
		minChannel := math.Min(math.Min(avgR, avgG), avgB)
		if maxChannel > 0 {
			features.Saturation = (maxChannel - minChannel) / maxChannel
		}
		
		// 边缘密度
		features.EdgeDensity = float64(edgeCount) / float64(sampleCount)
		
		// 纹理复杂度（基于边缘密度和对比度）
		features.TextureComplexity = (features.EdgeDensity + features.Contrast) / 2.0
		
		// 色彩复杂度（基于饱和度和对比度）
		features.ColorComplexity = (features.Saturation + features.Contrast) / 2.0
		
		// 噪声估计（基于高频边缘的比例）
		if features.EdgeDensity > 0.3 {
			features.NoiseLevel = (features.EdgeDensity - 0.3) / 0.7
		}
	}
	
	features.HasTransparency = hasAlpha
}

// classifyContent 内容分类
func (vqs *VisualQualityScorer) classifyContent(features *ImageFeatures) {
	// 照片特征：高色彩复杂度、中等纹理复杂度、较低噪声
	if features.ColorComplexity > 0.4 && features.TextureComplexity > 0.3 && 
	   features.TextureComplexity < 0.8 && features.NoiseLevel < 0.5 {
		features.IsPhoto = true
	}
	
	// 图形/设计稿特征：低噪声、高对比度、可能有透明度
	if features.NoiseLevel < 0.2 && features.Contrast > 0.5 {
		features.IsGraphic = true
	}
	
	// 截图特征：特定分辨率、可能有透明度、低噪声
	aspectRatio := features.AspectRatio
	isCommonScreenRatio := (aspectRatio > 1.33 && aspectRatio < 1.34) || // 4:3
		(aspectRatio > 1.77 && aspectRatio < 1.78) || // 16:9
		(aspectRatio > 1.59 && aspectRatio < 1.61)    // 16:10
	
	if isCommonScreenRatio && features.NoiseLevel < 0.1 {
		features.IsScreenshot = true
	}
}

// estimateQuality 估计质量
func (vqs *VisualQualityScorer) estimateQuality(features *ImageFeatures) {
	// 基于噪声和纹理复杂度估计原始质量
	qualityScore := 100.0
	
	// 噪声降低质量
	qualityScore -= features.NoiseLevel * 30
	
	// 低对比度可能表示低质量
	if features.Contrast < 0.3 {
		qualityScore -= (0.3 - features.Contrast) * 50
	}
	
	// 边缘密度过高可能表示压缩伪影
	if features.EdgeDensity > 0.7 {
		qualityScore -= (features.EdgeDensity - 0.7) * 30
	}
	
	features.EstimatedQuality = int(math.Max(0, math.Min(100, qualityScore)))
	
	// 推荐质量稍低于估计质量（避免过度压缩）
	if features.EstimatedQuality >= 95 {
		features.RecommendedQuality = features.EstimatedQuality // 高质量保持
	} else if features.EstimatedQuality >= 85 {
		features.RecommendedQuality = features.EstimatedQuality - 5
	} else {
		features.RecommendedQuality = features.EstimatedQuality - 10
	}
	
	features.RecommendedQuality = int(math.Max(60, math.Min(100, float64(features.RecommendedQuality))))
}

// RecommendQuality 推荐质量参数
func (vqs *VisualQualityScorer) RecommendQuality(features *ImageFeatures) *QualityRecommendation {
	rec := &QualityRecommendation{
		Quality:    features.RecommendedQuality,
		Effort:     7, // 默认努力
		Confidence: 0.8,
	}
	
	// 🎯 规则1: 高质量源 → 无损
	if features.EstimatedQuality >= 98 {
		rec.Lossless = true
		rec.Effort = 9
		rec.Confidence = 0.95
		rec.Reasoning = "检测到高质量源，推荐数学无损以保障质量"
		return rec
	}
	
	// 🎯 规则2: 设计稿/图形 + 高质量 → 无损
	if features.IsGraphic && features.EstimatedQuality >= 90 {
		rec.Lossless = true
		rec.Effort = 9
		rec.Confidence = 0.9
		rec.Reasoning = "检测到设计稿/图形，推荐无损以保留细节"
		return rec
	}
	
	// 🎯 规则3: 透明图像 + 高质量 → 无损
	if features.HasTransparency && features.EstimatedQuality >= 85 {
		rec.Lossless = true
		rec.Effort = 8
		rec.Confidence = 0.85
		rec.Reasoning = "检测到透明图像，推荐无损以保留透明度细节"
		return rec
	}
	
	// 🎯 规则4: 低噪点 + 高复杂度 → 可能是渲染图
	if features.NoiseLevel < 0.1 && features.TextureComplexity > 0.7 && features.EstimatedQuality >= 90 {
		rec.Lossless = true
		rec.Effort = 9
		rec.Confidence = 0.88
		rec.Reasoning = "检测到可能的渲染图/高质量设计，推荐无损"
		return rec
	}
	
	// 🎯 规则5: 照片 → 自适应质量
	if features.IsPhoto {
		rec.Lossless = false
		rec.Quality = features.RecommendedQuality
		rec.Effort = 7
		rec.Confidence = 0.8
		rec.Reasoning = fmt.Sprintf("检测到照片，推荐质量%d（基于图像分析）", rec.Quality)
		return rec
	}
	
	// 🎯 规则6: 截图 → 高质量有损
	if features.IsScreenshot {
		rec.Lossless = false
		rec.Quality = 90
		rec.Effort = 7
		rec.Confidence = 0.85
		rec.Reasoning = "检测到截图，推荐质量90以保持清晰度"
		return rec
	}
	
	// 默认：使用分析的推荐质量
	rec.Lossless = false
	rec.Reasoning = fmt.Sprintf("基于图像特征分析，推荐质量%d", rec.Quality)
	
	return rec
}

// OptimizeForFileSize 针对文件大小优化质量
func (vqs *VisualQualityScorer) OptimizeForFileSize(features *ImageFeatures, targetSizeMB float64) *QualityRecommendation {
	// 估计当前文件大小（粗略）
	currentSizeMB := float64(features.Pixels*3) / (1024 * 1024) // 未压缩RGB
	
	// 计算压缩目标
	compressionRatio := targetSizeMB / currentSizeMB
	
	rec := &QualityRecommendation{
		Effort:     7,
		Confidence: 0.7,
	}
	
	// 根据压缩比调整质量
	switch {
	case compressionRatio >= 0.8:
		// 轻度压缩
		rec.Quality = 90
		rec.Lossless = false
		rec.Reasoning = fmt.Sprintf("目标轻度压缩（%.0f%%），使用高质量参数", compressionRatio*100)
	case compressionRatio >= 0.5:
		// 中度压缩
		rec.Quality = 85
		rec.Lossless = false
		rec.Reasoning = fmt.Sprintf("目标中度压缩（%.0f%%），平衡质量和大小", compressionRatio*100)
	case compressionRatio >= 0.3:
		// 重度压缩
		rec.Quality = 75
		rec.Lossless = false
		rec.Reasoning = fmt.Sprintf("目标重度压缩（%.0f%%），优先减小大小", compressionRatio*100)
	default:
		// 极限压缩
		rec.Quality = 65
		rec.Lossless = false
		rec.Confidence = 0.5
		rec.Reasoning = fmt.Sprintf("目标极限压缩（%.0f%%），可能影响质量", compressionRatio*100)
	}
	
	return rec
}
