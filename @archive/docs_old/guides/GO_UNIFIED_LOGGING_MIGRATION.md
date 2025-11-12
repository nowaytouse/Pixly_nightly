# 🔄 Go统一日志系统迁移指南

> **Phase 46.8**: 从旧的`pixly/pkg/logging`迁移到统一`ai.logging`  
> **目标**: 确保Go代码使用统一的三端日志系统

---

## 📋 迁移清单

### 需要迁移的文件

| 文件 | 状态 | 旧导入 | logging调用数 |
|-----|------|--------|--------------|
| `http_gateway.go` | ⏳ | `pixly/pkg/logging` | ~15 |
| `training_queue.go` | ⏳ | `pixly/pkg/logging` | ~20 |
| `video_handlers.go` | ⏳ | `pixly/pkg/logging` | ~10 |
| `ensemble/fusion.go` | ⏳ | `pixly/pkg/logging` | ~5 |
| 其他使用logging的文件 | ⏳ | - | - |

---

## 🔄 API变更对照表

### 1. 导入变更

#### 旧方式 ❌
```go
import "pixly/pkg/logging"
```

#### 新方式 ✅
```go
// logging在同一个ai包内，无需导入
// 直接使用: Info(), Error(), Warning(), Debug()
```

---

### 2. Info日志

#### 旧API ❌
```go
logging.Info("Starting conversion | Tool=%s | Quality=%d", tool, quality)
```

#### 新API ✅
```go
Info("Converter", "Starting conversion | Tool=%s | Quality=%d", tool, quality)
```

**说明**: 新增第一个参数`component`，用于标识日志来源组件

---

### 3. Info日志（带上下文）

#### 旧方式（无结构化上下文）❌
```go
logging.Info("Conversion complete | Duration=%dms | Size=%d", elapsed, size)
```

#### 新方式（结构化上下文）✅
```go
InfoWithContext("Converter", "Conversion complete", map[string]interface{}{
    "duration_ms": elapsed,
    "output_size": size,
})
```

**优势**: 
- 结构化数据便于查询
- JSON格式一致
- 支持复杂对象

---

### 4. Error日志

#### 旧API ❌
```go
logging.Info("❌ AI prediction failed: %v", err)
```

#### 新API ✅
```go
Error("Predictor", "AI prediction failed: %v", err)
```

**注意**: 
- 使用`Error()`而不是`Info()`
- emoji会自动添加（在人类可读模式）

---

### 5. Warning日志

#### 旧API ❌
```go
logging.Info("⚠️ AI confidence low: %f", confidence)
```

#### 新API ✅
```go
Warning("Validator", "AI confidence low: %f", confidence)
```

---

### 6. 验证日志（便捷函数）

#### 旧方式 ❌
```go
// http_validator.go中的旧函数
LogValidationError("PredictRequest", err)
LogValidationSuccess("PredictRequest")
LogValidationWarning("PredictRequest", "Low confidence")
```

#### 新方式 ✅
```go
// 使用logging.go中的统一函数
LogValidationError("PredictRequest", err)
LogValidationSuccess("PredictRequest")
LogValidationWarning("PredictRequest", "Low confidence", map[string]interface{}{
    "confidence": 0.45,
})
```

**说明**: 
- `LogValidationError()`已在logging.go中统一实现
- http_validator.go中的旧实现已删除
- 新API支持PixlyError自动解析

---

### 7. 性能监控

#### 旧方式（手动计时）❌
```go
startTime := time.Now()
// ...执行操作
elapsed := time.Since(startTime).Milliseconds()
logging.Info("Operation completed | Time=%dms", elapsed)
```

#### 新方式（性能日志辅助）✅
```go
perf := StartPerformanceLog("Converter", "convert image")
// ...执行操作
perf.End()  // 自动记录耗时

// 或带上下文
perf.EndWithContext(map[string]interface{}{
    "output_size": size,
    "compression_ratio": ratio,
})
```

**优势**:
- 自动计时
- 统一格式
- 减少样板代码

---

## 🔧 迁移步骤

### Step 1: 删除旧导入

```go
// 删除这行
import "pixly/pkg/logging"
```

### Step 2: 更新日志调用

使用全局搜索替换：

```bash
# 查找所有logging.Info调用
grep -rn "logging\.Info" core/go/ai/

# 手动检查并更新每个调用
# 添加component参数作为第一个参数
```

### Step 3: 分类处理

**Info类日志**:
```go
// 旧: logging.Info("message", args...)
// 新: Info("Component", "message", args...)
```

**Error类日志**:
```go
// 旧: logging.Info("❌ error: %v", err)
// 新: Error("Component", "error: %v", err)
```

**Warning类日志**:
```go
// 旧: logging.Info("⚠️ warning: %s", msg)
// 新: Warning("Component", "warning: %s", msg)
```

### Step 4: 使用结构化上下文

对于需要多个参数的日志，优先使用`InfoWithContext`:

```go
// 旧方式
logging.Info("Result | Q=%d d=%.1f conf=%.2f time=%dms",
    quality, distance, confidence, elapsed)

// 新方式
InfoWithContext("Predictor", "Prediction completed", map[string]interface{}{
    "quality": quality,
    "distance": distance,
    "confidence": confidence,
    "duration_ms": elapsed,
})
```

---

## ⚠️ 常见陷阱

### 1. 旧API签名问题

**错误**:
```go
logging.Info("message", arg1, arg2)  // 旧API已不存在
```

**原因**: 新的统一logging在ai包内，不再是外部logging包

**解决**:
```go
Info("Component", "message", arg1, arg2)  // 新API
```

### 2. 类型转换问题

**错误**:
```go
// logging.Info调用参数类型不对
cannot use modelType (variable of string type ModelType) as map[string]interface{}
```

**原因**: 旧代码将变量直接传入，新API期望format string或context map

**解决**:
```go
// 方式1: 格式化字符串
Info("Queue", "Processing model | Type=%s", string(modelType))

// 方式2: 结构化上下文
InfoWithContext("Queue", "Processing model", map[string]interface{}{
    "model_type": string(modelType),
})
```

### 3. emoji emoji重复

**错误**:
```go
Error("Component", "❌ Error occurred")  // emoji会重复
```

**原因**: 新的logging.go在人类可读模式下自动添加emoji

**解决**:
```go
Error("Component", "Error occurred")  // 移除手动emoji
```

---

## ✅ 迁移验证

### 编译检查

```bash
cd core/go/ai
go build ./...
```

### 运行时验证

1. 启动服务
2. 触发各种操作
3. 检查日志输出格式

**期望的JSON格式**:
```json
{
  "timestamp": "2025-11-11T00:10:00.123Z",
  "level": "INFO",
  "layer": "go-ai",
  "component": "Predictor",
  "message": "Prediction completed",
  "context": {
    "quality": 90,
    "confidence": 0.85
  }
}
```

**期望的人类可读格式**:
```
2025-11-11T00:10:00.123Z [INFO] ℹ️  Predictor: Prediction completed | Context: {"quality":90,"confidence":0.85}
```

---

## 📊 迁移进度

| 类别 | 待迁移 | 已迁移 | 状态 |
|------|--------|--------|------|
| **导入删除** | 4文件 | 1文件 | ⏳ 25% |
| **Info调用** | ~50处 | ~2处 | ⏳ 4% |
| **Error调用** | ~20处 | 0处 | ⏳ 0% |
| **验证日志** | ~10处 | 已统一 | ✅ 100% |

---

## 🎯 优先级

### P0 (立即迁移)
- ✅ `http_validator.go` - LogValidation函数已统一
- ⏳ `http_gateway.go` - 核心HTTP服务
- ⏳ `python_bridge.go` - AI桥接

### P1 (尽快迁移)
- ⏳ `training_queue.go` - 训练队列
- ⏳ `video_handlers.go` - 视频处理

### P2 (后续迁移)
- ⏳ 其他辅助文件

---

## 💡 最佳实践

### 1. 选择合适的组件名

```go
// 好的组件名（清晰、一致）
Info("HTTPGateway", "Server started")
Info("Predictor", "Model loaded")
Info("Validator", "Request validated")

// 不好的组件名（过于笼统或冗长）
Info("Main", "...")              // 太笼统
Info("HTTPGatewayPrediction", "...")  // 太长
```

### 2. 结构化 vs 格式化

**使用格式化字符串**（简单场景）:
```go
Info("Converter", "Processing file: %s", filename)
```

**使用结构化上下文**（复杂数据）:
```go
InfoWithContext("Converter", "Processing completed", map[string]interface{}{
    "input_file": inputPath,
    "output_file": outputPath,
    "duration_ms": elapsed,
    "output_size": size,
    "compression_ratio": ratio,
})
```

### 3. 错误日志处理

**普通错误**:
```go
Error("Component", "Operation failed: %v", err)
```

**PixlyError**:
```go
pixlyErr := NewValidationError(ErrValInvalidRange, "Quality out of range")
LogPixlyError("Validator", pixlyErr)  // 自动包含错误码和上下文
```

---

**迁移负责人**: AI开发团队  
**预计完成时间**: Phase 46.9  
**状态追踪**: 本文档实时更新
