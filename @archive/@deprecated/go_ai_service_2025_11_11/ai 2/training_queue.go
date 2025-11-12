package ai

import (
	"context"
	"fmt"
	"sync"
	"time"
)

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Phase 33.2: 训练队列管理器 - 增量训练Pipeline
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// TrainingQueue 训练队列管理器
type TrainingQueue struct {
	mu sync.Mutex
	
	feedbackDB   *FeedbackDB
	modelManager *ModelManager
	
	// 配置
	config *TrainingConfig
	
	// 队列状态
	isRunning     bool
	currentBatch  *TrainingBatch
	
	// 上下文
	ctx    context.Context
	cancel context.CancelFunc
}

// TrainingConfig 训练配置
type TrainingConfig struct {
	MinSamples          int           `json:"min_samples"`           // 最小样本数（触发训练）
	MaxSamples          int           `json:"max_samples"`           // 最大样本数（单批次）
	CheckInterval       time.Duration `json:"check_interval"`        // 检查间隔
	AutoDeploy          bool          `json:"auto_deploy"`           // 自动部署新模型
	PerformanceThreshold float64      `json:"performance_threshold"` // 性能提升阈值
	MaxRetries          int           `json:"max_retries"`           // 最大重试次数
}

// TrainingBatch 训练批次
type TrainingBatch struct {
	ID           int                    `json:"id"`
	ModelType    string                 `json:"model_type"`
	StartTime    time.Time              `json:"start_time"`
	EndTime      time.Time              `json:"end_time"`
	Status       string                 `json:"status"` // pending, training, completed, failed
	Samples      []*FeedbackRecord      `json:"samples"`
	Metrics      map[string]interface{} `json:"metrics"`
	NewVersion   string                 `json:"new_version"`
	ErrorMessage string                 `json:"error_message"`
}

// NewTrainingQueue 创建训练队列管理器
func NewTrainingQueue(feedbackDB *FeedbackDB, modelManager *ModelManager, config *TrainingConfig) *TrainingQueue {
	ctx, cancel := context.WithCancel(context.Background())
	
	return &TrainingQueue{
		feedbackDB:   feedbackDB,
		modelManager: modelManager,
		config:       config,
		ctx:          ctx,
		cancel:       cancel,
	}
}

// Start 启动训练队列
func (tq *TrainingQueue) Start() error {
	tq.mu.Lock()
	if tq.isRunning {
		tq.mu.Unlock()
		return fmt.Errorf("training queue already running")
	}
	tq.isRunning = true
	tq.mu.Unlock()
	
	InfoWithContext("TrainingQueue", "Training queue started", map[string]interface{}{
		"check_interval_ms": tq.config.CheckInterval.Milliseconds(),
	})
	
	// 启动后台检查goroutine
	go tq.runBackgroundCheck()
	
	return nil
}

// Stop 停止训练队列
func (tq *TrainingQueue) Stop() {
	tq.mu.Lock()
	defer tq.mu.Unlock()
	
	if !tq.isRunning {
		return
	}
	
	tq.cancel()
	tq.isRunning = false
	
	Info("TrainingQueue", "Training queue stopped")
}

// runBackgroundCheck 后台检查任务
func (tq *TrainingQueue) runBackgroundCheck() {
	ticker := time.NewTicker(tq.config.CheckInterval)
	defer ticker.Stop()
	
	for {
		select {
		case <-tq.ctx.Done():
			return
		case <-ticker.C:
			tq.checkAndTriggerTraining()
		}
	}
}

// checkAndTriggerTraining 检查并触发训练
func (tq *TrainingQueue) checkAndTriggerTraining() {
	// 检查每种模型类型
	modelTypes := []ModelType{ModelLightGBM, ModelPPO}
	
	for _, modelType := range modelTypes {
		unusedCount, err := tq.getUnusedSampleCount(string(modelType))
		if err != nil {
			Warning("TrainingQueue", "Failed to get unused sample count for %s: %v", string(modelType), err)
			continue
		}
		
		if unusedCount >= tq.config.MinSamples {
			InfoWithContext("TrainingQueue", "Model has unused samples, triggering training", map[string]interface{}{
				"model_type":    string(modelType),
				"unused_count":  unusedCount,
				"min_threshold": tq.config.MinSamples,
			})
			
			if err := tq.TriggerTraining(string(modelType)); err != nil {
				Error("TrainingQueue", "Failed to trigger training for %s: %v", string(modelType), err)
			}
		}
	}
}

// getUnusedSampleCount 获取未使用的样本数
func (tq *TrainingQueue) getUnusedSampleCount(modelType string) (int, error) {
	stats, err := tq.feedbackDB.GetStats()
	if err != nil {
		return 0, err
	}
	
	// 从统计中获取未使用的样本数
	// 这里简化实现，实际应该按模型类型分类
	unusedRecords, ok := stats["unused_records"].(int)
	if !ok {
		return 0, nil
	}
	
	return unusedRecords, nil
}

// TriggerTraining 手动触发训练
func (tq *TrainingQueue) TriggerTraining(modelType string) error {
	tq.mu.Lock()
	
	// 检查是否已有训练任务在运行
	if tq.currentBatch != nil && tq.currentBatch.Status == "training" {
		tq.mu.Unlock()
		return fmt.Errorf("training already in progress")
	}
	
	tq.mu.Unlock()
	
	// 创建训练批次
	batch := &TrainingBatch{
		ModelType: modelType,
		StartTime: time.Now(),
		Status:    "pending",
	}
	
	// 获取训练样本
	samples, err := tq.feedbackDB.GetUnusedFeedback(modelType, tq.config.MaxSamples)
	if err != nil {
		return fmt.Errorf("failed to get unused feedback: %v", err)
	}
	
	if len(samples) < tq.config.MinSamples {
		return fmt.Errorf("insufficient samples: %d < %d", len(samples), tq.config.MinSamples)
	}
	
	batch.Samples = samples
	
	// 创建数据库记录
	batchID, err := tq.feedbackDB.CreateTrainingBatch(modelType, len(samples))
	if err != nil {
		return fmt.Errorf("failed to create training batch: %v", err)
	}
	batch.ID = batchID
	
	tq.mu.Lock()
	tq.currentBatch = batch
	tq.mu.Unlock()
	
	// 异步执行训练
	go tq.executeTraining(batch)
	
	InfoWithContext("TrainingQueue", "Training triggered", map[string]interface{}{
		"model_type":   modelType,
		"batch_id":     batchID,
		"sample_count": len(samples),
	})
	
	return nil
}

// executeTraining 执行训练
func (tq *TrainingQueue) executeTraining(batch *TrainingBatch) {
	batch.Status = "training"
	
	// 收集样本ID
	sampleIDs := make([]int64, len(batch.Samples))
	for i, sample := range batch.Samples {
		sampleIDs[i] = sample.ID
	}
	
	// 执行实际训练（这里模拟训练过程）
	success, metrics, newVersion, err := tq.performTraining(batch)
	
	if err != nil {
		Error("TrainingQueue", "Training failed for batch %d: %v", batch.ID, err)
		batch.Status = "failed"
		batch.ErrorMessage = err.Error()
		tq.feedbackDB.UpdateTrainingBatch(batch.ID, nil, "failed")
		return
	}
	
	batch.Status = "completed"
	batch.EndTime = time.Now()
	batch.Metrics = metrics
	batch.NewVersion = newVersion
	
	// 标记样本已使用
	tq.feedbackDB.MarkAsUsed(sampleIDs, batch.ID)
	
	// 更新训练批次
	tq.feedbackDB.UpdateTrainingBatch(batch.ID, metrics, "completed")
	
	// 记录模型性能
	tq.feedbackDB.RecordModelPerformance(batch.ModelType, newVersion, metrics)
	
	InfoWithContext("TrainingQueue", "Training completed", map[string]interface{}{
		"batch_id": batch.ID,
		"version":  newVersion,
	})
	
	// 如果性能提升且启用自动部署
	if success && tq.config.AutoDeploy {
		if err := tq.deployNewModel(batch.ModelType, newVersion); err != nil {
			Warning("TrainingQueue", "Failed to deploy new model: %v", err)
		} else {
			InfoWithContext("TrainingQueue", "New model deployed", map[string]interface{}{
				"model_type": batch.ModelType,
				"version":    newVersion,
			})
		}
	}
	
	tq.mu.Lock()
	tq.currentBatch = nil
	tq.mu.Unlock()
}

// performTraining 执行实际训练（调用Python训练脚本）
func (tq *TrainingQueue) performTraining(batch *TrainingBatch) (bool, map[string]interface{}, string, error) {
	Info("TrainingQueue", "Starting training for %s with %d samples...", batch.ModelType, len(batch.Samples))
	
	// 🔥 响亮报错：训练功能未实现
	// 【质量宣言】真实调用 > 演示代码
	Error("TrainingQueue", "ERROR: Model training not implemented")
	Info("TrainingQueue", "This is a placeholder for future ML training pipeline")
	Info("TrainingQueue", "Current behavior: Using pre-trained models only")
	Info("TrainingQueue", "")
	Info("TrainingQueue", "To implement training:")
	Info("TrainingQueue", "1. Export samples to training format (CSV/JSON)")
	Info("TrainingQueue", "2. Call Python training script")
	Info("TrainingQueue", "3. Evaluate new model performance")
	Info("TrainingQueue", "4. Compare with current model")
	
	return false, nil, "", fmt.Errorf(
		"🚨 Model training not implemented\n\n" +
		"This is a placeholder for future ML pipeline\n" +
		"Currently using pre-trained models only\n\n" +
		"Model type: %s\n" +
		"Sample count: %d",
		batch.ModelType, len(batch.Samples))
}

// deployNewModel 部署新模型
func (tq *TrainingQueue) deployNewModel(modelType, version string) error {
	Info("TrainingQueue", "Deploying new model: %s v%s", modelType, version)
	
	// 🔥 响亮报错：部署功能未实现
	Error("TrainingQueue", "ERROR: Model deployment not implemented")
	Info("TrainingQueue", "Model: %s v%s", modelType, version)
	
	return fmt.Errorf(
		"🚨 Model deployment not implemented\n\n" +
		"To implement deployment:\n" +
		"1. Validate new model file\n" +
		"2. Update model manager config\n" +
		"3. Hot reload or restart service\n\n" +
		"Model: %s v%s",
		modelType, version)
	
	// ❌ DELETED: 死代码（unreachable）
	// 函数已经返回错误，下面的代码永远不会执行
	// 
	// // 注册新版本
	// newModelVersion := &ModelVersion{ ... }
	// tq.modelManager.RegisterVersion(...)
	// config := tq.modelManager.GetModelConfig(...)
	// return nil
}

// GetStatus 获取队列状态（完整信息）
func (tq *TrainingQueue) GetStatus() map[string]interface{} {
	tq.mu.Lock()
	defer tq.mu.Unlock()
	
	status := map[string]interface{}{
		"is_running": tq.isRunning,
		"config":     tq.config,
	}
	
	if tq.currentBatch != nil {
		status["current_batch"] = map[string]interface{}{
			"id":          tq.currentBatch.ID,
			"model_type":  tq.currentBatch.ModelType,
			"status":      tq.currentBatch.Status,
			"start_time":  tq.currentBatch.StartTime,
			"sample_count": len(tq.currentBatch.Samples),
		}
	}
	
	return status
}

// GetQueueStatus 获取队列状态字符串
// 🔥 新增：用于HTTP API响应
// 注意：当前实现为单批次处理，无实际队列
func (tq *TrainingQueue) GetQueueStatus() string {
	tq.mu.Lock()
	defer tq.mu.Unlock()
	
	if !tq.isRunning {
		return "stopped"
	}
	if tq.currentBatch != nil {
		return "training"
	}
	return "idle"
}

// GetQueueSize 获取队列中等待的批次数量
// 🔥 新增：用于HTTP API响应
// 注意：当前实现为单批次处理，返回0或1
func (tq *TrainingQueue) GetQueueSize() int {
	tq.mu.Lock()
	defer tq.mu.Unlock()
	
	// 当前实现：没有实际队列，只有单个currentBatch
	// 如果正在训练则返回1，否则返回0
	if tq.currentBatch != nil {
		return 1
	}
	return 0
}

// GetDefaultConfig 获取默认配置
func GetDefaultTrainingConfig() *TrainingConfig {
	return &TrainingConfig{
		MinSamples:           100,  // 至少100个样本触发训练
		MaxSamples:           1000, // 最多1000个样本一批
		CheckInterval:        30 * time.Minute, // 30分钟检查一次
		AutoDeploy:           false, // 默认不自动部署
		PerformanceThreshold: 0.05,  // 5%性能提升
		MaxRetries:           3,
	}
}
