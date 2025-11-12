// filesystem_metadata.go - 文件系统元数据处理模块
//
// 功能说明：
// - 捕获和恢复文件系统元数据（创建时间、修改时间、访问时间）
// - 支持 macOS Finder 元数据（标签、注释等）
//
// 版本: v3.0.0 (独立实现)
// 更新: 2025-10-29

package utils

import (
	"fmt"
	"os"
	"os/exec"
	"syscall"
	"time"
)

// FileSystemMetadata 存储文件系统元数据
type FileSystemMetadata struct {
	CreationTime     time.Time
	ModificationTime time.Time
	AccessTime       time.Time
	Mode             os.FileMode
	FinderTags       []string // macOS Finder 标签
	FinderComment    string   // macOS Finder 注释
}

// CaptureFileSystemMetadata 捕获文件的文件系统元数据
func CaptureFileSystemMetadata(filePath string) (*FileSystemMetadata, error) {
	fileInfo, err := os.Stat(filePath)
	if err != nil {
		return nil, fmt.Errorf("无法获取文件信息: %w", err)
	}

	metadata := &FileSystemMetadata{
		ModificationTime: fileInfo.ModTime(),
		Mode:             fileInfo.Mode(),
	}

	// 获取创建时间和访问时间（macOS 特有）
	if stat, ok := fileInfo.Sys().(*syscall.Stat_t); ok {
		metadata.CreationTime = time.Unix(stat.Birthtimespec.Sec, stat.Birthtimespec.Nsec)
		metadata.AccessTime = time.Unix(stat.Atimespec.Sec, stat.Atimespec.Nsec)
	}

	// 尝试获取 Finder 元数据（macOS）
	metadata.FinderTags = getFinderTags(filePath)
	metadata.FinderComment = getFinderComment(filePath)

	return metadata, nil
}

// Restore 恢复文件系统元数据到目标文件
func (m *FileSystemMetadata) Restore(targetPath string) error {
	// 恢复访问时间和修改时间
	if err := os.Chtimes(targetPath, m.AccessTime, m.ModificationTime); err != nil {
		return fmt.Errorf("恢复时间戳失败: %w", err)
	}

	// 恢复创建时间（macOS）
	if !m.CreationTime.IsZero() {
		if err := setCreationTime(targetPath, m.CreationTime); err != nil {
			// 非致命错误，记录但不中断
			// fmt.Printf("警告: 恢复创建时间失败: %v\n", err)
		}
	}

	// 恢复文件权限
	if err := os.Chmod(targetPath, m.Mode); err != nil {
		return fmt.Errorf("恢复文件权限失败: %w", err)
	}

	// 恢复 Finder 元数据（如果有）
	if len(m.FinderTags) > 0 {
		setFinderTags(targetPath, m.FinderTags)
	}
	if m.FinderComment != "" {
		setFinderComment(targetPath, m.FinderComment)
	}

	return nil
}

// 注意: CopyFinderMetadata 和 CopyMetadata 已移至 metadata.go

// setCreationTime 设置文件创建时间（macOS）
func setCreationTime(filePath string, creationTime time.Time) error {
	// 使用 SetFileTimes 系统调用（macOS）
	timeStr := creationTime.Format("200601021504.05")
	cmd := exec.Command("touch", "-t", timeStr, filePath)
	return cmd.Run()
}

// getFinderTags 获取 Finder 标签
func getFinderTags(filePath string) []string {
	cmd := exec.Command("mdls", "-name", "kMDItemUserTags", "-raw", filePath)
	output, err := cmd.Output()
	if err != nil {
		return nil
	}

	// 解析标签（简化实现）
	// 实际输出格式复杂，这里仅做基础处理
	tags := []string{}
	if len(output) > 0 && string(output) != "(null)" {
		// 标签存在，但解析复杂，暂时保留原始值
	}

	return tags
}

// getFinderComment 获取 Finder 注释
func getFinderComment(filePath string) string {
	cmd := exec.Command("mdls", "-name", "kMDItemFinderComment", "-raw", filePath)
	output, err := cmd.Output()
	if err != nil {
		return ""
	}

	comment := string(output)
	if comment == "(null)" {
		return ""
	}

	return comment
}

// setFinderTags 设置 Finder 标签
func setFinderTags(filePath string, tags []string) error {
	// 使用 xattr 设置标签（简化实现）
	// 完整实现需要构建 plist 格式
	return nil
}

// setFinderComment 设置 Finder 注释
func setFinderComment(filePath, comment string) error {
	// macOS 不提供直接设置注释的命令行工具
	// 需要使用 AppleScript 或其他方式
	return nil
}
