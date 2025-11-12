package models

import (
	"encoding/json"
	"fmt"
	"os"

	"pixly/ai"
	"pixly/ai/features"
)

// LightGBMModel LightGBM 模型
// 简化版本：使用线性回归近似，等待真实模型训练后替换
type LightGBMModel struct {
	ModelPath string
	Format    string // jxl, avif, webp
	
	// 模型参数（简化版本）
	Weights   []float64
	Intercept float64
	Loaded    bool
}

// NewLightGBMModel 创建 LightGBM 模型
func NewLightGBMModel(modelPath, format string) *LightGBMModel {
	return &LightGBMModel{
		ModelPath: modelPath,
		Format:    format,
		Loaded:    false,
	}
}

// Load 加载模型
func (m *LightGBMModel) Load() error {
	// 如果模型文件不存在，使用默认权重
	if _, err := os.Stat(m.ModelPath); os.IsNotExist(err) {
		m.loadDefaultWeights()
		return nil
	}

	// 尝试加载 JSON 模型文件
	data, err := os.ReadFile(m.ModelPath)
	if err != nil {
		m.loadDefaultWeights()
		return nil
	}

	// 解析 JSON
	var modelData struct {
		Weights   []float64 `json:"weights"`
		Intercept float64   `json:"intercept"`
	}

	if err := json.Unmarshal(data, &modelData); err != nil {
		m.loadDefaultWeights()
		return nil
	}

	m.Weights = modelData.Weights
	m.Intercept = modelData.Intercept
	m.Loaded = true

	return nil
}

// loadDefaultWeights 加载默认权重
// 基于经验的启发式规则
func (m *LightGBMModel) loadDefaultWeights() {
	// 特征顺序：
	// [0] Width
	// [1] Height
	// [2] Size
	// [3] HasAlpha
	// [4] EdgeStrength
	// [5] TextureComplexity
	// [6] NoiseLevel
	// [7] DetailLevel
	// [8] HighFreqEnergy
	// [9] MidFreqEnergy
	// [10] LowFreqEnergy
	// [11] EntropyScore

	switch m.Format {
	case "jxl":
		m.Weights = []float64{
			0.0001,  // Width - 较小影响
			0.0001,  // Height - 较小影响
			0.00001, // Size - 较小影响
			5.0,     // HasAlpha - 有透明度需要更高质量
			0.3,     // EdgeStrength - 边缘多需要更高质量
			0.4,     // TextureComplexity - 纹理复杂需要更高质量
			-0.2,    // NoiseLevel - 噪声多可以降低质量
			0.35,    // DetailLevel - 细节多需要更高质量
			0.2,     // HighFreqEnergy - 高频能量影响质量
			0.15,    // MidFreqEnergy
			-0.1,    // LowFreqEnergy - 低频为主可以降低质量
			0.25,    // EntropyScore - 熵高需要更高质量
		}
		m.Intercept = 85.0 // 基础质量
		
	case "avif":
		m.Weights = []float64{
			0.0001, 0.0001, 0.00001,
			4.0, 0.35, 0.45, -0.25, 0.4,
			0.25, 0.2, -0.15, 0.3,
		}
		m.Intercept = 22.0 // AVIF 使用 quantizer，值越小质量越高
		
	case "webp":
		m.Weights = []float64{
			0.0001, 0.0001, 0.00001,
			4.5, 0.3, 0.4, -0.2, 0.35,
			0.2, 0.15, -0.1, 0.25,
		}
		m.Intercept = 82.0
	}

	m.Loaded = true
}

// Predict 预测参数
func (m *LightGBMModel) Predict(feat *features.ImageFeatures) (*ai.PredictionParams, error) {
	if !m.Loaded {
		if err := m.Load(); err != nil {
			return nil, err
		}
	}

	// 构建特征向量
	featureVector := m.featuresToVector(feat)

	// 线性回归预测
	prediction := m.Intercept
	for i, weight := range m.Weights {
		if i < len(featureVector) {
			prediction += weight * featureVector[i]
		}
	}

	// 创建预测参数
	params := &ai.PredictionParams{
		Format:        m.Format,
		Confidence:    0.75, // 默认置信度
		PredictorName: "lightgbm",
	}

	// 根据格式设置参数
	switch m.Format {
	case "jxl":
		params.Quality = m.clampQuality(int(prediction))
		params.Effort = m.predictEffort(feat)
		params.Distance = m.predictDistance(feat)
		params.Progressive = feat.Width*feat.Height > 1920*1080
		
	case "avif":
		params.Quantizer = m.clampQuantizer(int(prediction))
		params.Speed = m.predictSpeed(feat)
		
	case "webp":
		params.Quality = m.clampQuality(int(prediction))
		params.Method = m.predictMethod(feat)
	}

	// 预测文件大小
	params.ExpectedSize = m.predictSize(feat, params)
	params.ExpectedSaving = (1.0 - float64(params.ExpectedSize)/float64(feat.Size)) * 100.0

	return params, nil
}

// featuresToVector 将特征转换为向量
func (m *LightGBMModel) featuresToVector(f *features.ImageFeatures) []float64 {
	return []float64{
		float64(f.Width),
		float64(f.Height),
		float64(f.Size),
		boolToFloat(f.HasAlpha),
		f.EdgeStrength,
		f.TextureComplexity,
		f.NoiseLevel,
		f.DetailLevel,
		f.HighFreqEnergy,
		f.MidFreqEnergy,
		f.LowFreqEnergy,
		f.EntropyScore,
	}
}

// clampQuality 限制质量值范围
func (m *LightGBMModel) clampQuality(q int) int {
	if q < 70 {
		return 70
	}
	if q > 97 {
		return 97
	}
	return q
}

// clampQuantizer 限制 quantizer 范围
func (m *LightGBMModel) clampQuantizer(q int) int {
	if q < 10 {
		return 10
	}
	if q > 35 {
		return 35
	}
	return q
}

// predictEffort 预测 effort 参数
func (m *LightGBMModel) predictEffort(f *features.ImageFeatures) int {
	// 图像越大，effort 可以越低（速度优先）
	if f.Width*f.Height > 1920*1080*2 {
		return 5
	} else if f.Width*f.Height > 1920*1080 {
		return 6
	}
	return 7
}

// predictDistance 预测 distance 参数
func (m *LightGBMModel) predictDistance(f *features.ImageFeatures) float64 {
	// 基于纹理复杂度
	if f.TextureComplexity > 40 {
		return 0.5
	} else if f.TextureComplexity > 25 {
		return 0.8
	}
	return 1.0
}

// predictSpeed 预测 AVIF speed 参数
func (m *LightGBMModel) predictSpeed(f *features.ImageFeatures) int {
	if f.Width*f.Height > 1920*1080*2 {
		return 7
	} else if f.Width*f.Height > 1920*1080 {
		return 6
	}
	return 5
}

// predictMethod 预测 WebP method 参数
func (m *LightGBMModel) predictMethod(f *features.ImageFeatures) int {
	if f.TextureComplexity > 40 {
		return 6 // 最高质量
	} else if f.TextureComplexity > 25 {
		return 5
	}
	return 4
}

// predictSize 预测输出大小
func (m *LightGBMModel) predictSize(f *features.ImageFeatures, p *ai.PredictionParams) int64 {
	// 简化的大小预测
	baseSize := float64(f.Size)
	
	switch p.Format {
	case "jxl":
		// JXL 通常有很好的压缩率
		ratio := 0.3 + (100.0-float64(p.Quality))/100.0*0.3
		return int64(baseSize * ratio)
		
	case "avif":
		// AVIF 压缩率也很好
		ratio := 0.25 + float64(p.Quantizer)/63.0*0.4
		return int64(baseSize * ratio)
		
	case "webp":
		// WebP 压缩率中等
		ratio := 0.4 + (100.0-float64(p.Quality))/100.0*0.3
		return int64(baseSize * ratio)
	}
	
	return f.Size / 2
}

func boolToFloat(b bool) float64 {
	if b {
		return 1.0
	}
	return 0.0
}

// SaveModel 保存模型
func (m *LightGBMModel) SaveModel(path string) error {
	modelData := struct {
		Format    string    `json:"format"`
		Weights   []float64 `json:"weights"`
		Intercept float64   `json:"intercept"`
	}{
		Format:    m.Format,
		Weights:   m.Weights,
		Intercept: m.Intercept,
	}

	data, err := json.MarshalIndent(modelData, "", "  ")
	if err != nil {
		return fmt.Errorf("无法序列化模型: %w", err)
	}

	if err := os.WriteFile(path, data, 0644); err != nil {
		return fmt.Errorf("无法保存模型文件: %w", err)
	}

	return nil
}
