package ai

import (
	"encoding/json"
	"fmt"
	"time"
)

/**
 * Unified Messaging System - 统一消息传递系统 (Go端)
 * 
 * Phase 46.14+: 三端统一消息显示
 * 
 * 与Rust/Python保持一致的消息格式
 */

// MessageType 消息类型
type MessageType string

const (
	MessageTypeInfo     MessageType = "info"
	MessageTypeWarning  MessageType = "warning"
	MessageTypeError    MessageType = "error"
	MessageTypeSuccess  MessageType = "success"
	MessageTypeProgress MessageType = "progress"
	MessageTypeStatus   MessageType = "status"
)

// MessageLevel 消息级别
type MessageLevel int

const (
	MessageLevelDebug    MessageLevel = 0
	MessageLevelInfo     MessageLevel = 1
	MessageLevelWarning  MessageLevel = 2
	MessageLevelError    MessageLevel = 3
	MessageLevelCritical MessageLevel = 4
)

// UnifiedMessage 统一消息结构
type UnifiedMessage struct {
	ID        string                 `json:"id"`
	Type      MessageType            `json:"type"`
	Level     MessageLevel           `json:"level"`
	Source    string                 `json:"source"`    // "go-ai"
	Component string                 `json:"component"`
	Message   string                 `json:"message"`
	Timestamp int64                  `json:"timestamp"`
	Progress  *int                   `json:"progress,omitempty"`   // 0-100
	Data      map[string]interface{} `json:"data,omitempty"`
	ErrorCode *string                `json:"error_code,omitempty"`
	TraceID   *string                `json:"trace_id,omitempty"`
}

// NewMessage 创建新消息
func NewMessage(msgType MessageType, level MessageLevel, component string, message string) *UnifiedMessage {
	return &UnifiedMessage{
		ID:        generateMessageID(),
		Type:      msgType,
		Level:     level,
		Source:    "go-ai",
		Component: component,
		Message:   message,
		Timestamp: time.Now().Unix(),
	}
}

// generateMessageID 生成消息ID
func generateMessageID() string {
	return fmt.Sprintf("msg_%x", time.Now().UnixNano())
}

// Info 创建信息消息
func NewInfoMessage(component string, message string) *UnifiedMessage {
	return NewMessage(MessageTypeInfo, MessageLevelInfo, component, message)
}

// Warning 创建警告消息
func NewWarningMessage(component string, message string) *UnifiedMessage {
	return NewMessage(MessageTypeWarning, MessageLevelWarning, component, message)
}

// Error 创建错误消息
func NewErrorMessage(component string, message string) *UnifiedMessage {
	return NewMessage(MessageTypeError, MessageLevelError, component, message)
}

// Success 创建成功消息
func NewSuccessMessage(component string, message string) *UnifiedMessage {
	return NewMessage(MessageTypeSuccess, MessageLevelInfo, component, message)
}

// Progress 创建进度消息
func NewProgressMessage(component string, message string, progress int) *UnifiedMessage {
	msg := NewMessage(MessageTypeProgress, MessageLevelInfo, component, message)
	if progress > 100 {
		progress = 100
	}
	msg.Progress = &progress
	return msg
}

// WithErrorCode 设置错误码
func (m *UnifiedMessage) WithErrorCode(code string) *UnifiedMessage {
	m.ErrorCode = &code
	return m
}

// WithTraceID 设置追踪ID
func (m *UnifiedMessage) WithTraceID(traceID string) *UnifiedMessage {
	m.TraceID = &traceID
	return m
}

// WithData 设置附加数据
func (m *UnifiedMessage) WithData(data map[string]interface{}) *UnifiedMessage {
	m.Data = data
	return m
}

// ToJSON 转换为JSON
func (m *UnifiedMessage) ToJSON() string {
	data, err := json.Marshal(m)
	if err != nil {
		return "{}"
	}
	return string(data)
}

// FromJSON 从JSON解析
func FromJSON(jsonStr string) (*UnifiedMessage, error) {
	var msg UnifiedMessage
	err := json.Unmarshal([]byte(jsonStr), &msg)
	return &msg, err
}

// Emit 发送消息（输出到stdout供其他端读取）
func (m *UnifiedMessage) Emit() {
	// 输出到stdout，前缀PIXLY_MSG:供解析
	fmt.Printf("PIXLY_MSG:%s\n", m.ToJSON())
	
	// 同时记录到日志系统
	switch m.Level {
	case MessageLevelDebug:
		Debug(m.Component, m.Message)
	case MessageLevelInfo:
		Info(m.Component, m.Message)
	case MessageLevelWarning:
		Warning(m.Component, m.Message)
	case MessageLevelError, MessageLevelCritical:
		Error(m.Component, m.Message)
	}
}

// MessageChannel已移除 - 过度设计，直接使用快捷函数

// 快捷函数

// SendInfo 发送信息消息
func SendInfo(component string, message string) {
	NewInfoMessage(component, message).Emit()
}

// SendWarning 发送警告消息
func SendWarning(component string, message string) {
	NewWarningMessage(component, message).Emit()
}

// SendError 发送错误消息
func SendError(component string, message string, errorCode *string) {
	msg := NewErrorMessage(component, message)
	if errorCode != nil {
		msg = msg.WithErrorCode(*errorCode)
	}
	msg.Emit()
}

// SendSuccess 发送成功消息
func SendSuccess(component string, message string) {
	NewSuccessMessage(component, message).Emit()
}

// SendProgress 发送进度消息
func SendProgress(component string, message string, progress int) {
	NewProgressMessage(component, message, progress).Emit()
}

// MessageReader已移除 - 当前未使用进程间通信
