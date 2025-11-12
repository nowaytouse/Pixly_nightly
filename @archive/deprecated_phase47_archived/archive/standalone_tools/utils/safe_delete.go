// safe_delete.go - 安全删除模块
//
// 功能说明：
// - 提供安全的文件删除功能
// - 在删除前验证目标文件存在且有效
//
// 版本: v3.0.0 (独立实现)
// 更新: 2025-10-29

package utils

import (
	"fmt"
	"os"
)

// SafeDelete 安全删除原始文件
//
// 参数:
//   - originalPath: 原始文件路径（要删除的文件）
//   - targetPath: 目标文件路径（转换后的文件，用于验证）
//   - logger: 日志函数
//
// 返回:
//   - error: 删除失败时返回错误
func SafeDelete(originalPath, targetPath string, logger func(format string, v ...interface{})) error {
	// 1. 检查原始文件是否存在
	if _, err := os.Stat(originalPath); os.IsNotExist(err) {
		return fmt.Errorf("原始文件不存在: %s", originalPath)
	}

	// 2. 检查目标文件是否存在且有效
	targetInfo, err := os.Stat(targetPath)
	if err != nil {
		return fmt.Errorf("目标文件验证失败: %w", err)
	}

	// 3. 检查目标文件不是空文件
	if targetInfo.Size() == 0 {
		return fmt.Errorf("目标文件为空，拒绝删除原始文件")
	}

	// 4. 检查原始文件和目标文件不是同一个文件
	if originalPath == targetPath {
		return fmt.Errorf("原始文件和目标文件相同，拒绝删除")
	}

	// 5. 执行删除
	if err := os.Remove(originalPath); err != nil {
		return fmt.Errorf("删除文件失败: %w", err)
	}

	if logger != nil {
		logger("🗑️  已删除原始文件: %s", originalPath)
	}

	return nil
}

// DeleteFile 直接删除文件（不进行安全检查）
func DeleteFile(filePath string) error {
	return os.Remove(filePath)
}

// FileExists 检查文件是否存在
func FileExists(filePath string) bool {
	_, err := os.Stat(filePath)
	return err == nil
}
