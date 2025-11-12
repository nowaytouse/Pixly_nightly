package features

import (
	"fmt"
	"image"
	"image/color"
	_ "image/gif"
	_ "image/jpeg"
	_ "image/png"
	"math"
	"os"
)

// SWTExtractor SWT 小波变换特征提取器
// 简化版本：使用统计方法近似小波特征
type SWTExtractor struct {
	levels int  // 分解层数
	debug  bool
}

// NewSWTExtractor 创建 SWT 特征提取器
func NewSWTExtractor(levels int, debug bool) *SWTExtractor {
	if levels <= 0 {
		levels = 3 // 默认 3 层
	}
	return &SWTExtractor{
		levels: levels,
		debug:  debug,
	}
}

// Extract 提取 SWT 特征
// 简化版本：使用图像统计方法近似小波域特征
func (e *SWTExtractor) Extract(imagePath string) (*ImageFeatures, error) {
	// 先打开图像
	file, err := os.Open(imagePath)
	if err != nil {
		return nil, fmt.Errorf("无法打开图像: %w", err)
	}
	defer file.Close()

	// 解码图像
	img, _, err := image.Decode(file)
	if err != nil {
		return nil, fmt.Errorf("无法解码图像: %w", err)
	}

	// 转换为灰度图
	grayImg := e.toGrayscale(img)

	// 计算各种特征
	edgeStrength := e.calculateEdgeStrength(grayImg)
	textureComplexity := e.calculateTextureComplexity(grayImg)
	noiseLevel := e.calculateNoiseLevel(grayImg)
	detailLevel := e.calculateDetailLevel(grayImg)
	highFreqEnergy, midFreqEnergy, lowFreqEnergy := e.calculateFrequencyEnergy(grayImg)
	entropyScore := e.calculateEntropy(grayImg)
	smoothRegionRatio := e.calculateSmoothRegionRatio(grayImg)

	features := &ImageFeatures{
		EdgeStrength:      edgeStrength,
		TextureComplexity: textureComplexity,
		NoiseLevel:        noiseLevel,
		DetailLevel:       detailLevel,
		HighFreqEnergy:    highFreqEnergy,
		MidFreqEnergy:     midFreqEnergy,
		LowFreqEnergy:     lowFreqEnergy,
		EntropyScore:      entropyScore,
		SmoothRegionRatio: smoothRegionRatio,
	}

	if e.debug {
		fmt.Printf("[SWTExtractor] 边缘强度=%.2f, 纹理复杂度=%.2f, 噪声=%.2f\n",
			edgeStrength, textureComplexity, noiseLevel)
	}

	return features, nil
}

// toGrayscale 转换为灰度图
func (e *SWTExtractor) toGrayscale(img image.Image) *image.Gray {
	bounds := img.Bounds()
	gray := image.NewGray(bounds)

	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			gray.Set(x, y, img.At(x, y))
		}
	}

	return gray
}

// calculateEdgeStrength 计算边缘强度
// 使用 Sobel 算子近似
func (e *SWTExtractor) calculateEdgeStrength(img *image.Gray) float64 {
	bounds := img.Bounds()
	width, height := bounds.Dx(), bounds.Dy()

	totalEdge := 0.0
	count := 0

	// Sobel 核
	sobelX := [3][3]int{
		{-1, 0, 1},
		{-2, 0, 2},
		{-1, 0, 1},
	}
	sobelY := [3][3]int{
		{-1, -2, -1},
		{0, 0, 0},
		{1, 2, 1},
	}

	for y := 1; y < height-1; y++ {
		for x := 1; x < width-1; x++ {
			gx := 0.0
			gy := 0.0

			// 应用 Sobel 算子
			for dy := -1; dy <= 1; dy++ {
				for dx := -1; dx <= 1; dx++ {
					pixel := float64(img.GrayAt(x+dx, y+dy).Y)
					gx += pixel * float64(sobelX[dy+1][dx+1])
					gy += pixel * float64(sobelY[dy+1][dx+1])
				}
			}

			// 计算梯度幅值
			magnitude := math.Sqrt(gx*gx + gy*gy)
			totalEdge += magnitude
			count++
		}
	}

	if count == 0 {
		return 0
	}

	// 归一化到 0-100
	return (totalEdge / float64(count)) / 255.0 * 100.0
}

// calculateTextureComplexity 计算纹理复杂度
// 使用局部标准差
func (e *SWTExtractor) calculateTextureComplexity(img *image.Gray) float64 {
	bounds := img.Bounds()
	width, height := bounds.Dx(), bounds.Dy()

	windowSize := 7
	halfWindow := windowSize / 2

	totalStd := 0.0
	count := 0

	for y := halfWindow; y < height-halfWindow; y += windowSize {
		for x := halfWindow; x < width-halfWindow; x += windowSize {
			// 计算窗口内的标准差
			std := e.calculateLocalStd(img, x, y, halfWindow)
			totalStd += std
			count++
		}
	}

	if count == 0 {
		return 0
	}

	// 归一化到 0-100
	return (totalStd / float64(count)) / 255.0 * 100.0
}

// calculateLocalStd 计算局部标准差
func (e *SWTExtractor) calculateLocalStd(img *image.Gray, cx, cy, radius int) float64 {
	values := make([]float64, 0, (2*radius+1)*(2*radius+1))

	for dy := -radius; dy <= radius; dy++ {
		for dx := -radius; dx <= radius; dx++ {
			pixel := float64(img.GrayAt(cx+dx, cy+dy).Y)
			values = append(values, pixel)
		}
	}

	// 计算均值
	mean := 0.0
	for _, v := range values {
		mean += v
	}
	mean /= float64(len(values))

	// 计算标准差
	variance := 0.0
	for _, v := range values {
		diff := v - mean
		variance += diff * diff
	}
	variance /= float64(len(values))

	return math.Sqrt(variance)
}

// calculateNoiseLevel 计算噪声水平
// 使用 MAD (Median Absolute Deviation)
func (e *SWTExtractor) calculateNoiseLevel(img *image.Gray) float64 {
	bounds := img.Bounds()
	width, height := bounds.Dx(), bounds.Dy()

	// 采样像素差异
	diffs := make([]float64, 0, width*height/4)

	for y := 1; y < height; y += 2 {
		for x := 1; x < width; x += 2 {
			curr := float64(img.GrayAt(x, y).Y)
			prev := float64(img.GrayAt(x-1, y).Y)
			diff := math.Abs(curr - prev)
			diffs = append(diffs, diff)
		}
	}

	if len(diffs) == 0 {
		return 0
	}

	// 计算平均差异（简化的 MAD）
	totalDiff := 0.0
	for _, d := range diffs {
		totalDiff += d
	}

	// 归一化到 0-100
	return (totalDiff / float64(len(diffs))) / 255.0 * 100.0
}

// calculateDetailLevel 计算细节水平
// 使用高频分量的能量
func (e *SWTExtractor) calculateDetailLevel(img *image.Gray) float64 {
	bounds := img.Bounds()
	width, height := bounds.Dx(), bounds.Dy()

	totalHighFreq := 0.0
	count := 0

	// 高通滤波器（简化）
	for y := 1; y < height-1; y++ {
		for x := 1; x < width-1; x++ {
			center := float64(img.GrayAt(x, y).Y)
			avg := (float64(img.GrayAt(x-1, y).Y) +
				float64(img.GrayAt(x+1, y).Y) +
				float64(img.GrayAt(x, y-1).Y) +
				float64(img.GrayAt(x, y+1).Y)) / 4.0

			highFreq := math.Abs(center - avg)
			totalHighFreq += highFreq
			count++
		}
	}

	if count == 0 {
		return 0
	}

	// 归一化到 0-100
	return (totalHighFreq / float64(count)) / 255.0 * 100.0
}

// calculateFrequencyEnergy 计算频域能量分布
// 简化版本：使用多尺度平均
func (e *SWTExtractor) calculateFrequencyEnergy(img *image.Gray) (high, mid, low float64) {
	bounds := img.Bounds()
	width, height := bounds.Dx(), bounds.Dy()

	// 高频：像素级差异
	highEnergy := 0.0
	for y := 1; y < height; y++ {
		for x := 1; x < width; x++ {
			diff := math.Abs(float64(img.GrayAt(x, y).Y) - float64(img.GrayAt(x-1, y).Y))
			highEnergy += diff * diff
		}
	}

	// 中频：小区域平均差异
	midEnergy := 0.0
	for y := 2; y < height-2; y += 2 {
		for x := 2; x < width-2; x += 2 {
			center := e.calculateLocalStd(img, x, y, 2)
			midEnergy += center * center
		}
	}

	// 低频：大区域平均差异
	lowEnergy := 0.0
	for y := 4; y < height-4; y += 4 {
		for x := 4; x < width-4; x += 4 {
			center := e.calculateLocalStd(img, x, y, 4)
			lowEnergy += center * center
		}
	}

	// 归一化
	total := highEnergy + midEnergy + lowEnergy
	if total > 0 {
		return highEnergy / total, midEnergy / total, lowEnergy / total
	}

	return 0.33, 0.33, 0.34
}

// calculateEntropy 计算图像熵
func (e *SWTExtractor) calculateEntropy(img *image.Gray) float64 {
	// 计算灰度直方图
	histogram := make([]int, 256)
	bounds := img.Bounds()

	totalPixels := 0
	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			gray := img.GrayAt(x, y).Y
			histogram[gray]++
			totalPixels++
		}
	}

	// 计算熵
	entropy := 0.0
	for _, count := range histogram {
		if count > 0 {
			p := float64(count) / float64(totalPixels)
			entropy -= p * math.Log2(p)
		}
	}

	// 归一化到 0-100 (最大熵是 8 比特)
	return (entropy / 8.0) * 100.0
}

// calculateSmoothRegionRatio 计算平滑区域比例
func (e *SWTExtractor) calculateSmoothRegionRatio(img *image.Gray) float64 {
	bounds := img.Bounds()
	width, height := bounds.Dx(), bounds.Dy()

	smoothPixels := 0
	totalPixels := 0

	threshold := 10.0 // 局部标准差阈值

	for y := 2; y < height-2; y++ {
		for x := 2; x < width-2; x++ {
			std := e.calculateLocalStd(img, x, y, 2)
			if std < threshold {
				smoothPixels++
			}
			totalPixels++
		}
	}

	if totalPixels == 0 {
		return 0
	}

	return float64(smoothPixels) / float64(totalPixels) * 100.0
}

// Merge 合并基础特征和 SWT 特征
func (e *SWTExtractor) Merge(basic, swt *ImageFeatures) *ImageFeatures {
	merged := *basic // 复制基础特征

	// 添加 SWT 特征
	merged.EdgeStrength = swt.EdgeStrength
	merged.TextureComplexity = swt.TextureComplexity
	merged.NoiseLevel = swt.NoiseLevel
	merged.DetailLevel = swt.DetailLevel
	merged.HighFreqEnergy = swt.HighFreqEnergy
	merged.MidFreqEnergy = swt.MidFreqEnergy
	merged.LowFreqEnergy = swt.LowFreqEnergy
	merged.EntropyScore = swt.EntropyScore
	merged.SmoothRegionRatio = swt.SmoothRegionRatio

	return &merged
}

// Unused color import workaround
var _ = color.Gray{}
