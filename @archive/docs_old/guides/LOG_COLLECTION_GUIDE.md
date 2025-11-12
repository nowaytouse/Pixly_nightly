# 统一日志收集使用指南

## 🎯 概述

三端统一日志收集系统，支持 JS、Rust、Go 的 JSON 格式日志聚合和分析。

---

## 📊 日志格式

### 统一 JSON 格式

所有三端输出统一的 JSON 格式：

```json
{
  "timestamp": "2025-11-10T16:15:00.123Z",
  "level": "info",
  "module": "js.ui_handlers",
  "message": "User clicked convert button",
  "args": ["additional", "data"]
}
```

**字段说明**：
- `timestamp`: ISO 8601 时间戳
- `level`: 日志级别 (trace/debug/info/warn/error)
- `module`: 模块标识（含端标识，如 js./rust./go.）
- `message`: 日志消息
- `args`: 附加参数（可选）

---

## 🔧 启用 JSON 输出

### JS端

```javascript
// 浏览器控制台
window.pixlyLog.enableJSON();

// 或使用 localStorage
localStorage.setItem('pixly_log_json', 'true');

// 禁用
window.pixlyLog.disableJSON();
```

### Rust端

```bash
# 启用 JSON 输出
export PIXLY_LOG_JSON=1
export PIXLY_LOG_LEVEL=info

# 运行程序
pixly-rust convert input.jpg output.jxl
```

### Go端

```go
import "pixly/pkg/logging"

// 启用 JSON 输出
logging.InitLogging("info", true)  // true = JSON 模式

// 记录日志
logging.Info("Processing file", map[string]interface{}{
    "file": "photo.jpg",
    "size": 1024000,
})
```

---

## 📋 使用日志收集器

### 基本用法

```bash
# 开发环境 - 实时查看
tail -f /path/to/logs | node scripts/log-collector.js --dev

# 生产环境 - 保存到文件
node scripts/log-collector.js --prod --output logs/pixly.log < app.log

# 过滤日志级别
node scripts/log-collector.js --filter warn
```

### 参数说明

- `--dev`: 开发模式（美化输出，实时显示）
- `--prod`: 生产模式（JSON 格式，保存到文件）
- `--output <path>`: 输出文件路径
- `--filter <level>`: 最低日志级别（trace/debug/info/warn/error）

### 示例场景

#### 1. 开发调试

```bash
# 启动应用（JSON 模式）
PIXLY_LOG_JSON=1 pixly-rust convert test.jpg test.jxl 2>&1 | \
  node scripts/log-collector.js --dev --filter debug
```

#### 2. 生产环境监控

```bash
# 收集错误和警告
node scripts/log-collector.js --prod --filter warn \
  --output logs/errors-$(date +%Y%m%d).log < /var/log/pixly/app.log
```

#### 3. 跨端日志聚合

```bash
# 合并三端日志
cat js-logs.json rust-logs.json go-logs.json | \
  node scripts/log-collector.js --dev
```

---

## 🔍 日志分析

### 统计信息

日志收集器自动统计：
- 总日志数
- 按级别分组
- 按模块分组（Top 10）

示例输出：

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Log Statistics
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Total logs: 1543

By Level:
  info: 1200
  warn: 250
  error: 93

Top Modules:
  js.ui_handlers: 450
  rust.converter: 380
  go.ai_gateway: 320
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 手动分析

```bash
# 统计错误数
cat logs/pixly.log | jq 'select(.level=="error")' | wc -l

# 查找特定模块
cat logs/pixly.log | jq 'select(.module | contains("ai_client"))'

# 按时间过滤
cat logs/pixly.log | jq 'select(.timestamp > "2025-11-10T15:00:00Z")'

# 错误消息分组
cat logs/pixly.log | jq -r 'select(.level=="error") | .message' | sort | uniq -c
```

---

## 🚀 高级用法

### Request ID 追踪

跨端追踪单个请求：

```bash
# 记录日志时添加 request_id
logging.Info("Request started", map[string]interface{}{
    "request_id": "req_123456",
    "action": "convert",
})

# 查找所有相关日志
cat logs/pixly.log | jq 'select(.args[]? | contains("req_123456"))'
```

### 性能分析

```bash
# 提取耗时信息
cat logs/pixly.log | \
  jq 'select(.message | contains("duration"))' | \
  jq -r '[.timestamp, .message] | @tsv'
```

### 错误告警

```bash
# 监控错误日志
tail -f logs/pixly.log | \
  jq 'select(.level=="error")' | \
  while read line; do
    echo "$line" | mail -s "PIXLY Error Alert" admin@example.com
  done
```

---

## 📝 最佳实践

### 1. 日志级别使用

- **trace**: 极详细调试（函数调用、参数）
- **debug**: 开发调试（中间状态、流程）
- **info**: 关键业务事件（用户操作、转换完成）
- **warn**: 警告但不影响功能（性能问题、降级）
- **error**: 错误需要处理（转换失败、服务不可用）

### 2. 结构化日志

优先使用结构化字段而非字符串拼接：

```javascript
// ✅ 好
log.info('File converted', { 
    input: 'photo.jpg', 
    output: 'photo.webp', 
    duration_ms: 123 
});

// ❌ 差
log.info(`File photo.jpg converted to photo.webp in 123ms`);
```

### 3. 性能考虑

- 生产环境使用 `info` 级别
- 避免在循环中记录 `debug`/`trace`
- 使用日志采样（高频操作）

---

## 🐛 故障排查

### 日志未输出？

1. 检查日志级别：
   ```bash
   # JS
   window.pixlyLog.getLevel()
   
   # Rust
   echo $PIXLY_LOG_LEVEL
   
   # Go
   # 检查 logging.InitLogging() 调用
   ```

2. 验证 JSON 模式：
   ```bash
   # JS
   window.pixlyLog.isJSONEnabled()
   
   # Rust
   echo $PIXLY_LOG_JSON
   ```

### JSON 解析错误？

- 确保 `args` 可序列化（无循环引用）
- 检查特殊字符转义
- 使用 `jq` 验证 JSON 格式

### 性能问题？

- 降低日志级别
- 启用采样（仅记录部分日志）
- 使用异步日志写入

---

## 📚 相关文档

- `THREE_TIER_LOG_UNIFICATION_COMPLETE.md` - 完成报告
- `THREE_TIER_LOG_UNIFICATION_PLAN.md` - 原始规划
- Rust: `core/rust/src/logging.rs`
- Go: `core/go/pkg/logging/logging.go`
- JS: `core/plugin/js/plugin-modules/log-manager.js`

---

**Happy Logging!** 📊✨
