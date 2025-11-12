// metadata.go - 元数据处理模块
//
// 功能说明：
// - 使用 exiftool 复制元数据
// - 复制 macOS Finder 元数据
// - 复制文件系统元数据（时间戳、权限）
//
// 版本: v3.0.0 (独立实现)
// 更新: 2025-10-29

package utils

import (
	"context"
	"fmt"
	"os/exec"
	"time"
)

// CopyMetadataWithTimeout 使用 exiftool 在超时内复制元数据
func CopyMetadataWithTimeout(ctx context.Context, src, dst string, timeoutSec int) error {
	// 创建带超时的上下文
	ctxWithTimeout, cancel := context.WithTimeout(ctx, time.Duration(timeoutSec)*time.Second)
	defer cancel()

	// 执行 exiftool
	cmd := exec.CommandContext(ctxWithTimeout, "exiftool",
		"-overwrite_original",
		"-TagsFromFile", src,
		"-all:all",
		"-P", // 保留文件修改时间
		dst)

	if err := cmd.Run(); err != nil {
		// 检查是否是超时
		if ctxWithTimeout.Err() == context.DeadlineExceeded {
			return fmt.Errorf("exiftool 超时（%d秒）", timeoutSec)
		}
		return fmt.Errorf("exiftool 失败: %w", err)
	}

	return nil
}

// CopyMetadata 使用 exiftool 复制元数据（无超时限制）
func CopyMetadata(inputPath, outputPath string) error {
	cmd := exec.Command("exiftool",
		"-overwrite_original",
		"-TagsFromFile", inputPath,
		"-all:all",
		"-P", // 保留文件修改时间
		outputPath)

	if err := cmd.Run(); err != nil {
		return fmt.Errorf("exiftool 复制元数据失败: %w", err)
	}

	return nil
}

// CopyFinderMetadata 复制 macOS Finder 的标签和注释
func CopyFinderMetadata(src, dst string) error {
	// 使用 xattr 复制扩展属性
	// 1. 清除目标文件的扩展属性
	exec.Command("xattr", "-c", dst).Run()

	// 2. 列出源文件的所有扩展属性
	cmd := exec.Command("xattr", "-l", src)
	output, err := cmd.Output()
	if err != nil || len(output) == 0 {
		// 没有扩展属性，不是错误
		return nil
	}

	// 3. 复制每个扩展属性
	// 简化实现：只复制常见的 Finder 属性
	finderAttrs := []string{
		"com.apple.metadata:_kMDItemUserTags",
		"com.apple.FinderInfo",
		"com.apple.metadata:kMDItemFinderComment",
	}

	for _, attr := range finderAttrs {
		// 读取属性
		readCmd := exec.Command("xattr", "-px", attr, src)
		value, err := readCmd.Output()
		if err != nil {
			continue // 属性不存在，跳过
		}

		// 写入属性
		writeCmd := exec.Command("xattr", "-wx", attr, string(value), dst)
		writeCmd.Run() // 尽力而为，不报错
	}

	return nil
}

// CopyFilesystemMetadata 复制文件系统元数据（时间戳、权限等）
func CopyFilesystemMetadata(src, dst string) error {
	// 获取源文件的元数据
	metadata, err := CaptureFileSystemMetadata(src)
	if err != nil {
		return fmt.Errorf("捕获源文件元数据失败: %w", err)
	}

	// 恢复到目标文件
	if err := metadata.Restore(dst); err != nil {
		return fmt.Errorf("恢复文件系统元数据失败: %w", err)
	}

	return nil
}
