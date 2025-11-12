package ai

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"time"
)

// PythonBridge Python AI推理桥接器
type PythonBridge struct {
	pythonPath string
	scriptPath string
	modelDir   string
}

// NewPythonBridge 创建Python桥接器
func NewPythonBridge(modelDir string) *PythonBridge {
	// 🔥 优先使用系统Python（更可靠）
	pythonPath := "python3"
	
	// 获取当前工作目录
	cwd, err := os.Getwd()
	if err != nil {
		cwd = "."
	}
	
	// 🔥 智能查找Python脚本路径
	// Phase 46.14: 修正查找顺序
	possiblePaths := []string{
		// 1. 从bin/ai-service启动：../../../../tools/predict_params.py (bin/ai-service -> core/go -> core -> Pixly_Nightly -> tools)
		filepath.Join(cwd, "..", "..", "..", "..", "tools", "predict_params.py"),
		// 2. 从core/go/启动：../../tools/predict_params.py
		filepath.Join(cwd, "..", "..", "tools", "predict_params.py"),
		// 3. 从项目根目录启动：./tools/predict_params.py
		filepath.Join(cwd, "tools", "predict_params.py"),
		// 4. 从core/启动：../tools/predict_params.py
		filepath.Join(cwd, "..", "tools", "predict_params.py"),
	}
	
	var scriptPath string
	for _, path := range possiblePaths {
		if _, err := os.Stat(path); err == nil {
			scriptPath = path
			fmt.Printf("👍 [Python Bridge] Found script: %s\n", path)
			break
		}
	}
	
	if scriptPath == "" {
		// 如果都找不到，使用默认路径并警告
		scriptPath = "./tools/predict_params.py"
		fmt.Printf("⚠️ [Python Bridge] Script not found, using default: %s\n", scriptPath)
		fmt.Printf("   Current dir: %s\n", cwd)
		fmt.Printf("   Searched paths: %v\n", possiblePaths)
	}

	bridge := &PythonBridge{
		pythonPath: pythonPath,
		scriptPath: scriptPath,
		modelDir:   modelDir,
	}
	
	// 🔥 启动时输出配置信息
	fmt.Printf("✅ [Python Bridge] Initialized\n")
	fmt.Printf("   Python: %s\n", pythonPath)
	fmt.Printf("   Script: %s\n", scriptPath)
	fmt.Printf("   CWD: %s\n", cwd)
	
	return bridge
}

// PredictParams 预测参数
// Phase 46.12: 修正Method类型以匹配Rust期望
type PredictParams struct {
	Quality    int     `json:"quality"`
	Distance   float64 `json:"distance"`
	Effort     int     `json:"effort"`
	Quantizer  int     `json:"quantizer"`
	Speed      int     `json:"speed"`
	Method     string  `json:"method"`      // 🔄 Phase 46.12: int → string
	Confidence float64 `json:"confidence"`
}

// PythonPredictResult Python推理结果
type PythonPredictResult struct {
	Params            PredictParams          `json:"params"`
	Advanced          map[string]interface{} `json:"advanced,omitempty"`           // 🆕 高级参数
	Merged            map[string]interface{} `json:"merged,omitempty"`             // 🆕 合并参数
	Features          map[string]interface{} `json:"features,omitempty"`           // 特征分析
	Confidence        float64                `json:"confidence,omitempty"`         // 🆕 置信度
	RecommendedFormat string                 `json:"recommended_format,omitempty"` // 🎨 推荐格式
	FormatReason      string                 `json:"format_reason,omitempty"`      // 🎨 推荐原因
	UsedFormat        string                 `json:"used_format,omitempty"`        // 实际使用格式
	IsCustomFormat    bool                   `json:"is_custom_format,omitempty"`   // 是否自定义格式
}

// PyPredictOptions Python预测选项（避免与ai.PredictOptions冲突）
// Phase 46.11: 修正字段名以匹配Rust发送的JSON
type PyPredictOptions struct {
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

// ExecutionOptions 执行选项（增强错误处理）
type ExecutionOptions struct {
	Timeout      time.Duration // 超时时间（默认30秒）
	RetryCount   int           // 重试次数（默认0）
	RetryDelay   time.Duration // 重试延迟（默认1秒）
	EnableStderr bool          // 是否捕获stderr（默认true）
}

// Predict 执行Python AI推理（增强错误处理）
func (pb *PythonBridge) Predict(imagePath, tool string, targetQuality int, optimizeMode string, options *PyPredictOptions) (*PythonPredictResult, error) {
	// 设置默认执行选项
	execOpts := &ExecutionOptions{
		Timeout:      30 * time.Second,
		RetryCount:   0,
		RetryDelay:   1 * time.Second,
		EnableStderr: true,
	}

	// 构建命令参数
	args := []string{
		pb.scriptPath,
		imagePath,
		tool,
		fmt.Sprintf("%d", targetQuality),
		optimizeMode,
	}

	// 如果有options，转换为JSON字符串
	if options != nil {
		optionsJSON, err := json.Marshal(options)
		if err != nil {
			return nil, &PythonBridgeError{
				Op:      "marshal_options",
				Path:    imagePath,
				Message: "options序列化失败",
				Err:     err,
			}
		}
		args = append(args, string(optionsJSON))
	}

	// 重试机制
	var lastErr error
	for attempt := 0; attempt <= execOpts.RetryCount; attempt++ {
		if attempt > 0 {
			time.Sleep(execOpts.RetryDelay)
		}

		// 执行命令（带超时）
		ctx, cancel := context.WithTimeout(context.Background(), execOpts.Timeout)
		defer cancel()

		cmd := exec.CommandContext(ctx, pb.pythonPath, args...)
		output, err := cmd.Output()

		// 处理超时错误
		if ctx.Err() == context.DeadlineExceeded {
			lastErr = &PythonBridgeError{
				Op:      "execute",
				Path:    imagePath,
				Message: fmt.Sprintf("Python推理超时 (尝试 %d/%d)", attempt+1, execOpts.RetryCount+1),
				Err:     ctx.Err(),
			}
			continue
		}

		// 处理执行错误
		if err != nil {
			if exitErr, ok := err.(*exec.ExitError); ok && execOpts.EnableStderr {
				lastErr = &PythonBridgeError{
					Op:      "execute",
					Path:    imagePath,
					Message: fmt.Sprintf("Python推理失败 (尝试 %d/%d)", attempt+1, execOpts.RetryCount+1),
					Err:     err,
					Stderr:  string(exitErr.Stderr),
				}
			} else {
				lastErr = &PythonBridgeError{
					Op:      "execute",
					Path:    imagePath,
					Message: fmt.Sprintf("Python推理失败 (尝试 %d/%d)", attempt+1, execOpts.RetryCount+1),
					Err:     err,
				}
			}
			continue
		}

		// 解析JSON结果
		var result PythonPredictResult
		if err := json.Unmarshal(output, &result); err != nil {
			lastErr = &PythonBridgeError{
				Op:      "unmarshal",
				Path:    imagePath,
				Message: "JSON解析失败",
				Err:     err,
				Stdout:  string(output),
			}
			continue
		}

		// 成功
		return &result, nil
	}

	return nil, lastErr
}

// BatchPredict 批量预测（增强错误处理）
func (pb *PythonBridge) BatchPredict(images []string, tool string, targetQuality int, optimizeMode string, options *PyPredictOptions) ([]*PythonPredictResult, error) {
	results := make([]*PythonPredictResult, 0, len(images))
	errors := make([]error, 0)

	for i, img := range images {
		result, err := pb.Predict(img, tool, targetQuality, optimizeMode, options)
		if err != nil {
			errors = append(errors, fmt.Errorf("图像 %d (%s): %w", i+1, img, err))
			continue
		}
		results = append(results, result)
	}

	// 如果有错误，返回汇总错误
	if len(errors) > 0 {
		return results, &BatchPredictError{
			Total:   len(images),
			Success: len(results),
			Failed:  len(errors),
			Errors:  errors,
		}
	}

	return results, nil
}

// CheckHealth 检查Python环境健康度（增强错误处理）
func (pb *PythonBridge) CheckHealth() error {
	// 检查Python是否可用
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	cmd := exec.CommandContext(ctx, pb.pythonPath, "--version")
	if err := cmd.Run(); err != nil {
		return &PythonBridgeError{
			Op:      "health_check",
			Message: "Python不可用或版本检查失败",
			Err:     err,
		}
	}

	// 🔥 修复：使用os.Stat检查脚本文件（不是LookPath）
	if _, err := os.Stat(pb.scriptPath); err != nil {
		return &PythonBridgeError{
			Op:      "health_check",
			Path:    pb.scriptPath,
			Message: "推理脚本不存在",
			Err:     err,
		}
	}

	return nil
}

// PythonBridgeError 自定义错误类型（增强诊断）
type PythonBridgeError struct {
	Op      string // 操作: "execute", "unmarshal", "health_check"
	Path    string // 文件路径
	Message string // 错误消息
	Err     error  // 原始错误
	Stdout  string // 标准输出（用于调试）
	Stderr  string // 标准错误（用于调试）
}

func (e *PythonBridgeError) Error() string {
	if e.Stderr != "" {
		return fmt.Sprintf("%s: %s (stderr: %s)", e.Op, e.Message, e.Stderr)
	}
	if e.Stdout != "" {
		return fmt.Sprintf("%s: %s (stdout: %s)", e.Op, e.Message, e.Stdout)
	}
	if e.Err != nil {
		return fmt.Sprintf("%s: %s: %v", e.Op, e.Message, e.Err)
	}
	return fmt.Sprintf("%s: %s", e.Op, e.Message)
}

func (e *PythonBridgeError) Unwrap() error {
	return e.Err
}

// BatchPredictError 批量预测错误
type BatchPredictError struct {
	Total   int
	Success int
	Failed  int
	Errors  []error
}

func (e *BatchPredictError) Error() string {
	return fmt.Sprintf("批量预测部分失败: 总数=%d, 成功=%d, 失败=%d", e.Total, e.Success, e.Failed)
}
