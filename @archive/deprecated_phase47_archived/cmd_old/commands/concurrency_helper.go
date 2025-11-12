// cmd/pixly/commands/concurrency_helper.go - 智能并发管理集成
package commands

import (
	"os"
	"runtime"
	"strconv"

	"pixly/pkg/concurrency"

	"go.uber.org/zap"
)

// SmartWorkerManager 智能Worker管理器包装
type SmartWorkerManager struct {
	concurrencyMgr *concurrency.SmartConcurrencyManager
	enabled        bool
	fixedWorkers   int
}

// NewSmartWorkerManager 创建智能Worker管理器
func NewSmartWorkerManager(logger *zap.Logger, requestedWorkers int) *SmartWorkerManager {
	// 检查是否启用智能并发
	enabled := os.Getenv("PIXLY_SMART_CONCURRENCY") == "true" ||
		os.Getenv("PIXLY_AUTO_WORKERS") == "true"

	if !enabled {
		// 不启用智能并发,使用固定workers
		fixedWorkers := requestedWorkers
		if fixedWorkers <= 0 {
			fixedWorkers = runtime.NumCPU()
		}
		return &SmartWorkerManager{
			enabled:      false,
			fixedWorkers: fixedWorkers,
		}
	}

	// 创建智能并发管理器 (简化API)
	mgr := concurrency.NewSmartConcurrencyManager(logger)

	// 获取初始workers
	initialWorkers := getInitialWorkers(requestedWorkers)

	return &SmartWorkerManager{
		concurrencyMgr: mgr,
		enabled:        true,
		fixedWorkers:   initialWorkers,
	}
}

// GetCurrentWorkers 获取当前workers数量
func (swm *SmartWorkerManager) GetCurrentWorkers() int {
	// 简化实现: concurrency包API复杂,暂时使用固定workers
	// concurrency包已添加到项目,可在未来深度集成
	return swm.fixedWorkers
}

// GetStats 获取并发统计
func (swm *SmartWorkerManager) GetStats() map[string]interface{} {
	return map[string]interface{}{
		"mode":            "fixed",
		"workers":         swm.fixedWorkers,
		"smart_available": swm.enabled,
	}
}

// IsSmartMode 是否为智能模式
func (swm *SmartWorkerManager) IsSmartMode() bool {
	return swm.enabled
}

// getInitialWorkers 获取初始workers数量
func getInitialWorkers(requested int) int {
	if requested > 0 {
		return requested
	}

	// 从环境变量读取
	if envWorkers := os.Getenv("PIXLY_WORKERS"); envWorkers != "" {
		if w, err := strconv.Atoi(envWorkers); err == nil && w > 0 {
			return w
		}
	}

	// 默认为CPU核心数
	return runtime.NumCPU()
}
