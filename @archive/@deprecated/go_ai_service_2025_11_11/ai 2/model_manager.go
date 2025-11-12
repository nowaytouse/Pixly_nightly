package ai

import (
	"fmt"
	"sync"
	"time"
)

// ModelType 模型类型
type ModelType string

const (
	ModelLightGBM    ModelType = "lightgbm"    // LightGBM模型
	ModelPPO         ModelType = "ppo"         // PPO强化学习模型
	ModelTransformer ModelType = "transformer" // Transformer模型
	ModelEnsemble    ModelType = "ensemble"    // 集成模型
	ModelBaseline    ModelType = "baseline"    // 基线规则引擎
)

// ModelVersion 模型版本
type ModelVersion struct {
	Version   string        `json:"version"`    // 版本号（如 "1.0.0"）
	Path      string        `json:"path"`       // 模型文件路径
	Checksum  string        `json:"checksum"`   // 文件校验和
	CreatedAt time.Time     `json:"created_at"` // 创建时间
	Metrics   *ModelMetrics `json:"metrics"`    // 性能指标
	IsActive  bool          `json:"is_active"`  // 是否激活
	IsDefault bool          `json:"is_default"` // 是否默认
}

// ModelMetrics 模型性能指标
type ModelMetrics struct {
	Accuracy        float64 `json:"accuracy"`          // 准确率
	RMSE            float64 `json:"rmse"`              // 均方根误差
	MAE             float64 `json:"mae"`               // 平均绝对误差
	InferenceTimeMs float64 `json:"inference_time_ms"` // 推理时间（毫秒）
	TestSamples     int     `json:"test_samples"`      // 测试样本数
}

// ModelConfig 模型配置
type ModelConfig struct {
	Type     ModelType `json:"type"`     // 模型类型
	Version  string    `json:"version"`  // 使用的版本
	Weight   float64   `json:"weight"`   // 集成权重（0-1）
	Enabled  bool      `json:"enabled"`  // 是否启用
	Priority int       `json:"priority"` // 优先级（越大越优先）
}

// ModelManager 模型管理器
type ModelManager struct {
	mu sync.RWMutex

	// 模型版本管理
	versions map[ModelType]map[string]*ModelVersion // type -> version -> ModelVersion

	// 当前激活配置
	activeConfigs map[ModelType]*ModelConfig

	// A/B测试配置
	abTestConfig *ABTestConfig

	// 统计数据
	stats *ModelStats
}

// ABTestConfig A/B测试配置
type ABTestConfig struct {
	Enabled    bool                        `json:"enabled"`
	SplitRatio float64                     `json:"split_ratio"` // A组占比（0-1）
	ModelA     ModelType                   `json:"model_a"`     // A组模型
	ModelB     ModelType                   `json:"model_b"`     // B组模型
	MetricName string                      `json:"metric_name"` // 评估指标
	Duration   time.Duration               `json:"duration"`    // 测试时长
	Results    map[ModelType]*ABTestResult `json:"results"`     // 测试结果
}

// ABTestResult A/B测试结果
type ABTestResult struct {
	ModelType    ModelType `json:"model_type"`
	RequestCount int       `json:"request_count"`
	AvgLatency   float64   `json:"avg_latency"`
	AvgAccuracy  float64   `json:"avg_accuracy"`
	ErrorRate    float64   `json:"error_rate"`
}

// ModelStats 模型统计
type ModelStats struct {
	mu           sync.Mutex
	RequestCount map[ModelType]int64     // 请求次数
	ErrorCount   map[ModelType]int64     // 错误次数
	TotalLatency map[ModelType]float64   // 总延迟（毫秒）
	LastUsed     map[ModelType]time.Time // 最后使用时间
}

// NewModelManager 创建模型管理器
func NewModelManager() *ModelManager {
	mm := &ModelManager{
		versions:      make(map[ModelType]map[string]*ModelVersion),
		activeConfigs: make(map[ModelType]*ModelConfig),
		stats: &ModelStats{
			RequestCount: make(map[ModelType]int64),
			ErrorCount:   make(map[ModelType]int64),
			TotalLatency: make(map[ModelType]float64),
			LastUsed:     make(map[ModelType]time.Time),
		},
	}

	// 初始化默认配置
	mm.initDefaultConfigs()

	return mm
}

// initDefaultConfigs 初始化默认配置
func (mm *ModelManager) initDefaultConfigs() {
	// LightGBM: 主要模型，权重最高
	mm.activeConfigs[ModelLightGBM] = &ModelConfig{
		Type:     ModelLightGBM,
		Version:  "1.0.0",
		Weight:   0.6,
		Enabled:  true,
		Priority: 100,
	}

	// PPO: 强化学习模型，用于在线优化
	mm.activeConfigs[ModelPPO] = &ModelConfig{
		Type:     ModelPPO,
		Version:  "1.0.0",
		Weight:   0.3,
		Enabled:  false, // 默认禁用，待训练
		Priority: 80,
	}

	// Baseline: 规则引擎，作为后备
	mm.activeConfigs[ModelBaseline] = &ModelConfig{
		Type:     ModelBaseline,
		Version:  "1.0.0",
		Weight:   0.1,
		Enabled:  true,
		Priority: 50,
	}
}

// SelectModel 选择模型（支持A/B测试）
func (mm *ModelManager) SelectModel(requestID string) ModelType {
	mm.mu.RLock()
	defer mm.mu.RUnlock()

	// 如果A/B测试启用
	if mm.abTestConfig != nil && mm.abTestConfig.Enabled {
		// 简单hash分流
		hash := hashString(requestID)
		if float64(hash%100)/100.0 < mm.abTestConfig.SplitRatio {
			return mm.abTestConfig.ModelA
		}
		return mm.abTestConfig.ModelB
	}

	// 否则选择优先级最高的启用模型
	var bestModel ModelType
	var bestPriority int = -1

	for modelType, config := range mm.activeConfigs {
		if config.Enabled && config.Priority > bestPriority {
			bestModel = modelType
			bestPriority = config.Priority
		}
	}

	if bestModel == "" {
		return ModelBaseline // fallback到基线
	}

	return bestModel
}

// GetModelConfig 获取模型配置
func (mm *ModelManager) GetModelConfig(modelType ModelType) *ModelConfig {
	mm.mu.RLock()
	defer mm.mu.RUnlock()
	return mm.activeConfigs[modelType]
}

// UpdateModelConfig 更新模型配置
func (mm *ModelManager) UpdateModelConfig(config *ModelConfig) error {
	mm.mu.Lock()
	defer mm.mu.Unlock()

	mm.activeConfigs[config.Type] = config
	return nil
}

// RegisterVersion 注册模型版本
func (mm *ModelManager) RegisterVersion(modelType ModelType, version *ModelVersion) error {
	mm.mu.Lock()
	defer mm.mu.Unlock()

	if mm.versions[modelType] == nil {
		mm.versions[modelType] = make(map[string]*ModelVersion)
	}

	mm.versions[modelType][version.Version] = version
	return nil
}

// GetModelVersion 获取模型版本
func (mm *ModelManager) GetModelVersion(modelType ModelType, version string) (*ModelVersion, error) {
	mm.mu.RLock()
	defer mm.mu.RUnlock()

	versions, ok := mm.versions[modelType]
	if !ok {
		return nil, fmt.Errorf("model type %s not found", modelType)
	}

	v, ok := versions[version]
	if !ok {
		return nil, fmt.Errorf("version %s not found for model %s", version, modelType)
	}

	return v, nil
}

// ListVersions 列出所有版本
func (mm *ModelManager) ListVersions(modelType ModelType) []*ModelVersion {
	mm.mu.RLock()
	defer mm.mu.RUnlock()

	versions, ok := mm.versions[modelType]
	if !ok {
		return nil
	}

	result := make([]*ModelVersion, 0, len(versions))
	for _, v := range versions {
		result = append(result, v)
	}

	return result
}

// StartABTest 启动A/B测试
func (mm *ModelManager) StartABTest(config *ABTestConfig) error {
	mm.mu.Lock()
	defer mm.mu.Unlock()

	config.Enabled = true
	config.Results = make(map[ModelType]*ABTestResult)
	config.Results[config.ModelA] = &ABTestResult{ModelType: config.ModelA}
	config.Results[config.ModelB] = &ABTestResult{ModelType: config.ModelB}

	mm.abTestConfig = config
	return nil
}

// StopABTest 停止A/B测试
func (mm *ModelManager) StopABTest() *ABTestConfig {
	mm.mu.Lock()
	defer mm.mu.Unlock()

	if mm.abTestConfig != nil {
		mm.abTestConfig.Enabled = false
	}

	return mm.abTestConfig
}

// RecordRequest 记录请求
func (mm *ModelManager) RecordRequest(modelType ModelType, latencyMs float64, success bool) {
	mm.stats.mu.Lock()
	defer mm.stats.mu.Unlock()

	mm.stats.RequestCount[modelType]++
	mm.stats.TotalLatency[modelType] += latencyMs
	mm.stats.LastUsed[modelType] = time.Now()

	if !success {
		mm.stats.ErrorCount[modelType]++
	}

	// 如果A/B测试启用，更新结果
	if mm.abTestConfig != nil && mm.abTestConfig.Enabled {
		if result, ok := mm.abTestConfig.Results[modelType]; ok {
			result.RequestCount++
			result.AvgLatency = (result.AvgLatency*float64(result.RequestCount-1) + latencyMs) / float64(result.RequestCount)
			if !success {
				result.ErrorRate = float64(mm.stats.ErrorCount[modelType]) / float64(mm.stats.RequestCount[modelType])
			}
		}
	}
}

// GetStats 获取统计信息
func (mm *ModelManager) GetStats() map[string]interface{} {
	mm.stats.mu.Lock()
	defer mm.stats.mu.Unlock()

	stats := make(map[string]interface{})

	for modelType, count := range mm.stats.RequestCount {
		modelStats := map[string]interface{}{
			"request_count": count,
			"error_count":   mm.stats.ErrorCount[modelType],
			"error_rate":    float64(mm.stats.ErrorCount[modelType]) / float64(count),
			"avg_latency":   mm.stats.TotalLatency[modelType] / float64(count),
			"last_used":     mm.stats.LastUsed[modelType],
		}
		stats[string(modelType)] = modelStats
	}

	return stats
}

// hashString 简单字符串hash
func hashString(s string) uint32 {
	h := uint32(0)
	for i := 0; i < len(s); i++ {
		h = h*31 + uint32(s[i])
	}
	return h
}
