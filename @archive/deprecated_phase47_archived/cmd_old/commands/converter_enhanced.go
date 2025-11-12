// cmd/pixly/converter_enhanced.go - 增强的格式转换器

package commands

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

// ConversionPipeline 转换管道
type ConversionPipeline struct {
	inputFile    string
	outputFile   string
	inputFormat  string
	outputFormat string
	quality      int
	timeout      time.Duration
}

// NewConversionPipeline 创建转换管道
func NewConversionPipeline(inputFile, outputFile string, quality int) *ConversionPipeline {
	return &ConversionPipeline{
		inputFile:    inputFile,
		outputFile:   outputFile,
		inputFormat:  strings.ToLower(strings.TrimPrefix(filepath.Ext(inputFile), ".")),
		outputFormat: strings.ToLower(strings.TrimPrefix(filepath.Ext(outputFile), ".")),
		quality:      quality,
		timeout:      5 * time.Minute,
	}
}

// Convert 执行转换
func (cp *ConversionPipeline) Convert() error {
	// 检查是否需要中间格式
	if cp.needsIntermediateFormat() {
		return cp.convertWithIntermediate()
	}

	// 直接转换
	return cp.directConvert()
}

// needsIntermediateFormat 判断是否需要中间格式
func (cp *ConversionPipeline) needsIntermediateFormat() bool {
	// 定义哪些转换需要中间格式
	needsIntermediate := map[string]map[string]bool{
		"jxl": {
			"avif": true,
			"webp": true,
		},
		"avif": {
			"jxl":  true,
			"webp": true,
		},
		"webp": {
			"jxl":  true,
			"avif": true,
		},
		"gif": {
			"jxl":  true,
			"avif": true,
			"webp": true, // GIF动图需要特殊处理
		},
	}

	if formats, ok := needsIntermediate[cp.inputFormat]; ok {
		return formats[cp.outputFormat]
	}

	return false
}

// convertWithIntermediate 通过中间格式转换
func (cp *ConversionPipeline) convertWithIntermediate() error {
	// 创建临时PNG文件作为中间格式
	tempDir := os.TempDir()
	tempFile := filepath.Join(tempDir, fmt.Sprintf("pixly_temp_%d.png", time.Now().UnixNano()))
	defer os.Remove(tempFile)

	// 步骤1: 转换为PNG
	if err := cp.decodeToIntermediate(tempFile); err != nil {
		return fmt.Errorf("解码失败: %v", err)
	}

	// 步骤2: 从PNG转换为目标格式
	tempPipeline := &ConversionPipeline{
		inputFile:    tempFile,
		outputFile:   cp.outputFile,
		inputFormat:  "png",
		outputFormat: cp.outputFormat,
		quality:      cp.quality,
		timeout:      cp.timeout,
	}

	if err := tempPipeline.directConvert(); err != nil {
		return fmt.Errorf("编码失败: %v", err)
	}

	return nil
}

// decodeToIntermediate 解码为中间格式(PNG)
func (cp *ConversionPipeline) decodeToIntermediate(tempFile string) error {
	var cmd *exec.Cmd

	switch cp.inputFormat {
	case "jxl":
		// djxl input.jxl output.png
		cmd = exec.Command("djxl", cp.inputFile, tempFile)

	case "avif":
		// avifdec input.avif output.png
		cmd = exec.Command("avifdec", cp.inputFile, tempFile)

	case "webp":
		// dwebp input.webp -o output.png
		cmd = exec.Command("dwebp", cp.inputFile, "-o", tempFile)

	case "gif":
		// 使用 ffmpeg 提取第一帧 (或使用 ImageMagick)
		// ffmpeg -i input.gif -vframes 1 output.png
		cmd = exec.Command("ffmpeg", "-i", cp.inputFile, "-vframes", "1", tempFile, "-y")

	default:
		return fmt.Errorf("不支持的输入格式: %s", cp.inputFormat)
	}

	cmd.Env = os.Environ()
	output, err := cmd.CombinedOutput()
	if err != nil {
		return fmt.Errorf("%s 解码失败: %v\n输出: %s", cp.inputFormat, err, string(output))
	}

	return nil
}

// directConvert 直接转换
func (cp *ConversionPipeline) directConvert() error {
	var cmd *exec.Cmd

	switch cp.outputFormat {
	case "jxl":
		// cjxl input output.jxl --distance=X
		cmd = exec.Command("cjxl", cp.inputFile, cp.outputFile,
			fmt.Sprintf("--distance=%d", 100-cp.quality))

	case "avif":
		// avifenc input output.avif -s 4 -q X
		cmd = exec.Command("avifenc", cp.inputFile, cp.outputFile,
			"-s", "4",
			"-q", fmt.Sprintf("%d", cp.quality))

	case "webp":
		// cwebp input -q X -o output.webp
		cmd = exec.Command("cwebp", cp.inputFile,
			"-q", fmt.Sprintf("%d", cp.quality),
			"-o", cp.outputFile)

	default:
		return fmt.Errorf("不支持的输出格式: %s", cp.outputFormat)
	}

	cmd.Env = os.Environ()
	output, err := cmd.CombinedOutput()
	if err != nil {
		return fmt.Errorf("%s 编码失败: %v\n输出: %s", cp.outputFormat, err, string(output))
	}

	return nil
}

// ConvertFileEnhanced 增强的文件转换函数
func ConvertFileEnhanced(inputFile, outputFile string, quality int) error {
	pipeline := NewConversionPipeline(inputFile, outputFile, quality)
	return pipeline.Convert()
}

// CheckConverterTools 检查转换工具是否可用
func CheckConverterTools() map[string]bool {
	tools := map[string]bool{
		"cjxl":    false,
		"djxl":    false,
		"avifenc": false,
		"avifdec": false,
		"cwebp":   false,
		"dwebp":   false,
		"ffmpeg":  false,
	}

	for tool := range tools {
		if _, err := exec.LookPath(tool); err == nil {
			tools[tool] = true
		}
	}

	return tools
}

// GetMissingTools 获取缺失的工具列表
func GetMissingTools() []string {
	tools := CheckConverterTools()
	var missing []string

	for tool, available := range tools {
		if !available {
			missing = append(missing, tool)
		}
	}

	return missing
}
