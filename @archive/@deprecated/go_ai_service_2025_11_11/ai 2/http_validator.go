package ai

import (
	"fmt"
	"os"
	"strings"
)

// 🔥 Phase 46.7: HTTP API参数验证器
//
// 适配HTTP API的验证层，将HTTP请求转换为验证格式
// 核心原则：响亮报错 > 静默降级

// ValidateHTTPPredictRequest 验证HTTP预测请求
//
// 🚨 响亮报错原则：
// - 任何无效参数都会返回错误
// - 错误消息清晰明确
// - 不使用默认值覆盖无效输入
func ValidateHTTPPredictRequest(req *PredictRequest) error {
	// 1. 验证图像路径
	if err := validateImagePath(req.ImagePath); err != nil {
		return fmt.Errorf("图像路径验证失败: %w", err)
	}

	// 2. 验证工具名称
	if req.Tool != "" {
		if err := validateToolName(req.Tool); err != nil {
			return fmt.Errorf("工具名称验证失败: %w", err)
		}
	}

	// 3. 验证目标质量
	if req.TargetQuality != 0 {
		if err := validateTargetQuality(req.TargetQuality); err != nil {
			return fmt.Errorf("目标质量验证失败: %w", err)
		}
	}

	// 4. 验证优化模式
	if req.OptimizeMode != "" {
		if err := validateOptimizeMode(req.OptimizeMode); err != nil {
			return fmt.Errorf("优化模式验证失败: %w", err)
		}
	}

	// 5. 验证选项
	if req.Options != nil {
		if err := validateRequestOptions(req.Options); err != nil {
			return fmt.Errorf("请求选项验证失败: %w", err)
		}
	}

	return nil
}

// validateImagePath 验证图像路径
func validateImagePath(path string) error {
	if strings.TrimSpace(path) == "" {
		return fmt.Errorf("图像路径为空")
	}

	// 检查文件是否存在
	if _, err := os.Stat(path); os.IsNotExist(err) {
		return fmt.Errorf("图像文件不存在: %s", path)
	}

	// 检查文件扩展名
	validExtensions := map[string]bool{
		".jpg":  true,
		".jpeg": true,
		".png":  true,
		".webp": true,
		".gif":  true,
		".bmp":  true,
		".tiff": true,
		".tif":  true,
		".avif": true,
		".jxl":  true,
		".heic": true,
		".heif": true,
	}

	ext := strings.ToLower(getFileExtension(path))
	if !validExtensions[ext] {
		return fmt.Errorf("不支持的图像格式: %s (支持: jpg/png/webp/gif/bmp/tiff/avif/jxl/heic)", ext)
	}

	return nil
}

// getFileExtension 获取文件扩展名
func getFileExtension(path string) string {
	for i := len(path) - 1; i >= 0; i-- {
		if path[i] == '.' {
			return path[i:]
		}
		if path[i] == '/' || path[i] == '\\' {
			break
		}
	}
	return ""
}

// validateToolName 验证工具名称
func validateToolName(tool string) error {
	if strings.TrimSpace(tool) == "" {
		return fmt.Errorf("工具名称为空")
	}

	validTools := map[string]bool{
		"jxl":     true,
		"cjxl":    true,
		"avif":    true,
		"avifenc": true,
		"webp":    true,
		"cwebp":   true,
		"ffmpeg":  true,
		"magick":  true,
	}

	toolLower := strings.ToLower(strings.TrimSpace(tool))
	if !validTools[toolLower] {
		return fmt.Errorf("不支持的工具: %s (支持: jxl/cjxl/avif/avifenc/webp/cwebp/ffmpeg/magick)", tool)
	}

	return nil
}

// validateTargetQuality 验证目标质量
func validateTargetQuality(quality int) error {
	if quality < 1 || quality > 100 {
		return fmt.Errorf("目标质量超出范围: %d (应为1-100)", quality)
	}

	return nil
}

// validateOptimizeMode 验证优化模式
func validateOptimizeMode(mode string) error {
	if strings.TrimSpace(mode) == "" {
		return nil // 空值允许，会使用默认值
	}

	validModes := map[string]bool{
		"size":      true,
		"balanced":  true,
		"quality":   true,
		"universal": true,
		"fast":      true,
		"extreme":   true,
	}

	modeLower := strings.ToLower(strings.TrimSpace(mode))
	if !validModes[modeLower] {
		return fmt.Errorf("无效的优化模式: %s (支持: size/balanced/quality/universal/fast/extreme)", mode)
	}

	return nil
}

// validateRequestOptions 验证请求选项
func validateRequestOptions(opts *RequestOptions) error {
	if opts == nil {
		return nil
	}

	// 验证期望格式
	if opts.ExpectedFormat != "" {
		validFormats := map[string]bool{
			"jpg":  true,
			"jpeg": true,
			"png":  true,
			"webp": true,
			"avif": true,
			"jxl":  true,
			"gif":  true,
			"heic": true,
		}

		formatLower := strings.ToLower(strings.TrimSpace(opts.ExpectedFormat))
		if !validFormats[formatLower] {
			return fmt.Errorf("无效的期望格式: %s (支持: jpg/png/webp/avif/jxl/gif/heic)", opts.ExpectedFormat)
		}
	}

	// 选项的布尔值默认都是false，不需要额外验证

	return nil
}

// ValidateHTTPPredictResponse 验证HTTP预测响应（用于内部检查）
//
// 🚨 确保AI返回的参数合理
func ValidateHTTPPredictResponse(resp *PredictResponse) error {
	if resp == nil {
		return fmt.Errorf("响应为空")
	}

	// 验证参数
	if resp.Params != nil {
		if err := validatePredictParams(resp.Params); err != nil {
			return fmt.Errorf("响应参数无效: %w", err)
		}
	}

	// 验证置信度
	if resp.Confidence < 0.0 || resp.Confidence > 1.0 {
		return fmt.Errorf("置信度超出范围: %f (应为0.0-1.0)", resp.Confidence)
	}

	// 验证推理时间
	if resp.InferenceTimeMs < 0 {
		return fmt.Errorf("推理时间无效: %f (必须 >= 0)", resp.InferenceTimeMs)
	}

	return nil
}

// validatePredictParams 验证预测参数
func validatePredictParams(params *PredictParams) error {
	if params == nil {
		return nil
	}

	// 验证质量
	if params.Quality < 0 || params.Quality > 100 {
		return fmt.Errorf("质量参数超出范围: %d (应为0-100)", params.Quality)
	}

	// 验证速度/effort
	if params.Effort < 0 || params.Effort > 10 {
		return fmt.Errorf("速度参数超出范围: %d (应为0-10)", params.Effort)
	}

	// 验证距离 (for JXL)
	if params.Distance < 0.0 || params.Distance > 15.0 {
		return fmt.Errorf("距离参数超出范围: %f (应为0.0-15.0)", params.Distance)
	}

	// 验证Quantizer (for AVIF)
	if params.Quantizer < 0 || params.Quantizer > 63 {
		return fmt.Errorf("Quantizer参数超出范围: %d (应为0-63)", params.Quantizer)
	}

	// 验证Speed (for AVIF)
	if params.Speed < 0 || params.Speed > 10 {
		return fmt.Errorf("Speed参数超出范围: %d (应为0-10)", params.Speed)
	}

	// 验证Method (for WebP)
	// Phase 46.12: Method改为string类型
	if params.Method != "" {
		validMethods := []string{"fast", "balanced", "quality", "maximum", "default", ""}
		isValid := false
		for _, vm := range validMethods {
			if params.Method == vm {
				isValid = true
				break
			}
		}
		if !isValid {
			return fmt.Errorf("Method参数无效: %s (应为fast/balanced/quality/maximum/default)", params.Method)
		}
	}

	// 验证Confidence
	if params.Confidence < 0.0 || params.Confidence > 1.0 {
		return fmt.Errorf("置信度超出范围: %f (应为0.0-1.0)", params.Confidence)
	}

	return nil
}

// 🔥 Phase 46.8: LogValidation函数已移至统一logging.go
// 使用logging.LogValidationError(), logging.LogValidationSuccess(), logging.LogValidationWarning()
