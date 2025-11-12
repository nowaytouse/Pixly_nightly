package ai

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"time"
)

// HTTPGateway HTTP网关
type HTTPGateway struct {
	pythonBridge  *PythonBridge
	modelRouter   *ModelRouter   // 模型路由器
	modelManager  *ModelManager  // 🆕 模型管理器（Phase 33.1）
	feedbackDB    *FeedbackDB    // 🆕 反馈数据库（Phase 33.2）
	trainingQueue *TrainingQueue // 🆕 训练队列（Phase 33.2）
	port          int
}

// NewHTTPGateway 创建HTTP网关
func NewHTTPGateway(modelDir string, port int) *HTTPGateway {
	// 初始化反馈数据库
	feedbackDB, err := NewFeedbackDB(modelDir + "/feedback.db")
	if err != nil {
		Warning("HTTPGateway", "Failed to initialize feedback DB: %v", err)
	}

	// 初始化模型管理器
	modelManager := NewModelManager()

	// 初始化训练队列
	var trainingQueue *TrainingQueue
	if feedbackDB != nil {
		trainingQueue = NewTrainingQueue(feedbackDB, modelManager, GetDefaultTrainingConfig())
	}

	return &HTTPGateway{
		pythonBridge:  NewPythonBridge(modelDir),
		modelManager:  modelManager,
		feedbackDB:    feedbackDB,
		trainingQueue: trainingQueue,
		port:          port,
	}
}

// PredictRequest HTTP请求（基于可选参数，无预设Mode）
type PredictRequest struct {
	ImagePath     string `json:"image_path"`
	Tool          string `json:"tool"`
	TargetQuality int    `json:"target_quality"`
	OptimizeMode  string `json:"optimize_mode,omitempty"` // size, balanced, quality, universal

	// 🆕 可选参数（无预设Mode）
	Options *RequestOptions `json:"options,omitempty"`

	// 🆕 Phase 33.1: 模型选择
	ModelType string `json:"model_type,omitempty"` // lightgbm, ppo, baseline, auto
	RequestID string `json:"request_id,omitempty"` // 用于A/B测试分流
}

// RequestOptions 请求选项
// Phase 46.11: 修正字段名以匹配Rust发送的JSON
type RequestOptions struct {
	ReturnAdvancedParams       bool   `json:"return_advanced_params"`
	EnableFeatureAnalysis      bool   `json:"enable_feature_analysis"`
	EnableQualityConstraint    bool   `json:"enable_quality_constraint"`
	ExpectedFormat             string `json:"expected_format,omitempty"`
	EnableFormatRecommendation bool   `json:"enable_format_recommendation"`  // 🎨 格式智能推荐
	
	// Phase 46.11: 修正字段名以匹配Rust
	EnableVideoForAnim         bool   `json:"enable_video_for_anim"`     // 🔄 修正: recommend_video_for_animation → enable_video_for_anim
	EnableBayesian             bool   `json:"enable_bayesian"`            // 贝叶斯优化
	EnablePPO                  bool   `json:"enable_ppo"`                 // PPO强化学习
	
	// Phase 46.11: 新增缺失的字段
	EnableSmartQuality         bool   `json:"enable_smart_quality"`      // 🆕 智能质量预测
	EnableAutoOptimize         bool   `json:"enable_auto_optimize"`      // 🆕 自动参数优化
}

// PreprocessStep 预处理步骤建议
// Phase 46.14: AI推荐的预处理步骤
type PreprocessStep struct {
	Step   string                 `json:"step"`   // "resize", "quantize", "sharpen"
	Params map[string]interface{} `json:"params"` // 步骤参数
	Reason string                 `json:"reason"` // 推荐原因
}

// PredictResponse HTTP响应
// Phase 46.12: 添加Rust期望的缺失字段
type PredictResponse struct {
	Success  bool                   `json:"success"`
	Params   *PredictParams         `json:"params,omitempty"`
	Advanced map[string]interface{} `json:"advanced,omitempty"` // 🆕 高级参数
	Merged   map[string]interface{} `json:"merged,omitempty"`   // 🆕 合并参数
	Features map[string]interface{} `json:"features,omitempty"` // 特征分析

	// 🆕 Phase 33.1: 模型信息
	ModelUsed         string  `json:"model_used,omitempty"`         // 使用的模型
	ModelVersion      string  `json:"model_version,omitempty"`      // 模型版本
	InferenceTimeMs   float64 `json:"inference_time_ms,omitempty"`  // 推理时间
	Confidence        float64 `json:"confidence,omitempty"`         // 🆕 Confidence
	RecommendedFormat string  `json:"recommended_format,omitempty"` // 🎨 推荐格式
	FormatReason      string  `json:"format_reason,omitempty"`      // 🎨 推荐原因
	UsedFormat        string  `json:"used_format,omitempty"`        // 实际使用格式
	IsCustomFormat    bool    `json:"is_custom_format,omitempty"`   // 是否自定义格式
	
	// Phase 46.12: 添加Rust期望的字段
	Reasoning         string                      `json:"reasoning,omitempty"`        // AI推理说明
	Lossless          *bool                       `json:"lossless,omitempty"`         // 无损模式
	LosslessJpeg      *bool                       `json:"lossless_jpeg,omitempty"`    // JPEG无损转码
	FormatOptions     []map[string]string         `json:"format_options,omitempty"`   // 格式选项
	
	// Phase 46.14: 预处理建议 (参考Rimage)
	PreprocessingSteps []PreprocessStep `json:"preprocessing_steps,omitempty"` // 预处理步骤建议
	OptimizationPath   string           `json:"optimization_path,omitempty"`   // 优化路径说明
	
	Error             string  `json:"error,omitempty"`
	ErrorCode         string  `json:"error_code,omitempty"` // 错误码（Phase 46.14+）
	TimeMs            int64   `json:"time_ms"`
}

// HealthResponse Health check响应
type HealthResponse struct {
	Status  string `json:"status"`
	Version string `json:"version"`
	Ready   bool   `json:"ready"`
}

// Start 启动HTTP服务
func (gw *HTTPGateway) Start() error {
	// 初始化模型路由器
	if err := gw.InitializeModelRouter(); err != nil {
		Warning("HTTPGateway", "Failed to initialize model router: %v", err)
		Warning("HTTPGateway", "Continuing without model router...")
	} else {
		Info("HTTPGateway", "Model router initialized successfully")
	}

	// CORS中间件
	corsMiddleware := func(next http.HandlerFunc) http.HandlerFunc {
		return func(w http.ResponseWriter, r *http.Request) {
			w.Header().Set("Access-Control-Allow-Origin", "*")
			w.Header().Set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
			w.Header().Set("Access-Control-Allow-Headers", "Content-Type")

			if r.Method == "OPTIONS" {
				w.WriteHeader(http.StatusOK)
				return
			}

			next(w, r)
		}
	}

	// 模型管理路由 (Phase 33.1)
	http.HandleFunc("/api/v1/models", corsMiddleware(gw.handleListModels))
	http.HandleFunc("/api/v1/models/list", corsMiddleware(gw.handleListModels)) // 🔥 Phase 40.17: 向后兼容别名
	http.HandleFunc("/api/v1/models/active", corsMiddleware(gw.handleGetActiveModels))
	http.HandleFunc("/api/v1/models/register", corsMiddleware(gw.handleRegisterModel))
	http.HandleFunc("/api/v1/models/promote", corsMiddleware(gw.handlePromoteModel))
	http.HandleFunc("/api/v1/models/compare", corsMiddleware(gw.handleCompareModels))
	http.HandleFunc("/api/v1/models/ab-testing", corsMiddleware(gw.handleABTesting))
	http.HandleFunc("/api/v1/models/stats", corsMiddleware(gw.handleModelStats))
	http.HandleFunc("/api/v1/predict/with-model", corsMiddleware(gw.handlePredictWithModel))

	// 🆕 Phase 33.1: 新模型管理端点
	http.HandleFunc("/api/v1/models/config", corsMiddleware(gw.handleUpdateModelConfig))
	http.HandleFunc("/api/v1/models/register-version", corsMiddleware(gw.handleRegisterModelVersion))
	http.HandleFunc("/api/v1/models/versions", corsMiddleware(gw.handleListModelVersions))
	http.HandleFunc("/api/v1/models/abtest/start", corsMiddleware(gw.handleStartABTest))
	http.HandleFunc("/api/v1/models/abtest/stop", corsMiddleware(gw.handleStopABTest))
	http.HandleFunc("/api/v1/models/abtest/status", corsMiddleware(gw.handleGetABTestStatus))

	// 🆕 Phase 33.2: 在线学习端点
	http.HandleFunc("/api/v1/training/start", corsMiddleware(gw.handleStartTrainingQueue))
	http.HandleFunc("/api/v1/training/stop", corsMiddleware(gw.handleStopTrainingQueue))
	http.HandleFunc("/api/v1/training/status", corsMiddleware(gw.handleTrainingQueueStatus))
	http.HandleFunc("/api/v1/training/stats", corsMiddleware(gw.handleTrainingStats)) // 🔥 Phase 37: 训练统计
	http.HandleFunc("/api/v1/training/trigger", corsMiddleware(gw.handleTriggerTraining))
	http.HandleFunc("/api/v1/feedback/record", corsMiddleware(gw.handleRecordFeedback))
	http.HandleFunc("/api/v1/feedback/stats", corsMiddleware(gw.handleFeedbackStats))

	// 预测路由
	http.HandleFunc("/api/v1/predict", corsMiddleware(gw.handlePredict))
	http.HandleFunc("/api/v1/predict/video", corsMiddleware(gw.handleVideoPredict)) // 🎬 Video AI prediction
	http.HandleFunc("/api/v1/validate/vmaf", corsMiddleware(gw.handleVMAF))         // 📊 VMAFQuality验证
	http.HandleFunc("/api/v1/health", corsMiddleware(gw.handleHealth))
	http.HandleFunc("/health", corsMiddleware(gw.handleHealth)) // 🔥 Phase 37: 标准健康检查端点
	http.HandleFunc("/api/v1/version", corsMiddleware(gw.handleVersion))
	http.HandleFunc("/api/v1/capabilities", corsMiddleware(gw.handleCapabilities)) // 🔥 修复：服务能力列表
	http.HandleFunc("/api/v1/observations", corsMiddleware(gw.handleObservations)) // 📊 观测数据收集（PPO训练用）

	addr := fmt.Sprintf(":%d", gw.port)
	Info("HTTPGateway", "AI HTTP service started at http://localhost%s", addr)
	Info("HTTPGateway", "Image AI prediction: POST http://localhost%s/api/v1/predict", addr)
	Info("HTTPGateway", "Video AI prediction: POST http://localhost%s/api/v1/predict/video", addr)
	Info("HTTPGateway", "VMAF validation: POST http://localhost%s/api/v1/validate/vmaf", addr)
	Info("HTTPGateway", "Health check: GET http://localhost%s/api/v1/health", addr)

	return http.ListenAndServe(addr, nil)
}

// handlePredict 处理预测请求
func (gw *HTTPGateway) handlePredict(w http.ResponseWriter, r *http.Request) {
	startTime := time.Now()

	if r.Method != http.MethodPost {
		gw.sendError(w, "Method not allowed", http.StatusMethodNotAllowed, startTime)
		return
	}

	// 解析请求
	var req PredictRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		// 使用PixlyError
		pixlyErr := NewValidationError(ErrValInvalidFormat, "Failed to parse JSON request")
		pixlyErr.WithContext("error", err.Error())
		gw.sendPixlyError(w, pixlyErr, http.StatusBadRequest, startTime)
		return
	}

	// 🔥 Phase 46.7: 验证请求参数
	if err := ValidateHTTPPredictRequest(&req); err != nil {
		LogValidationError("PredictRequest", err)
		gw.sendError(w, fmt.Sprintf("参数验证失败: %v", err), http.StatusBadRequest, startTime)
		return
	}
	LogValidationSuccess("PredictRequest")

	// 设置默认值（验证通过后）
	if req.Tool == "" {
		req.Tool = "jxl" // 默认JXL
	}
	if req.TargetQuality == 0 {
		// 根据优化Mode设置默认Quality
		req.TargetQuality = gw.getQualityForMode(req.OptimizeMode)
	}

	// 确定优化Mode
	optimizeMode := req.OptimizeMode
	if optimizeMode == "" {
		optimizeMode = "balanced"
	}

	InfoWithContext("Predictor", "AI prediction request", map[string]interface{}{
		"tool":    req.Tool,
		"quality": req.TargetQuality,
		"mode":    optimizeMode,
		"image":   filepath.Base(req.ImagePath),
	})

	// 🆕 转换options为Python Bridge格式
	var pyOptions *PyPredictOptions
	if req.Options != nil {
		pyOptions = &PyPredictOptions{
			ReturnAdvancedParams:       req.Options.ReturnAdvancedParams,
			EnableFeatureAnalysis:      req.Options.EnableFeatureAnalysis,
			EnableQualityConstraint:    req.Options.EnableQualityConstraint,
			ExpectedFormat:             req.Options.ExpectedFormat,
			EnableFormatRecommendation: req.Options.EnableFormatRecommendation,
			
			// Phase 46.11: 使用修正后的字段名
			EnableVideoForAnim:         req.Options.EnableVideoForAnim,
			EnableBayesian:             req.Options.EnableBayesian,
			EnablePPO:                  req.Options.EnablePPO,
			EnableSmartQuality:         req.Options.EnableSmartQuality,
			EnableAutoOptimize:         req.Options.EnableAutoOptimize,
		}
	}

	// 调用Python推理
	result, err := gw.pythonBridge.Predict(req.ImagePath, req.Tool, req.TargetQuality, optimizeMode, pyOptions)
	if err != nil {
		// 使用PixlyError包装错误
		pixlyErr := NewBusinessError(ErrBizPredictFailed, "AI prediction failed")
		pixlyErr.WithContext("image", filepath.Base(req.ImagePath))
		pixlyErr.WithContext("tool", req.Tool)
		pixlyErr.WithContext("error", err.Error())
		gw.sendPixlyError(w, pixlyErr, http.StatusInternalServerError, startTime)
		return
	}

	// 构建响应
	elapsed := time.Since(startTime).Milliseconds()
	resp := PredictResponse{
		Success:           true,
		Params:            &result.Params,
		Advanced:          result.Advanced,          // 🆕 高级参数
		Merged:            result.Merged,            // 🆕 合并参数
		Features:          result.Features,          // 特征分析
		Confidence:        result.Confidence,        // 🆕 Confidence
		RecommendedFormat: result.RecommendedFormat, // 🎨 推荐格式
		FormatReason:      result.FormatReason,      // 🎨 推荐原因
		UsedFormat:        result.UsedFormat,        // 实际使用格式
		IsCustomFormat:    result.IsCustomFormat,    // 是否自定义格式
		TimeMs:            elapsed,
	}

	InfoWithContext("Predictor", "AI prediction succeeded", map[string]interface{}{
		"quality":    result.Params.Quality,
		"distance":   result.Params.Distance,
		"confidence": result.Confidence,
		"duration_ms": elapsed,
	})

	// 🔥 调试：输出完整响应
	respJSON, _ := json.MarshalIndent(resp, "", "  ")
	fmt.Printf("📤 [DEBUG] Sending response:\n%s\n", string(respJSON))

	gw.sendJSON(w, resp, http.StatusOK)
}

// handleHealth Health check
func (gw *HTTPGateway) handleHealth(w http.ResponseWriter, r *http.Request) {
	err := gw.pythonBridge.CheckHealth()
	ready := err == nil

	resp := HealthResponse{
		Status:  "ok",
		Version: "4.2.0",
		Ready:   ready,
	}

	if !ready {
		resp.Status = "degraded"
		Warning("HealthCheck", "AI service degraded: Python environment check failed")
	}

	gw.sendJSON(w, resp, http.StatusOK)
}

// handleVersion 版本信息
func (gw *HTTPGateway) handleVersion(w http.ResponseWriter, r *http.Request) {
	version := map[string]interface{}{
		"version":    "4.2.0",
		"name":       "Pixly AI Service",
		"ai_enabled": true,
		"features": []string{
			"SWT小波变换Quality分析",
			"LightGBM参数预测",
			"多Tool支持(JXL/AVIF/WebP)",
			"智能优化Mode",
		},
	}
	gw.sendJSON(w, version, http.StatusOK)
}

// handleCapabilities 服务能力列表（修复404错误）
func (gw *HTTPGateway) handleCapabilities(w http.ResponseWriter, r *http.Request) {
	capabilities := map[string]interface{}{
		"version": "4.2.0",
		"endpoints": map[string]interface{}{
			"image_prediction": "/api/v1/predict",
			"video_prediction": "/api/v1/predict/video",
			"health_check":     "/api/v1/health",
			"version_info":     "/api/v1/version",
			"observations":     "/api/v1/observations",
			"vmaf_validation":  "/api/v1/validate/vmaf",
		},
		"features": []string{
			"image_quality_prediction",
			"video_quality_prediction",
			"lightgbm_model",
			"ppo_reinforcement_learning",
			"ab_testing",
			"online_learning",
			"model_versioning",
		},
		"supported_formats": map[string][]string{
			"image": {"jxl", "avif", "webp", "png", "jpeg"},
			"video": {"mp4", "mov", "avi", "mkv", "webm"},
		},
		"ai_models": map[string]interface{}{
			"lightgbm": map[string]interface{}{
				"type":    "gradient_boosting",
				"enabled": true,
				"features": []string{
					"quality_prediction",
					"parameter_optimization",
				},
			},
			"ppo": map[string]interface{}{
				"type":    "reinforcement_learning",
				"enabled": gw.modelRouter != nil,
				"features": []string{
					"adaptive_learning",
					"reward_based_optimization",
				},
			},
		},
	}
	gw.sendJSON(w, capabilities, http.StatusOK)
}

// handleObservations 处理观测数据（PPO训练用）
func (gw *HTTPGateway) handleObservations(w http.ResponseWriter, r *http.Request) {
	startTime := time.Now()

	if r.Method != http.MethodPost {
		gw.sendError(w, "Method not allowed", http.StatusMethodNotAllowed, startTime)
		return
	}

	// 解析观测数据
	var observation map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&observation); err != nil {
		gw.sendError(w, fmt.Sprintf("Invalid JSON: %v", err), http.StatusBadRequest, startTime)
		return
	}

	// 确保observations目录存在
	obsDir := "data/observations"
	if err := os.MkdirAll(obsDir, 0755); err != nil {
		Error("Observations", "创建observations目录失败: %v", err)
		gw.sendError(w, "Failed to create observations directory", http.StatusInternalServerError, startTime)
		return
	}

	// 生成文件名（时间戳 + 随机数）
	timestamp := time.Now().Format("20060102_150405")
	randomID := fmt.Sprintf("%04d", time.Now().UnixNano()%10000)
	filename := fmt.Sprintf("%s/%s_%s.json", obsDir, timestamp, randomID)

	// 保存观测数据
	obsJSON, err := json.MarshalIndent(observation, "", "  ")
	if err != nil {
		gw.sendError(w, "Failed to marshal observation", http.StatusInternalServerError, startTime)
		return
	}

	if err := os.WriteFile(filename, obsJSON, 0644); err != nil {
		Error("Observations", "保存观测数据失败: %v", err)
		gw.sendError(w, "Failed to save observation", http.StatusInternalServerError, startTime)
		return
	}

	// 统计观测总数
	files, _ := filepath.Glob(fmt.Sprintf("%s/*.json", obsDir))
	totalObservations := len(files)

	InfoWithContext("Observations", "观测数据已保存", map[string]interface{}{
		"filename": filepath.Base(filename),
		"total":    totalObservations,
	})

	// 返回成功响应
	response := map[string]interface{}{
		"success":            true,
		"message":            "Observation recorded",
		"total_observations": totalObservations,
		"file":               filename,
	}
	gw.sendJSON(w, response, http.StatusOK)
}

// 辅助函数

func (gw *HTTPGateway) sendJSON(w http.ResponseWriter, data interface{}, statusCode int) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(statusCode)
	json.NewEncoder(w).Encode(data)
}

// sendError 发送简单错误响应（无错误码）
func (gw *HTTPGateway) sendError(w http.ResponseWriter, message string, statusCode int, startTime time.Time) {
	elapsed := time.Since(startTime).Milliseconds()
	resp := PredictResponse{
		Success: false,
		Error:   message,
		TimeMs:  elapsed,
	}
	
	// 记录错误日志
	ErrorWithContext("HTTPGateway", message, map[string]interface{}{
		"status_code": statusCode,
		"duration_ms": elapsed,
	})
	
	gw.sendJSON(w, resp, statusCode)
}

// sendPixlyError 发送PixlyError响应（G-004）
func (gw *HTTPGateway) sendPixlyError(w http.ResponseWriter, err *PixlyError, httpStatus int, startTime time.Time) {
	elapsed := time.Since(startTime).Milliseconds()
	
	response := PredictResponse{
		Success:   false,
		Error:     err.Message,
		ErrorCode: err.Code,
		TimeMs:    elapsed,
	}
	
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(httpStatus)
	json.NewEncoder(w).Encode(response)
	
	ErrorWithContext("HTTPGateway", err.Message, map[string]interface{}{
		"code":      err.Code,
		"severity":  err.Severity,
		"http_code": httpStatus,
		"elapsed_ms": elapsed,
	})
}

func (gw *HTTPGateway) getQualityForMode(mode string) int {
	switch mode {
	case "size":
		return 80
	case "balanced":
		return 90
	case "quality":
		return 100
	default:
		return 90 // 默认平衡Mode
	}
}
