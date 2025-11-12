// cmd/pixly/commands/progress_helper.go - 进度条集成辅助
package commands

import (
	"fmt"
	"os"
	"time"

	"pixly/pkg/progressui"

	"go.uber.org/zap"
)

// SimpleProgressUI 简化的进度UI包装
type SimpleProgressUI struct {
	advancedUI *progressui.AdvancedProgressUI
	logger     *zap.Logger
	enabled    bool
}

// NewSimpleProgressUI 创建简化进度UI
func NewSimpleProgressUI() *SimpleProgressUI {
	// 检查是否启用进度条
	enabled := os.Getenv("PIXLY_PROGRESS_UI") == "true" || os.Getenv("PIXLY_FANCY_PROGRESS") == "true"

	if !enabled {
		return &SimpleProgressUI{enabled: false}
	}

	// 创建logger
	logger, _ := zap.NewDevelopment()

	// 创建高级进度UI
	advancedUI := progressui.NewAdvancedProgressUI(logger)

	return &SimpleProgressUI{
		advancedUI: advancedUI,
		logger:     logger,
		enabled:    true,
	}
}

// StartScanning 开始扫描阶段
func (sp *SimpleProgressUI) StartScanning(totalFiles int64) {
	if !sp.enabled || sp.advancedUI == nil {
		return
	}
	sp.advancedUI.StartScanningPhase(totalFiles)
}

// UpdateScanning 更新扫描进度
func (sp *SimpleProgressUI) UpdateScanning(scannedCount int64) {
	if !sp.enabled || sp.advancedUI == nil {
		return
	}
	sp.advancedUI.UpdateScanProgress(scannedCount)
}

// StartProcessing 开始处理阶段
func (sp *SimpleProgressUI) StartProcessing(totalFiles int64) {
	if !sp.enabled || sp.advancedUI == nil {
		return
	}

	// 直接开始处理阶段 (扫描阶段会自动结束)
	sp.advancedUI.StartProcessingPhase(totalFiles)
}

// UpdateProcessing 更新处理进度
func (sp *SimpleProgressUI) UpdateProcessing(processed, success, failed, skipped int64) {
	if !sp.enabled || sp.advancedUI == nil {
		return
	}
	// UpdateProcessingProgress需要processed, success, skipped, failed, throughputMB
	sp.advancedUI.UpdateProcessingProgress(processed, success, skipped, failed, 0.0)
}

// UpdateStats 更新统计信息 (简化版)
func (sp *SimpleProgressUI) UpdateStats(success, failed, skipped int64) {
	if !sp.enabled || sp.advancedUI == nil {
		return
	}
	// 使用UpdateProcessing
	total := success + failed + skipped
	sp.UpdateProcessing(total, success, failed, skipped)
}

// Complete 完成所有进度
func (sp *SimpleProgressUI) Complete() {
	if !sp.enabled || sp.advancedUI == nil {
		return
	}

	// 完成处理阶段
	sp.advancedUI.CompleteProcessing()

	// 等待显示完成
	time.Sleep(200 * time.Millisecond)

	// 清理
	if sp.logger != nil {
		sp.logger.Sync()
	}
}

// ShowSummary 显示摘要
func (sp *SimpleProgressUI) ShowSummary(totalFiles, success, failed, skipped int64, totalSize, savedSize int64, duration time.Duration) {
	if !sp.enabled {
		// 标准文本输出
		fmt.Printf("\nStatistics Summary:\n")
		fmt.Printf("  Total: %d files\n", totalFiles)
		fmt.Printf("  Success: %d (%.1f%%)\n", success, float64(success)/float64(totalFiles)*100)
		fmt.Printf("  Failed: %d\n", failed)
		fmt.Printf("  Skipped: %d\n", skipped)
		fmt.Printf("  Original size: %.2f GB\n", float64(totalSize)/1024/1024/1024)
		fmt.Printf("  Compressed: %.2f GB\n", float64(savedSize)/1024/1024/1024)
		fmt.Printf("  Saved: %.1f%%\n", float64(totalSize-savedSize)/float64(totalSize)*100)
		fmt.Printf("  Duration: %v\n", duration)
		fmt.Printf("  Speed: %.1f files/s\n", float64(totalFiles)/duration.Seconds())
		return
	}

	// 高级UI已经显示了,不需要重复
}

// IsEnabled 是否启用
func (sp *SimpleProgressUI) IsEnabled() bool {
	return sp.enabled
}
