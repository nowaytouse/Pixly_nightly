// cmd/pixly/commands/errorhandling_helper.go - 错误恢复集成
package commands

import (
	"os"

	"pixly/pkg/errorhandling"

	"go.uber.org/zap"
)

// SmartErrorRecovery 智能错误恢复包装
type SmartErrorRecovery struct {
	recoveryMgr *errorhandling.ErrorRecoveryManager
	enabled     bool
	logger      *zap.Logger
}

// NewSmartErrorRecovery 创建智能错误恢复
func NewSmartErrorRecovery(logger *zap.Logger) *SmartErrorRecovery {
	// 检查是否启用错误恢复
	enabled := os.Getenv("PIXLY_ERROR_RECOVERY") == "true" ||
		os.Getenv("PIXLY_AUTO_RECOVERY") == "true"

	if !enabled {
		return &SmartErrorRecovery{
			enabled: false,
			logger:  logger,
		}
	}

	// 创建默认恢复配置
	config := &errorhandling.RecoveryConfig{
		MaxRetries:          3,
		EnableFallback:      true,
		EnableLearning:      false,
		AutoRecoveryEnabled: true,
	}
	mgr := errorhandling.NewErrorRecoveryManager(logger, config)

	return &SmartErrorRecovery{
		recoveryMgr: mgr,
		enabled:     true,
		logger:      logger,
	}
}

// TryRecover 尝试从错误中恢复
func (ser *SmartErrorRecovery) TryRecover(err error, filePath string) *RecoveryResult {
	result := &RecoveryResult{
		Success:   false,
		Recovered: false,
		Error:     err,
	}

	// 如果未启用,直接返回失败
	if !ser.enabled || ser.recoveryMgr == nil {
		return result
	}

	// 简化实现: errorhandling包API复杂
	// 包已添加到项目,可在未来深度集成
	// 当前只记录错误
	ser.logger.Warn("错误发生 (恢复功能待深度集成)",
		zap.String("file", filePath),
		zap.Error(err))

	return result
}

// GetStats 获取错误恢复统计
func (ser *SmartErrorRecovery) GetStats() map[string]interface{} {
	return map[string]interface{}{
		"enabled":   ser.enabled,
		"available": true, // 包已添加,API待集成
	}
}

// IsEnabled 是否启用
func (ser *SmartErrorRecovery) IsEnabled() bool {
	return ser.enabled
}

// RecoveryResult 恢复结果
type RecoveryResult struct {
	Success   bool   // 是否成功
	Recovered bool   // 是否恢复
	Message   string // 消息
	Error     error  // 错误
}
