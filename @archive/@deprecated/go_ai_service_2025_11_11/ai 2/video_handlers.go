package ai

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"time"
)

// ========================================================================
// 视频AI预测
// ========================================================================

// VideoPredictRequest 视频预测请求
type VideoPredictRequest struct {
	VideoPath    string               `json:"video_path"`
	OptimizeMode string               `json:"optimize_mode"` // size, balanced, quality
	Options      *VideoRequestOptions `json:"options,omitempty"`
}

// VideoRequestOptions 视频请求选项
type VideoRequestOptions struct {
	UseAdvancedAI     bool   `json:"use_advanced_ai"`    // 高级特征分析
	EnableTransformer bool   `json:"enable_transformer"` // 强制Transformer
	EnableVMAF        bool   `json:"enable_vmaf"`        // VMAF验证
	AllowHEVC         bool   `json:"allow_hevc"`         // 允许H.265
	PreferSpeed       bool   `json:"prefer_speed"`       // 速度优先
	TargetEncoder     string `json:"target_encoder"`     // 目标Encoder
}

// VideoPredictResponse 视频预测响应
type VideoPredictResponse struct {
	Success          bool                   `json:"success"`
	Params           *VideoParams           `json:"params,omitempty"`
	Features         map[string]interface{} `json:"features,omitempty"`
	VideoType        map[string]interface{} `json:"video_type,omitempty"` // 🔥 Phase 45.9: 视频类型识别
	AdvancedFeatures map[string]interface{} `json:"advanced_features,omitempty"`
	ComplexityScore  int                    `json:"complexity_score,omitempty"`
	UseTransformer   bool                   `json:"use_transformer,omitempty"`
	Mode             string                 `json:"mode"`
	Confidence       float64                `json:"confidence"`
	Error            string                 `json:"error,omitempty"`
	TimeMs           int64                  `json:"time_ms"`
}

// VideoParams 视频编码参数
type VideoParams struct {
	Encoder       string  `json:"encoder"`                  // h264, h265, av1, vp9
	CRF           int     `json:"crf"`                      // 恒定速率因子
	Preset        string  `json:"preset"`                   // 编码preset
	FPS           *int    `json:"fps"`                      // 帧率（null保持Original）
	Scale         *string `json:"scale"`                    // 分辨率缩放
	Channel       string  `json:"channel,omitempty"`        // lightgbm, transformer, rules
	TargetBitrate int     `json:"target_bitrate,omitempty"` // 🔥 Phase 45.9: 目标码率(kbps)
	MaxBitrate    int     `json:"max_bitrate,omitempty"`    // 🔥 Phase 45.9: 最大码率(kbps)
	TwoPass       bool    `json:"two_pass,omitempty"`       // 🔥 Phase 45.9: 两遍编码
	Confidence    float64 `json:"confidence,omitempty"`     // 预测置信度
}

// handleVideoPred视频AI预测处理器
func (gw *HTTPGateway) handleVideoPredict(w http.ResponseWriter, r *http.Request) {
	startTime := time.Now()

	if r.Method != http.MethodPost {
		gw.sendError(w, "Method not allowed", http.StatusMethodNotAllowed, startTime)
		return
	}

	// 解析请求
	var req VideoPredictRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		gw.sendError(w, fmt.Sprintf("Failed to parse JSON: %v", err), http.StatusBadRequest, startTime)
		return
	}

	// 验证参数
	if req.VideoPath == "" {
		gw.sendError(w, "video_path不能为空", http.StatusBadRequest, startTime)
		return
	}

	// 默认值
	if req.OptimizeMode == "" {
		req.OptimizeMode = "balanced"
	}

	InfoWithContext("VideoPredictor", "Video AI prediction request", map[string]interface{}{
		"mode":  req.OptimizeMode,
		"video": filepath.Base(req.VideoPath),
	})

	// 调用Python脚本进行预测
	result, err := gw.callVideoPredictScript(req)
	if err != nil {
		// 🔥 响亮报错：视频AI预测失败
		Error("VideoPredictor", "Video prediction failed: %v", err)
		gw.sendError(w, fmt.Sprintf("🚨 Video AI prediction failed: %v", err),
			http.StatusInternalServerError, startTime)
		return
	}

	result.TimeMs = time.Since(startTime).Milliseconds()
	InfoWithContext("VideoPredictor", "Video prediction succeeded", map[string]interface{}{
		"encoder":     result.Params.Encoder,
		"crf":         result.Params.CRF,
		"duration_ms": result.TimeMs,
	})

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(result)
}

// callVideoPredictScript 调用Python脚本
func (gw *HTTPGateway) callVideoPredictScript(req VideoPredictRequest) (*VideoPredictResponse, error) {
	// 准备options JSON
	options := map[string]interface{}{
		"use_advanced_ai":    true,
		"enable_transformer": false,
		"enable_vmaf":        false,
		"allow_hevc":         true,
		"prefer_speed":       true,
		"target_encoder":     nil,
	}

	if req.Options != nil {
		options["use_advanced_ai"] = req.Options.UseAdvancedAI
		options["enable_transformer"] = req.Options.EnableTransformer
		options["enable_vmaf"] = req.Options.EnableVMAF
		options["allow_hevc"] = req.Options.AllowHEVC
		options["prefer_speed"] = req.Options.PreferSpeed
		if req.Options.TargetEncoder != "" {
			options["target_encoder"] = req.Options.TargetEncoder
		}
	}

	optionsJSON, _ := json.Marshal(options)

	// 🔥 Phase 46: 直接使用v3.0主脚本（已替换为predict_video_params.py）
	scriptPath := filepath.Join("tools", "predict_video_params.py")

	// 检查脚本是否存在（回退机制保留）
	if _, err := os.Stat(scriptPath); os.IsNotExist(err) {
		Warning("VideoPredictor", "主脚本不存在，尝试使用legacy版本")
		scriptPath = filepath.Join("tools", "predict_video_params_legacy.py")
	}

	// 执行Python脚本
	cmd := exec.Command("python3", scriptPath, req.VideoPath, req.OptimizeMode, string(optionsJSON))
	output, err := cmd.CombinedOutput()
	if err != nil {
		return nil, fmt.Errorf("Python script execution failed: %v, output: %s", err, string(output))
	}

	// 解析输出
	var result VideoPredictResponse
	if err := json.Unmarshal(output, &result); err != nil {
		return nil, fmt.Errorf("Failed to parse Python output: %v, output: %s", err, string(output))
	}

	return &result, nil
}

// ❌ DELETED: getVideoFallbackParams
//
// 【质量宣言执行】
// - ❌ Fallback代码（让AI成为摆设）
// - ✅ 响亮报错 > 静默降级
//
// AI不可用时，应该直接返回错误，而不是静默降级到hardcode参数。

// ========================================================================
// VMAF质量验证
// ========================================================================

// VMAFRequest VMAF validation request
type VMAFRequest struct {
	OriginalPath  string `json:"original_path"`
	ConvertedPath string `json:"converted_path"`
	MinScore      int    `json:"min_score"` // 最低可接受Score
	UseModel      string `json:"use_model"` // VMAF模型版本
	NThreads      int    `json:"n_threads"` // 线程数
}

// VMAFResponse VMAF验证响应
type VMAFResponse struct {
	Success     bool         `json:"success"`
	Score       float64      `json:"score,omitempty"`
	Passed      bool         `json:"passed,omitempty"`
	MinScore    int          `json:"min_score,omitempty"`
	TargetScore int          `json:"target_score,omitempty"`
	Details     *VMAFDetails `json:"details,omitempty"`
	Error       string       `json:"error,omitempty"`
	TimeMs      int64        `json:"time_ms"`
}

// VMAFDetails VMAF详细信息
type VMAFDetails struct {
	Rating         string `json:"rating"`         // 评级：优秀/良好/可接受/较差
	Recommendation string `json:"recommendation"` // 优化建议
}

// handleVMAF VMAF验证处理器
func (gw *HTTPGateway) handleVMAF(w http.ResponseWriter, r *http.Request) {
	startTime := time.Now()

	if r.Method != http.MethodPost {
		gw.sendError(w, "Method not allowed", http.StatusMethodNotAllowed, startTime)
		return
	}

	// 解析请求
	var req VMAFRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		gw.sendError(w, fmt.Sprintf("Failed to parse JSON: %v", err), http.StatusBadRequest, startTime)
		return
	}

	// 验证参数
	if req.OriginalPath == "" || req.ConvertedPath == "" {
		gw.sendError(w, "original_path和converted_path不能为空", http.StatusBadRequest, startTime)
		return
	}

	// 默认值
	if req.MinScore == 0 {
		req.MinScore = 85
	}
	if req.UseModel == "" {
		req.UseModel = "vmaf_v0.6.1"
	}
	if req.NThreads == 0 {
		req.NThreads = 4
	}

	InfoWithContext("VMAF", "VMAF validation request", map[string]interface{}{
		"min_score": req.MinScore,
		"original": filepath.Base(req.OriginalPath),
		"converted": filepath.Base(req.ConvertedPath),
	})

	// 执行VMAF分析
	result, err := gw.runVMAFAnalysis(req)
	if err != nil {
		Error("VMAF", "VMAF validation failed: %v", err)
		errorResp := &VMAFResponse{
			Success: false,
			Error:   fmt.Sprintf("VMAF validation failed: %v", err),
			TimeMs:  time.Since(startTime).Milliseconds(),
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(errorResp)
		return
	}

	result.TimeMs = time.Since(startTime).Milliseconds()
	InfoWithContext("VMAF", "VMAF validation completed", map[string]interface{}{
		"score":       result.Score,
		"passed":      result.Passed,
		"duration_ms": result.TimeMs,
	})

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(result)
}

// runVMAFAnalysis 执行VMAF分析
func (gw *HTTPGateway) runVMAFAnalysis(req VMAFRequest) (*VMAFResponse, error) {
	// 检查ffmpeg是否支持libvmaf
	checkCmd := exec.Command("ffmpeg", "-filters")
	checkOutput, _ := checkCmd.CombinedOutput()
	if !stringContains(string(checkOutput), "libvmaf") && !stringContains(string(checkOutput), "vmaf") {
		return nil, fmt.Errorf("ffmpeg does not support libvmaf, please recompileffmpeg并启用--enable-libvmaf")
	}

	// 获取视频分辨率
	resolution, err := gw.getVideoResolution(req.ConvertedPath)
	if err != nil {
		return nil, fmt.Errorf("Failed to get video resolution: %v", err)
	}

	// 构建VMAF filter
	tempJSON := filepath.Join("/tmp", fmt.Sprintf("vmaf_%d.json", time.Now().Unix()))
	vmafFilter := fmt.Sprintf(
		"[0:v]scale=%d:%d:flags=bicubic[ref];[1:v]scale=%d:%d:flags=bicubic[dist];[dist][ref]libvmaf=log_fmt=json:log_path=%s:n_threads=%d",
		resolution.Width, resolution.Height,
		resolution.Width, resolution.Height,
		tempJSON, req.NThreads,
	)

	// 执行ffmpeg
	cmd := exec.Command(
		"ffmpeg",
		"-i", req.ConvertedPath,
		"-i", req.OriginalPath,
		"-lavfi", vmafFilter,
		"-f", "null", "-",
	)

	output, err := cmd.CombinedOutput()
	if err != nil {
		return nil, fmt.Errorf("ffmpeg execution failed: %v, output: %s", err, string(output))
	}

	// 解析VMAF JSON结果
	score, err := parseVMAFJSON(tempJSON)
	if err != nil {
		return nil, fmt.Errorf("Failed to parse VMAF result: %v", err)
	}

	// 构建响应
	passed := score >= float64(req.MinScore)
	response := &VMAFResponse{
		Success:     true,
		Score:       score,
		Passed:      passed,
		MinScore:    req.MinScore,
		TargetScore: 92,
		Details: &VMAFDetails{
			Rating:         getRating(score),
			Recommendation: getRecommendation(score, req.MinScore),
		},
	}

	return response, nil
}

// 辅助函数

type Resolution struct {
	Width  int
	Height int
}

func (gw *HTTPGateway) getVideoResolution(videoPath string) (*Resolution, error) {
	cmd := exec.Command(
		"ffprobe",
		"-v", "error",
		"-select_streams", "v:0",
		"-show_entries", "stream=width,height",
		"-of", "json",
		videoPath,
	)

	output, err := cmd.Output()
	if err != nil {
		return nil, err
	}

	var result struct {
		Streams []struct {
			Width  int `json:"width"`
			Height int `json:"height"`
		} `json:"streams"`
	}

	if err := json.Unmarshal(output, &result); err != nil {
		return nil, err
	}

	if len(result.Streams) == 0 {
		return nil, fmt.Errorf("Unable to get video stream info")
	}

	return &Resolution{
		Width:  result.Streams[0].Width,
		Height: result.Streams[0].Height,
	}, nil
}

func parseVMAFJSON(jsonPath string) (float64, error) {
	data, err := os.ReadFile(jsonPath)
	if err != nil {
		return 0, err
	}

	var vmafData struct {
		PooledMetrics struct {
			VMAF struct {
				Mean float64 `json:"mean"`
			} `json:"vmaf"`
		} `json:"pooled_metrics"`
	}

	if err := json.Unmarshal(data, &vmafData); err != nil {
		return 0, err
	}

	return vmafData.PooledMetrics.VMAF.Mean, nil
}

func getRating(score float64) string {
	if score >= 95 {
		return "优秀"
	} else if score >= 90 {
		return "良好"
	} else if score >= 85 {
		return "可接受"
	} else if score >= 75 {
		return "较差"
	}
	return "不可接受"
}

func getRecommendation(score float64, minScore int) string {
	targetScore := 92.0
	if score >= targetScore {
		return "质量优秀，无需调整"
	} else if score >= float64(minScore) {
		return "质量可接受，可考虑稍微降低CRF以获得更好质量"
	}
	return "质量不达标，建议降低CRF值或使用更慢的preset"
}

func stringContains(s, substr string) bool {
	return len(s) > 0 && len(substr) > 0 && (s == substr || len(s) >= len(substr) && (s[:len(substr)] == substr || s[len(s)-len(substr):] == substr || stringContains(s[1:], substr)))
}
