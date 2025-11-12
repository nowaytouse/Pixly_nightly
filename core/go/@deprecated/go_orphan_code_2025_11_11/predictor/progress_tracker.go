package predictor

import (
	"fmt"
	"sync"
	"time"
)

// ProgressTracker 实时进度追踪器 (EMA算法)
// 🎯 灵感来自Squoosh的实时进度显示
// 使用指数移动平均(EMA)平滑估算剩余时间和速度
type ProgressTracker struct {
	mu sync.Mutex

	// 总体进度
	totalFiles     int
	processedFiles int
	startTime      time.Time

	// EMA速度追踪
	emaSpeed  float64 // 文件/秒
	alpha     float64 // EMA平滑因子 (0.2-0.3)
	lastCheck time.Time

	// 统计
	successCount int
	failCount    int
	skipCount    int
}

// NewProgressTracker 创建进度追踪器
func NewProgressTracker(totalFiles int) *ProgressTracker {
	return &ProgressTracker{
		totalFiles: totalFiles,
		startTime:  time.Now(),
		alpha:      0.25, // 平滑因子 (0.25 = 快速响应+平滑)
		lastCheck:  time.Now(),
		emaSpeed:   0, // 初始速度未知
	}
}

// Update 更新进度 (每处理一个文件调用一次)
func (pt *ProgressTracker) Update(success bool, skipped bool) {
	pt.mu.Lock()
	defer pt.mu.Unlock()

	pt.processedFiles++

	if skipped {
		pt.skipCount++
	} else if success {
		pt.successCount++
	} else {
		pt.failCount++
	}

	// 更新EMA速度
	now := time.Now()
	elapsed := now.Sub(pt.lastCheck).Seconds()

	if elapsed > 0 {
		// 当前瞬时速度 (文件/秒)
		currentSpeed := 1.0 / elapsed

		// EMA更新
		if pt.emaSpeed == 0 {
			// 第一次,直接使用当前速度
			pt.emaSpeed = currentSpeed
		} else {
			// EMA公式: new = α × current + (1-α) × old
			pt.emaSpeed = pt.alpha*currentSpeed + (1-pt.alpha)*pt.emaSpeed
		}
	}

	pt.lastCheck = now
}

// GetProgress 获取当前进度信息
func (pt *ProgressTracker) GetProgress() ProgressInfo {
	pt.mu.Lock()
	defer pt.mu.Unlock()

	remainingFiles := pt.totalFiles - pt.processedFiles
	percentage := float64(pt.processedFiles) / float64(pt.totalFiles) * 100

	// 计算剩余时间
	var remainingTime time.Duration
	if pt.emaSpeed > 0 && remainingFiles > 0 {
		remainingSeconds := float64(remainingFiles) / pt.emaSpeed
		remainingTime = time.Duration(remainingSeconds * float64(time.Second))
	}

	// 总耗时
	elapsed := time.Since(pt.startTime)

	// 平均速度 (基于总时间)
	var avgSpeed float64
	if elapsed.Seconds() > 0 {
		avgSpeed = float64(pt.processedFiles) / elapsed.Seconds()
	}

	return ProgressInfo{
		TotalFiles:     pt.totalFiles,
		ProcessedFiles: pt.processedFiles,
		RemainingFiles: remainingFiles,
		Percentage:     percentage,
		EMASpeed:       pt.emaSpeed,
		AvgSpeed:       avgSpeed,
		RemainingTime:  remainingTime,
		Elapsed:        elapsed,
		SuccessCount:   pt.successCount,
		FailCount:      pt.failCount,
		SkipCount:      pt.skipCount,
	}
}

// GetProgressString 获取进度字符串 (友好显示)
func (pt *ProgressTracker) GetProgressString() string {
	info := pt.GetProgress()

	return fmt.Sprintf(
		"[进度] %d/%d (%.1f%%) | 剩余: %s | 速度: %.1f文件/秒 | 成功: %d | 失败: %d",
		info.ProcessedFiles,
		info.TotalFiles,
		info.Percentage,
		formatDuration(info.RemainingTime),
		info.EMASpeed,
		info.SuccessCount,
		info.FailCount,
	)
}

// ProgressInfo 进度信息
type ProgressInfo struct {
	TotalFiles     int
	ProcessedFiles int
	RemainingFiles int
	Percentage     float64

	EMASpeed      float64       // EMA平滑速度 (文件/秒)
	AvgSpeed      float64       // 平均速度 (文件/秒)
	RemainingTime time.Duration // 预计剩余时间
	Elapsed       time.Duration // 已用时间

	SuccessCount int
	FailCount    int
	SkipCount    int
}

// formatDuration 格式化时间显示
func formatDuration(d time.Duration) string {
	if d == 0 {
		return "计算中..."
	}

	// 小于1分钟
	if d < time.Minute {
		return fmt.Sprintf("~%d秒", int(d.Seconds()))
	}

	// 1分钟-1小时
	if d < time.Hour {
		minutes := int(d.Minutes())
		seconds := int(d.Seconds()) % 60
		return fmt.Sprintf("~%d分%d秒", minutes, seconds)
	}

	// 大于1小时
	hours := int(d.Hours())
	minutes := int(d.Minutes()) % 60
	return fmt.Sprintf("~%d小时%d分", hours, minutes)
}
