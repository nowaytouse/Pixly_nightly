package quality

import (
	"fmt"
	"image"
	_ "image/gif"
	_ "image/jpeg"
	_ "image/png"
	"math"
	"os"
)

// QualityMetrics 质量评估指标
type QualityMetrics struct {
	SSIM float64 `json:"ssim"` // 结构相似性指数 (0-1)
	PSNR float64 `json:"psnr"` // 峰值信噪比 (dB)
	MSE  float64 `json:"mse"`  // 均方误差
}

// Calculator 质量评估计算器
type Calculator struct {
	debug bool
}

// NewCalculator 创建质量评估计算器
func NewCalculator(debug bool) *Calculator {
	return &Calculator{
		debug: debug,
	}
}

// Compare 比较两张图像的质量
// original: 原始图像路径
// compressed: 压缩后图像路径
func (c *Calculator) Compare(originalPath, compressedPath string) (*QualityMetrics, error) {
	// 加载原始图像
	origImg, err := loadImage(originalPath)
	if err != nil {
		return nil, fmt.Errorf("无法加载原始图像: %w", err)
	}
	
	// 加载压缩图像
	compImg, err := loadImage(compressedPath)
	if err != nil {
		return nil, fmt.Errorf("无法加载压缩图像: %w", err)
	}
	
	// 检查尺寸一致性
	if origImg.Bounds() != compImg.Bounds() {
		return nil, fmt.Errorf("图像尺寸不一致: %v vs %v", origImg.Bounds(), compImg.Bounds())
	}
	
	// 计算MSE
	mse := c.calculateMSE(origImg, compImg)
	
	// 计算PSNR
	psnr := c.calculatePSNR(mse)
	
	// 计算SSIM
	ssim := c.calculateSSIM(origImg, compImg)
	
	metrics := &QualityMetrics{
		SSIM: ssim,
		PSNR: psnr,
		MSE:  mse,
	}
	
	if c.debug {
		fmt.Printf("[QualityMetrics] SSIM=%.4f, PSNR=%.2fdB, MSE=%.2f\n", ssim, psnr, mse)
	}
	
	return metrics, nil
}

// calculateMSE 计算均方误差 (Mean Squared Error)
func (c *Calculator) calculateMSE(img1, img2 image.Image) float64 {
	bounds := img1.Bounds()
	width, height := bounds.Dx(), bounds.Dy()
	
	var sumSquaredDiff float64
	totalPixels := width * height
	
	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			r1, g1, b1, _ := img1.At(x, y).RGBA()
			r2, g2, b2, _ := img2.At(x, y).RGBA()
			
			// 归一化到 0-255
			r1, g1, b1 = r1>>8, g1>>8, b1>>8
			r2, g2, b2 = r2>>8, g2>>8, b2>>8
			
			// 计算每个通道的差异
			diffR := float64(r1) - float64(r2)
			diffG := float64(g1) - float64(g2)
			diffB := float64(b1) - float64(b2)
			
			sumSquaredDiff += diffR*diffR + diffG*diffG + diffB*diffB
		}
	}
	
	// 平均三个通道
	mse := sumSquaredDiff / float64(totalPixels*3)
	return mse
}

// calculatePSNR 计算峰值信噪比 (Peak Signal-to-Noise Ratio)
// PSNR = 10 * log10(MAX^2 / MSE)
// MAX = 255 (8-bit图像)
func (c *Calculator) calculatePSNR(mse float64) float64 {
	if mse == 0 {
		return 100.0 // 完全相同
	}
	
	maxPixelValue := 255.0
	psnr := 10.0 * math.Log10((maxPixelValue*maxPixelValue)/mse)
	
	return psnr
}

// calculateSSIM 计算结构相似性指数 (Structural Similarity Index)
// 简化版本：使用8x8窗口的局部SSIM平均值
func (c *Calculator) calculateSSIM(img1, img2 image.Image) float64 {
	bounds := img1.Bounds()
	width, height := bounds.Dx(), bounds.Dy()
	
	windowSize := 8
	var ssimSum float64
	var windowCount int
	
	// 滑动窗口计算局部SSIM
	for y := 0; y < height-windowSize; y += windowSize {
		for x := 0; x < width-windowSize; x += windowSize {
			localSSIM := c.calculateLocalSSIM(img1, img2, x, y, windowSize)
			ssimSum += localSSIM
			windowCount++
		}
	}
	
	if windowCount == 0 {
		return 0.0
	}
	
	return ssimSum / float64(windowCount)
}

// calculateLocalSSIM 计算局部SSIM
// SSIM(x,y) = (2*μx*μy + C1)(2*σxy + C2) / ((μx^2 + μy^2 + C1)(σx^2 + σy^2 + C2))
func (c *Calculator) calculateLocalSSIM(img1, img2 image.Image, startX, startY, size int) float64 {
	const (
		C1 = 6.5025  // (0.01 * 255)^2
		C2 = 58.5225 // (0.03 * 255)^2
	)
	
	// 提取窗口像素值（灰度化）
	pixels1 := c.extractWindow(img1, startX, startY, size)
	pixels2 := c.extractWindow(img2, startX, startY, size)
	
	// 计算均值
	mean1 := calculateMean(pixels1)
	mean2 := calculateMean(pixels2)
	
	// 计算方差和协方差
	var1 := calculateVariance(pixels1, mean1)
	var2 := calculateVariance(pixels2, mean2)
	covar := calculateCovariance(pixels1, pixels2, mean1, mean2)
	
	// 计算SSIM
	numerator := (2*mean1*mean2 + C1) * (2*covar + C2)
	denominator := (mean1*mean1 + mean2*mean2 + C1) * (var1 + var2 + C2)
	
	if denominator == 0 {
		return 1.0
	}
	
	ssim := numerator / denominator
	return ssim
}

// extractWindow 提取窗口像素（转换为灰度）
func (c *Calculator) extractWindow(img image.Image, startX, startY, size int) []float64 {
	pixels := make([]float64, 0, size*size)
	
	for y := startY; y < startY+size; y++ {
		for x := startX; x < startX+size; x++ {
			r, g, b, _ := img.At(x, y).RGBA()
			// 归一化到 0-255
			r, g, b = r>>8, g>>8, b>>8
			// 转换为灰度
			gray := 0.299*float64(r) + 0.587*float64(g) + 0.114*float64(b)
			pixels = append(pixels, gray)
		}
	}
	
	return pixels
}

// Helper functions

func loadImage(path string) (image.Image, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	
	img, _, err := image.Decode(file)
	if err != nil {
		return nil, err
	}
	
	return img, nil
}

func calculateMean(values []float64) float64 {
	sum := 0.0
	for _, v := range values {
		sum += v
	}
	return sum / float64(len(values))
}

func calculateVariance(values []float64, mean float64) float64 {
	sum := 0.0
	for _, v := range values {
		diff := v - mean
		sum += diff * diff
	}
	return sum / float64(len(values))
}

func calculateCovariance(values1, values2 []float64, mean1, mean2 float64) float64 {
	sum := 0.0
	for i := range values1 {
		sum += (values1[i] - mean1) * (values2[i] - mean2)
	}
	return sum / float64(len(values1))
}

// EstimateSSIM 根据图像特征估算预期SSIM
// 用于在转换前预测质量
func (c *Calculator) EstimateSSIM(quality int, hasAlpha bool, complexity float64) float64 {
	// 基于质量参数的基础SSIM
	baseSSIM := 0.75 + (float64(quality)/100.0)*0.20
	
	// 透明通道会略微降低SSIM
	if hasAlpha {
		baseSSIM -= 0.02
	}
	
	// 高复杂度图像SSIM略低
	if complexity > 40 {
		baseSSIM -= 0.03
	}
	
	// 确保在合理范围内
	if baseSSIM > 0.99 {
		baseSSIM = 0.99
	}
	if baseSSIM < 0.70 {
		baseSSIM = 0.70
	}
	
	return baseSSIM
}

// IsSS IMAcceptable 判断SSIM是否可接受
func (c *Calculator) IsSSIMAcceptable(ssim float64) bool {
	return ssim >= 0.95 // 一般认为0.95以上是高质量
}

// GetQualityLevel 根据SSIM获取质量等级
func (c *Calculator) GetQualityLevel(ssim float64) string {
	if ssim >= 0.98 {
		return "excellent" // 极佳
	} else if ssim >= 0.95 {
		return "good" // 良好
	} else if ssim >= 0.90 {
		return "fair" // 尚可
	} else {
		return "poor" // 较差
	}
}
