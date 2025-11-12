// Package logging 提供统一的日志管理
// 遵循CROSS_PLATFORM_LOG_SPEC.md规范
// 与JS插件和Rust内核保持一致的日志级别和格式
//
// ⚠️ DEPRECATED (2025-11-11)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// 本包已被废弃，请使用 pixly/ai 包中的统一日志系统。
// 
// 迁移指南:
//   旧: logging.Info("message", fields)
//   新: ai.Info("component", "message")
//       ai.InfoWithContext("component", "message", context)
//
// 参见: core/go/ai/logging.go (统一结构化日志)
//      docs/architecture/PROJECT_QUALITY_MANIFESTO.md (三端统一日志规范)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
package logging

import (
	"os"
	"time"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

// LogLevel 统一日志级别（与JS/Rust一致）
type LogLevel int

const (
	ErrorLevel LogLevel = 0
	WarnLevel  LogLevel = 1
	InfoLevel  LogLevel = 2
	DebugLevel LogLevel = 3
	TraceLevel LogLevel = 4
)

// String 转换为字符串
func (l LogLevel) String() string {
	switch l {
	case ErrorLevel:
		return "ERROR"
	case WarnLevel:
		return "WARN"
	case InfoLevel:
		return "INFO"
	case DebugLevel:
		return "DEBUG"
	case TraceLevel:
		return "TRACE"
	default:
		return "INFO"
	}
}

// ToZerologLevel 转换为zerolog级别
func (l LogLevel) ToZerologLevel() zerolog.Level {
	switch l {
	case ErrorLevel:
		return zerolog.ErrorLevel
	case WarnLevel:
		return zerolog.WarnLevel
	case InfoLevel:
		return zerolog.InfoLevel
	case DebugLevel:
		return zerolog.DebugLevel
	case TraceLevel:
		return zerolog.TraceLevel
	default:
		return zerolog.InfoLevel
	}
}

// ParseLogLevel 从字符串解析日志级别
func ParseLogLevel(s string) LogLevel {
	switch s {
	case "ERROR", "error":
		return ErrorLevel
	case "WARN", "warn":
		return WarnLevel
	case "INFO", "info":
		return InfoLevel
	case "DEBUG", "debug":
		return DebugLevel
	case "TRACE", "trace":
		return TraceLevel
	default:
		return InfoLevel
	}
}

// InitLogging 初始化日志系统
// 
// 参数:
//   - level: 日志级别字符串 ("ERROR", "WARN", "INFO", "DEBUG", "TRACE")
//   - jsonOutput: 是否输出JSON格式（用于JS插件解析）
//
// 示例:
//   // 人类可读格式（开发）
//   InitLogging("INFO", false)
//
//   // JSON格式（生产/JS集成）
//   InitLogging("DEBUG", true)
func InitLogging(level string, jsonOutput bool) {
	logLevel := ParseLogLevel(level)
	zerolog.SetGlobalLevel(logLevel.ToZerologLevel())

	if jsonOutput {
		// JSON格式输出（用于JS插件解析）
		log.Logger = zerolog.New(os.Stdout).
			With().
			Timestamp().
			Logger()
	} else {
		// 人类可读格式输出（开发调试）
		output := zerolog.ConsoleWriter{
			Out:        os.Stdout,
			TimeFormat: time.RFC3339,
		}
		log.Logger = zerolog.New(output).
			With().
			Timestamp().
			Logger()
	}
}

// Logger 全局日志实例
var Logger = log.Logger

// WithFields 创建带有字段的日志事件
//
// 示例:
//   logging.WithFields(map[string]interface{}{
//       "file": "photo.jpg",
//       "size": 1024000,
//   }).Info("File uploaded")
func WithFields(fields map[string]interface{}) *zerolog.Event {
	event := log.Info()
	for k, v := range fields {
		event = event.Interface(k, v)
	}
	return event
}

// Error 记录错误日志
func Error(msg string, fields ...map[string]interface{}) {
	event := log.Error()
	if len(fields) > 0 {
		for k, v := range fields[0] {
			event = event.Interface(k, v)
		}
	}
	event.Msg(msg)
}

// Warn 记录警告日志
func Warn(msg string, fields ...map[string]interface{}) {
	event := log.Warn()
	if len(fields) > 0 {
		for k, v := range fields[0] {
			event = event.Interface(k, v)
		}
	}
	event.Msg(msg)
}

// Info 记录信息日志
func Info(msg string, fields ...map[string]interface{}) {
	event := log.Info()
	if len(fields) > 0 {
		for k, v := range fields[0] {
			event = event.Interface(k, v)
		}
	}
	event.Msg(msg)
}

// Debug 记录调试日志
func Debug(msg string, fields ...map[string]interface{}) {
	event := log.Debug()
	if len(fields) > 0 {
		for k, v := range fields[0] {
			event = event.Interface(k, v)
		}
	}
	event.Msg(msg)
}

// Trace 记录追踪日志
func Trace(msg string, fields ...map[string]interface{}) {
	event := log.Trace()
	if len(fields) > 0 {
		for k, v := range fields[0] {
			event = event.Interface(k, v)
		}
	}
	event.Msg(msg)
}

// TimedLogger 带计时功能的日志记录器
type TimedLogger struct {
	start  time.Time
	fields map[string]interface{}
}

// NewTimedLogger 创建计时日志记录器
//
// 示例:
//   timer := logging.NewTimedLogger("operation", map[string]interface{}{
//       "file": "photo.jpg",
//   })
//   defer timer.End("Operation completed")
func NewTimedLogger(operation string, fields map[string]interface{}) *TimedLogger {
	if fields == nil {
		fields = make(map[string]interface{})
	}
	fields["operation"] = operation
	return &TimedLogger{
		start:  time.Now(),
		fields: fields,
	}
}

// End 结束计时并记录日志
func (t *TimedLogger) End(msg string) {
	duration := time.Since(t.start)
	t.fields["duration_ms"] = duration.Milliseconds()
	Info(msg, t.fields)
}

// EndWithError 结束计时并记录错误
func (t *TimedLogger) EndWithError(msg string, err error) {
	duration := time.Since(t.start)
	t.fields["duration_ms"] = duration.Milliseconds()
	t.fields["error"] = err.Error()
	Error(msg, t.fields)
}
