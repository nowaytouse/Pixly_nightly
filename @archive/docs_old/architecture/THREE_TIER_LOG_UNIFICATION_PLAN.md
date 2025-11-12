# 三端日志统一方案

**Date**: 2025-11-10 15:25  
**Status**: 📋 **规划阶段**  
**Priority**: 🔥 **高优先级**

---

## 🎯 目标

统一 Rust、Go、JS 三端的日志系统，实现：
1. **统一格式**：JSON 结构化日志
2. **统一级别**：trace/debug/info/warn/error
3. **统一输出**：支持多目标（控制台/文件/网络）
4. **统一上下文**：request_id、module、timestamp

---

## 📊 当前状态

### 三端日志情况
| 端 | 日志库 | 调用数 | 状态 |
|----|--------|--------|------|
| 🦀 Rust | `log` crate | 408 | 需要统一格式 |
| 🐹 Go | `log` package | 99 | 需要统一格式 |
| 🌐 JS | `pixlyLog` | 934 | ✅ 已统一 |

### 现有问题
1. **格式不一致**：
   - Rust: 自定义格式
   - Go: 标准库 log
   - JS: pixlyLog 格式化

2. **级别不统一**：
   - Rust: trace/debug/info/warn/error
   - Go: Print/Fatal/Panic
   - JS: trace/debug/info/warn/error

3. **无法聚合**：
   - 三端日志分散
   - 难以追踪跨端请求
   - 缺少统一视图

---

## 🏗️ 统一架构设计

### 核心原则
1. **最小侵入**：不改变现有日志调用方式
2. **格式统一**：JSON 结构化输出
3. **上下文传递**：request_id 跨端追踪
4. **性能优先**：零拷贝、异步输出

### 统一日志格式（JSON）

```json
{
  "timestamp": "2025-11-10T15:25:00.123Z",
  "level": "info",
  "module": "rust.media_analyzer",
  "message": "Video conversion started",
  "context": {
    "request_id": "req_123456",
    "file": "input.mp4",
    "format": "jxl"
  },
  "metadata": {
    "service": "rust-core",
    "version": "1.0.0",
    "host": "localhost"
  }
}
```

### 字段说明
- **timestamp**: ISO 8601 格式
- **level**: trace/debug/info/warn/error（统一5级）
- **module**: `<语言>.<模块名>`（如 `rust.media_analyzer`）
- **message**: 主日志消息
- **context**: 业务上下文（request_id、参数等）
- **metadata**: 系统元信息（service、version、host）

---

## 🔧 实施方案

### Phase 1: Rust 日志统一

#### 1.1 引入结构化日志库
```toml
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }
serde_json = "1.0"
```

#### 1.2 初始化日志系统
```rust
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;

fn init_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .json()
            .with_span_events(FmtSpan::CLOSE))
        .init();
}
```

#### 1.3 迁移现有日志
```rust
// 旧方式
log::info!("Video conversion started");

// 新方式（结构化）
tracing::info!(
    module = "rust.media_analyzer",
    request_id = %request_id,
    file = %input_path,
    "Video conversion started"
);
```

### Phase 2: Go 日志统一

#### 2.1 引入 zap 日志库
```go
// go.mod
require (
    go.uber.org/zap v1.24.0
)
```

#### 2.2 初始化日志系统
```go
import "go.uber.org/zap"

func initLogging() *zap.Logger {
    config := zap.NewProductionConfig()
    config.OutputPaths = []string{"stdout"}
    logger, _ := config.Build()
    return logger
}
```

#### 2.3 迁移现有日志
```go
// 旧方式
log.Printf("AI prediction started: %s", filePath)

// 新方式（结构化）
logger.Info("AI prediction started",
    zap.String("module", "go.ai_service"),
    zap.String("request_id", requestID),
    zap.String("file", filePath),
)
```

### Phase 3: JS 日志增强

#### 3.1 增强 pixlyLog 输出
```javascript
// log-manager.js 增强
const pixlyLog = {
    _format(level, module, message, context) {
        return JSON.stringify({
            timestamp: new Date().toISOString(),
            level,
            module: `js.${module}`,
            message,
            context,
            metadata: {
                service: 'js-ui',
                version: window.PIXLY?.VERSION || 'unknown'
            }
        });
    },
    
    info(module, message, context = {}) {
        const log = this._format('info', module, message, context);
        console.log(log);
        this._send(log);
    }
};
```

#### 3.2 添加日志收集
```javascript
// 发送到统一日志服务
_send(logEntry) {
    if (window.pixlyLogCollector) {
        window.pixlyLogCollector.collect(logEntry);
    }
}
```

---

## 🌐 日志聚合方案

### 选项1: 文件聚合
```bash
# 三端输出到同一目录
rust-core.log    → /var/log/pixly/
go-ai.log        → /var/log/pixly/
js-ui.log        → /var/log/pixly/

# 使用 tail 实时聚合
tail -f /var/log/pixly/*.log | jq '.'
```

### 选项2: HTTP 收集器
```
┌─────────┐
│ Rust    │──┐
└─────────┘  │
             ├──► HTTP Log Collector ──► 统一日志文件
┌─────────┐  │         (Go)
│ Go      │──┤
└─────────┘  │
             │
┌─────────┐  │
│ JS      │──┘
└─────────┘
```

### 选项3: 标准输出 + Docker
```yaml
# docker-compose.yml
services:
  pixly-rust:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        
  pixly-go:
    logging:
      driver: "json-file"
      
  # 自动聚合所有容器日志
```

---

## 📋 实施步骤

### Step 1: 设计验证（1天）
- [ ] 确认统一日志格式
- [ ] 选择日志聚合方案
- [ ] 设计 request_id 传递机制

### Step 2: Rust 端实施（2-3天）
- [ ] 引入 tracing 库
- [ ] 配置 JSON 输出
- [ ] 迁移现有 408 个日志调用
- [ ] 测试验证

### Step 3: Go 端实施（1-2天）
- [ ] 引入 zap 库
- [ ] 配置 JSON 输出
- [ ] 迁移现有 99 个日志调用
- [ ] 测试验证

### Step 4: JS 端增强（1天）
- [ ] 增强 pixlyLog 格式化
- [ ] 添加 JSON 输出
- [ ] 实现日志收集器
- [ ] 测试验证

### Step 5: 日志聚合（1天）
- [ ] 实现统一收集器
- [ ] 配置输出目标
- [ ] 实现日志查询工具
- [ ] 文档编写

**总预估**: 5-7 天

---

## 🎯 成功指标

### 技术指标
- ✅ 100% JSON 格式输出
- ✅ 统一 5 级日志（trace/debug/info/warn/error）
- ✅ request_id 跨端追踪
- ✅ 性能影响 < 5%

### 体验指标
- ✅ 单一命令查看所有日志
- ✅ 按 request_id 过滤
- ✅ 按 level 过滤
- ✅ 按 module 过滤

---

## 🔍 示例：跨端日志追踪

### 用户操作：转换视频
```bash
# 查看完整流程日志（按 request_id）
cat /var/log/pixly/*.log | jq 'select(.context.request_id == "req_123456")'
```

### 输出示例
```json
// 1. JS 发起请求
{
  "timestamp": "2025-11-10T15:25:00.100Z",
  "level": "info",
  "module": "js.ui_handlers",
  "message": "User clicked convert button",
  "context": { "request_id": "req_123456", "files": 1 }
}

// 2. Go AI 预测
{
  "timestamp": "2025-11-10T15:25:00.150Z",
  "level": "info",
  "module": "go.ai_service",
  "message": "AI prediction started",
  "context": { "request_id": "req_123456", "file": "video.mp4" }
}

// 3. Rust 执行转换
{
  "timestamp": "2025-11-10T15:25:00.200Z",
  "level": "info",
  "module": "rust.media_analyzer",
  "message": "Video conversion started",
  "context": { "request_id": "req_123456", "format": "jxl" }
}
```

---

## 💡 最佳实践

### 日志级别使用
- **trace**: 极详细调试（仅开发）
- **debug**: 调试信息（开发+测试）
- **info**: 关键业务事件（生产）
- **warn**: 警告但不影响功能
- **error**: 错误需要处理

### 上下文传递
```rust
// Rust: 通过 HTTP Header 传递
headers.insert("X-Request-ID", request_id.clone());

// Go: 从 Header 读取
requestID := r.Header.Get("X-Request-ID")

// JS: 生成并传递
const requestId = `req_${Date.now()}_${Math.random()}`;
fetch(url, { headers: { 'X-Request-ID': requestId } });
```

---

## 🚧 注意事项

### 性能考虑
1. **异步输出**: 日志写入不阻塞主流程
2. **批量发送**: 攒批发送减少 I/O
3. **日志采样**: 高频日志可采样

### 兼容性
1. **保留原有接口**: 不破坏现有代码
2. **逐步迁移**: 分模块逐步替换
3. **降级方案**: 日志系统失败不影响主功能

### 隐私安全
1. **敏感信息脱敏**: 密码、token 等
2. **日志轮转**: 防止磁盘占满
3. **访问控制**: 日志查看权限

---

## 📚 参考资料

### Rust
- [tracing](https://docs.rs/tracing/)
- [tracing-subscriber](https://docs.rs/tracing-subscriber/)

### Go
- [zap](https://github.com/uber-go/zap)
- [zerolog](https://github.com/rs/zerolog)

### 日志聚合
- [Loki](https://grafana.com/oss/loki/)
- [Fluent Bit](https://fluentbit.io/)
- [Vector](https://vector.dev/)

---

## 🎯 下一步行动

### 立即行动
1. **确认方案**: 与团队讨论统一格式
2. **选择工具**: 确定 Rust/Go 日志库
3. **设计 API**: request_id 传递机制

### 准备工作
1. **环境搭建**: 安装依赖库
2. **原型验证**: 小范围测试
3. **性能测试**: 确保无性能影响

---

**准备开始三端日志统一！** 🚀

质量 > 速度 | 真实性 > 演示 | 响亮报错 > 静默降级
