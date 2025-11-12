# 🔧 PIXLY跨端统一日志管理规范

## 概述

统一PIXLY三层架构的日志管理系统：
- **JS插件层**：用户界面和交互逻辑
- **GO AI层**：智能决策和AI预测
- **Rust执行层**：高性能文件处理

---

## 统一日志级别定义

### 标准五级分类

| 级别 | 数值 | 用途 | 去重 | 示例场景 |
|------|------|------|------|----------|
| **ERROR** | 0 | 严重错误，需要立即处理 | ❌ 否 | 文件损坏、核心崩溃、API失败 |
| **WARN** | 1 | 警告信息，可能影响功能 | ❌ 否 | 依赖缺失、配置问题、性能警告 |
| **INFO** | 2 | 重要操作和状态变化 | ✅ 是 | 文件转换、模式切换、服务启动 |
| **DEBUG** | 3 | 调试信息和内部状态 | ✅ 是 | UI事件、参数变化、状态更新 |
| **TRACE** | 4 | 详细追踪信息 | ✅ 是 | 函数调用、数据流、细节日志 |

### 跨端映射规则

```
JavaScript (log-manager.js)  →  GO (zerolog)      →  Rust (tracing)
---------------------------------------------------------------------
ERROR (0)                    →  ErrorLevel        →  error!()
WARN (1)                     →  WarnLevel         →  warn!()
INFO (2)                     →  InfoLevel         →  info!()
DEBUG (3)                    →  DebugLevel        →  debug!()
TRACE (4)                    →  TraceLevel        →  trace!()
```

---

## JavaScript插件端（当前实现）

### 核心组件

**log-manager.js**：
- ✅ 五级日志系统
- ✅ 日志去重（1秒窗口）
- ✅ 日志节流（500ms）
- ✅ 参数化消息（formatLog）
- ✅ ERROR/WARN不去重
- ✅ 性能追踪（perf）

**log-constants.js**：
- ✅ 200+统一消息常量
- ✅ 参数化模板：`{param}`
- ✅ 分类管理（按模块）

### 使用规范

```javascript
// 1. 导入LOG常量
const LOG = window.LOG;
const log = window.pixlyLog || console;

// 2. 基础日志
log.info('Module', LOG.OPERATION_START);

// 3. 参数化日志
log.info('Module', LOG.FILE_PROCESSED, { count: 5, time: 123 });

// 4. 性能追踪
log.perf.start('operation');
// ... do work ...
log.perf.end('operation'); // 输出: ⏱️ operation: 123.45ms
```

---

## Rust内核端（需要实现）

### 建议架构

**使用tracing crate**：
```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }
```

### 日志结构规范

```rust
use tracing::{error, warn, info, debug, trace};

// 1. 基础日志
info!("File processing started");

// 2. 结构化字段
info!(
    file_count = 5,
    total_size = 1024000,
    "Batch processing started"
);

// 3. Span追踪（性能）
let span = info_span!("image_conversion", format = "webp");
let _guard = span.enter();
// ... do work ...
```

### JSON输出格式

```json
{
  "timestamp": "2025-11-10T09:54:00.123Z",
  "level": "INFO",
  "target": "pixly_rust::converter",
  "message": "File converted successfully",
  "fields": {
    "input": "image.jpg",
    "output": "image.webp",
    "duration_ms": 123
  }
}
```

### 与JS插件通信

**方式1：标准输出JSON**
```rust
// Rust输出JSON日志
tracing_subscriber::fmt()
    .json()
    .with_target(true)
    .with_level(true)
    .init();

// JS插件解析
rustCLI.on('stdout', (data) => {
    try {
        const logEntry = JSON.parse(data);
        if (logEntry.level === 'ERROR') {
            pixlyLog.error('Rust', logEntry.message, logEntry.fields);
        }
    } catch (e) {
        // 非JSON输出，原样显示
    }
});
```

**方式2：IPC消息**
```rust
// Rust通过stdin/stdout与JS通信
#[derive(Serialize)]
struct LogMessage {
    level: String,
    module: String,
    message: String,
    params: HashMap<String, Value>,
}
```

---

## GO AI服务端（需要实现）

### 建议架构

**使用zerolog**：
```go
package main

import (
    "github.com/rs/zerolog"
    "github.com/rs/zerolog/log"
)

func init() {
    // JSON输出
    zerolog.TimeFieldFormat = zerolog.TimeFormatUnix
    log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stdout})
}
```

### 日志规范

```go
// 1. 基础日志
log.Info().Msg("AI service started")

// 2. 结构化字段
log.Info().
    Int("port", 50052).
    Str("version", "4.2.0").
    Msg("gRPC server listening")

// 3. 错误日志
log.Error().
    Err(err).
    Str("file", filename).
    Msg("Failed to process image")

// 4. 性能追踪
start := time.Now()
// ... do work ...
log.Debug().
    Dur("duration", time.Since(start)).
    Msg("AI prediction completed")
```

### HTTP日志API（可选）

```go
// GET /api/v1/logs?level=INFO&limit=100
func LogsHandler(w http.ResponseWriter, r *http.Request) {
    // 返回最近的日志
}

// WebSocket实时日志流
func LogStreamHandler(ws *websocket.Conn) {
    // 推送实时日志到JS插件
}
```

---

## 统一日志收集器（建议实现）

### 架构设计

```
┌─────────────┐
│  JS插件层   │  → log-manager.js
└─────┬───────┘
      │
      ↓ (存储到内存/文件)
┌─────────────────────────────┐
│   统一日志收集器             │
│   LogCollector              │
│   - 接收三端日志            │
│   - 统一格式化              │
│   - 持久化存储              │
│   - 搜索过滤                │
│   - 导出功能                │
└─────┬───────────────────────┘
      │
      ├──→ Rust (JSON stdout)
      └──→ GO (HTTP API / WebSocket)
```

### JS实现示例

```javascript
// log-collector.js
class LogCollector {
    constructor() {
        this.logs = [];
        this.maxLogs = 10000;
        this.subscribers = new Set();
    }

    // 收集JS日志
    collectJS(level, module, message, params) {
        this.addLog({
            timestamp: Date.now(),
            source: 'JS',
            level,
            module,
            message,
            params
        });
    }

    // 收集Rust日志
    collectRust(jsonLog) {
        this.addLog({
            timestamp: Date.now(),
            source: 'Rust',
            level: jsonLog.level,
            module: jsonLog.target,
            message: jsonLog.message,
            params: jsonLog.fields
        });
    }

    // 收集GO日志
    collectGO(jsonLog) {
        this.addLog({
            timestamp: jsonLog.time,
            source: 'GO',
            level: jsonLog.level,
            module: jsonLog.module || 'GO',
            message: jsonLog.message,
            params: jsonLog
        });
    }

    addLog(entry) {
        this.logs.push(entry);
        if (this.logs.length > this.maxLogs) {
            this.logs.shift();
        }
        this.notify(entry);
    }

    // 实时通知
    notify(entry) {
        this.subscribers.forEach(fn => fn(entry));
    }

    // 查询日志
    query(filters) {
        return this.logs.filter(log => {
            if (filters.level && log.level !== filters.level) return false;
            if (filters.source && log.source !== filters.source) return false;
            if (filters.module && !log.module.includes(filters.module)) return false;
            return true;
        });
    }

    // 导出日志
    export(format = 'json') {
        if (format === 'json') {
            return JSON.stringify(this.logs, null, 2);
        } else if (format === 'csv') {
            // CSV格式
        }
    }
}

window.logCollector = new LogCollector();
```

---

## 统一日志格式

### 标准日志条目

```typescript
interface LogEntry {
    timestamp: number | string;      // Unix时间戳或ISO字符串
    source: 'JS' | 'Rust' | 'GO';   // 日志来源
    level: 'ERROR' | 'WARN' | 'INFO' | 'DEBUG' | 'TRACE';
    module: string;                  // 模块名称
    message: string;                 // 消息内容
    params?: Record<string, any>;    // 参数对象
    spanId?: string;                 // 追踪ID（可选）
    error?: {                        // 错误详情（可选）
        name: string;
        message: string;
        stack?: string;
    };
}
```

### JSON示例

```json
{
    "timestamp": 1699600440123,
    "source": "Rust",
    "level": "INFO",
    "module": "pixly_rust::image_converter",
    "message": "Image converted successfully",
    "params": {
        "input": "photo.jpg",
        "output": "photo.webp",
        "format": "webp",
        "quality": 85,
        "duration_ms": 234
    }
}
```

---

## 实施计划

### Phase 1：JS插件端（已完成✅）
- [x] log-manager.js实现
- [x] log-constants.js建立
- [x] 核心文件迁移（file-handler, theme）
- [x] ui-handlers部分迁移

### Phase 2：跨端基础（当前）
- [ ] 创建log-collector.js
- [ ] 实现Rust日志JSON输出
- [ ] 实现GO日志HTTP API
- [ ] 建立统一日志格式

### Phase 3：Rust内核端
- [ ] 集成tracing crate
- [ ] 定义LOG常量（类似JS）
- [ ] JSON结构化输出
- [ ] 与JS插件通信

### Phase 4：GO服务端
- [ ] 集成zerolog
- [ ] HTTP日志API
- [ ] WebSocket实时流（可选）
- [ ] 与JS插件通信

### Phase 5：统一管理
- [ ] LogCollector完整实现
- [ ] 日志搜索过滤
- [ ] 日志导出功能
- [ ] 调试面板（可选）

---

## 配置管理

### 统一配置文件

**config/logging.json**：
```json
{
    "global": {
        "defaultLevel": "INFO",
        "maxLogs": 10000,
        "enableDedup": true,
        "dedupWindow": 1000,
        "enableThrottle": true,
        "throttleInterval": 500
    },
    "sources": {
        "JS": {
            "level": "DEBUG",
            "modules": {
                "PIXLY Theme": "INFO",
                "PIXLY File": "DEBUG"
            }
        },
        "Rust": {
            "level": "INFO",
            "targets": {
                "pixly_rust::converter": "INFO",
                "pixly_rust::optimizer": "DEBUG"
            }
        },
        "GO": {
            "level": "INFO",
            "modules": {
                "ai_service": "INFO",
                "grpc_server": "DEBUG"
            }
        }
    },
    "output": {
        "console": true,
        "file": {
            "enabled": false,
            "path": "logs/pixly.log",
            "maxSize": "10MB",
            "maxBackups": 5
        }
    }
}
```

---

## 使用示例

### 场景1：文件转换全链路日志

```javascript
// JS插件发起
pixlyLog.info('PIXLY File', LOG.FILE_CONVERSION_START, { 
    file: 'photo.jpg', 
    format: 'webp' 
});

// ↓ 调用Rust CLI
```

```rust
// Rust处理
info!(
    input = "photo.jpg",
    output = "photo.webp",
    format = "webp",
    "Starting image conversion"
);

// ... 转换过程 ...

info!(
    duration_ms = 234,
    size_before = 1024000,
    size_after = 512000,
    "Conversion completed"
);
```

```javascript
// JS插件接收结果
pixlyLog.info('PIXLY File', LOG.FILE_CONVERSION_COMPLETE, {
    file: 'photo.webp',
    duration: 234,
    compression: '50%'
});
```

### 场景2：AI预测全链路

```javascript
// JS请求AI
pixlyLog.info('PIXLY AI', LOG.AI_PREDICTION_REQUEST, {
    file: 'photo.jpg',
    task: 'quality_predict'
});
```

```go
// GO处理
log.Info().
    Str("file", "photo.jpg").
    Str("task", "quality_predict").
    Msg("AI prediction started")

// ... 模型推理 ...

log.Info().
    Int("predicted_quality", 85).
    Float64("confidence", 0.92).
    Dur("duration", time.Since(start)).
    Msg("Prediction completed")
```

```javascript
// JS接收结果
pixlyLog.info('PIXLY AI', LOG.AI_PREDICTION_COMPLETE, {
    quality: 85,
    confidence: 0.92,
    duration: 123
});
```

---

## 调试面板（可选功能）

### UI设计

```
┌─────────────────────────────────────────┐
│  🔍 PIXLY统一日志查看器                 │
├─────────────────────────────────────────┤
│ 来源: [JS] [Rust] [GO]  级别: [INFO▼] │
│ 模块: [All▼]  搜索: [_____________] 🔎 │
├─────────────────────────────────────────┤
│ 09:54:12.123 | JS   | INFO  | PIXLY File   │
│   File selected: photo.jpg (1.2MB)        │
├─────────────────────────────────────────┤
│ 09:54:12.234 | Rust | INFO  | converter    │
│   Image conversion started                │
│   { input: "photo.jpg", format: "webp" }  │
├─────────────────────────────────────────┤
│ 09:54:12.468 | Rust | INFO  | converter    │
│   Conversion completed (234ms)            │
├─────────────────────────────────────────┤
│ [导出JSON] [导出CSV] [清空] [自动滚动✓] │
└─────────────────────────────────────────┘
```

---

## 性能考虑

1. **日志去重**：避免重复输出（JS已实现）
2. **日志节流**：高频日志限流（JS已实现）
3. **循环缓冲**：限制内存占用（最多10000条）
4. **异步写入**：日志不阻塞主流程
5. **条件编译**：生产环境可关闭DEBUG/TRACE

---

## 总结

### 当前状态
- ✅ JS插件端：完善的日志系统
- ⏳ Rust内核端：待实现统一规范
- ⏳ GO服务端：待实现统一规范

### 核心优势
- 🎯 **统一标准**：五级分类跨端一致
- 📊 **结构化**：JSON格式，易于解析
- 🔍 **可追溯**：全链路日志关联
- ⚡ **高性能**：去重节流优化
- 🔧 **易调试**：统一收集器查看

### 下一步
1. 继续完成JS端迁移（当前30%）
2. 实现Rust端统一日志
3. 实现GO端统一日志
4. 建立跨端日志收集器

**质量第一，稳步推进！** 🚀
