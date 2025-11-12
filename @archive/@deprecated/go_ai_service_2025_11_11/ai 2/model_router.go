package ai

import (
	"encoding/json"
	"fmt"
	"math/rand"
	"sync"
	"time"
)

// ModelInfo 模型信息
type ModelInfo struct {
	Name       string    `json:"name"`         // 模型名称 (lightgbm, ppo, etc)
	Version    string    `json:"version"`      // 版本号 (v1.0.0, v1.0.1, etc)
	Path       string    `json:"path"`         // 模型文件路径
	Type       string    `json:"type"`         // 模型类型 (predictor, optimizer, validator)
	Status     string    `json:"status"`       // 状态 (active, testing, deprecated)
	Priority   int       `json:"priority"`     // 优先级 (越高越优先)
	ABWeight   float64   `json:"ab_weight"`    // A/B测试权重 (0.0-1.0)
	Metrics    *Metrics  `json:"metrics"`      // 性能指标
	CreatedAt  time.Time `json:"created_at"`   // 创建时间
	LastUsedAt time.Time `json:"last_used_at"` // 最后使用时间
	UsageCount int64     `json:"usage_count"`  // 使用次数
}

// Metrics 模型性能指标
type Metrics struct {
	Accuracy     float64 `json:"accuracy"`      // 准确率
	Precision    float64 `json:"precision"`     // 精确率
	Recall       float64 `json:"recall"`        // 召回率
	F1Score      float64 `json:"f1_score"`      // F1分数
	AvgLatency   float64 `json:"avg_latency"`   // 平均延迟(ms)
	ErrorRate    float64 `json:"error_rate"`    // 错误率
	TotalCalls   int64   `json:"total_calls"`   // 总调用次数
	SuccessCalls int64   `json:"success_calls"` // 成功调用次数
}

// ModelRouter 模型路由器
type ModelRouter struct {
	models    map[string][]*ModelInfo // name -> versions
	active    map[string]*ModelInfo   // name -> active version
	mu        sync.RWMutex
	abEnabled bool // A/B测试是否启用
	rand      *rand.Rand
}

// NewModelRouter 创建模型路由器
func NewModelRouter() *ModelRouter {
	return &ModelRouter{
		models:    make(map[string][]*ModelInfo),
		active:    make(map[string]*ModelInfo),
		abEnabled: true,
		rand:      rand.New(rand.NewSource(time.Now().UnixNano())),
	}
}

// RegisterModel 注册模型
func (r *ModelRouter) RegisterModel(info *ModelInfo) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	if info.Name == "" || info.Version == "" {
		return fmt.Errorf("model name and version are required")
	}

	// 添加到模型列表
	versions, exists := r.models[info.Name]
	if !exists {
		versions = []*ModelInfo{}
	}

	// 检查是否已存在相同版本
	for _, v := range versions {
		if v.Version == info.Version {
			return fmt.Errorf("model %s version %s already exists", info.Name, info.Version)
		}
	}

	// 初始化
	if info.CreatedAt.IsZero() {
		info.CreatedAt = time.Now()
	}
	if info.Metrics == nil {
		info.Metrics = &Metrics{}
	}

	versions = append(versions, info)
	r.models[info.Name] = versions

	// 如果是第一个版本或状态为active且优先级更高，设置为活跃版本
	if !exists || (info.Status == "active" && (r.active[info.Name] == nil || info.Priority > r.active[info.Name].Priority)) {
		r.active[info.Name] = info
	}

	return nil
}

// GetModel 获取模型（支持A/B测试）
func (r *ModelRouter) GetModel(name string) (*ModelInfo, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	versions, exists := r.models[name]
	if !exists || len(versions) == 0 {
		return nil, fmt.Errorf("model %s not found", name)
	}

	// 如果未启用A/B测试，返回活跃版本
	if !r.abEnabled {
		if active, ok := r.active[name]; ok {
			return active, nil
		}
		return versions[0], nil
	}

	// A/B测试：根据权重选择模型
	return r.selectModelByWeight(versions), nil
}

// GetModelByVersion 获取指定版本的模型
func (r *ModelRouter) GetModelByVersion(name, version string) (*ModelInfo, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	versions, exists := r.models[name]
	if !exists {
		return nil, fmt.Errorf("model %s not found", name)
	}

	for _, v := range versions {
		if v.Version == version {
			return v, nil
		}
	}

	return nil, fmt.Errorf("model %s version %s not found", name, version)
}

// selectModelByWeight 根据权重选择模型（A/B测试）
func (r *ModelRouter) selectModelByWeight(versions []*ModelInfo) *ModelInfo {
	// 过滤出active和testing状态的模型
	candidates := []*ModelInfo{}
	totalWeight := 0.0

	for _, v := range versions {
		if v.Status == "active" || v.Status == "testing" {
			candidates = append(candidates, v)
			totalWeight += v.ABWeight
		}
	}

	if len(candidates) == 0 {
		return versions[0] // fallback
	}

	if len(candidates) == 1 {
		return candidates[0]
	}

	// 归一化权重
	if totalWeight <= 0 {
		totalWeight = float64(len(candidates))
		for _, v := range candidates {
			v.ABWeight = 1.0
		}
	}

	// 加权随机选择
	random := r.rand.Float64() * totalWeight
	cumulative := 0.0

	for _, v := range candidates {
		cumulative += v.ABWeight
		if random <= cumulative {
			return v
		}
	}

	return candidates[len(candidates)-1] // fallback
}

// ListModels 列出所有模型
func (r *ModelRouter) ListModels() map[string][]*ModelInfo {
	r.mu.RLock()
	defer r.mu.RUnlock()

	result := make(map[string][]*ModelInfo)
	for name, versions := range r.models {
		result[name] = append([]*ModelInfo{}, versions...)
	}
	return result
}

// GetActiveModels 获取所有活跃模型
func (r *ModelRouter) GetActiveModels() map[string]*ModelInfo {
	r.mu.RLock()
	defer r.mu.RUnlock()

	result := make(map[string]*ModelInfo)
	for name, model := range r.active {
		result[name] = model
	}
	return result
}

// UpdateMetrics 更新模型指标
func (r *ModelRouter) UpdateMetrics(name, version string, metrics *Metrics) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	versions, exists := r.models[name]
	if !exists {
		return fmt.Errorf("model %s not found", name)
	}

	for _, v := range versions {
		if v.Version == version {
			v.Metrics = metrics
			v.LastUsedAt = time.Now()
			v.UsageCount++
			return nil
		}
	}

	return fmt.Errorf("model %s version %s not found", name, version)
}

// RecordUsage 记录模型使用
func (r *ModelRouter) RecordUsage(name, version string, success bool, latency float64) {
	r.mu.Lock()
	defer r.mu.Unlock()

	versions, exists := r.models[name]
	if !exists {
		return
	}

	for _, v := range versions {
		if v.Version == version {
			v.UsageCount++
			v.LastUsedAt = time.Now()
			if v.Metrics != nil {
				v.Metrics.TotalCalls++
				if success {
					v.Metrics.SuccessCalls++
				}
				// 更新平均延迟（移动平均）
				if v.Metrics.AvgLatency == 0 {
					v.Metrics.AvgLatency = latency
				} else {
					v.Metrics.AvgLatency = (v.Metrics.AvgLatency*0.9 + latency*0.1)
				}
				// 更新错误率
				if v.Metrics.TotalCalls > 0 {
					v.Metrics.ErrorRate = float64(v.Metrics.TotalCalls-v.Metrics.SuccessCalls) / float64(v.Metrics.TotalCalls)
				}
			}
			return
		}
	}
}

// SetABTesting 启用/禁用A/B测试
func (r *ModelRouter) SetABTesting(enabled bool) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.abEnabled = enabled
}

// PromoteModel 提升模型为活跃版本
func (r *ModelRouter) PromoteModel(name, version string) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	versions, exists := r.models[name]
	if !exists {
		return fmt.Errorf("model %s not found", name)
	}

	for _, v := range versions {
		if v.Version == version {
			// 将旧的活跃版本降级
			if old, ok := r.active[name]; ok {
				old.Status = "deprecated"
				old.ABWeight = 0.0
			}

			// 提升新版本
			v.Status = "active"
			v.Priority = 100
			v.ABWeight = 1.0
			r.active[name] = v

			return nil
		}
	}

	return fmt.Errorf("model %s version %s not found", name, version)
}

// CompareModels 比较两个模型的性能
func (r *ModelRouter) CompareModels(name1, version1, name2, version2 string) (map[string]interface{}, error) {
	model1, err := r.GetModelByVersion(name1, version1)
	if err != nil {
		return nil, err
	}

	model2, err := r.GetModelByVersion(name2, version2)
	if err != nil {
		return nil, err
	}

	comparison := map[string]interface{}{
		"model1": map[string]interface{}{
			"name":    model1.Name,
			"version": model1.Version,
			"metrics": model1.Metrics,
		},
		"model2": map[string]interface{}{
			"name":    model2.Name,
			"version": model2.Version,
			"metrics": model2.Metrics,
		},
		"winner":  "",
		"reasons": []string{},
	}

	reasons := []string{}
	score1 := 0
	score2 := 0

	// 比较准确率
	if model1.Metrics.Accuracy > model2.Metrics.Accuracy {
		score1++
		reasons = append(reasons, fmt.Sprintf("Model1 has higher accuracy: %.2f%% vs %.2f%%",
			model1.Metrics.Accuracy*100, model2.Metrics.Accuracy*100))
	} else if model2.Metrics.Accuracy > model1.Metrics.Accuracy {
		score2++
		reasons = append(reasons, fmt.Sprintf("Model2 has higher accuracy: %.2f%% vs %.2f%%",
			model2.Metrics.Accuracy*100, model1.Metrics.Accuracy*100))
	}

	// 比较延迟
	if model1.Metrics.AvgLatency < model2.Metrics.AvgLatency {
		score1++
		reasons = append(reasons, fmt.Sprintf("Model1 is faster: %.2fms vs %.2fms",
			model1.Metrics.AvgLatency, model2.Metrics.AvgLatency))
	} else if model2.Metrics.AvgLatency < model1.Metrics.AvgLatency {
		score2++
		reasons = append(reasons, fmt.Sprintf("Model2 is faster: %.2fms vs %.2fms",
			model2.Metrics.AvgLatency, model1.Metrics.AvgLatency))
	}

	// 比较错误率
	if model1.Metrics.ErrorRate < model2.Metrics.ErrorRate {
		score1++
		reasons = append(reasons, fmt.Sprintf("Model1 has lower error rate: %.2f%% vs %.2f%%",
			model1.Metrics.ErrorRate*100, model2.Metrics.ErrorRate*100))
	} else if model2.Metrics.ErrorRate < model1.Metrics.ErrorRate {
		score2++
		reasons = append(reasons, fmt.Sprintf("Model2 has lower error rate: %.2f%% vs %.2f%%",
			model2.Metrics.ErrorRate*100, model1.Metrics.ErrorRate*100))
	}

	if score1 > score2 {
		comparison["winner"] = fmt.Sprintf("%s v%s", model1.Name, model1.Version)
	} else if score2 > score1 {
		comparison["winner"] = fmt.Sprintf("%s v%s", model2.Name, model2.Version)
	} else {
		comparison["winner"] = "tie"
	}

	comparison["reasons"] = reasons

	return comparison, nil
}

// ExportStats 导出统计数据
func (r *ModelRouter) ExportStats() (string, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	stats := map[string]interface{}{
		"total_models": len(r.models),
		"models":       r.models,
		"active":       r.active,
		"ab_enabled":   r.abEnabled,
		"timestamp":    time.Now(),
	}

	data, err := json.MarshalIndent(stats, "", "  ")
	if err != nil {
		return "", err
	}

	return string(data), nil
}
