package rl

import (
	"encoding/json"
	"fmt"
	"os"
	"time"
)

// ============================================================================
// PPO模型序列化/反序列化 v4.2.0
// 注：由于PolicyNetwork和ValueNetwork内部结构未导出，
//     当前实现仅序列化配置和统计信息
// ============================================================================

// ModelCheckpoint 模型检查点（简化版）
type ModelCheckpoint struct {
	Version   string         `json:"version"`
	Config    *PPOConfig     `json:"config"`
	Stats     *TrainingStats `json:"stats"`
	Timestamp int64          `json:"timestamp"`
}

// TrainingStats 训练统计
type TrainingStats struct {
	TotalEpisodes int       `json:"total_episodes"`
	TotalSteps    int       `json:"total_steps"`
	AverageReward float64   `json:"average_reward"`
	BestReward    float64   `json:"best_reward"`
	LastUpdate    time.Time `json:"last_update"`
}

// SaveConfig 保存PPO配置
func (ppo *PPOOptimizer) SaveConfig(filepath string) error {
	checkpoint := &ModelCheckpoint{
		Version: "4.2.0",
		Config:  ppo.config,
		Stats: &TrainingStats{
			LastUpdate: time.Now(),
		},
		Timestamp: time.Now().Unix(),
	}

	data, err := json.MarshalIndent(checkpoint, "", "  ")
	if err != nil {
		return fmt.Errorf("序列化配置失败: %v", err)
	}

	return os.WriteFile(filepath, data, 0644)
}

// LoadConfig 加载PPO配置
func (ppo *PPOOptimizer) LoadConfig(filepath string) error {
	data, err := os.ReadFile(filepath)
	if err != nil {
		return fmt.Errorf("读取配置文件失败: %v", err)
	}

	var checkpoint ModelCheckpoint
	err = json.Unmarshal(data, &checkpoint)
	if err != nil {
		return fmt.Errorf("反序列化配置失败: %v", err)
	}

	if checkpoint.Config != nil {
		ppo.config = checkpoint.Config
	}

	return nil
}

// ============================================================================
// 模型管理器
// ============================================================================

// ModelManager PPO模型管理器
type ModelManager struct {
	modelDir string
}

// NewModelManager 创建模型管理器
func NewModelManager(modelDir string) *ModelManager {
	// 确保目录存在
	os.MkdirAll(modelDir, 0755)
	return &ModelManager{modelDir: modelDir}
}

// SaveCheckpoint 保存检查点
func (mm *ModelManager) SaveCheckpoint(ppo *PPOOptimizer, name string) error {
	filename := fmt.Sprintf("%s/%s_%d.json", mm.modelDir, name, time.Now().Unix())
	return ppo.SaveConfig(filename)
}

// LoadLatestCheckpoint 加载最新检查点
func (mm *ModelManager) LoadLatestCheckpoint(ppo *PPOOptimizer, name string) error {
	// 简化实现：使用固定文件名
	filename := fmt.Sprintf("%s/%s_latest.json", mm.modelDir, name)
	return ppo.LoadConfig(filename)
}

// ExportMetadata 导出元数据（用于监控）
func (mm *ModelManager) ExportMetadata(ppo *PPOOptimizer, name string) error {
	metadata := map[string]interface{}{
		"version":   "4.2.0",
		"name":      name,
		"timestamp": time.Now().Unix(),
		"config": map[string]interface{}{
			"gamma":         ppo.config.Gamma,
			"epsilon":       ppo.config.Epsilon,
			"learning_rate": ppo.config.LearningRate,
			"batch_size":    ppo.config.BatchSize,
		},
	}

	data, err := json.MarshalIndent(metadata, "", "  ")
	if err != nil {
		return fmt.Errorf("导出元数据失败: %v", err)
	}

	filename := fmt.Sprintf("%s/%s_metadata.json", mm.modelDir, name)
	return os.WriteFile(filename, data, 0644)
}

// ValidateConfig 验证配置文件
func ValidateConfig(filepath string) error {
	data, err := os.ReadFile(filepath)
	if err != nil {
		return fmt.Errorf("读取配置文件失败: %v", err)
	}

	var checkpoint ModelCheckpoint
	err = json.Unmarshal(data, &checkpoint)
	if err != nil {
		return fmt.Errorf("配置文件格式错误: %v", err)
	}

	if checkpoint.Version == "" {
		return fmt.Errorf("版本信息缺失")
	}

	if checkpoint.Config == nil {
		return fmt.Errorf("配置信息缺失")
	}

	return nil
}

// ============================================================================
// 统计信息持久化
// ============================================================================

// StatsRecorder 统计记录器
type StatsRecorder struct {
	filepath string
	stats    *TrainingStats
}

// NewStatsRecorder 创建统计记录器
func NewStatsRecorder(filepath string) *StatsRecorder {
	return &StatsRecorder{
		filepath: filepath,
		stats: &TrainingStats{
			LastUpdate: time.Now(),
		},
	}
}

// RecordEpisode 记录一个回合
func (sr *StatsRecorder) RecordEpisode(reward float64) {
	sr.stats.TotalEpisodes++
	sr.stats.AverageReward = (sr.stats.AverageReward*float64(sr.stats.TotalEpisodes-1) + reward) / float64(sr.stats.TotalEpisodes)

	if reward > sr.stats.BestReward {
		sr.stats.BestReward = reward
	}

	sr.stats.LastUpdate = time.Now()
}

// Save 保存统计信息
func (sr *StatsRecorder) Save() error {
	data, err := json.MarshalIndent(sr.stats, "", "  ")
	if err != nil {
		return fmt.Errorf("序列化统计信息失败: %v", err)
	}

	return os.WriteFile(sr.filepath, data, 0644)
}

// Load 加载统计信息
func (sr *StatsRecorder) Load() error {
	data, err := os.ReadFile(sr.filepath)
	if err != nil {
		return fmt.Errorf("读取统计信息失败: %v", err)
	}

	return json.Unmarshal(data, &sr.stats)
}

// GetStats 获取统计信息
func (sr *StatsRecorder) GetStats() *TrainingStats {
	return sr.stats
}
