package ai

import (
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

// InitializeModelRouter 初始化模型路由器并注册默认模型
func (gw *HTTPGateway) InitializeModelRouter() error {
	gw.modelRouter = NewModelRouter()

	// 注册默认模型: LightGBM v1.0.0
	lightGBM := &ModelInfo{
		Name:     "lightgbm",
		Version:  "v1.0.0",
		Path:     "models/lightgbm_v1.0.0.pkl",
		Type:     "predictor",
		Status:   "active",
		Priority: 100,
		ABWeight: 1.0,
		Metrics: &Metrics{
			Accuracy:  0.92,
			Precision: 0.89,
			Recall:    0.91,
			F1Score:   0.90,
		},
	}

	if err := gw.modelRouter.RegisterModel(lightGBM); err != nil {
		return fmt.Errorf("failed to register lightgbm: %w", err)
	}

	// 注册PPO模型 v2.1.3
	ppo := &ModelInfo{
		Name:     "ppo",
		Version:  "v2.1.3",
		Path:     "models/ppo_v2.1.3.pkl",
		Type:     "optimizer",
		Status:   "active",
		Priority: 90,
		ABWeight: 0.0, // 默认不启用，可通过A/B测试逐步开启
		Metrics: &Metrics{
			Accuracy:  0.94,
			Precision: 0.93,
			Recall:    0.92,
			F1Score:   0.925,
		},
	}

	if err := gw.modelRouter.RegisterModel(ppo); err != nil {
		return fmt.Errorf("failed to register ppo: %w", err)
	}

	Info("ModelRouter", "Model router initialized with default models")
	return nil
}

// handleListModels 列出所有模型 [GET /api/v1/models]
func (gw *HTTPGateway) handleListModels(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	models := gw.modelRouter.ListModels()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"models":  models,
	})
}

// handleGetActiveModels 获取活跃模型 [GET /api/v1/models/active]
func (gw *HTTPGateway) handleGetActiveModels(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	active := gw.modelRouter.GetActiveModels()

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"active":  active,
	})
}

// handleRegisterModel 注册新模型 [POST /api/v1/models/register]
func (gw *HTTPGateway) handleRegisterModel(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var model ModelInfo
	if err := json.NewDecoder(r.Body).Decode(&model); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	if err := gw.modelRouter.RegisterModel(&model); err != nil {
		http.Error(w, fmt.Sprintf("Failed to register model: %v", err), http.StatusInternalServerError)
		return
	}

	InfoWithContext("ModelRouter", "Registered model", map[string]interface{}{
		"name":    model.Name,
		"version": model.Version,
	})

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": fmt.Sprintf("Model %s v%s registered successfully", model.Name, model.Version),
	})
}

// handlePromoteModel 提升模型为活跃版本 [POST /api/v1/models/promote]
func (gw *HTTPGateway) handlePromoteModel(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		Name    string `json:"name"`
		Version string `json:"version"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	if err := gw.modelRouter.PromoteModel(req.Name, req.Version); err != nil {
		http.Error(w, fmt.Sprintf("Failed to promote model: %v", err), http.StatusInternalServerError)
		return
	}

	InfoWithContext("ModelRouter", "Promoted model to active", map[string]interface{}{
		"name":    req.Name,
		"version": req.Version,
	})

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": fmt.Sprintf("Model %s v%s promoted to active", req.Name, req.Version),
	})
}

// handleCompareModels 比较两个模型 [POST /api/v1/models/compare]
func (gw *HTTPGateway) handleCompareModels(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		Model1Name    string `json:"model1_name"`
		Model1Version string `json:"model1_version"`
		Model2Name    string `json:"model2_name"`
		Model2Version string `json:"model2_version"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	comparison, err := gw.modelRouter.CompareModels(
		req.Model1Name, req.Model1Version,
		req.Model2Name, req.Model2Version,
	)
	if err != nil {
		http.Error(w, fmt.Sprintf("Failed to compare models: %v", err), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success":    true,
		"comparison": comparison,
	})
}

// handleABTesting 启用/禁用A/B测试 [POST /api/v1/models/ab-testing]
func (gw *HTTPGateway) handleABTesting(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		Enabled bool `json:"enabled"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	gw.modelRouter.SetABTesting(req.Enabled)

	status := "disabled"
	if req.Enabled {
		status = "enabled"
	}

	Info("ModelRouter", "A/B testing %s", status)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"success": true,
		"message": fmt.Sprintf("A/B testing %s", status),
		"enabled": req.Enabled,
	})
}

// handleModelRouterStats 导出模型路由器统计 [GET /api/v1/models/router-stats]
// 已废弃：使用handleModelStats (http_gateway_model_management.go)
func (gw *HTTPGateway) handleModelRouterStats(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	stats, err := gw.modelRouter.ExportStats()
	if err != nil {
		http.Error(w, fmt.Sprintf("Failed to export stats: %v", err), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.Write([]byte(stats))
}

// handlePredictWithModel 使用指定模型进行预测 [POST /api/v1/predict/with-model]
func (gw *HTTPGateway) handlePredictWithModel(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		ModelName    string          `json:"model_name"`
		ModelVersion string          `json:"model_version,omitempty"`
		Request      *PredictRequest `json:"request"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, fmt.Sprintf("Invalid request: %v", err), http.StatusBadRequest)
		return
	}

	// 获取指定模型
	var model *ModelInfo
	var err error

	if req.ModelVersion != "" {
		model, err = gw.modelRouter.GetModelByVersion(req.ModelName, req.ModelVersion)
	} else {
		model, err = gw.modelRouter.GetModel(req.ModelName)
	}

	if err != nil {
		http.Error(w, fmt.Sprintf("Model not found: %v", err), http.StatusNotFound)
		return
	}

	InfoWithContext("ModelPredictor", "Using model for prediction", map[string]interface{}{
		"name":    model.Name,
		"version": model.Version,
	})

	// 记录开始时间
	startTime := time.Now()

	// 🔥 响亮报错：删除TODO mock代码
	// 【质量宣言】真实调用 > 演示代码
	Error("ModelPredictor", "TODO: handlePredictWithModel not implemented yet")
	gw.sendError(w, 
		"🚨 Direct model prediction not implemented\n\n"+
		"Please use /api/v1/predict endpoint instead\n\n"+
		"【质量宣言】真实调用 > Mock数据",
		http.StatusNotImplemented, startTime)
	return

	// ❌ DELETED: mock返回值
	// response := &PredictResponse{
	//   Quality: 85,  // hardcode!
	// }
}
