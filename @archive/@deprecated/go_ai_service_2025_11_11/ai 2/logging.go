// 🎯 Pixly统一日志系统 (Go)
//
// Phase 46.8: 三端共享的结构化日志
// 确保Go/Rust/JS使用相同的日志格式

package ai

import (
	"encoding/json"
	"fmt"
	"os"
	"runtime"
	"time"
)

// LogLevel 日志级别
type LogLevel string

const (
	LogLevelDebug   LogLevel = "DEBUG"
	LogLevelInfo    LogLevel = "INFO"
	LogLevelWarning LogLevel = "WARNING"
	LogLevelError   LogLevel = "ERROR"
)

// LogEntry 日志条目
type LogEntry struct {
	Timestamp string                 `json:"timestamp"`         // ISO 8601时间戳
	Level     LogLevel               `json:"level"`             // 日志级别
	Layer     string                 `json:"layer"`             // 层级标识 (go-ai)
	Component string                 `json:"component"`         // 组件名称
	Message   string                 `json:"message"`           // 日志消息
	Code      string                 `json:"code,omitempty"`    // 错误码（可选）
	Context   map[string]interface{} `json:"context,omitempty"` // 上下文信息
	TraceID   string                 `json:"trace_id,omitempty"` // 追踪ID（可选）
	File      string                 `json:"file,omitempty"`    // 源文件
	Line      int                    `json:"line,omitempty"`    // 行号
}

var (
	currentLogLevel = LogLevelInfo
	enableJSON      = false // 默认人类可读格式
)

// SetLogLevel 设置日志级别
func SetLogLevel(level LogLevel) {
	currentLogLevel = level
}

// EnableJSONLog 启用JSON格式日志
func EnableJSONLog(enable bool) {
	enableJSON = enable
}

// log 内部日志函数
func log(level LogLevel, component string, format string, args ...interface{}) {
	if !shouldLog(level) {
		return
	}

	// 获取调用位置
	_, file, line, _ := runtime.Caller(2)

	entry := LogEntry{
		Timestamp: time.Now().UTC().Format(time.RFC3339Nano),
		Level:     level,
		Layer:     "go-ai",
		Component: component,
		Message:   fmt.Sprintf(format, args...),
		File:      file,
		Line:      line,
		Context:   make(map[string]interface{}),
	}

	if enableJSON {
		outputJSON(entry)
	} else {
		outputHuman(entry)
	}
}

// logWithContext 带上下文的日志
func logWithContext(level LogLevel, component string, message string, context map[string]interface{}) {
	if !shouldLog(level) {
		return
	}

	_, file, line, _ := runtime.Caller(2)

	entry := LogEntry{
		Timestamp: time.Now().UTC().Format(time.RFC3339Nano),
		Level:     level,
		Layer:     "go-ai",
		Component: component,
		Message:   message,
		Context:   context,
		File:      file,
		Line:      line,
	}

	if enableJSON {
		outputJSON(entry)
	} else {
		outputHuman(entry)
	}
}

// logError 记录错误日志
func logError(err *PixlyError, component string) {
	if !shouldLog(LogLevelError) {
		return
	}

	_, file, line, _ := runtime.Caller(2)

	entry := LogEntry{
		Timestamp: time.Now().UTC().Format(time.RFC3339Nano),
		Level:     LogLevelError,
		Layer:     "go-ai",
		Component: component,
		Message:   err.Message,
		Code:      err.Code,
		Context:   err.Context,
		File:      file,
		Line:      line,
	}

	if enableJSON {
		outputJSON(entry)
	} else {
		outputHuman(entry)
	}
}

// 公共日志函数

// Debug 调试日志
func Debug(component string, format string, args ...interface{}) {
	log(LogLevelDebug, component, format, args...)
}

// Info 信息日志
func Info(component string, format string, args ...interface{}) {
	log(LogLevelInfo, component, format, args...)
}

// Warning 警告日志
func Warning(component string, format string, args ...interface{}) {
	log(LogLevelWarning, component, format, args...)
}

// Error 错误日志
func Error(component string, format string, args ...interface{}) {
	log(LogLevelError, component, format, args...)
}

// InfoWithContext 带上下文的信息日志
func InfoWithContext(component string, message string, context map[string]interface{}) {
	logWithContext(LogLevelInfo, component, message, context)
}

// WarningWithContext 带上下文的警告日志
func WarningWithContext(component string, message string, context map[string]interface{}) {
	logWithContext(LogLevelWarning, component, message, context)
}

// ErrorWithContext 带上下文的错误日志
func ErrorWithContext(component string, message string, context map[string]interface{}) {
	logWithContext(LogLevelError, component, message, context)
}

// LogPixlyError 记录PixlyError
func LogPixlyError(component string, err *PixlyError) {
	logError(err, component)
}

// LogValidationError 记录验证错误（便捷函数）
func LogValidationError(component string, err error) {
	if pixlyErr, ok := AsPixlyError(err); ok {
		logError(pixlyErr, component)
	} else {
		Error(component, "Validation error: %v", err)
	}
}

// LogValidationSuccess 记录验证成功（便捷函数）
func LogValidationSuccess(component string) {
	Info(component, "✅ Validation passed")
}

// LogValidationWarning 记录验证警告（便捷函数）
func LogValidationWarning(component string, message string, context map[string]interface{}) {
	WarningWithContext(component, message, context)
}

// 辅助函数

func shouldLog(level LogLevel) bool {
	levels := map[LogLevel]int{
		LogLevelDebug:   0,
		LogLevelInfo:    1,
		LogLevelWarning: 2,
		LogLevelError:   3,
	}
	return levels[level] >= levels[currentLogLevel]
}

func outputJSON(entry LogEntry) {
	data, err := json.Marshal(entry)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to marshal log entry: %v\n", err)
		return
	}
	fmt.Println(string(data))
}

func outputHuman(entry LogEntry) {
	// 颜色代码
	var color string
	switch entry.Level {
	case LogLevelDebug:
		color = "\033[36m" // 青色
	case LogLevelInfo:
		color = "\033[32m" // 绿色
	case LogLevelWarning:
		color = "\033[33m" // 黄色
	case LogLevelError:
		color = "\033[31m" // 红色
	default:
		color = "\033[0m" // 默认
	}
	reset := "\033[0m"

	// 基本格式：时间 [级别] 组件: 消息
	fmt.Printf("%s[%s%s%s] %s: %s",
		entry.Timestamp,
		color,
		entry.Level,
		reset,
		entry.Component,
		entry.Message,
	)

	// 如果有错误码，显示
	if entry.Code != "" {
		fmt.Printf(" [%s]", entry.Code)
	}

	// 如果有上下文，显示
	if len(entry.Context) > 0 {
		contextJSON, _ := json.Marshal(entry.Context)
		fmt.Printf(" | Context: %s", string(contextJSON))
	}

	fmt.Println()
}

// PerformanceLogger 性能日志辅助结构
type PerformanceLogger struct {
	component string
	operation string
	startTime time.Time
}

// StartPerformanceLog 开始性能监控
func StartPerformanceLog(component string, operation string) *PerformanceLogger {
	Info(component, "🚀 Starting: %s", operation)
	return &PerformanceLogger{
		component: component,
		operation: operation,
		startTime: time.Now(),
	}
}

// End 结束性能监控
func (p *PerformanceLogger) End() {
	elapsed := time.Since(p.startTime)
	Info(p.component, "✅ Completed: %s (took %s)", p.operation, elapsed)
}

// EndWithContext 结束性能监控并附加上下文
func (p *PerformanceLogger) EndWithContext(context map[string]interface{}) {
	elapsed := time.Since(p.startTime)
	context["duration_ms"] = elapsed.Milliseconds()
	InfoWithContext(p.component, fmt.Sprintf("✅ Completed: %s", p.operation), context)
}
