// 🎯 Pixly统一错误码系统 (Go)
//
// Phase 46.8: 三端共享的错误码定义
// 确保Go/Rust/JS使用相同的错误码格式和消息

package ai

import (
	"encoding/json"
	"fmt"
)

// ErrorSeverity 错误严重级别
type ErrorSeverity string

const (
	SeverityCritical ErrorSeverity = "CRITICAL" // 系统级致命错误
	SeverityError    ErrorSeverity = "ERROR"    // 业务级错误
	SeverityWarning  ErrorSeverity = "WARNING"  // 警告（不影响主流程）
	SeverityInfo     ErrorSeverity = "INFO"     // 信息提示
)

// ErrorCode 错误码常量
const (
	// 验证类错误 (PIXLY-GO-VAL-xxx)
	ErrValInvalidRange     = "PIXLY-GO-VAL-001" // 参数超出范围
	ErrValMissingField     = "PIXLY-GO-VAL-002" // 必填字段缺失
	ErrValInvalidFormat    = "PIXLY-GO-VAL-003" // 格式无效
	ErrValInvalidPath      = "PIXLY-GO-VAL-004" // 路径无效
	ErrValLowConfidence    = "PIXLY-GO-VAL-005" // AI置信度过低
	ErrValImageTooLarge    = "PIXLY-GO-VAL-006" // 图像超限
	ErrValFormatUnsupported = "PIXLY-GO-VAL-007" // 格式不支持

	// 文件类错误 (PIXLY-GO-FILE-xxx)
	ErrFileNotFound   = "PIXLY-GO-FILE-001" // 文件不存在
	ErrFileReadFailed = "PIXLY-GO-FILE-002" // 文件读取失败
	ErrFileWriteFailed = "PIXLY-GO-FILE-003" // 文件写入失败
	ErrFileCorrupted  = "PIXLY-GO-FILE-004" // 文件损坏

	// 网络类错误 (PIXLY-GO-NET-xxx)
	ErrNetTimeout     = "PIXLY-GO-NET-001" // 请求超时
	ErrNetConnection  = "PIXLY-GO-NET-002" // 连接失败
	ErrNetHTTPError   = "PIXLY-GO-NET-003" // HTTP错误

	// 系统类错误 (PIXLY-GO-SYS-xxx)
	ErrSysInternal    = "PIXLY-GO-SYS-001" // 内部错误
	ErrSysOutOfMemory = "PIXLY-GO-SYS-002" // 内存不足
	ErrSysToolMissing = "PIXLY-GO-SYS-003" // 工具缺失

	// AI业务类错误 (PIXLY-GO-BIZ-xxx)
	ErrBizPredictFailed = "PIXLY-GO-BIZ-001" // AI预测失败
	ErrBizModelError    = "PIXLY-GO-BIZ-002" // 模型错误
	ErrBizPythonBridge  = "PIXLY-GO-BIZ-003" // Python桥接失败
	ErrBizModelNotFound = "PIXLY-GO-BIZ-004" // 模型未找到
	ErrBizFeatureExtract = "PIXLY-GO-BIZ-005" // 特征提取失败
	ErrBizBatchFailed   = "PIXLY-GO-BIZ-006" // 批量处理失败
)

// PixlyError 统一错误结构
type PixlyError struct {
	Code     string                 `json:"code"`               // 错误码
	Message  string                 `json:"message"`            // 错误消息
	Severity ErrorSeverity          `json:"severity"`           // 严重级别
	Context  map[string]interface{} `json:"context,omitempty"`  // 上下文信息
	Cause    error                  `json:"-"`                  // 原始错误（不序列化）
}

// Error 实现error接口
func (e *PixlyError) Error() string {
	if e.Cause != nil {
		return fmt.Sprintf("[%s] %s: %s (caused by: %v)", e.Code, e.Severity, e.Message, e.Cause)
	}
	return fmt.Sprintf("[%s] %s: %s", e.Code, e.Severity, e.Message)
}

// WithContext 添加上下文信息
func (e *PixlyError) WithContext(key string, value interface{}) *PixlyError {
	if e.Context == nil {
		e.Context = make(map[string]interface{})
	}
	e.Context[key] = value
	return e
}

// WithCause 设置原始错误
func (e *PixlyError) WithCause(cause error) *PixlyError {
	e.Cause = cause
	return e
}

// MarshalJSON 自定义JSON序列化
func (e *PixlyError) MarshalJSON() ([]byte, error) {
	type Alias PixlyError
	aux := &struct {
		CauseMessage string `json:"cause,omitempty"`
		*Alias
	}{
		Alias: (*Alias)(e),
	}
	if e.Cause != nil {
		aux.CauseMessage = e.Cause.Error()
	}
	return json.Marshal(aux)
}

// ErrorBuilder 错误构建器
type ErrorBuilder struct {
	err *PixlyError
}

// NewErrorBuilder 创建错误构建器
func NewErrorBuilder(code string, message string) *ErrorBuilder {
	severity := determineSeverity(code)
	return &ErrorBuilder{
		err: &PixlyError{
			Code:     code,
			Message:  message,
			Severity: severity,
			Context:  make(map[string]interface{}),
		},
	}
}

// WithSeverity 设置严重级别
func (b *ErrorBuilder) WithSeverity(severity ErrorSeverity) *ErrorBuilder {
	b.err.Severity = severity
	return b
}

// WithContext 添加上下文
func (b *ErrorBuilder) WithContext(key string, value interface{}) *ErrorBuilder {
	b.err.WithContext(key, value)
	return b
}

// WithCause 设置原始错误
func (b *ErrorBuilder) WithCause(cause error) *ErrorBuilder {
	b.err.WithCause(cause)
	return b
}

// Build 构建错误
func (b *ErrorBuilder) Build() *PixlyError {
	return b.err
}

// 快捷构造函数

// NewValidationError 创建验证错误
func NewValidationError(code string, message string) *PixlyError {
	return &PixlyError{
		Code:     code,
		Message:  message,
		Severity: SeverityError,
		Context:  make(map[string]interface{}),
	}
}

// NewFileError 创建文件错误
func NewFileError(code string, message string, path string) *PixlyError {
	return &PixlyError{
		Code:     code,
		Message:  message,
		Severity: SeverityError,
		Context: map[string]interface{}{
			"file_path": path,
		},
	}
}

// NewNetworkError 创建网络错误
func NewNetworkError(code string, message string) *PixlyError {
	return &PixlyError{
		Code:     code,
		Message:  message,
		Severity: SeverityError,
		Context:  make(map[string]interface{}),
	}
}

// NewSystemError 创建系统错误
func NewSystemError(code string, message string) *PixlyError {
	return &PixlyError{
		Code:     code,
		Message:  message,
		Severity: SeverityCritical,
		Context:  make(map[string]interface{}),
	}
}

// NewBusinessError 创建业务错误
func NewBusinessError(code string, message string) *PixlyError {
	return &PixlyError{
		Code:     code,
		Message:  message,
		Severity: SeverityError,
		Context:  make(map[string]interface{}),
	}
}

// 辅助函数

// determineSeverity 根据错误码推断严重级别
func determineSeverity(code string) ErrorSeverity {
	// 简单规则：SYS前缀为CRITICAL，其他为ERROR
	if len(code) >= 13 && code[10:13] == "SYS" {
		return SeverityCritical
	}
	return SeverityError
}

// IsPixlyError 判断是否为PixlyError
func IsPixlyError(err error) bool {
	_, ok := err.(*PixlyError)
	return ok
}

// AsPixlyError 转换为PixlyError
func AsPixlyError(err error) (*PixlyError, bool) {
	pixlyErr, ok := err.(*PixlyError)
	return pixlyErr, ok
}

// WrapError 包装普通错误为PixlyError
func WrapError(err error, code string, message string) *PixlyError {
	return NewErrorBuilder(code, message).
		WithCause(err).
		Build()
}
