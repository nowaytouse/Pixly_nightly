# 🚨 统一错误码规范

> **目标**: 三端错误码统一、分类清晰、易于追踪  
> **原则**: 响亮报错、错误可追溯、用户友好

## 📋 错误码体系

### 错误码格式

```
PIXLY-[LAYER]-[CATEGORY]-[CODE]
```

- **LAYER**: 层级标识 (UI/CORE/AI)
- **CATEGORY**: 错误类别 (VAL/FILE/NET/SYS)
- **CODE**: 具体错误编号 (001-999)

### 示例

```
PIXLY-CORE-VAL-001  // Rust Core层，参数验证错误，编号001
PIXLY-AI-NET-002    // AI层，网络错误，编号002
PIXLY-UI-FILE-003   // UI层，文件错误，编号003
```

## 🎯 错误分类

### 1. 参数验证错误 (VAL)

#### PIXLY-CORE-VAL-001: 参数超出范围
```rust
// Rust
bail!("PIXLY-CORE-VAL-001: Quality参数超出范围: {} (应为1-100)", quality)
```
```go
// Go
return fmt.Errorf("PIXLY-AI-VAL-001: 图像宽度无效: %d (应为1-65535)", width)
```
```javascript
// JS
throw new Error('PIXLY-UI-VAL-001: 参数验证失败: quality超出范围')
```

**用户消息**: "参数设置有误，请检查质量值（应为1-100）"

#### PIXLY-CORE-VAL-002: 参数类型错误
```rust
bail!("PIXLY-CORE-VAL-002: 参数类型错误: 期望数字，实际为字符串")
```

#### PIXLY-CORE-VAL-003: 必填参数缺失
```rust
bail!("PIXLY-CORE-VAL-003: 必填参数缺失: image_path")
```

#### PIXLY-CORE-VAL-004: 参数组合冲突
```rust
bail!("PIXLY-CORE-VAL-004: 参数冲突: JPEG不支持无损模式")
```

#### PIXLY-AI-VAL-005: AI置信度过低
```rust
bail!("PIXLY-AI-VAL-005: AI置信度过低: {:.1}% (阈值: 50%)", confidence * 100.0)
```

**用户消息**: "AI预测不够确定，建议手动设置参数"

#### PIXLY-CORE-VAL-006: 参数完整性验证失败
```javascript
throw new Error('PIXLY-UI-VAL-006: 参数在传递过程中被修改')
```

**用户消息**: "参数传递异常，请重试"

### 2. 文件错误 (FILE)

#### PIXLY-CORE-FILE-001: 文件不存在
```rust
bail!("PIXLY-CORE-FILE-001: 输入文件不存在: {}", path)
```

**用户消息**: "找不到指定的图像文件"

#### PIXLY-CORE-FILE-002: 文件无法读取
```rust
bail!("PIXLY-CORE-FILE-002: 文件无法读取: {} (权限不足)", path)
```

**用户消息**: "无法读取文件，请检查文件权限"

#### PIXLY-CORE-FILE-003: 文件格式不支持
```rust
bail!("PIXLY-CORE-FILE-003: 不支持的文件格式: {} (仅支持jpg/png/webp等)", ext)
```

**用户消息**: "文件格式不支持，请使用jpg、png或webp格式"

#### PIXLY-CORE-FILE-004: 文件已损坏
```rust
bail!("PIXLY-CORE-FILE-004: 文件已损坏，无法解析")
```

**用户消息**: "图像文件已损坏，无法处理"

#### PIXLY-CORE-FILE-005: 文件类型伪装
```rust
bail!("PIXLY-CORE-FILE-005: 文件类型不匹配: 扩展名.png，实际为JPEG")
```

**用户消息**: "文件类型不匹配，建议修正文件扩展名"

#### PIXLY-CORE-FILE-006: 输出文件写入失败
```rust
bail!("PIXLY-CORE-FILE-006: 输出文件写入失败: {} (磁盘空间不足)", path)
```

**用户消息**: "保存文件失败，请检查磁盘空间"

### 3. 网络错误 (NET)

#### PIXLY-AI-NET-001: AI服务不可用
```go
return fmt.Errorf("PIXLY-AI-NET-001: AI服务连接失败")
```

**用户消息**: "AI服务暂时不可用，请稍后重试"

#### PIXLY-AI-NET-002: AI请求超时
```go
return fmt.Errorf("PIXLY-AI-NET-002: AI请求超时 (30s)")
```

**用户消息**: "AI响应超时，请重试"

#### PIXLY-UI-NET-003: Rust服务不可用
```javascript
throw new Error('PIXLY-UI-NET-003: 无法连接到Rust转换服务')
```

**用户消息**: "转换服务未启动，请检查服务状态"

### 4. 系统错误 (SYS)

#### PIXLY-CORE-SYS-001: 内存不足
```rust
bail!("PIXLY-CORE-SYS-001: 内存不足，无法处理大图像")
```

**用户消息**: "系统内存不足，请关闭其他程序后重试"

#### PIXLY-CORE-SYS-002: 工具未安装
```rust
bail!("PIXLY-CORE-SYS-002: 转换工具未安装: cjxl")
```

**用户消息**: "转换工具未安装，请运行安装脚本"

#### PIXLY-CORE-SYS-003: 工具执行失败
```rust
bail!("PIXLY-CORE-SYS-003: 工具执行失败: cjxl返回错误码1")
```

**用户消息**: "图像转换失败，请检查图像是否正常"

### 5. 业务逻辑错误 (BIZ)

#### PIXLY-CORE-BIZ-001: 图像尺寸过大
```rust
bail!("PIXLY-CORE-BIZ-001: 图像尺寸过大: {}x{} (最大65535x65535)", w, h)
```

**用户消息**: "图像尺寸超限，建议先缩小图像"

#### PIXLY-CORE-BIZ-002: 批量任务失败
```rust
bail!("PIXLY-CORE-BIZ-002: 批量转换失败: {}/{}成功", success, total)
```

**用户消息**: "部分图像转换失败，请查看详细报告"

#### PIXLY-AI-BIZ-003: 模型未加载
```go
return fmt.Errorf("PIXLY-AI-BIZ-003: AI模型未加载: jxl")
```

**用户消息**: "AI模型未准备好，请稍后重试"

## 📊 错误严重级别

### 级别定义

```go
type ErrorSeverity string

const (
    SeverityCritical ErrorSeverity = "CRITICAL" // 致命错误，立即终止
    SeverityError    ErrorSeverity = "ERROR"    // 错误，阻断流程
    SeverityWarning  ErrorSeverity = "WARNING"  // 警告，可继续
    SeverityInfo     ErrorSeverity = "INFO"     // 信息，仅提示
)
```

### 错误级别映射

| 错误类型 | 严重级别 | 是否阻断 | 用户操作 |
|---------|---------|---------|---------|
| VAL-001~004 | ERROR | ✅ | 修正参数 |
| VAL-005 | WARNING | ❌ | 可手动设置 |
| VAL-006 | ERROR | ✅ | 重试 |
| FILE-001~004 | ERROR | ✅ | 检查文件 |
| FILE-005 | WARNING | ❌ | 可继续或修正 |
| FILE-006 | ERROR | ✅ | 检查磁盘 |
| NET-001~003 | ERROR | ✅ | 检查服务 |
| SYS-001~003 | ERROR | ✅ | 检查系统 |
| BIZ-001~003 | ERROR | ✅ | 业务处理 |

## 🔧 错误处理实现

### Rust实现

```rust
// core/rust/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PixlyError {
    // 参数验证错误
    #[error("PIXLY-CORE-VAL-{code:03}: {message}")]
    ValidationError { code: u16, message: String },
    
    // 文件错误
    #[error("PIXLY-CORE-FILE-{code:03}: {message}")]
    FileError { code: u16, message: String },
    
    // 系统错误
    #[error("PIXLY-CORE-SYS-{code:03}: {message}")]
    SystemError { code: u16, message: String },
    
    // 业务错误
    #[error("PIXLY-CORE-BIZ-{code:03}: {message}")]
    BusinessError { code: u16, message: String },
}

impl PixlyError {
    pub fn validation(code: u16, message: impl Into<String>) -> Self {
        Self::ValidationError {
            code,
            message: message.into(),
        }
    }
    
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::ValidationError { code, .. } if *code == 5 => ErrorSeverity::Warning,
            _ => ErrorSeverity::Error,
        }
    }
    
    pub fn user_message(&self) -> String {
        match self {
            Self::ValidationError { code: 1, .. } => 
                "参数设置有误，请检查参数范围".to_string(),
            Self::FileError { code: 1, .. } => 
                "找不到指定的图像文件".to_string(),
            _ => "发生错误，请查看详细信息".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorSeverity {
    Critical,
    Error,
    Warning,
    Info,
}
```

### Go实现

```go
// core/go/ai/errors.go
package ai

import "fmt"

// ErrorCode 错误码
type ErrorCode string

const (
    // 参数验证错误
    ErrValInvalidRange    ErrorCode = "PIXLY-AI-VAL-001"
    ErrValInvalidType     ErrorCode = "PIXLY-AI-VAL-002"
    ErrValMissingRequired ErrorCode = "PIXLY-AI-VAL-003"
    ErrValConflict        ErrorCode = "PIXLY-AI-VAL-004"
    ErrValLowConfidence   ErrorCode = "PIXLY-AI-VAL-005"
    
    // 网络错误
    ErrNetServiceDown     ErrorCode = "PIXLY-AI-NET-001"
    ErrNetTimeout         ErrorCode = "PIXLY-AI-NET-002"
    
    // 业务错误
    ErrBizModelNotLoaded  ErrorCode = "PIXLY-AI-BIZ-003"
)

// PixlyError 统一错误类型
type PixlyError struct {
    Code     ErrorCode
    Message  string
    Severity ErrorSeverity
}

func (e *PixlyError) Error() string {
    return fmt.Sprintf("%s: %s", e.Code, e.Message)
}

func (e *PixlyError) UserMessage() string {
    switch e.Code {
    case ErrValInvalidRange:
        return "参数设置有误，请检查参数范围"
    case ErrValLowConfidence:
        return "AI预测不够确定，建议手动设置参数"
    case ErrNetServiceDown:
        return "AI服务暂时不可用，请稍后重试"
    default:
        return "发生错误，请查看详细信息"
    }
}

// NewValidationError 创建验证错误
func NewValidationError(code ErrorCode, message string) *PixlyError {
    return &PixlyError{
        Code:     code,
        Message:  message,
        Severity: SeverityError,
    }
}

// ErrorSeverity 错误严重级别
type ErrorSeverity string

const (
    SeverityCritical ErrorSeverity = "CRITICAL"
    SeverityError    ErrorSeverity = "ERROR"
    SeverityWarning  ErrorSeverity = "WARNING"
    SeverityInfo     ErrorSeverity = "INFO"
)
```

### JavaScript实现

```javascript
// plugin/converter/js/plugin-modules/errors.js

/**
 * Pixly统一错误类
 */
class PixlyError extends Error {
    constructor(code, message, severity = 'ERROR') {
        super(`${code}: ${message}`);
        this.name = 'PixlyError';
        this.code = code;
        this.originalMessage = message;
        this.severity = severity;
        this.timestamp = new Date().toISOString();
    }

    /**
     * 获取用户友好的错误消息
     */
    getUserMessage() {
        const messages = {
            'PIXLY-UI-VAL-001': '参数设置有误，请检查参数范围',
            'PIXLY-UI-VAL-006': '参数传递异常，请重试',
            'PIXLY-UI-NET-003': '转换服务未启动，请检查服务状态',
        };
        return messages[this.code] || '发生错误，请查看详细信息';
    }

    /**
     * 是否应该阻断流程
     */
    shouldBlock() {
        return this.severity === 'ERROR' || this.severity === 'CRITICAL';
    }

    /**
     * 转换为日志格式
     */
    toLogFormat() {
        return {
            code: this.code,
            message: this.originalMessage,
            severity: this.severity,
            timestamp: this.timestamp,
            stack: this.stack,
        };
    }
}

// 错误码常量
const ErrorCodes = {
    // 参数验证
    VAL_INVALID_RANGE: 'PIXLY-UI-VAL-001',
    VAL_INTEGRITY_FAILED: 'PIXLY-UI-VAL-006',
    
    // 网络错误
    NET_SERVICE_DOWN: 'PIXLY-UI-NET-003',
};

// 严重级别常量
const ErrorSeverity = {
    CRITICAL: 'CRITICAL',
    ERROR: 'ERROR',
    WARNING: 'WARNING',
    INFO: 'INFO',
};

// 导出
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { PixlyError, ErrorCodes, ErrorSeverity };
}
```

## 📝 错误日志格式

### 统一日志格式

```json
{
  "timestamp": "2025-11-10T19:30:45.123Z",
  "level": "ERROR",
  "layer": "CORE",
  "code": "PIXLY-CORE-VAL-001",
  "message": "Quality参数超出范围: 150 (应为1-100)",
  "user_message": "参数设置有误，请检查质量值（应为1-100）",
  "severity": "ERROR",
  "context": {
    "input": "quality=150",
    "expected": "1-100",
    "file": "param_validators.rs",
    "line": 42
  },
  "stack_trace": "..."
}
```

## 📊 错误统计和监控

### 需要记录的指标

1. **错误频率**: 每种错误码的出现次数
2. **错误趋势**: 错误率随时间的变化
3. **用户影响**: 有多少用户遇到错误
4. **恢复率**: 错误后用户是否成功重试

### 监控告警阈值

| 指标 | 阈值 | 告警级别 |
|------|------|---------|
| 错误率 > 5% | 连续5分钟 | WARNING |
| 错误率 > 10% | 连续5分钟 | ERROR |
| 致命错误 > 0 | 任何时候 | CRITICAL |
| 特定错误 > 100次/小时 | 连续 | WARNING |

## ✅ 最佳实践

### DO ✅

1. **总是使用统一错误码**
   ```rust
   ✅ bail!("PIXLY-CORE-VAL-001: ...")
   ```

2. **提供清晰的错误消息**
   ```rust
   ✅ "Quality参数超出范围: 150 (应为1-100)"
   ```

3. **区分技术消息和用户消息**
   ```rust
   ✅ technical: "PIXLY-CORE-VAL-001: Invalid quality: 150"
   ✅ user: "参数设置有误，请检查质量值（应为1-100）"
   ```

4. **记录完整上下文**
   ```rust
   ✅ 包含: 输入值、期望值、文件位置
   ```

### DON'T ❌

1. **不要使用模糊错误消息**
   ```rust
   ❌ "Invalid parameter"
   ✅ "PIXLY-CORE-VAL-001: Quality参数超出范围"
   ```

2. **不要吞掉错误**
   ```rust
   ❌ catch(e) { /* 忽略 */ }
   ✅ catch(e) { logError(e); throw e; }
   ```

3. **不要使用默认值覆盖错误**
   ```rust
   ❌ if invalid { quality = 100; } // 静默修复
   ✅ if invalid { bail!("PIXLY-CORE-VAL-001"); } // 响亮报错
   ```

4. **不要混淆错误级别**
   ```rust
   ❌ 致命错误标记为WARNING
   ✅ 使用正确的严重级别
   ```

---

**统一错误码规范，让错误可追溯、易修复、用户友好！** 🚨
