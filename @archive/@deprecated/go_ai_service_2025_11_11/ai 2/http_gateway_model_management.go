package ai

import (
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Phase 33.1: 模型管理 HTTP端点
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// handleModelStats 获取模型统计信息 [GET /api/v1/models/stats]
func (gw *HTTPGateway) handleModelStats(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	stats := gw.modelManager.GetStats()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"stats":   stats,
	})
}

// handleUpdateModelConfig 更新模型配置 [POST /api/v1/models/config]
func (gw *HTTPGateway) handleUpdateModelConfig(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var config ModelConfig
	if err := json.NewDecoder(r.Body).Decode(&config); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	if err := gw.modelManager.UpdateModelConfig(&config); err != nil {
		http.Error(w, fmt.Sprintf("Failed to update config: %v", err), http.StatusInternalServerError)
		return
	}

	InfoWithContext("ModelManager", "Updated model config", map[string]interface{}{
		"type":    config.Type,
		"version": config.Version,
	})

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": "Model config updated successfully",
	})
}

// handleRegisterModelVersion 注册新模型版本 [POST /api/v1/models/register-version]
func (gw *HTTPGateway) handleRegisterModelVersion(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var request struct {
		ModelType ModelType     `json:"model_type"`
		Version   *ModelVersion `json:"version"`
	}

	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	if err := gw.modelManager.RegisterVersion(request.ModelType, request.Version); err != nil {
		http.Error(w, fmt.Sprintf("Failed to register version: %v", err), http.StatusInternalServerError)
		return
	}

	InfoWithContext("ModelManager", "Registered new version", map[string]interface{}{
		"model_type": request.ModelType,
		"version":    request.Version.Version,
	})

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": "Model version registered successfully",
	})
}

// handleListModelVersions 列出模型版本 [GET /api/v1/models/versions?type=lightgbm]
func (gw *HTTPGateway) handleListModelVersions(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	modelTypeStr := r.URL.Query().Get("type")
	if modelTypeStr == "" {
		http.Error(w, "model type is required", http.StatusBadRequest)
		return
	}

	modelType := ModelType(modelTypeStr)
	versions := gw.modelManager.ListVersions(modelType)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success":  true,
		"type":     modelType,
		"versions": versions,
	})
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// A/B测试端点
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// handleStartABTest 启动A/B测试 [POST /api/v1/models/abtest/start]
func (gw *HTTPGateway) handleStartABTest(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var request struct {
		ModelA      string  `json:"model_a"`
		ModelB      string  `json:"model_b"`
		SplitRatio  float64 `json:"split_ratio"`   // A组占比
		DurationMin int     `json:"duration_min"`  // 测试时长（分钟）
		MetricName  string  `json:"metric_name"`   // 评估指标
	}

	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	config := &ABTestConfig{
		ModelA:     ModelType(request.ModelA),
		ModelB:     ModelType(request.ModelB),
		SplitRatio: request.SplitRatio,
		Duration:   time.Duration(request.DurationMin) * time.Minute,
		MetricName: request.MetricName,
	}

	if err := gw.modelManager.StartABTest(config); err != nil {
		http.Error(w, fmt.Sprintf("Failed to start A/B test: %v", err), http.StatusInternalServerError)
		return
	}

	InfoWithContext("ModelManager", "Started A/B test", map[string]interface{}{
		"model_a":      request.ModelA,
		"model_b":      request.ModelB,
		"split_ratio":  request.SplitRatio,
		"duration_min": request.DurationMin,
	})

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": "A/B test started successfully",
		"config":  config,
	})
}

// handleStopABTest 停止A/B测试 [POST /api/v1/models/abtest/stop]
func (gw *HTTPGateway) handleStopABTest(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	result := gw.modelManager.StopABTest()

	Info("ModelManager", "Stopped A/B test")

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": "A/B test stopped successfully",
		"result":  result,
	})
}

// handleGetABTestStatus 获取A/B测试状态 [GET /api/v1/models/abtest/status]
func (gw *HTTPGateway) handleGetABTestStatus(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// 通过modelManager获取当前状态
	stats := gw.modelManager.GetStats()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"stats":   stats,
	})
}
