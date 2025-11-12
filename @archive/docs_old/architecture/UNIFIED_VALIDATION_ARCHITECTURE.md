# 🛡️ 统一验证层架构设计

> **目标**: 实现三端统一、透明、完善的验证处理机制  
> **原则**: 响亮报错 > 静默降级，杜绝沉默失败

## 🎯 核心原则

### 1. 响亮报错 (Loud Failure)
- ❌ **禁止**: 静默失败、默认值覆盖、自动降级
- ✅ **必须**: 立即报错、终止流程、明确提示

### 2. 分层验证 (Layered Validation)
```
Layer 1: 输入验证 (Input Validation)
Layer 2: 业务逻辑验证 (Business Logic Validation)  
Layer 3: 输出验证 (Output Validation)
Layer 4: 完整性验证 (Integrity Validation)
```

### 3. 错误透明 (Error Transparency)
- 错误来源追踪
- 错误传播链路
- 用户友好提示

## 🏗️ 统一验证层架构

### 架构图

```
┌─────────────────────────────────────────────────────┐
│                   用户输入 (User Input)               │
└────────────────────┬────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────┐
│             JS UI 验证层 (Layer 1 + 4)               │
│  ✓ UI参数范围验证                                     │
│  ✓ 文件选择验证                                       │
│  ✓ 格式兼容性预检                                     │
│  ✓ 参数快照捕获                                       │
│  ❌ 错误 → Eagle通知 + Console.error                  │
└────────────────────┬────────────────────────────────┘
                     │ HTTP API
                     ▼
┌─────────────────────────────────────────────────────┐
│            Rust Core 验证层 (Layer 2 + 3)            │
│  ✓ 输入文件验证 (Magika)                             │
│  ✓ 参数范围二次验证                                   │
│  ✓ AI返回值验证                                       │
│  ✓ 输出文件验证                                       │
│  ✓ 参数完整性回显                                     │
│  ❌ 错误 → HTTP 400/500 + JSON Error                  │
└────────────────────┬────────────────────────────────┘
                     │ gRPC
                     ▼
┌─────────────────────────────────────────────────────┐
│             Go AI 验证层 (Layer 2)                    │
│  ✓ 请求参数边界验证                                   │
│  ✓ 图像属性合理性检查                                 │
│  ✓ 选项值范围验证                                     │
│  ✓ 响应参数合理性保证                                 │
│  ❌ 错误 → gRPC Error + 详细message                   │
└─────────────────────────────────────────────────────┘
```

## 📋 验证层详细设计

### Layer 1: 输入验证 (Input Validation)

#### JS UI层
```javascript
class InputValidator {
    // 1. 文件选择验证
    validateFileSelection(files) {
        if (!files || files.length === 0) {
            throw new ValidationError('未选择任何文件', 'FILE_SELECTION_EMPTY');
        }
        // ... 更多验证
    }
    
    // 2. 参数范围验证
    validateParamRanges(params) {
        if (params.quality < 1 || params.quality > 100) {
            throw new ValidationError(
                `质量参数超出范围: ${params.quality} (应为1-100)`,
                'PARAM_QUALITY_OUT_OF_RANGE'
            );
        }
        // ... 更多验证
    }
    
    // 3. 格式兼容性验证
    validateFormatCompatibility(format, fileType) {
        if (format === 'heic' && fileType.hasAlpha) {
            throw new ValidationError(
                'HEIC格式不支持透明通道，将丢失透明度',
                'FORMAT_ALPHA_INCOMPATIBLE',
                'warning'  // 警告级别，不阻断
            );
        }
    }
}
```

#### Rust层
```rust
// FileValidator - 输入文件验证
pub struct FileValidator {
    level: ValidationLevel,
}

impl FileValidator {
    pub fn validate_input(&self, path: &str) -> Result<ValidationResult> {
        // 1. 文件存在性
        if !Path::new(path).exists() {
            bail!(ValidationError::FileNotFound(path.to_string()));
        }
        
        // 2. 文件可读性
        if !self.is_readable(path)? {
            bail!(ValidationError::FileNotReadable(path.to_string()));
        }
        
        // 3. Magika AI检测
        let detected = self.detect_with_magika(path)?;
        if !detected.is_safe {
            bail!(ValidationError::FileSafetyCheckFailed(path.to_string()));
        }
        
        Ok(ValidationResult::passed())
    }
}
```

#### Go AI层
```go
// ValidatePredictRequest - 验证预测请求
func ValidatePredictRequest(req *pb.PredictRequest) error {
    if req == nil {
        return fmt.Errorf("predict request is nil")
    }
    
    // 1. 验证图像信息
    if err := validateImageInfo(req.Image); err != nil {
        return fmt.Errorf("invalid image info: %w", err)
    }
    
    // 2. 验证工具名称
    validTools := []string{"cjxl", "avifenc", "cwebp", "ffmpeg"}
    if !contains(validTools, req.Tool) {
        return fmt.Errorf("invalid tool: %s, must be one of %v", req.Tool, validTools)
    }
    
    // 3. 验证选项
    if req.Options != nil {
        if err := validatePredictOptions(req.Options); err != nil {
            return fmt.Errorf("invalid options: %w", err)
        }
    }
    
    return nil
}

func validateImageInfo(img *pb.ImageInfo) error {
    if img == nil {
        return errors.New("image info is nil")
    }
    
    // 验证尺寸
    if img.Width < 1 || img.Width > 65535 {
        return fmt.Errorf("invalid width: %d (must be 1-65535)", img.Width)
    }
    if img.Height < 1 || img.Height > 65535 {
        return fmt.Errorf("invalid height: %d (must be 1-65535)", img.Height)
    }
    
    // 验证文件大小
    if img.Size <= 0 {
        return fmt.Errorf("invalid file size: %d (must be > 0)", img.Size)
    }
    
    // 验证格式
    validFormats := []string{"jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff"}
    if !contains(validFormats, strings.ToLower(img.Format)) {
        return fmt.Errorf("invalid format: %s", img.Format)
    }
    
    return nil
}

func validatePredictOptions(opts *pb.PredictOptions) error {
    if opts == nil {
        return nil
    }
    
    // SSIM阈值验证
    if opts.SsimThreshold < 0.0 || opts.SsimThreshold > 1.0 {
        return fmt.Errorf("invalid SSIM threshold: %f (must be 0.0-1.0)", opts.SsimThreshold)
    }
    
    // 迭代次数验证
    if opts.MaxIterations < 0 || opts.MaxIterations > 100 {
        return fmt.Errorf("invalid max iterations: %d (must be 0-100)", opts.MaxIterations)
    }
    
    return nil
}
```

### Layer 2: 业务逻辑验证

#### Rust层
```rust
// 参数组合验证
pub fn validate_param_combination(config: &ConversionConfig) -> Result<()> {
    // JPEG不支持无损
    if config.format == "jpeg" && config.lossless {
        bail!(ValidationError::IncompatibleParams {
            reason: "JPEG格式不支持无损模式".to_string(),
        });
    }
    
    // 无损模式质量应为100
    if config.lossless && config.quality < 100 {
        warn!("⚠️ 无损模式下质量参数应为100，当前为{}", config.quality);
    }
    
    Ok(())
}

// AI返回值验证（已实施）
fn validate_ai_response(response: &PredictionResponse) -> Result<()> {
    // 置信度检查
    if response.confidence < 0.5 {
        bail!(ValidationError::AiConfidenceTooLow {
            confidence: response.confidence,
            threshold: 0.5,
        });
    }
    
    // 参数范围检查
    if let Some(q) = response.quality {
        if q < 1 || q > 100 {
            bail!(ValidationError::AiInvalidQuality(q));
        }
    }
    
    Ok(())
}
```

### Layer 3: 输出验证

#### Rust层
```rust
pub struct OutputValidator;

impl OutputValidator {
    pub fn validate(&self, output_path: &str, expected: &OutputExpectation) -> Result<ValidationResult> {
        // 1. 文件存在性
        if !Path::new(output_path).exists() {
            bail!(ValidationError::OutputFileNotGenerated {
                path: output_path.to_string(),
            });
        }
        
        // 2. 文件非空
        let metadata = fs::metadata(output_path)?;
        if metadata.len() == 0 {
            bail!(ValidationError::OutputFileEmpty {
                path: output_path.to_string(),
            });
        }
        
        // 3. 格式正确性
        let detected_format = self.detect_format(output_path)?;
        if detected_format != expected.format {
            bail!(ValidationError::OutputFormatMismatch {
                expected: expected.format.clone(),
                actual: detected_format,
            });
        }
        
        // 4. 大小合理性
        let ratio = metadata.len() as f64 / expected.input_size as f64;
        if ratio < 0.01 || ratio > 10.0 {
            warn!("⚠️ 输出文件大小异常: {:.2}x", ratio);
        }
        
        Ok(ValidationResult::passed())
    }
}
```

### Layer 4: 完整性验证

#### JS层（已实施）
```javascript
class ParamIntegrityValidator {
    validateIntegrity(id, actualParams) {
        const snapshot = this.paramSnapshots.get(id);
        const diffs = this.compareParams(snapshot.ui_params, actualParams);
        
        // 格式不匹配 - 致命错误
        if (snapshot.ui_params.format !== actualParams.format) {
            throw new ValidationError(
                `格式不匹配: UI=${snapshot.ui_params.format}, Rust=${actualParams.format}`,
                'PARAM_FORMAT_MISMATCH'
            );
        }
        
        // 参数变化过大 - 警告
        if (Math.abs(diffs.quality_change) > 20) {
            this.warn(`质量参数变化过大: ${diffs.quality_change}`);
        }
        
        return { valid: true, diffs, warnings };
    }
}
```

## 🚨 统一错误处理机制

### 错误分类

```typescript
enum ErrorSeverity {
    FATAL = 'fatal',      // 致命错误，立即终止
    ERROR = 'error',      // 错误，终止当前操作
    WARNING = 'warning',  // 警告，记录但继续
    INFO = 'info'         // 信息提示
}

interface ValidationError {
    code: string;           // 错误码
    message: string;        // 错误消息
    severity: ErrorSeverity;
    source: string;         // 来源 (js/rust/go)
    details?: any;          // 详细信息
    stack?: string;         // 堆栈跟踪
}
```

### JS错误处理

```javascript
class ErrorHandler {
    handleValidationError(error) {
        const log = window.pixlyLog;
        
        switch (error.severity) {
            case 'fatal':
            case 'error':
                // 响亮报错
                log.error('Validation', `❌ ${error.message}`, error.details);
                
                // Eagle通知
                eagle.notification.show({
                    title: '❌ 验证失败',
                    description: error.message,
                    duration: 5000,
                    type: 'error'
                });
                
                // 终止流程
                throw error;
                
            case 'warning':
                // 警告提示
                log.warn('Validation', `⚠️ ${error.message}`, error.details);
                
                // Eagle通知（较短）
                eagle.notification.show({
                    title: '⚠️ 验证警告',
                    description: error.message,
                    duration: 3000,
                    type: 'warning'
                });
                break;
                
            case 'info':
                log.info('Validation', `ℹ️ ${error.message}`, error.details);
                break;
        }
    }
}
```

### Rust错误处理

```rust
// 统一错误类型
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("文件未找到: {0}")]
    FileNotFound(String),
    
    #[error("文件不可读: {0}")]
    FileNotReadable(String),
    
    #[error("AI置信度过低: {confidence:.1}% (阈值: {threshold:.1}%)")]
    AiConfidenceTooLow { confidence: f32, threshold: f32 },
    
    #[error("参数超出范围: {param}={value} (应为{min}-{max})")]
    ParamOutOfRange {
        param: String,
        value: String,
        min: String,
        max: String,
    },
    
    #[error("参数组合不兼容: {reason}")]
    IncompatibleParams { reason: String },
    
    #[error("输出文件未生成: {path}")]
    OutputFileNotGenerated { path: String },
    
    #[error("输出格式不匹配: 期望{expected}, 实际{actual}")]
    OutputFormatMismatch { expected: String, actual: String },
}

// 错误处理
impl ValidationError {
    pub fn log_and_return(&self) -> Result<()> {
        // 记录详细日志
        error!("❌ Validation Error: {}", self);
        
        // 如果有堆栈跟踪，记录
        if let Some(backtrace) = std::backtrace::Backtrace::capture().status() == std::backtrace::BacktraceStatus::Captured {
            error!("Backtrace: {:?}", backtrace);
        }
        
        // 返回错误，不吞噬
        Err(self.into())
    }
}
```

### Go错误处理

```go
// 统一错误类型
type ValidationError struct {
    Code     string                 `json:"code"`
    Message  string                 `json:"message"`
    Severity string                 `json:"severity"`
    Details  map[string]interface{} `json:"details,omitempty"`
}

func (e *ValidationError) Error() string {
    return fmt.Sprintf("[%s] %s", e.Code, e.Message)
}

// 错误包装
func WrapValidationError(err error, code string) error {
    return &ValidationError{
        Code:     code,
        Message:  err.Error(),
        Severity: "error",
        Details:  map[string]interface{}{"original": err.Error()},
    }
}

// gRPC错误返回
func (s *AIService) Predict(ctx context.Context, req *pb.PredictRequest) (*pb.PredictResponse, error) {
    // 验证请求
    if err := ValidatePredictRequest(req); err != nil {
        log.Printf("❌ [Validation] Request validation failed: %v", err)
        
        // 返回gRPC错误
        return nil, status.Errorf(
            codes.InvalidArgument,
            "请求参数验证失败: %v", err,
        )
    }
    
    // ... 处理
}
```

## 🔍 验证透明度

### 验证日志规范

```
层级: [Source] Action Result Details
示例:
✅ [JS UI] 参数验证通过 quality=85, speed=4
⚠️ [Rust] AI置信度中等 confidence=68%
❌ [Go AI] 请求验证失败 width=-100 (must be 1-65535)
```

### 验证报告

```javascript
class ValidationReport {
    generate() {
        return {
            timestamp: Date.now(),
            layers: {
                js_ui: this.jsResults,
                rust_core: this.rustResults,
                go_ai: this.goResults
            },
            total_errors: this.countErrors(),
            total_warnings: this.countWarnings(),
            validation_chain: this.getChain(),
            blocked: this.hasBlockingErrors()
        };
    }
}
```

## 📊 验证指标

### 监控指标
- 验证通过率
- 错误类型分布
- 警告触发频率
- 验证耗时统计

### 质量要求
- 错误捕获率: 100%
- 沉默失败率: 0%
- 用户可见错误: 100%
- 错误可追溯性: 100%

---

**核心价值**: 响亮报错、杜绝沉默失败、全链路透明验证
