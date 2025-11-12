package ai

import (
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Phase 33.2: 训练队列和反馈 HTTP端点
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// handleStartTrainingQueue 启动训练队列 [POST /api/v1/training/start]
func (gw *HTTPGateway) handleStartTrainingQueue(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	if gw.trainingQueue == nil {
		http.Error(w, "Training queue not available", http.StatusServiceUnavailable)
		return
	}

	if err := gw.trainingQueue.Start(); err != nil {
		http.Error(w, fmt.Sprintf("Failed to start training queue: %v", err), http.StatusInternalServerError)
		return
	}

	Info("TrainingAPI", "Training queue started via API")

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": "Training queue started successfully",
	})
}

// handleStopTrainingQueue 停止训练队列 [POST /api/v1/training/stop]
func (gw *HTTPGateway) handleStopTrainingQueue(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	if gw.trainingQueue == nil {
		http.Error(w, "Training queue not available", http.StatusServiceUnavailable)
		return
	}

	gw.trainingQueue.Stop()

	Info("TrainingAPI", "Training queue stopped via API")

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": "Training queue stopped successfully",
	})
}

// handleTrainingQueueStatus 获取训练队列状态 [GET /api/v1/training/status]
func (gw *HTTPGateway) handleTrainingQueueStatus(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	if gw.trainingQueue == nil {
		http.Error(w, "Training queue not available", http.StatusServiceUnavailable)
		return
	}

	status := gw.trainingQueue.GetStatus()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"status":  status,
	})
}

// handleTriggerTraining 手动触发训练 [POST /api/v1/training/trigger]
func (gw *HTTPGateway) handleTriggerTraining(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	if gw.trainingQueue == nil {
		http.Error(w, "Training queue not available", http.StatusServiceUnavailable)
		return
	}

	var request struct {
		ModelType string `json:"model_type"`
	}

	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	if err := gw.trainingQueue.TriggerTraining(request.ModelType); err != nil {
		http.Error(w, fmt.Sprintf("Failed to trigger training: %v", err), http.StatusInternalServerError)
		return
	}

	Info("TrainingAPI", "Training triggered for %s via API", request.ModelType)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": fmt.Sprintf("Training triggered for %s", request.ModelType),
	})
}

// handleRecordFeedback 记录反馈 [POST /api/v1/feedback/record]
func (gw *HTTPGateway) handleRecordFeedback(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	if gw.feedbackDB == nil {
		http.Error(w, "Feedback database not available", http.StatusServiceUnavailable)
		return
	}

	var record FeedbackRecord
	if err := json.NewDecoder(r.Body).Decode(&record); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	if err := gw.feedbackDB.RecordFeedback(&record); err != nil {
		http.Error(w, fmt.Sprintf("Failed to record feedback: %v", err), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": "Feedback recorded successfully",
		"id":      record.ID,
	})
}

// handleFeedbackStats 获取反馈统计 [GET /api/v1/feedback/stats]
func (gw *HTTPGateway) handleFeedbackStats(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	if gw.feedbackDB == nil {
		http.Error(w, "Feedback database not available", http.StatusServiceUnavailable)
		return
	}

	stats, err := gw.feedbackDB.GetStats()
	if err != nil {
		http.Error(w, fmt.Sprintf("Failed to get stats: %v", err), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"stats":   stats,
	})
}

// handleTrainingStats 获取训练统计 [GET /api/v1/training/stats]
// Phase 37: 修复JS插件API不一致问题
func (gw *HTTPGateway) handleTrainingStats(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// 获取反馈数据库统计
	var totalFeedback int64
	var avgCompressionRatio float64

	if gw.feedbackDB != nil {
		if stats, err := gw.feedbackDB.GetStats(); err == nil {
			totalFeedback = int64(stats["total_records"].(float64))
			if cr, ok := stats["avg_compression_ratio"]; ok {
				avgCompressionRatio = cr.(float64)
			}
		}
	}

	// 获取训练队列状态
	var queueStatus string
	var queueSize int
	if gw.trainingQueue != nil {
		// ✅ 已实现：获取队列状态
		queueStatus = gw.trainingQueue.GetQueueStatus()
		queueSize = gw.trainingQueue.GetQueueSize()
	} else {
		queueStatus = "inactive"
		queueSize = 0
	}

	// 返回完整的训练统计
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"stats": map[string]interface{}{
			"total_predictions":     totalFeedback,
			"total_feedback":        totalFeedback,
			"avg_compression_ratio": avgCompressionRatio,
			"training_queue": map[string]interface{}{
				"status": queueStatus,
				"size":   queueSize,
			},
			"active_models": len(gw.modelRouter.ListModels()),
			"last_updated":  time.Now().Unix(),
		},
	})
}
