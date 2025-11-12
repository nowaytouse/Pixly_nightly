# 📝 统一日志规范

> **目标**: 三端日志格式统一、结构化、易于追踪和分析  
> **原则**: 可读性、可搜索性、可分析性

## 🎯 日志格式标准

### 统一JSON格式

```json
{
  "timestamp": "2025-11-10T19:30:45.123Z",
  "level": "ERROR",
  "layer": "CORE",
  "component": "param_validator",
  "message": "参数验证失败",
  "code": "PIXLY-CORE-VAL-001",
  "context": {
    "function": "validate_quality",
    "input": { "quality": 150 },
    "expected": "1-100"
  },
  "trace_id": "req-abc123",
  "session_id": "session-xyz789"
}
```

### 简化文本格式（用于终端输出）

```
2025-11-10 19:30:45.123 [ERROR] [CORE/param_validator] PIXLY-CORE-VAL-001: 参数验证失败 (quality=150, expected=1-100) [trace:req-abc123]
```

## 📊 日志级别

### 级别定义

| 级别 | 用途 | 何时使用 | 示例 |
|------|------|---------|------|
| **CRITICAL** | 致命错误 | 系统崩溃、核心功能失效 | AI服务完全不可用 |
| **ERROR** | 错误 | 操作失败、需要处理 | 参数验证失败、文件读取失败 |
| **WARNING** | 警告 | 潜在问题、降级服务 | AI置信度低、文件类型不匹配 |
| **INFO** | 信息 | 重要操作、状态变化 | 转换开始、转换完成 |
| **DEBUG** | 调试 | 详细执行过程 | 参数传递、中间结果 |
| **TRACE** | 跟踪 | 极度详细信息 | 函数调用栈、变量值 |

### 日志级别使用指南

```rust
// CRITICAL - 系统级致命错误
error!("[CRITICAL] AI服务连接完全失败，无法恢复");

// ERROR - 操作失败
error!("PIXLY-CORE-VAL-001: Quality参数超出范围: {}", quality);

// WARNING - 警告但可继续
warn!("AI置信度较低: {:.1}%, 建议手动检查", confidence * 100.0);

// INFO - 重要操作
info!("✅ 图像转换完成: {} -> {} (耗时: {}ms)", input, output, elapsed);

// DEBUG - 调试信息
debug!("参数验证: quality={}, speed={}", quality, speed);

// TRACE - 详细跟踪
trace!("进入函数: validate_params");
```

## 🏗️ 三端日志实现

### Rust实现

```rust
// core/rust/src/logging.rs
use serde::Serialize;
use serde_json::json;
use tracing::{error, warn, info, debug, trace};

#[derive(Serialize)]
struct LogEntry {
    timestamp: String,
    level: String,
    layer: String,
    component: String,
    message: String,
    code: Option<String>,
    context: serde_json::Value,
    trace_id: Option<String>,
}

impl LogEntry {
    fn new(
        level: &str,
        component: &str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            level: level.to_string(),
            layer: "CORE".to_string(),
            component: component.to_string(),
            message: message.into(),
            code: None,
            context: json!({}),
            trace_id: None,
        }
    }

    fn with_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }

    fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = context;
        self
    }

    fn with_trace_id(mut self, trace_id: &str) -> Self {
        self.trace_id = Some(trace_id.to_string());
        self
    }

    fn log(&self) {
        let json = serde_json::to_string(self).unwrap();
        match self.level.as_str() {
            "ERROR" | "CRITICAL" => error!("{}", json),
            "WARNING" => warn!("{}", json),
            "INFO" => info!("{}", json),
            "DEBUG" => debug!("{}", json),
            "TRACE" => trace!("{}", json),
            _ => info!("{}", json),
        }
    }

    fn log_pretty(&self) {
        // 终端友好格式
        let icon = match self.level.as_str() {
            "CRITICAL" => "💥",
            "ERROR" => "❌",
            "WARNING" => "⚠️",
            "INFO" => "✅",
            "DEBUG" => "🔍",
            "TRACE" => "📍",
            _ => "ℹ️",
        };

        println!(
            "{} {} [{}] {}: {}",
            icon,
            self.timestamp,
            self.component,
            self.code.as_deref().unwrap_or(""),
            self.message
        );

        if !self.context.is_null() {
            println!("   Context: {}", self.context);
        }
    }
}

// 便捷宏
#[macro_export]
macro_rules! log_validation_error {
    ($code:expr, $msg:expr, $($key:ident = $value:expr),*) => {
        {
            let mut context = serde_json::json!({});
            $(
                context[stringify!($key)] = serde_json::json!($value);
            )*

            LogEntry::new("ERROR", "validator", $msg)
                .with_code($code)
                .with_context(context)
                .log_pretty();
        }
    };
}

// 使用示例
pub fn validate_quality(quality: u8) -> Result<()> {
    if quality < 1 || quality > 100 {
        log_validation_error!(
            "PIXLY-CORE-VAL-001",
            "Quality参数超出范围",
            quality = quality,
            expected = "1-100"
        );
        bail!("PIXLY-CORE-VAL-001");
    }
    Ok(())
}
```

### Go实现

```go
// core/go/ai/logging.go
package ai

import (
    "encoding/json"
    "fmt"
    "time"
)

// LogEntry 日志条目
type LogEntry struct {
    Timestamp string                 `json:"timestamp"`
    Level     string                 `json:"level"`
    Layer     string                 `json:"layer"`
    Component string                 `json:"component"`
    Message   string                 `json:"message"`
    Code      string                 `json:"code,omitempty"`
    Context   map[string]interface{} `json:"context,omitempty"`
    TraceID   string                 `json:"trace_id,omitempty"`
}

// Logger 日志记录器
type Logger struct {
    component string
    traceID   string
}

// NewLogger 创建日志记录器
func NewLogger(component string) *Logger {
    return &Logger{
        component: component,
    }
}

// WithTraceID 设置追踪ID
func (l *Logger) WithTraceID(traceID string) *Logger {
    l.traceID = traceID
    return l
}

// Error 记录错误
func (l *Logger) Error(code, message string, context map[string]interface{}) {
    entry := LogEntry{
        Timestamp: time.Now().UTC().Format(time.RFC3339Nano),
        Level:     "ERROR",
        Layer:     "AI",
        Component: l.component,
        Message:   message,
        Code:      code,
        Context:   context,
        TraceID:   l.traceID,
    }
    l.output(entry)
}

// Warning 记录警告
func (l *Logger) Warning(message string, context map[string]interface{}) {
    entry := LogEntry{
        Timestamp: time.Now().UTC().Format(time.RFC3339Nano),
        Level:     "WARNING",
        Layer:     "AI",
        Component: l.component,
        Message:   message,
        Context:   context,
        TraceID:   l.traceID,
    }
    l.output(entry)
}

// Info 记录信息
func (l *Logger) Info(message string, context map[string]interface{}) {
    entry := LogEntry{
        Timestamp: time.Now().UTC().Format(time.RFC3339Nano),
        Level:     "INFO",
        Layer:     "AI",
        Component: l.component,
        Message:   message,
        Context:   context,
        TraceID:   l.traceID,
    }
    l.output(entry)
}

// output 输出日志
func (l *Logger) output(entry LogEntry) {
    // JSON格式（用于日志文件）
    jsonBytes, _ := json.Marshal(entry)
    fmt.Println(string(jsonBytes))

    // 或终端友好格式
    // l.outputPretty(entry)
}

// outputPretty 终端友好输出
func (l *Logger) outputPretty(entry LogEntry) {
    icon := map[string]string{
        "CRITICAL": "💥",
        "ERROR":    "❌",
        "WARNING":  "⚠️",
        "INFO":     "✅",
        "DEBUG":    "🔍",
        "TRACE":    "📍",
    }[entry.Level]

    fmt.Printf("%s %s [%s] %s: %s\n",
        icon,
        entry.Timestamp,
        entry.Component,
        entry.Code,
        entry.Message,
    )

    if len(entry.Context) > 0 {
        contextJSON, _ := json.MarshalIndent(entry.Context, "   ", "  ")
        fmt.Printf("   Context: %s\n", string(contextJSON))
    }
}

// 便捷函数
func LogValidationError(code, message string, context map[string]interface{}) {
    logger := NewLogger("validator")
    logger.Error(code, message, context)
}

// 使用示例
func ValidateImageInfo(img *ImageInfo) error {
    if img.Width < 1 || img.Width > 65535 {
        LogValidationError(
            "PIXLY-AI-VAL-001",
            "图像宽度无效",
            map[string]interface{}{
                "width":    img.Width,
                "expected": "1-65535",
            },
        )
        return fmt.Errorf("PIXLY-AI-VAL-001: 图像宽度无效: %d", img.Width)
    }
    return nil
}
```

### JavaScript实现

```javascript
// plugin/converter/js/plugin-modules/logger.js

/**
 * 统一日志记录器
 */
class Logger {
    constructor(component) {
        this.component = component;
        this.layer = 'UI';
        this.traceID = null;
    }

    /**
     * 设置追踪ID
     */
    withTraceID(traceID) {
        this.traceID = traceID;
        return this;
    }

    /**
     * 创建日志条目
     */
    createEntry(level, message, code = null, context = {}) {
        return {
            timestamp: new Date().toISOString(),
            level: level,
            layer: this.layer,
            component: this.component,
            message: message,
            code: code,
            context: context,
            trace_id: this.traceID,
        };
    }

    /**
     * 记录错误
     */
    error(code, message, context = {}) {
        const entry = this.createEntry('ERROR', message, code, context);
        this.output(entry);
    }

    /**
     * 记录警告
     */
    warning(message, context = {}) {
        const entry = this.createEntry('WARNING', message, null, context);
        this.output(entry);
    }

    /**
     * 记录信息
     */
    info(message, context = {}) {
        const entry = this.createEntry('INFO', message, null, context);
        this.output(entry);
    }

    /**
     * 记录调试
     */
    debug(message, context = {}) {
        const entry = this.createEntry('DEBUG', message, null, context);
        this.output(entry);
    }

    /**
     * 输出日志
     */
    output(entry) {
        // JSON格式（发送到日志服务器）
        this.outputJSON(entry);
        
        // 终端友好格式（浏览器控制台）
        this.outputPretty(entry);
    }

    /**
     * JSON格式输出
     */
    outputJSON(entry) {
        // 可以发送到日志收集服务
        if (window.PIXLY && window.PIXLY.logCollector) {
            window.PIXLY.logCollector.send(entry);
        }
    }

    /**
     * 终端友好输出
     */
    outputPretty(entry) {
        const icons = {
            CRITICAL: '💥',
            ERROR: '❌',
            WARNING: '⚠️',
            INFO: '✅',
            DEBUG: '🔍',
            TRACE: '📍',
        };

        const icon = icons[entry.level] || 'ℹ️';
        const color = {
            CRITICAL: 'color: #ff0000; font-weight: bold;',
            ERROR: 'color: #ff3333;',
            WARNING: 'color: #ff9900;',
            INFO: 'color: #00aa00;',
            DEBUG: 'color: #0066cc;',
            TRACE: 'color: #666666;',
        }[entry.level] || '';

        console.log(
            `%c${icon} [${entry.component}] ${entry.code || ''}: ${entry.message}`,
            color
        );

        if (Object.keys(entry.context).length > 0) {
            console.log('   Context:', entry.context);
        }
    }
}

/**
 * 便捷函数
 */
function logValidationError(code, message, context = {}) {
    const logger = new Logger('validator');
    logger.error(code, message, context);
}

// 使用示例
function validateQuality(quality) {
    if (quality < 1 || quality > 100) {
        logValidationError(
            'PIXLY-UI-VAL-001',
            '参数验证失败: quality超出范围',
            {
                quality: quality,
                expected: '1-100',
            }
        );
        throw new Error('PIXLY-UI-VAL-001: Quality参数超出范围');
    }
}

// 导出
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { Logger, logValidationError };
}
```

## 📈 日志聚合和分析

### 日志收集

```javascript
// 前端日志收集器
class LogCollector {
    constructor() {
        this.buffer = [];
        this.maxBufferSize = 100;
        this.flushInterval = 5000; // 5秒
        this.startAutoFlush();
    }

    send(entry) {
        this.buffer.push(entry);
        if (this.buffer.length >= this.maxBufferSize) {
            this.flush();
        }
    }

    flush() {
        if (this.buffer.length === 0) return;

        // 发送到后端
        fetch('/api/logs', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ logs: this.buffer }),
        }).catch(err => {
            console.error('日志发送失败:', err);
        });

        this.buffer = [];
    }

    startAutoFlush() {
        setInterval(() => this.flush(), this.flushInterval);
    }
}
```

### 日志查询

```sql
-- 查询特定错误码的日志
SELECT * FROM logs 
WHERE code = 'PIXLY-CORE-VAL-001' 
AND timestamp > NOW() - INTERVAL '1 hour'
ORDER BY timestamp DESC;

-- 查询特定trace_id的完整链路
SELECT * FROM logs 
WHERE trace_id = 'req-abc123' 
ORDER BY timestamp ASC;

-- 统计错误分布
SELECT code, COUNT(*) as count 
FROM logs 
WHERE level = 'ERROR' 
AND timestamp > NOW() - INTERVAL '24 hours'
GROUP BY code 
ORDER BY count DESC;
```

## 🎯 最佳实践

### DO ✅

1. **总是包含上下文**
```rust
✅ error!("参数验证失败: quality={}, expected=1-100", quality);
```

2. **使用结构化日志**
```rust
✅ log_entry.with_context(json!({"quality": 150, "expected": "1-100"}));
```

3. **添加追踪ID**
```rust
✅ logger.with_trace_id("req-abc123").error(...);
```

4. **选择正确的日志级别**
```rust
✅ ERROR用于失败操作，WARNING用于潜在问题
```

### DON'T ❌

1. **不要记录敏感信息**
```rust
❌ error!("密码错误: {}", password);
✅ error!("认证失败: 用户名={}", username);
```

2. **不要过度记录**
```rust
❌ 循环中每次都记录 (产生海量日志)
✅ 记录关键事件和错误
```

3. **不要使用不一致的格式**
```rust
❌ println!("Error: something wrong");
✅ logger.error("PIXLY-CORE-VAL-001", "参数验证失败", context);
```

---

**统一日志规范，让问题追踪更简单！** 📝
