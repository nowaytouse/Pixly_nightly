package main

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"syscall"
	"time"

	"pixly/pkg/converter"
	"pixly/pkg/validation"
	"pixly/utils"
)

// MOVConverter 实现 Converter 接口，用于视频重新包装为 MOV
type MOVConverter struct {
	*converter.BaseConverter
}

// NewMOVConverter 创建新的 MOV 转换器
func NewMOVConverter(opts converter.ConvertOptions) *MOVConverter {
	return &MOVConverter{
		BaseConverter: converter.NewBaseConverter(opts),
	}
}

// Convert 重新包装视频为 MOV 容器
func (c *MOVConverter) Convert(ctx context.Context, input string, output string) error {
	// ⚠️ 步骤0: 【关键】在转换前先捕获源文件的文件系统元数据
	srcInfo, _ := os.Stat(input)
	var creationTime, modTime time.Time
	if srcInfo != nil {
		modTime = srcInfo.ModTime()
		if stat, ok := srcInfo.Sys().(*syscall.Stat_t); ok {
			creationTime = time.Unix(stat.Birthtimespec.Sec, stat.Birthtimespec.Nsec)
		}
	}

	// 步骤1: 使用 ffmpeg 重新包装视频（保留原始编码）
	// -c copy: 复制流，不重新编码
	// -map_metadata 0: 复制所有元数据流
	// -movflags +faststart: 优化web播放
	cmd := exec.CommandContext(ctx, "ffmpeg",
		"-i", input,
		"-c", "copy", // 复制编码，不重新编码
		"-map_metadata", "0", // 复制元数据
		"-movflags", "+faststart", // Web优化
		"-y", // 覆盖输出文件
		output)

	output_bytes, err := cmd.CombinedOutput()
	if err != nil {
		return fmt.Errorf("ffmpeg 重新包装失败: %w\n输出: %s", err, string(output_bytes))
	}

	// 步骤2: 使用 exiftool 复制详细元数据（ffmpeg -map_metadata 可能不完整）
	// exiftool 能更全面地复制各类元数据
	if c.Options.PreserveMetadata {
		if err := utils.CopyMetadata(input, output); err != nil {
			logger.Printf("⚠️  exiftool元数据复制失败: %v", err)
			// 不中断流程，继续
		}
	}

	// 步骤3: 复制 Finder 元数据（标签、注释等）
	if c.Options.PreserveMetadata {
		if err := utils.CopyFinderMetadata(input, output); err != nil {
			logger.Printf("⚠️  Finder元数据复制失败: %v", err)
		}
	}

	// ✅ 步骤4: 恢复文件系统元数据（创建时间、修改时间）
	if srcInfo != nil {
		// 恢复修改时间
		if err := os.Chtimes(output, modTime, modTime); err != nil {
			logger.Printf("⚠️  文件时间恢复失败 %s: %v", filepath.Base(output), err)
		}

		// 恢复创建时间（macOS）
		if !creationTime.IsZero() {
			timeStr := creationTime.Format("200601021504.05")
			exec.Command("touch", "-t", timeStr, output).Run()
		}
	}

	return nil
}

// Validate 验证重新包装结果
func (c *MOVConverter) Validate(original string, converted string) error {
	if !c.Options.StrictMode {
		// 非严格模式，只检查文件存在和容器格式
		if _, err := os.Stat(converted); err != nil {
			return fmt.Errorf("输出文件不存在: %w", err)
		}

		// 检查是否是 MOV 格式
		ext := filepath.Ext(converted)
		if ext != ".mov" {
			return fmt.Errorf("输出文件不是 MOV 格式: %s", ext)
		}

		return nil
	}

	// 严格模式：使用视频验证系统
	return validation.ValidateVideoConversion(
		original,
		converted,
		validation.VideoValidationOptions{
			StrictMode:        true,
			TimeoutSeconds:    c.Options.TimeoutSeconds,
			CheckResolution:   true,
			CheckDuration:     true,
			CheckCodec:        false, // 重新包装不改变编码
			CheckBitrate:      false, // 复制流，比特率可能略有变化
			AllowDurationDiff: 0.1,   // 允许0.1秒的时长差异
		},
	)
}

// GetSupportedExtensions 获取支持的输入扩展名
func (c *MOVConverter) GetSupportedExtensions() []string {
	return []string{".mp4", ".avi", ".mkv", ".webm", ".m4v", ".flv", ".wmv", ".mpg", ".mpeg"}
}

// GetOutputExtension 获取输出扩展名
func (c *MOVConverter) GetOutputExtension() string {
	return ".mov"
}

